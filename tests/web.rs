//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use std::str::FromStr;
use web_sys::console;

use bdk_wallet::{bip39::Mnemonic, bitcoin::AddressType};
use bdk_wasm::{
    bitcoin::BitcoinEsploraWallet,
    mnemonic_to_descriptor, set_panic_hook,
    types::{KeychainKind, Network},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STOP_GAP: usize = 100;
const PARALLEL_REQUESTS: usize = 5;
const NETWORK: Network = Network::Testnet;

fn new_descriptors() -> Result<(String, String), String> {
    let mnemonic_str = "drip drum plug universe beyond gasp cram action hurt keep awake tortoise luggage return luxury net jar awake mimic hurry critic curtain quiz kit";

    let mnemonic = Mnemonic::from_str(mnemonic_str).expect("mnemonic");
    let (descriptor, change_descriptor) =
        mnemonic_to_descriptor(mnemonic, NETWORK, AddressType::P2wpkh).expect("descriptor");

    console::log_1(&format!("descriptor: {}", descriptor).into());
    console::log_1(&format!("change_descriptor: {}", change_descriptor).into());

    Ok((descriptor, change_descriptor))
}

#[wasm_bindgen_test]
async fn test_esplora_wallet() {
    set_panic_hook();

    let (descriptor, change_descriptor) = new_descriptors().expect("descriptors");
    let esplora_url = match NETWORK {
        Network::Bitcoin => "https://blockstream.info/api",
        Network::Testnet => "https://blockstream.info/testnet/api",
        Network::Testnet4 => "https://blockstream.info/testnet/api",
        Network::Signet => "https://mutinynet.com/api",
        Network::Regtest => "https://localhost:3000",
    };

    let wallet = BitcoinEsploraWallet::new(
        NETWORK,
        descriptor,
        change_descriptor,
        esplora_url.to_string(),
    )
    .expect("esplora_wallet");

    wallet
        .full_scan(STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("full_scan");

    let address0 = wallet.peek_address(KeychainKind::External, 0);
    assert_eq!(
        address0.address(),
        "tb1q8vl3qjdxnm54psxn5vgzdf402ky23r0jjfd8cj".to_string()
    );

    let balance = wallet.balance();
    assert_eq!(balance, 0);

    let address1 = wallet.next_unused_address(KeychainKind::External);
    assert_eq!(address1.keychain(), KeychainKind::External);
    assert_eq!(address1.index(), 0);

    let address2 = wallet.reveal_next_address(KeychainKind::External);
    assert_eq!(address2.index(), 1);

    let address3 = wallet.next_unused_address(KeychainKind::External);
    assert_eq!(address3.index(), 0);

    // Should do a single call to the server (for each keychain)
    wallet.sync(1).await.expect("sync");

    // Should do a stop_gap calls to the server (for each keychain) and not start from beginning
    wallet
        .full_scan(STOP_GAP, PARALLEL_REQUESTS)
        .await
        .expect("second full_scan");

    let unused_addresses = wallet.list_unused_addresses(KeychainKind::External);
    assert_eq!(unused_addresses.len(), 2);
}

/* JSON RPC client is not available in the browser as it uses raw TCP sockets
#[wasm_bindgen_test]
async fn test_rpc_wallet() {
    set_panic_hook();

    let (descriptor, change_descriptor) = new_descriptors().expect("descriptors");
    let rpc_url = "http://127.0.0.1:18443";

    let wallet = BitcoinRpcWallet::new(
        NETWORK,
        descriptor,
        change_descriptor,
        rpc_url.to_string(),
        "polaruser".to_string(),
        "polarpass".to_string(),
    )
    .expect("rpc_wallet");

    let info = wallet.get_blockchain_info().expect("get_blockchain_info");

    console::log_1(&info);
}
*/
