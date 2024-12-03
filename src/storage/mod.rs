#[cfg(feature = "snap")]
mod snap_persister;

#[cfg(feature = "snap")]
pub use snap_persister::SnapPersister;
