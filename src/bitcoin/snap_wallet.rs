use std::str::FromStr;

use bdk_esplora::{
    esplora_client::{AsyncClient, Builder},
    EsploraAsyncExt,
};
use bdk_wallet::{descriptor::IntoWalletDescriptor, PersistedWallet, Wallet as BdkWallet};
use bitcoin::bip32::{Fingerprint, Xpriv, Xpub};
use js_sys::Date;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::{prelude::wasm_bindgen, JsError, JsValue};

use crate::{
    bitcoin::{seed_to_descriptor, xpriv_to_descriptor, xpub_to_descriptor},
    result::JsResult,
    storage::SnapPersister,
    types::{AddressInfo, AddressType, Balance, KeychainKind, Network},
};

const STORAGE_KEY: &str = "wallet";

#[wasm_bindgen]
pub struct SnapWallet {
    wallet: PersistedWallet<SnapPersister>,
    client: AsyncClient,
    persister: SnapPersister,
}

#[wasm_bindgen]
impl SnapWallet {
    async fn create<D>(
        network: Network,
        external_descriptor: D,
        internal_descriptor: D,
        url: &str,
    ) -> JsResult<SnapWallet>
    where
        D: IntoWalletDescriptor + Send + Clone + 'static,
    {
        let mut persister = SnapPersister::new(STORAGE_KEY);
        let wallet = BdkWallet::create(external_descriptor, internal_descriptor)
            .network(network.into())
            .create_wallet_async(&mut persister)
            .await?;

        let client = Builder::new(&url).build_async()?;

        Ok(SnapWallet {
            wallet,
            client,
            persister,
        })
    }

    pub async fn load(url: &str) -> JsResult<SnapWallet> {
        let mut persister = SnapPersister::new(STORAGE_KEY);
        let wallet_opt = BdkWallet::load().load_wallet_async(&mut persister).await?;

        let wallet = match wallet_opt {
            Some(wallet) => wallet,
            None => return Err(JsError::new("Failed to load wallet, check the changeset")),
        };

        let client = Builder::new(&url).build_async()?;

        Ok(SnapWallet {
            wallet,
            client,
            persister,
        })
    }

    pub async fn from_descriptors(
        network: Network,
        external_descriptor: String,
        internal_descriptor: String,
        url: &str,
    ) -> JsResult<SnapWallet> {
        Self::create(network, external_descriptor, internal_descriptor, url).await
    }

    pub async fn from_seed(
        seed: &[u8],
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<SnapWallet> {
        let (external_descriptor, internal_descriptor) =
            seed_to_descriptor(seed, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url).await
    }

    pub async fn from_xpriv(
        extended_privkey: &str,
        fingerprint: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<SnapWallet> {
        let xprv = Xpriv::from_str(extended_privkey).map_err(|e| JsError::new(&e.to_string()))?;
        let fingerprint = Fingerprint::from_hex(fingerprint)?;

        let (external_descriptor, internal_descriptor) =
            xpriv_to_descriptor(xprv, fingerprint, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url).await
    }

    pub async fn from_xpub(
        extended_pubkey: &str,
        fingerprint: &str,
        network: Network,
        address_type: AddressType,
        url: &str,
    ) -> JsResult<SnapWallet> {
        let xpub = Xpub::from_str(extended_pubkey)?;
        let fingerprint = Fingerprint::from_hex(fingerprint)?;

        let (external_descriptor, internal_descriptor) =
            xpub_to_descriptor(xpub, fingerprint, network.into(), address_type.into())
                .map_err(|e| JsError::new(&e.to_string()))?;

        Self::create(network, external_descriptor, internal_descriptor, url).await
    }

    pub async fn full_scan(&mut self, stop_gap: usize, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_full_scan();
        let update = self
            .client
            .full_scan(request, stop_gap, parallel_requests)
            .await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, now)?;

        Ok(())
    }

    pub async fn sync(&mut self, parallel_requests: usize) -> JsResult<()> {
        let request = self.wallet.start_sync_with_revealed_spks();
        let update = self.client.sync(request, parallel_requests).await?;

        let now = (Date::now() / 1000.0) as u64;
        self.wallet.apply_update_at(update, now)?;

        Ok(())
    }

    pub fn network(&self) -> Network {
        self.wallet.network().into()
    }

    pub fn balance(&self) -> Balance {
        self.wallet.balance().into()
    }

    pub fn next_unused_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.next_unused_address(keychain.into()).into()
    }

    pub fn peek_address(&self, keychain: KeychainKind, index: u32) -> AddressInfo {
        self.wallet.peek_address(keychain.into(), index).into()
    }

    pub fn reveal_next_address(&mut self, keychain: KeychainKind) -> AddressInfo {
        self.wallet.reveal_next_address(keychain.into()).into()
    }

    pub fn reveal_addresses_to(&mut self, keychain: KeychainKind, index: u32) -> Vec<AddressInfo> {
        self.wallet
            .reveal_addresses_to(keychain.into(), index)
            .map(Into::into)
            .collect()
    }

    pub fn list_unused_addresses(&self, keychain: KeychainKind) -> Vec<AddressInfo> {
        self.wallet
            .list_unused_addresses(keychain.into())
            .map(Into::into)
            .collect()
    }

    pub fn list_unspent(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .list_unspent()
            .map(|output| to_value(&output).map_err(Into::into))
            .collect()
    }

    pub fn transactions(&self) -> JsResult<Vec<JsValue>> {
        self.wallet
            .transactions()
            .map(|tx| to_value(&tx.tx_node.tx).map_err(Into::into))
            .collect()
    }

    pub async fn persist(&mut self) -> JsResult<bool> {
        self.wallet
            .persist_async(&mut self.persister)
            .await
            .map_err(Into::into)
    }
}
