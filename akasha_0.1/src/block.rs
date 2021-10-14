use std::fmt::{Debug, Formatter};

/// The hash of a block is a type alias to `Vec<u8>`.
type BlockHash = Vec<u8>;

pub struct Block {
    /// Index of the block in the chain.
    pub index: u32,
    /// The time when the block was created.
    pub timestamp: u128,
    /// The hash of the block.
    pub hash: BlockHash,
    /// The hash of the previous block.
    pub prev_block_hash: BlockHash,
    ///
    pub nonce: u64,
    /// The content of the block.
    pub payload: String,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block{{ idx:{}, at:{} hash:{} payload:\"{}\" }}",
            self.index,
            self.timestamp,
            hex::encode(&self.hash),
            self.payload
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nonce: u64,
        payload: String,
    ) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0; 32],
            nonce,
            payload,
        }
    }
}
