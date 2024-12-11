//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bdk_wallet::bip39::Mnemonic;
use bdk_wasm::{
    bitcoin::{EsploraClient, Wallet},
    set_panic_hook,
    types::{AddressType, KeychainKind, Network},
};
use js_sys::Date;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 1;
const NETWORK: Network = Network::Testnet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_esplora_client() {
    set_panic_hook();

    let esplora_url = match NETWORK {
        Network::Bitcoin => "https://blockstream.info/api",
        Network::Testnet => "https://blockstream.info/testnet/api",
        Network::Testnet4 => "https://blockstream.info/testnet/api",
        Network::Signet => "https://mutinynet.com/api",
        Network::Regtest => "https://localhost:3000",
    };

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let mut wallet = Wallet::from_seed(&seed, NETWORK, ADDRESS_TYPE).expect("wallet");
    let mut blockchain_client = EsploraClient::new(esplora_url).expect("esplora_client");

    let block_height = wallet.latest_checkpoint().height();
    assert_eq!(block_height, 0);

    let full_scan_request = wallet.start_full_scan();
    let update = blockchain_client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");
    wallet
        .apply_update_at(update, Some((Date::now() / 1000.0) as u64))
        .expect("full_scan apply_update_at");

    let fullscan_block_height = wallet.latest_checkpoint().height();
    assert!(fullscan_block_height > 0);

    wallet.reveal_addresses_to(KeychainKind::External, 5);

    let sync_request = wallet.start_sync_with_revealed_spks();
    let update = blockchain_client
        .sync(sync_request, PARALLEL_REQUESTS)
        .await
        .expect("sync");
    wallet
        .apply_update_at(update, Some((Date::now() / 1000.0) as u64))
        .expect("sync apply_update_at");

    // TODO: Find a better way to assert that the sync was successful
    let sync_block_height = wallet.latest_checkpoint().height();
    assert!(sync_block_height >= fullscan_block_height);
}
