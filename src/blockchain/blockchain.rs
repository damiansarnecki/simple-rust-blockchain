use serde_derive::{Deserialize, Serialize};

use super::block::Block;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, new_block: Block) {
        self.blocks.push(new_block)
    }

    pub fn get_last_block(&self) -> Option<&Block> {
        match self.blocks.last() {
            Some(block) => Some(block),
            None => None,
        }
    }

    pub fn current_block_height(&self) -> usize {
        return self.blocks.len();
    }
}
