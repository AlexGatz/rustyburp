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

        Api: /editor POST:  request {PROXY: boolean, ACTION: FORWARD|DROP, boolean, CLIENT_REQUEST: String}
                            response {SERVER_RESPONSE: String}


*/
enum Action {
    Forward,
    Drop,
    Passthrough,
}
struct Proxy {
    ip: String,
    port: u16,
    client_req: String,
    client_res: String,
    action: Action,
}

impl Proxy {
    fn new() {
        dbg!("Proxy::new called.");
    }
    fn action(a: Action){        
        let b = match a {
            Action::Forward => "Forward Action",
            Action::Drop => "Drop Action",
            Action::Passthrough => "Passthrough Action",
        };

        dbg!(b);
    }
}

struct Controller {
    ip: String,
    port: u16,
    controller_req: String,
    controller_res: String,
}

impl Controller {
    // TODO:
}
fn main() {
    let proxy = Proxy::new();
    let action = Action::Forward;
    Proxy::action(action);
}
