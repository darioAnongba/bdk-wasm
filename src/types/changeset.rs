use std::ops::Deref;

use bdk_core::Merge;
use bdk_wallet::{
    serde_json::{from_str, to_string},
    ChangeSet as BdkChangeSet,
};
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::result::JsResult;

/// A changeset for [`Wallet`].
#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct ChangeSet(BdkChangeSet);

#[wasm_bindgen]
impl ChangeSet {
    /// Merge another [`ChangeSet`] into itself.
    pub fn merge(&mut self, other: ChangeSet) {
        self.0.merge(other.into());
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Serialize `ChangeSet` to JSON.
    pub fn to_json(&self) -> String {
        to_string(&self.0).expect("Serialization should not fail")
    }

    /// Serialize `ChangeSet` to JSON compatible with WASM.
    pub fn to_js(&self) -> JsValue {
        to_value(&self.0).expect("Serialization should not fail")
    }

    /// Create a new `ChangeSet` from a JSON string.
    #[wasm_bindgen]
    pub fn from_json(val: &str) -> JsResult<ChangeSet> {
        Ok(ChangeSet(from_str(val)?))
    }

    /// Create a new `ChangeSet` from a JS object.
    #[wasm_bindgen]
    pub fn from_js(js_value: JsValue) -> JsResult<ChangeSet> {
        Ok(ChangeSet(from_value(js_value)?))
    }
}

impl Deref for ChangeSet {
    type Target = BdkChangeSet;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BdkChangeSet> for ChangeSet {
    fn from(inner: BdkChangeSet) -> Self {
        ChangeSet(inner)
    }
}

impl From<ChangeSet> for BdkChangeSet {
    fn from(changeset: ChangeSet) -> Self {
        changeset.0
    }
}
