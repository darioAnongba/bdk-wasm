mod descriptor;
mod wallet;

pub use descriptor::*;
pub use wallet::*;

#[cfg(feature = "esplora")]
mod esplora_client;

#[cfg(feature = "snap")]
mod snap_wallet;

#[cfg(feature = "esplora")]
pub use esplora_client::EsploraClient;

#[cfg(feature = "snap")]
pub use snap_wallet::SnapWallet;
