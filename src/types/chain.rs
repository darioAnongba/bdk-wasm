use std::ops::Deref;

use bdk_core::spk_client::{
    FullScanRequest as BdkFullScanRequest, FullScanResponse as BdkFullScanResponse, SyncRequest as BdkSyncRequest,
    SyncResponse as BdkSyncResponse,
};
use bdk_wallet::{KeychainKind, Update as BdkUpdate};
use wasm_bindgen::prelude::wasm_bindgen;

/// Data required to perform a spk-based blockchain client sync.
///
/// A client sync fetches relevant chain data for a known list of scripts, transaction ids and
/// outpoints.
#[wasm_bindgen]
pub struct SyncRequest {
    request: BdkSyncRequest<(KeychainKind, u32)>,
}

impl Deref for SyncRequest {
    type Target = BdkSyncRequest<(KeychainKind, u32)>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl From<BdkSyncRequest<(KeychainKind, u32)>> for SyncRequest {
    fn from(request: BdkSyncRequest<(KeychainKind, u32)>) -> Self {
        SyncRequest { request }
    }
}

impl From<SyncRequest> for BdkSyncRequest<(KeychainKind, u32)> {
    fn from(request: SyncRequest) -> Self {
        request.request
    }
}

/// Data required to perform a spk-based blockchain client full scan.
///
/// A client full scan iterates through all the scripts for the given keychains, fetching relevant
/// data until some stop gap number of scripts is found that have no data. This operation is
/// generally only used when importing or restoring previously used keychains in which the list of
/// used scripts is not known.
#[wasm_bindgen]
pub struct FullScanRequest {
    request: BdkFullScanRequest<KeychainKind>,
}

impl Deref for FullScanRequest {
    type Target = BdkFullScanRequest<KeychainKind>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl From<BdkFullScanRequest<KeychainKind>> for FullScanRequest {
    fn from(request: BdkFullScanRequest<KeychainKind>) -> Self {
        FullScanRequest { request }
    }
}

impl From<FullScanRequest> for BdkFullScanRequest<KeychainKind> {
    fn from(request: FullScanRequest) -> Self {
        request.request
    }
}

/// An update to [`Wallet`].
#[wasm_bindgen]
#[derive(Debug)]
pub struct Update {
    update: BdkUpdate,
}

impl Deref for Update {
    type Target = BdkUpdate;

    fn deref(&self) -> &Self::Target {
        &self.update
    }
}

impl From<BdkUpdate> for Update {
    fn from(update: BdkUpdate) -> Self {
        Update { update }
    }
}

impl From<Update> for BdkUpdate {
    fn from(update: Update) -> Self {
        update.update
    }
}

impl From<BdkFullScanResponse<KeychainKind>> for Update {
    fn from(result: BdkFullScanResponse<KeychainKind>) -> Self {
        Update { update: result.into() }
    }
}

impl From<BdkSyncResponse> for Update {
    fn from(result: BdkSyncResponse) -> Self {
        Update { update: result.into() }
    }
}
