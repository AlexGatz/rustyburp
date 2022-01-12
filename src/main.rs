/*
Learning exercise by: Alex Gatz

 Rustyburp is a Burp Suite like proxy written in Rust.

 https://github.com/AlexGatz/rustyburp/blob/master/src/main.rs
*/

/* 

=== Design ===

Possible internal app states:
    1. Connection established.
    2. Connection failed, retry.
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

        Api: /editor POST:  request {PROXY: boolean, FORWARD: boolean, DROP: boolean, REQUEST: String}
                            response {SERVER_RESPONSE: String}
 

*/

// This application is currently an MVP that only writes to a file and handles basic CLI input.
// It will also forward data from a file that can be edited to a listener on another port.
// *** See above for the new design and plan ***

// Current problem: After learning the basics of rust, this needs to be redesigned to be more modular and flexible.

// TODO: Send request to intended server (host/uri from request). In other words, make this an actual proxy.
// TODO: Write response from server to file

// TODO: Add support for SSL
// TODO: Add support for editor integration
// TODO: Proper error handling instead of unwrap()

// To test:
// 1.) start "nc -lvnp 8080"
// 2.) cargo run
// 3.) execute: curl -iv http://localhost:7878 -d '{"stuff":10, "otherstuff":50}'
// 4.) You can then edit request.txt and choose to forward to nc on port 8080. 

use std::fs::write;
use std::io::prelude::*;
use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::str;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    dbg!("Waiting for initial connection/request from client...");

    // Is calling incoming().next() needed here?
    let stream = listener.incoming().next().unwrap().unwrap();
    handle_client_request(stream);

    loop {
        let mut input = String::new();
        dbg!("Please type 'forward' or 'exit'.");
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "forward" {
            handle_server_request();
            dbg!("\nRequest forwarded. Waiting for next request.\n");
            let stream = listener.incoming().next().unwrap().unwrap();
            handle_client_request(stream);
            continue;

        // TODO: Add drop request functionality.
        } else if input.trim() == "exit" {
            break;
        } else {
            dbg!("Invalid input");
            continue;
        }
    }
}

fn handle_client_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap_or_default();
    let buffer_str = str::from_utf8(&buffer).unwrap();

    println!("{}", buffer_str);

    // trim_matches removes the null characters from the end of the string.
    write(
        "/home/neo/request.txt",
        buffer_str.trim_matches(char::from(0)),
    )
    .expect("Unable to write file");
    dbg!("\nRequest written to request.txt\n");
}

// REFACTOR: This is just a placeholder for now. The stream dies after the first request. The driver loop should maintain the server_stream in main().
fn handle_server_request() {
    let mut file = std::fs::File::open("/home/neo/request.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // This will be replaced with intended host from client_stream.
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    stream.write(contents.as_bytes()).unwrap();
    stream.flush().unwrap();
}