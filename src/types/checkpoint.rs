use std::ops::Deref;

use bdk_core::CheckPoint as BdkCheckPoint;
use wasm_bindgen::prelude::wasm_bindgen;

use super::BlockId;

/// A checkpoint is a node of a reference-counted linked list of [`BlockId`]s.
///
/// Checkpoints are cheaply cloneable and are useful to find the agreement point between two sparse
/// block chains.
#[wasm_bindgen]
#[derive(Debug)]
pub struct CheckPoint {
    checkpoint: BdkCheckPoint,
}

#[wasm_bindgen]
impl CheckPoint {
    /// Get the [`BlockId`] of the checkpoint.
    #[wasm_bindgen(getter)]
    pub fn block_id(&self) -> BlockId {
        self.checkpoint.block_id().into()
    }

    /// Get the height of the checkpoint.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.checkpoint.height()
    }

    /// Get the block hash of the checkpoint.
    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.checkpoint.hash().to_string()
    }

    /// Get the previous checkpoint in the chain
    #[wasm_bindgen(getter)]
    pub fn prev(&self) -> Option<Self> {
        self.checkpoint.prev().map(Into::into)
    }

    /// Get checkpoint at `height`.
    ///
    /// Returns `None` if checkpoint at `height` does not exist.
    pub fn get(&self, height: u32) -> Option<Self> {
        self.checkpoint.get(height).map(Into::into)
    }
}

impl Deref for CheckPoint {
    type Target = BdkCheckPoint;

    fn deref(&self) -> &Self::Target {
        &self.checkpoint
    }
}

impl From<BdkCheckPoint> for CheckPoint {
    fn from(checkpoint: BdkCheckPoint) -> Self {
        CheckPoint { checkpoint }
    }
}
