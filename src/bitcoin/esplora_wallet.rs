use std::{cell::RefCell, rc::Rc, str::FromStr};

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder, Sleeper},
    EsploraAsyncExt,
};
use bdk_wallet::Wallet;
use bitcoin::BlockHash;
use gloo_timers::future::{sleep, TimeoutFuture};
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

use crate::{
    types::{AddressInfo, KeychainKind, Network},
    SendSyncWrapper,
};

#[derive(Clone)]

struct GlooTimersSleeper;

impl Sleeper for GlooTimersSleeper {
    type Sleep = SendSyncWrapper<TimeoutFuture>;

    fn sleep(dur: std::time::Duration) -> Self::Sleep {
        SendSyncWrapper(sleep(dur))
    }
}

#[wasm_bindgen]
pub struct BitcoinEsploraWallet {
    wallet: Rc<RefCell<Wallet>>,
    client: Rc<RefCell<AsyncClient<GlooTimersSleeper>>>,
}

#[wasm_bindgen]
impl BitcoinEsploraWallet {
    #[wasm_bindgen(constructor)]
    pub fn new(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: String,
    ) -> Result<BitcoinEsploraWallet, String> {
        let wallet = Wallet::create(external_descriptor.clone(), internal_descriptor.clone())
            .network(network.into())
            .create_wallet_no_persist()
            .map_err(|e| format!("{:?}", e))?;

        let client = Builder::new(&url)
            .build_async_with_sleeper::<GlooTimersSleeper>()
            .map_err(|e| format!("{:?}", e))?;

        Ok(BitcoinEsploraWallet {
            wallet: Rc::new(RefCell::new(wallet)),
            client: Rc::new(RefCell::new(client)),
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

    pub fn reveal_next_address(&self, keychain: KeychainKind) -> AddressInfo {
        self.wallet
            .borrow_mut()
            .reveal_next_address(keychain.into())
            .into()
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
