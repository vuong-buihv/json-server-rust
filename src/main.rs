use std::fs;
use std::env;
use std::process;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("No .json file was provided as argument!");
        process::exit(0);
    }


    let json_filename = &args[1];
    let json_content = fs::read_to_string(json_filename).unwrap();


    let ip = "127.0.0.1";
    let port = "7878";
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &json_content);
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
