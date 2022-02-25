use std::fs;
use std::env;
use std::process;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No .json file was provided as argument!");
        process::exit(0);
    }


    let json_filename = &args[1];
    let json_content = fs::read_to_string(json_filename).unwrap();

    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_IP, DEFAULT_PORT)).unwrap();
    println!("\tServer started at http://{}:{}", DEFAULT_IP, DEFAULT_PORT);

    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), &json_content);
    }
}

fn handle_connection(mut stream: TcpStream, json_content: &String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            json_content.len(),
            json_content
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
