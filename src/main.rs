/* 
 Rustyburp is a Burp Suite like proxy written in Rust.
 This binary uses Hyper to act as a proxy server.
 The proxy server will intercept reqeusts from a client and export them to an editor. 
 Once editing is complete, the request will can either be forwarded or dropped.

 https://github.com/AlexGatz/rustyburp/blob/master/src/main.rs
*/

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::io::{BufWriter, Read};
use std::fs::write;
use std::str;

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

    // Efficently uses the BufWRite to write data to file.
    //let f = File::create("/home/neo/test.txt").expect("Unable to create file");
    //let mut f = BufWriter::new(f);
    //f.write_all(&buffer).expect("Unable to write data");

    // Converts &buffer to utf8 slice, but still shows a bunch of binary data. Many "^@" are written to the file.
    let buffer_str = str::from_utf8(&buffer).unwrap();

    // trim_matches removed the null characters from the end of the string.
    write("/home/neo/test.txt", buffer_str.trim_matches(char::from(0))).expect("Unable to write file");

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}