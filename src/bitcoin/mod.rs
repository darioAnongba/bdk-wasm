mod descriptor;
mod wallet;

pub use descriptor::*;
pub use wallet::*;

#[cfg(feature = "esplora")]
mod esplora_wallet;

#[cfg(feature = "snap")]
mod snap_wallet;

#[cfg(feature = "esplora")]
pub use esplora_wallet::EsploraWallet;

#[cfg(feature = "snap")]
pub use snap_wallet::SnapWallet;
