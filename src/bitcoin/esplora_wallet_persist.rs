use std::{cell::RefCell, rc::Rc, str::FromStr};

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{PersistedWallet, Wallet};
use bitcoin::BlockHash;
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::bitcoin::storage::SnapWalletPersister;
use crate::types::{AddressInfo, BrowserSleeper, KeychainKind, Network};

const STORAGE_KEY: &str = "bitcoin_wallet";

#[wasm_bindgen]
pub struct BitcoinEsploraWalletPersist {
    wallet: Rc<RefCell<PersistedWallet<SnapWalletPersister>>>,
    client: Rc<RefCell<AsyncClient<BrowserSleeper>>>,
    persister: SnapWalletPersister,
}

#[wasm_bindgen]
impl BitcoinEsploraWalletPersist {
    pub async fn new(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: String,
    ) -> Result<BitcoinEsploraWalletPersist, String> {
        let mut persister = SnapWalletPersister::new(STORAGE_KEY.to_string());

        let wallet = Wallet::create(external_descriptor.clone(), internal_descriptor.clone())
            .network(network.into())
            .create_wallet_async(&mut persister)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let client = Builder::new(&url)
            .build_async_with_sleeper::<BrowserSleeper>()
            .map_err(|e| format!("{:?}", e))?;

        Ok(BitcoinEsploraWalletPersist {
            wallet: Rc::new(RefCell::new(wallet)),
            client: Rc::new(RefCell::new(client)),
            persister,
        })
    }

    pub async fn full_scan(&self, stop_gap: usize, parallel_requests: usize) -> Result<(), String> {
        let request = self.wallet.borrow().start_full_scan();
        let update = self
            .client
            .borrow()
            .full_scan(request, stop_gap, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .borrow_mut()
            .apply_update_at(update, now)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub async fn sync(&self, parallel_requests: usize) -> Result<(), String> {
        let request = self.wallet.borrow().start_sync_with_revealed_spks();
        let update = self
            .client
            .borrow()
            .sync(request, parallel_requests)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet
            .borrow_mut()
            .apply_update_at(update, now)
            .map_err(|e| format!("{:?}", e))?;

        Ok(())
    }

    pub fn balance(&self) -> u64 {
        let balance = self.wallet.borrow().balance();
        balance.total().to_sat()
    }

    pub fn next_unused_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.wallet
            .borrow_mut()
            .next_unused_address(keychain.into())
            .into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.wallet
            .borrow()
            .peek_address(keychain.into(), index)
            .into()
    }

    pub async fn reveal_next_address(
        &mut self,
        keychain: KeychainKind,
    ) -> Result<AddressInfo, JsValue> {
        let mut wallet = self.wallet.borrow_mut();

        let address = wallet.reveal_next_address(keychain.into());
        wallet
            .persist_async(&mut self.persister)
            .await
            .map_err(|e| JsValue::from_str(&format!("persist_async failed: {:?}", e)))?;

        Ok(address.into())
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .borrow()
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn take_staged(&self) -> JsValue {
        let changeset_opt = self.wallet.borrow_mut().take_staged();

        match changeset_opt {
            Some(changeset) => {
                let changeset_js = to_value(&changeset)
                    .map_err(|e| format!("{:?}", e))
                    .expect("should not fail to serialize changeset");
                changeset_js
            }
            None => JsValue::null(),
        }
    }

    pub async fn get_block_by_hash(&self, block_hash: String) -> Result<JsValue, String> {
        let block_hash =
            BlockHash::from_str(block_hash.as_str()).map_err(|e| format!("{:?}", e))?;

        let block = self
            .client
            .borrow()
            .get_block_by_hash(&block_hash)
            .await
            .map_err(|e| format!("{:?}", e))?;

        let block_js = to_value(&block).map_err(|e| format!("{:?}", e))?;

        Ok(block_js)
    }
}
