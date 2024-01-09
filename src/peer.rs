use crate::AppState;
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::{env, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, RwLock},
    time::sleep,
};
use tokio_tungstenite::tungstenite::Message;

pub struct PeerManager {
    shared_state: Arc<RwLock<AppState>>,
}

impl PeerManager {
    pub fn new(shared_state: Arc<RwLock<AppState>>) -> Self {
        PeerManager {
            shared_state: shared_state,
        }
    }

    pub async fn start(&mut self) {
        loop {
            self.init().await;
        }
    }

    async fn init(&mut self) {
        let addr = env::args()
            .nth(1)
            .unwrap_or_else(|| "127.0.0.1:8080".to_string());

        let (tx, _) = broadcast::channel::<Message>(10);

        let ws_tx = Arc::new(tx);

        // Create the event loop and TCP listener we'll accept connections on.
        let try_socket = TcpListener::bind(&addr).await;
        let listener = try_socket.expect("Failed to bind");
        println!("Listening on: {}", addr);

        tokio::spawn(PeerManager::accept_connections(
            listener,
            Arc::clone(&ws_tx),
        ));

        tokio::spawn(PeerManager::manage_peers(
            Arc::clone(&self.shared_state),
            Arc::clone(&ws_tx),
        ));
    }

    async fn manage_peers(state_clone: Arc<RwLock<AppState>>, tx: Arc<broadcast::Sender<Message>>) {
        let mut last_block_index = 1;

        loop {
            let amount = {
                let blockchain = &state_clone.read().await.blockchain;

                blockchain.current_block_height()
            };

            if (amount > last_block_index) {
                let last_block = {
                    let chain = &state_clone.read().await.blockchain;

                    Message::Text((serde_json::to_string(&chain.clone().get_last_block()).unwrap()))
                };

                last_block_index += 1;

                if let Err(err) = tx.send(last_block) {
                    eprintln!("Error sending mined block to WebSocket clients: {}", err);
                }
            }

            sleep(Duration::from_millis(500)).await;
        }
    }

    async fn accept_connections(listener: TcpListener, ws_tx: Arc<broadcast::Sender<Message>>) {
        while let Ok((stream, addr)) = listener.accept().await {
            tokio::spawn(PeerManager::handle_connection(
                stream,
                addr,
                Arc::clone(&ws_tx),
            ));
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        ws_tx: Arc<broadcast::Sender<Message>>,
    ) {
        println!("Incoming TCP connection from: {:?}", addr);

        let ws_stream = tokio_tungstenite::accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        let (mut write, mut read) = ws_stream.split();

        let mut receiver = ws_tx.subscribe();
        while let Ok(message) = receiver.recv().await {
            // Wysłanie nowej wiadomości do klienta WebSocket
            if let Err(err) = write.send(message).await {
                eprintln!("Error sending message to WebSocket client: {:?}", err);
                break;
            }
        }

        // Prosta logika obsługi komunikatów WebSocket
        while let Some(Ok(msg)) = read.next().await {
            match msg {
                Message::Text(text) => {
                    // Obsługa wiadomości tekstowych
                    println!("Received text message: {}", text);

                    // Odpowiedź (opcjonalne)
                    let response = Message::Text("Hello, client!".into());
                    if let Err(err) = write.send(response).await {
                        eprintln!("Error sending response: {}", err);
                        break;
                    }
                }
                Message::Binary(data) => {
                    // Obsługa wiadomości binarnych
                    println!("Received binary message with length: {}", data.len());
                }
                Message::Ping(ping_data) => {
                    // Obsługa wiadomości Ping
                    println!("Received Ping: {:?}", ping_data);
                }
                Message::Pong(pong_data) => {
                    // Obsługa wiadomości Pong
                    println!("Received Pong: {:?}", pong_data);
                }
                Message::Close(close_frame) => {
                    // Obsługa wiadomości Close
                    println!("Received Close: {:?}", close_frame);
                    break; // Zakończ pętlę, gdy klient wysyła Close
                }
                _ => {
                    // Obsługa innych typów wiadomości (jeśli istnieją)
                    println!("Unsupported message type");
                }
            }
        }

        // Jeśli pętla się zakończyła, to połączenie zostało zamknięte
        println!("WebSocket connection closed");
    }
}
