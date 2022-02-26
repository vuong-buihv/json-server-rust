use clap::Parser;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

const DEFAULT_IP: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

#[derive(Parser)]
#[clap(author, version)]
struct Args {
    /// JSON file to load
    json_filename: String,

    /// IP address
    #[clap(short, long)]
    ip_address: Option<String>,

    /// Port
    #[clap(short, long)]
    port: Option<String>,
}

fn main() {
    let args = Args::parse();

    let ip_address = args.ip_address.unwrap_or(DEFAULT_IP.to_string());
    let port = args.port.unwrap_or(DEFAULT_PORT.to_string());

    let listener = TcpListener::bind(format!("{}:{}", ip_address, port)).unwrap();
    println!("\tServer started at http://{}:{}", ip_address, port);

    let json_content = fs::read_to_string(args.json_filename).unwrap();
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
