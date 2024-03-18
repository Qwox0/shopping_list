use super::BarcodeError;
use crate::util::DecLen;
use serde::{Deserialize, Serialize};
use std::num::NonZeroI64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Barcode {
    digits: NonZeroI64,
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::Sqlite> for Barcode {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Sqlite>>::type_info()
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Encode<'_, sqlx::Sqlite> for Barcode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        let int = self.digits.get() as i64;
        <i64 as sqlx::Encode<'_, sqlx::Sqlite>>::encode_by_ref(&int, buf)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BarcodeType {
    Ean13,
    Ean8,
}

impl Barcode {
    pub fn ean13(digits: u64) -> Result<Barcode, BarcodeError> {
        let dec_len = digits.dec_len();
        if dec_len != 13 {
            return Err(BarcodeError::InvalidBarcodeLength { expected: 13, got: dec_len });
        }
        let digits =
            NonZeroI64::new(digits.try_into()?).ok_or(BarcodeError::InvalidBarcode(digits))?;
        // TODO: more checks?
        Ok(Barcode { digits })
    }

    pub fn ean8(digits: u64) -> Result<Barcode, BarcodeError> {
        let dec_len = digits.dec_len();
        if dec_len != 8 {
            return Err(BarcodeError::InvalidBarcodeLength { expected: 8, got: dec_len });
        }
        let digits =
            NonZeroI64::new(digits.try_into()?).ok_or(BarcodeError::InvalidBarcode(digits))?;
        // TODO: more checks?
        Ok(Barcode { digits })
    }

    pub fn get_digits(self) -> i64 {
        self.digits.get()
    }

    pub fn get_type(self) -> BarcodeType {
        match (self.digits.get() as u64).dec_len() {
            8 => BarcodeType::Ean8,
            13 => BarcodeType::Ean13,
            _ => unreachable!(),
        }
    }
}

impl From<u64> for Barcode {
    /// # Panics
    ///
    /// Panics if the `digits` aren't a valid Barcode
    fn from(digits: u64) -> Self {
        match digits.dec_len() {
            8 => Barcode::ean8(digits).unwrap(),
            13 => Barcode::ean13(digits).unwrap(),
            len => panic!("invalid number of digits: {}", len),
        }
    }
}

impl From<i64> for Barcode {
    /// # Panics
    ///
    /// Panics if the `digits` aren't a valid Barcode
    fn from(digits: i64) -> Self {
        Barcode::from(digits as u64)
    }
}

impl TryFrom<rxing_wasm::BarcodeResult> for Barcode {
    type Error = BarcodeError;

    fn try_from(value: rxing_wasm::BarcodeResult) -> Result<Self, Self::Error> {
        let digits = value.text().parse().map_err(BarcodeError::ParseBarcodeErr);
        match value.format() {
            rxing_wasm::BarcodeFormat::Ean8 => digits.and_then(Barcode::ean8),
            rxing_wasm::BarcodeFormat::Ean13 => digits.and_then(Barcode::ean13),
            f => Err(BarcodeError::UnsupportedBarcodeType(format!("{:?}", f))),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionBarcode(pub Option<Barcode>);

impl OptionBarcode {
    pub fn some(barcode: Barcode) -> OptionBarcode {
        OptionBarcode(Some(barcode))
    }

    pub fn none() -> OptionBarcode {
        OptionBarcode(None)
    }
}

impl<T: Into<Barcode>> From<Option<T>> for OptionBarcode {
    fn from(value: Option<T>) -> Self {
        OptionBarcode(value.map(Into::into))
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Type<sqlx::Sqlite> for OptionBarcode {
    fn type_info() -> <sqlx::Sqlite as sqlx::Database>::TypeInfo {
        <i64 as sqlx::Type<sqlx::Sqlite>>::type_info()
    }

    fn compatible(ty: &<sqlx::Sqlite as sqlx::Database>::TypeInfo) -> bool {
        <i64 as sqlx::Type<sqlx::Sqlite>>::compatible(ty)
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Encode<'_, sqlx::Sqlite> for OptionBarcode {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Sqlite as sqlx::database::HasArguments<'_>>::ArgumentBuffer,
    ) -> sqlx::encode::IsNull {
        self.0.encode_by_ref(buf)
    }
}

#[cfg(feature = "ssr")]
impl sqlx::Decode<'_, sqlx::Sqlite> for OptionBarcode {
    fn decode(
        value: <sqlx::Sqlite as sqlx::database::HasValueRef<'_>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let int = <i64 as sqlx::Decode<'_, sqlx::Sqlite>>::decode(value)?;
        Ok(OptionBarcode(NonZeroI64::new(int).map(|digits| Barcode { digits })))
    }
}
