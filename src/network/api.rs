type SharedState = Arc<RwLock<AppState>>;

use axum::{extract::State, routing::get, Router};
use num_bigint::BigUint;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

#[path = "../blockchain/mod.rs"]
mod blockchain;
use blockchain::{block::Block, blockchain::Blockchain};

#[derive(Default, Debug)]
struct AppState {
    blockchain: Blockchain,
}

pub async fn init() {
    let shared_state = SharedState::default();
    shared_state
        .write()
        .unwrap()
        .blockchain
        .add_block(Block::genesis());

    let app = Router::new()
        .route("/", get(root))
        .route("/mine", get(mine))
        .with_state(Arc::clone(&shared_state));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<SharedState>) -> String {
    serde_json::to_string(&state.read().unwrap().blockchain).unwrap()
}

async fn mine(State(state): State<SharedState>) -> String {
    let blockchain = &mut state.write().unwrap().blockchain;

    loop {
        let block = blockchain.get_last_block().unwrap();

        match Block::mine_block(&block, BigUint::from(0u32)) {
            Some(new_block) => match Block::validate_block(&block, &new_block) {
                Ok(_) => {
                    println!("{:#?}", &new_block);

                    let result = serde_json::to_string(&new_block).unwrap();

                    &blockchain.add_block(new_block);

                    return result;
                }
                Err(_) => {
                    println!("Error")
                }
            },
            None => println!("Error two"),
        }
    }
}
