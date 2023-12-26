use std::error::Error;
use futures::SinkExt;
use tungstenite::{protocol::WebSocketConfig, handshake::client::Request};
use futures_util::StreamExt;
use tokio_tungstenite::Connector::NativeTls;

async fn connect(port: u32, basic_auth: String) -> Result<(), Box<dyn Error>> {
    // allow self signed ssl certificates
    let connector = native_tls::TlsConnector::builder()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true).build().unwrap();
    // connect to the local websocket
    let request = Request::builder()
    .method("GET")
    .uri(format!("wss://127.0.0.1:{}/",port))
    .header("Authorization", format!("Basic {}",basic_auth))
    .body(())
    .unwrap();
    let (mut socket, _)= tokio_tungstenite::connect_async_tls_with_config(request,
    Some(WebSocketConfig::default()), 
    Some(NativeTls(connector)))
    .await.unwrap();
    // subscribe to all events
    let message = "[5, \"OnJsonApiEvent\"]";
    socket.send(tungstenite::Message::Text(message.to_owned())).await.unwrap();
    // we've successfully connected to the websocket
    loop {
        while let Some(msg) = socket.next().await {
            if let Ok(msg) = msg {
                if msg.is_text() || msg.is_binary() {
                    println!("{}",msg);
                }
            }else {
                // do something with the error
            }
        }
    }

}

#[tokio::main]
async fn main() {
    let socket = connect(42069, String::from("cmlvdDp0REJYNGQ5MW5hTF9GUkRCaVVrNV9R")).await;
}