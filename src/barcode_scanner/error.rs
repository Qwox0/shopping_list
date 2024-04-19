use crate::camera::CameraError;
use std::num::{ParseIntError, TryFromIntError};
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, thiserror::Error)]
pub enum BarcodeError {
    // #[error("The camera is unavailable during SSR")]
    // OnServer,

    // #[error("The device doesn't have a camera")]
    // NoCamera,
    #[error(transparent)]
    CameraError(#[from] CameraError),

    #[error("couldn't get the context for the canvas element")]
    GetCanvasContextErr,

    #[error("couldn't draw the image: {:?}", .0)]
    DrawImgErr(JsValue),

    #[error("couldn't get the image data: {:?}", .0)]
    GetImgDataErr(JsValue),

    #[error("couldn't set interval: {:?}", .0)]
    SetIntervalErr(JsValue),

    #[error("couldn't cast Js Object")]
    JsDynCastError,

    #[error("unsupported barcode type: {}", .0)]
    UnsupportedBarcodeType(String),

    #[error("cann't parse barcode: {}", .0)]
    ParseBarcodeErr(ParseIntError),

    #[error("invalid barcode length (expected: {expected}, got: {got})")]
    InvalidBarcodeLength { expected: u8, got: u8 },

    #[error("invalid barcode: {}", .0)]
    InvalidBarcode(u64),

    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
}
