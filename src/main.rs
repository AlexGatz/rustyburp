/*

Learning exercise by: Alex Gatz
Date: 2021/01/13

Rustyburp is a Burp Suite like proxy written in Rust.

https://github.com/AlexGatz/rustyburp/blob/master/src/main.rs


=== New Design ===

Possible internal app states:
    1. Connection established.
    2. Connection failed, retry?
    3. Disconnect
        If connection established:
        1. Proxy on ---> Display req editor and res viewer, can forward.
        2. Proxy off ---> Do nothing, can't forward, hide req editor and res viewer.
            If proxy on:
            1. Drop ---> Drop request. Can't forward.
            2. Forward ---> Forward request to server.
            3. Display request editor
            4. Display response viewer
            If proxy off:
            1. Hide request editor
            2. Hide response viewer
        If disconnect:
        1. Close connection

Paths and API calls:

    GET / --> App starts with 2 fields and a button:
        1. IP Field (default localhost)
        2. PORT Field (default 8080)
        3. Connect button

        Api: /connect POST: {IP: ip, PORT: port}

    GET /editor Editor page:
        1. Proxy on/off toggle
        2. Forward button
        3. Drop button
        4. Request Editor
        5. Server Response Viewer

        Api: /editor POST:  request {PROXY: boolean, ACTION: FORWARD|DROP, CLIENT_REQUEST: String}
                            response {SERVER_RESPONSE: String}


*/
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
enum Action {
    Forward,
    Drop,
}

#[derive(Debug, Deserialize)]
enum State {
    Capture,
    Bypass,
}

#[derive(Debug, Deserialize)]
enum RustyMessage {
    ProxyConnection {
        ip: String,
        port: String,
    },
    Controller {
        state: State,
        action: Option<Action>,
        request: Option<String>,
    },
}

struct ProxyData {
    request: String,
    response: String,
    action: Action,
    state: State,
}

async fn websocket_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(text) = msg {
            handle_message(text).await
        }
    }
}

async fn handle_message(text: String) {
    match serde_json::from_str::<RustyMessage>(&text) {
        Ok(RustyMessage::ProxyConnection { ip, port }) => {
            println!("State changed to: {ip:?}:{port:?}");
        }
        Ok(RustyMessage::Controller { state, action, request }) => {
            println!("State changed to: {state:?} {action:?} {request:?}");
        }
        Err(e) => {
            println!("Failed to parse message: {}", e);
        }
    }
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let app = Router::new().route("/", get(websocket_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8888")
        .await
        .unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
