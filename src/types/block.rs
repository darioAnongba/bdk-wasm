use bdk_core::BlockId as BdkBlockId;
use wasm_bindgen::prelude::wasm_bindgen;

/// A reference to a block in the canonical chain.
#[wasm_bindgen]
#[derive(Debug)]
pub struct BlockId {
    block_id: BdkBlockId,
}

#[wasm_bindgen]
impl BlockId {
    /// The height of the block.
    #[wasm_bindgen(getter)]
    pub fn height(&self) -> u32 {
        self.block_id.height
    }

    /// The hash of the block.
    #[wasm_bindgen(getter)]
    pub fn hash(&self) -> String {
        self.block_id.hash.to_string()
    }
}

impl From<BdkBlockId> for BlockId {
    fn from(block_id: BdkBlockId) -> Self {
        BlockId { block_id }
    }
}
