use std::ops::Deref;

use bdk_core::spk_client::{
    FullScanRequest as BdkFullScanRequest, FullScanResult as BdkFullScanResult, SyncRequest as BdkSyncRequest,
    SyncResult as BdkSyncResult,
};
use bdk_wallet::KeychainKind;
use wasm_bindgen::prelude::wasm_bindgen;

/// Data required to perform a spk-based blockchain client sync.
///
/// A client sync fetches relevant chain data for a known list of scripts, transaction ids and
/// outpoints.
#[wasm_bindgen]
pub struct SyncRequest {
    request: BdkSyncRequest<KeychainKind>,
}

impl Deref for SyncRequest {
    type Target = BdkSyncRequest<KeychainKind>;

    fn deref(&self) -> &Self::Target {
        &self.request
    }
}

impl From<BdkSyncRequest<KeychainKind>> for SyncRequest {
    fn from(request: BdkSyncRequest<KeychainKind>) -> Self {
        SyncRequest { request }
    }
}

impl From<SyncRequest> for BdkSyncRequest<KeychainKind> {
    fn from(request: SyncRequest) -> Self {
        request.request
    }
}

/// Data returned from a spk-based blockchain client sync.
///
/// See also [`SyncRequest`].
#[wasm_bindgen]
#[derive(Debug)]
pub struct SyncResult {
    result: BdkSyncResult,
}

impl Deref for SyncResult {
    type Target = BdkSyncResult;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl From<BdkSyncResult> for SyncResult {
    fn from(result: BdkSyncResult) -> Self {
        SyncResult { result }
    }
}

impl From<SyncResult> for BdkSyncResult {
    fn from(result: SyncResult) -> Self {
        result.result
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

/// Data returned from a spk-based blockchain client full scan.
///
/// See also [`FullScanRequest`].
#[wasm_bindgen]
#[derive(Debug)]
pub struct FullScanResult {
    result: BdkFullScanResult<KeychainKind>,
}

impl Deref for FullScanResult {
    type Target = BdkFullScanResult<KeychainKind>;

    fn deref(&self) -> &Self::Target {
        &self.result
    }
}

impl From<BdkFullScanResult<KeychainKind>> for FullScanResult {
    fn from(result: BdkFullScanResult<KeychainKind>) -> Self {
        FullScanResult { result }
    }
}

impl From<FullScanResult> for BdkFullScanResult<KeychainKind> {
    fn from(result: FullScanResult) -> Self {
        result.result
    }
}
