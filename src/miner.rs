use num_bigint::BigUint;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{blockchain::block::Block, AppState};

pub struct Miner {
    shared_state: Arc<RwLock<AppState>>,
}

impl Miner {
    pub fn new(shared_state: Arc<RwLock<AppState>>) -> Self {
        Miner {
            shared_state,
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.mine(self.shared_state.clone()).await;
        }
    }

    async fn mine(&mut self, state_clone: Arc<RwLock<AppState>>) -> Option<()> {
        let state = Arc::clone(&state_clone);

        let block = {
            let blockchain = &state.read().await.blockchain;
            blockchain.get_last_block().unwrap().clone()
        };

        match Block::mine_block(&block, BigUint::from(0u32)) {
            Some(new_block) => match Block::validate_block(&block, &new_block) {
                Ok(_) => {
                    println!("{:#?}", &new_block);

                    let editable = &mut state.write().await.blockchain;
                    editable.add_block(new_block.clone());

                    Some(())
                }
                Err(_) => {
                    println!("Error: Validation failed");
                    None
                }
            },
            None => {
                println!("Error: Mining failed");
                None
            }
        }
    }
}
