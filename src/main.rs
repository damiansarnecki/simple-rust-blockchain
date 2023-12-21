mod network;
use network::api::init;

#[tokio::main]
async fn main() {
    //Start node API
    let _ = tokio::spawn(init());

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for Ctrl+C");
}
