use std::sync::Arc;
use tokio::sync::RwLock;

mod blockchain;
use blockchain::blockchain::Blockchain;

mod miner;
use miner::Miner;

mod rpc;
use rpc::Rpc;

mod peer;

type SharedState = Arc<RwLock<AppState>>;

#[derive(Default, Debug)]
struct AppState {
    blockchain: Blockchain,
}

#[tokio::main]
async fn main() {
    let shared_state: Arc<RwLock<AppState>> = SharedState::default();

    let mut miner = Miner::new(Arc::clone(&shared_state));
    let mut peer_manager = peer::PeerManager::new(Arc::clone(&shared_state));
    let mut rpc = Rpc::new(Arc::clone(&shared_state));

    tokio::spawn(async move {
        miner.start().await;
    });

    tokio::spawn(async move {
        rpc.start().await;
    });
    
    tokio::spawn(async move {
        peer_manager.start().await;
    });

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
}
