use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures_util::StreamExt;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/ws", get(ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("WebSocket server running at ws://{}/ws", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}
async fn handle_socket(mut socket: WebSocket) {
    println!("Client connected");
    while let Some(Ok(msg)) = socket.next().await {
        match msg {
            Message::Text(text) => {
                println!("Received: {}", text);

                if socket
                    .send(Message::Text(format!("echo: {}", text)))
                    .await
                    .is_err()
                {
                    println!("Failed to send message, closing socket");
                    return;
                }
            }
            Message::Close(_) => {
                println!("Client disconnected");
                return;
            }
            _ => {}
        }
    }
}
