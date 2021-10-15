use std::fmt::{Debug, Formatter};

use crate::{
    difficulty_bytes_as_u128, hashable::Hashable, u128_bytes, u32_bytes, u64_bytes, Transaction,
};

/// The hash of a block is a type alias to `Vec<u8>`.
pub type Hash = Vec<u8>;

pub struct Block {
    /// Index of the block in the chain.
    pub index: u32,
    /// The time when the block was created.
    pub timestamp: u128,
    /// The hash of the block.
    pub hash: Hash,
    /// The hash of the previous block.
    pub prev_block_hash: Hash,
    /// The arbitrary value that contributes to the hash for matching the difficultiy.
    pub nonce: u64,
    /// The content of the block.
    pub transactions: Vec<Transaction>,
    /// The difficulty of the block
    pub difficulty: u128,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block{{ idx:{}, at:{} hash:{} txns:\"{}\" nonce: {} }}",
            self.index,
            self.timestamp,
            hex::encode(&self.hash),
            self.transactions.len(),
            self.nonce
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        Block {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0; 32],
            transactions,
            difficulty,
            nonce: 0,
        }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::MAX) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|txn| txn.bytes())
                .collect::<Vec<u8>>(),
        );
        bytes.extend(&u128_bytes(&self.difficulty));
        bytes
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_as_u128(hash)
}
