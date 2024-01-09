use crate::{AppState, SharedState};
use axum::{extract::State, routing::get, Router};
use std::{env, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};

pub struct Rpc {
    shared_state: Arc<RwLock<AppState>>,
}

impl Rpc {
    pub fn new(shared_state: Arc<RwLock<AppState>>) -> Self {
        Rpc {
            shared_state: shared_state,
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.init().await;
        }
    }

    async fn init(&mut self) {
        let app = Router::new()
            .route("/", get(Rpc::root))
            .with_state(Arc::clone(&self.shared_state));

        let addr = env::args()
            .nth(2)
            .unwrap_or_else(|| "127.0.0.1:3000".to_string());

        let listener = TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }

    async fn root(State(state): State<SharedState>) -> String {
        serde_json::to_string(&state.read().await.blockchain).unwrap()
    }
}
