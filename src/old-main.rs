// *** OLD CODE 2022/01/13 ***
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