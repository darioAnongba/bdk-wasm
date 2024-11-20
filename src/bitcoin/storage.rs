use std::{future::Future, pin::Pin};

use crate::{types::WalletError, utils::SendSyncWrapper};
use bdk_wallet::{AsyncWalletPersister, ChangeSet};
use js_sys::Promise;
use serde_json::Value;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) struct SnapWalletPersister {
    /// Key to use for storing the ChangeSet
    key: String,
}

impl SnapWalletPersister {
    pub fn new(key: String) -> Self {
        Self { key }
    }

    async fn read_changeset(&self) -> Result<ChangeSet, WalletError> {
        let state = self.read_snap_state().await?;

        if let Some(data) = state.get(&self.key) {
            serde_json::from_value(data.clone())
                .map_err(|e| WalletError::Deserialize(e.to_string()))
        } else {
            Ok(ChangeSet::default())
        }
    }

    async fn write_changeset(&self, changeset: &ChangeSet) -> Result<(), WalletError> {
        let mut state = self
            .read_snap_state()
            .await
            .unwrap_or_else(|_| serde_json::json!({}));

        let serialized =
            serde_json::to_value(changeset).map_err(|e| WalletError::Serialize(e.to_string()))?;
        state[&self.key] = serialized;

        let params = to_value(&serde_json::json!({
            "method": "snap_manageState",
            "params": { "operation": "update", "newState": state }
        }))
        .map_err(|e| WalletError::Serialize(e.to_string()))?;

        let promise = snap_request(&params);
        JsFuture::from(promise)
            .await
            .map_err(|e| WalletError::Future(format!("{:?}", e)))?;

        Ok(())
    }

    async fn read_snap_state(&self) -> Result<Value, WalletError> {
        let params = to_value(&serde_json::json!({
            "method": "snap_manageState",
            "params": { "operation": "get" }
        }))
        .expect("should not fail to serialize request params");

        let promise = snap_request(&params);
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| WalletError::Future(format!("{:?}", e)))?;

        from_value(result).map_err(|e| WalletError::Deserialize(e.to_string()))
    }
}

impl AsyncWalletPersister for SnapWalletPersister {
    type Error = WalletError;

    fn initialize<'a>(
        persister: &'a mut Self,
    ) -> Pin<Box<dyn Future<Output = Result<ChangeSet, Self::Error>> + Send + 'a>>
    where
        Self: 'a,
    {
        let fut = async move { persister.read_changeset().await };
        let send_fut = SendSyncWrapper(fut);
        Box::pin(send_fut)
    }

    fn persist<'a>(
        persister: &'a mut Self,
        changeset: &'a ChangeSet,
    ) -> Pin<Box<dyn Future<Output = Result<(), Self::Error>> + Send + 'a>>
    where
        Self: 'a,
    {
        let fut = async move { persister.write_changeset(changeset).await };
        let send_fut = SendSyncWrapper(fut);
        Box::pin(send_fut)
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = snap, js_name = request)]
    fn snap_request(params: &JsValue) -> Promise;
}
