//! Test suite for the Web and headless browsers.

#![cfg(all(feature = "esplora", target_arch = "wasm32"))]

extern crate wasm_bindgen_test;

use bdk_wallet::bip39::Mnemonic;
use bdk_wasm::{
    bitcoin::EsploraWallet,
    set_panic_hook,
    types::{AddressType, KeychainKind, Network},
};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 1;
const NETWORK: Network = Network::Testnet;
const ADDRESS_TYPE: AddressType = AddressType::P2wpkh;
const MNEMONIC: &str = "journey embrace permit coil indoor stereo welcome maid movie easy clock spider tent slush bright luxury awake waste legal modify awkward answer acid goose";

#[wasm_bindgen_test]
async fn test_esplora_wallet() {
    set_panic_hook();

    let esplora_url = match NETWORK {
        Network::Bitcoin => "https://blockstream.info/api",
        Network::Testnet => "https://blockstream.info/testnet/api",
        Network::Testnet4 => "https://blockstream.info/testnet/api",
        Network::Signet => "https://mutinynet.com/api",
        Network::Regtest => "https://localhost:3000",
    };

    let seed = Mnemonic::parse(MNEMONIC).unwrap().to_seed("");
    let mut wallet = EsploraWallet::from_seed(&seed, NETWORK, ADDRESS_TYPE, esplora_url).expect("esplora_wallet");

    wallet.full_scan(STOP_GAP, PARALLEL_REQUESTS).await.expect("full_scan");

    let address0 = wallet.peek_address(KeychainKind::External, 0);
    assert_eq!(
        address0.address(),
        "tb1qjtgffm20l9vu6a7gacxvpu2ej4kdcsgc26xfdz".to_string()
    );

    let balance = wallet.balance();
    assert_eq!(balance.total(), 0);

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
