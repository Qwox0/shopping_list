use crate::util::DecLen;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Barcode {
    digits: u64,
}

pub enum BarcodeType {
    Ean13,
    Ean8,
}

impl Barcode {
    pub fn ean13(digits: u64) -> Barcode {
        assert!(digits.dec_len() == 13); // TODO: no panic
        // TODO: more checks?
        Barcode { digits }
    }

    pub fn ean8(digits: u64) -> Barcode {
        assert!(digits.dec_len() == 8); // TODO: no panic
        // TODO: more checks?
        Barcode { digits }
    }

    pub fn get_digits(self) -> u64 {
        self.digits
    }

    pub fn get_type(self) -> BarcodeType {
        match self.digits.dec_len() {
            8 => BarcodeType::Ean8,
            13 => BarcodeType::Ean13,
            _ => unreachable!(),
        }
    }
}
