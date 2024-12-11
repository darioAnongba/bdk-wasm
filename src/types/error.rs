use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Error, Debug)]
pub enum SnapPersisterError {
    #[error("Failed to deserialize wallet state: {:?}", 0)]
    Reflect(JsValue),
    #[error("Failed to read snap state: {:?}", 0)]
    ReadSnapState(JsValue),
    #[error("Failed to write snap state: {:?}", 0)]
    WriteSnapState(JsValue),
    #[error("Failed to encode MessagePack: {0}")]
    EncodeMP(#[source] rmp_serde::encode::Error),
    #[error("Failed to decode MessagePack: {0}")]
    DecodeMP(#[source] rmp_serde::decode::Error),
    #[error("Failed to decode base64: {0}")]
    DecodeBase64(#[source] bitcoin::base64::DecodeError),
    #[error("Failed to deserialize: {0}")]
    Deserialize(#[source] serde_wasm_bindgen::Error),
}
