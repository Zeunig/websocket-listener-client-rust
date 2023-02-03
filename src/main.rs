use futures::SinkExt;
use tungstenite::{protocol::WebSocketConfig, client::IntoClientRequest};
use futures_util::StreamExt;
use http::{header::AUTHORIZATION, HeaderValue};

#[tokio::main]
async fn main() {
    let go_to_connector = native_tls::TlsConnector::builder()
    .danger_accept_invalid_certs(true)
    .danger_accept_invalid_hostnames(true).build().unwrap();
    let connfig = WebSocketConfig::default();
    let connector = tokio_tungstenite::Connector::NativeTls(go_to_connector);
    let mut request = "wss://127.0.0.1:63761/".into_client_request().unwrap();
    request.headers_mut().insert(AUTHORIZATION,HeaderValue::from_static("Basic cmlvdDp0REJYNGQ5MW5hTF9GUkRCaVVrNV9R"));
    let (mut socket, _)= tokio_tungstenite::connect_async_tls_with_config(request,
    Some(connfig), 
    Some(connector))
    .await.unwrap();
    println!("Connected");
  
    let message = "[5, \"OnJsonApiEvent\"]";
    socket.send(tungstenite::Message::Text(message.to_owned())).await.unwrap();
    loop {


            while let Some(msg) = socket.next().await {
                let msg = msg.unwrap();
                if msg.is_text() || msg.is_binary() {
                    println!("{}",msg);
                }
            }

    }
}