extern crate serde_derive;
use core::time;
use std::str::FromStr;

use hex;
use serde_derive::{Deserialize, Serialize};
use tiny_keccak::Keccak;
extern crate num_bigint;

#[path = "./helpers.rs"]
mod helpers;
use helpers::{get_current_timestamp, keccak256, sort_characters};

use num_bigint::BigUint;
use num_traits::One;

pub const MAX_U256_NUMBER_TEXT: &str =
    "115792089237316195423570985008687907853269984665640564039457584007913129639934";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeaders {
    number: u32,
    difficulty: u32,
    timestamp: u64,
    parent_hash: BigUint,
    beneficiary: BigUint,
}

pub enum ValidateBlockError {
    InvalidTargetHash,
    InvalidDifficulty,
    InvalidBlockNumber,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    block_headers: BlockHeaders,
    nonce: BigUint,
}

pub const HASH_LENGTH: usize = 64;

impl Block {
    pub fn new(
        number: u32,
        parent_hash: BigUint,
        beneficiary: BigUint,
        difficulty: u32,
        timestamp: u64,
        nonce: BigUint,
    ) -> Self {
        Block {
            block_headers: BlockHeaders {
                number,
                difficulty,
                timestamp,
                parent_hash,
                beneficiary,
            },
            nonce,
        }
    }

    pub fn calculateBlockTargetHash(lastBlock: &Block) -> BigUint {
        return BigUint::from_str(MAX_U256_NUMBER_TEXT).unwrap()
            / BigUint::from(lastBlock.block_headers.difficulty);
    }

    pub fn get_block_hash(block_headers: &BlockHeaders) -> Option<BigUint> {
        match serde_json::to_string(block_headers) {
            Ok(text) => match sort_characters(&text) {
                Ok(val) => Some(keccak256(&val)),
                Error => None,
            },
            Error => None,
        }
    }

    pub fn mine_block(lastBlock: &Block, beneficiary: BigUint) -> Option<Block> {
        let target_hash: BigUint = Block::calculateBlockTargetHash(&lastBlock);
        let mut under_target_hash;

        let headers_string = serde_json::to_string(&lastBlock.block_headers.parent_hash).unwrap();
        let sorted = sort_characters(&headers_string).unwrap();

        let mut nonce = BigUint::from(0u64);

        loop {
            let timestamp = get_current_timestamp().unwrap();

            let new_block_headers = BlockHeaders {
                number: lastBlock.block_headers.number + 1,
                difficulty: Block::adjust_difficulty(lastBlock, timestamp),
                beneficiary: beneficiary.to_owned(),
                parent_hash: keccak256(&sorted),
                timestamp: timestamp,
            };

            let hashed = Block::get_block_hash(&new_block_headers).unwrap();

            let result = (hashed + &nonce).to_string();

            under_target_hash = keccak256(&result);

            if (under_target_hash <= target_hash) {
                let new_block = Block {
                    block_headers: new_block_headers,
                    nonce: nonce,
                };

                return Some(new_block);
            }

            nonce += BigUint::from(1u64);
        }
    }

    pub fn adjust_difficulty(lastBlock: &Block, timestamp: u64) -> u32 {
        if timestamp - lastBlock.block_headers.timestamp > 2 {
            if (lastBlock.block_headers.difficulty - 1 == 0) {
                1
            } else {
                lastBlock.block_headers.difficulty - 1
            }
        } else {
            lastBlock.block_headers.difficulty + 1
        }
    }

    pub fn validate_block(
        last_block: &Block,
        new_block: &Block,
    ) -> Result<bool, ValidateBlockError> {
        //handle genesis
        if (Block::get_block_hash(&new_block.block_headers).unwrap()
            == Block::get_block_hash(&Block::genesis().block_headers).unwrap())
        {
            return Ok(true);
        }

        //handle invalid block number
        if (new_block.block_headers.number != last_block.block_headers.number + 1) {
            return Err(ValidateBlockError::InvalidBlockNumber);
        }

        //handle invalid difficulty
        if (new_block.block_headers.difficulty
            != Block::adjust_difficulty(last_block, new_block.block_headers.timestamp))
        {
            return Err(ValidateBlockError::InvalidDifficulty);
        }

        //handle invalid target hash
        let last_block_target_hash = Block::calculateBlockTargetHash(&last_block);

        let new_block_header_hash = Block::get_block_hash(&new_block.block_headers).unwrap();
        let nonce = &new_block.nonce;

        let stringified = (new_block_header_hash + nonce).to_string();

        let under_target_hash = keccak256(&stringified);

        if (under_target_hash > last_block_target_hash) {
            return Err(ValidateBlockError::InvalidTargetHash);
        }

        return Ok(true);
    }

    pub fn genesis() -> Block {
        Block {
            block_headers: BlockHeaders {
                number: 0,
                difficulty: 1,
                timestamp: get_current_timestamp().unwrap(),
                parent_hash: BigUint::one(),
                beneficiary: BigUint::one(),
            },
            nonce: BigUint::one(),
        }
    }
}
