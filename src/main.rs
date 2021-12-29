/* 
 Rustyburp is a Burp Suite like proxy written in Rust.
 This binary uses Hyper to act as a proxy server.
 The proxy server will intercept reqeusts from a client and export them to an editor. 
 Once editing is complete, the request will can either be forwarded or dropped.

 https://github.com/AlexGatz/rustyburp/blob/master/src/main.rs
*/

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::Path;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

