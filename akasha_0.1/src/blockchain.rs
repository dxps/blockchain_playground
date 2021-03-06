use std::{collections::HashSet};

use crate::{check_difficulty, Block, Hash, Hashable};

#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    /// Create new blockchain.
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    } 
    
    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();

        if block.index != i as u32 {
            println!("Index mismatch {} != {}", &block.index, &i);
            return Err(BlockValidationErr::MismatchedIndex);
        }
        if !check_difficulty(&block.hash(), block.difficulty) {
            println!("Difficulty fail");
            return Err(BlockValidationErr::InvalidHash);
        }
        if i != 0 {
            let prev_block = &self.blocks[i - 1];
            if block.timestamp <= prev_block.timestamp {
                println!("Time did not increase");
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }
            if block.prev_block_hash != prev_block.hash {
                println!("Hash mismatch");
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } else {
            // This is the genesis block.
            if block.prev_block_hash != vec![0; 32] {
                println!("Genesis block prev_block_hash invalid");
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();
                // Checking that all input entries are also in the unspent.
                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || 
                    // Checking for double spending.
                    !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationErr::InvalidInput);
                }
                // Checking that output is not bigger than input.
                let input_value = transaction.input_value();
                let output_value = transaction.output_value();
                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }
                
                // Collecting the fee.
                let fee = input_value - output_value; 
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());
            }
        
            if coinbase.output_value() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        self.blocks.push(block);

        Ok(())
    }
}
