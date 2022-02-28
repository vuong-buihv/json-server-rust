mod request_methods;
mod args;

use clap::Parser;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str::from_utf8;
use request_methods::RequestMethods;
use args::Args;
use serde_json::{Value};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

fn main() {
    let args = Args::parse();

    let host = args.host.unwrap_or(DEFAULT_HOST.to_string());
    let port = args.port.unwrap_or(DEFAULT_PORT.to_string());

    let json_content = fs::read_to_string(&args.json_filename).unwrap();
    let json_value: Value = serde_json::from_str(&*json_content).unwrap();

    let listener = TcpListener::bind(format!("{}:{}", DEFAULT_HOST, port)).unwrap();
    println!("\tServer started at http://{}:{}", host, port);

    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), &json_value);
    }
}

fn handle_connection(mut stream: TcpStream, json_value: &Value) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: &str = from_utf8(&buffer).unwrap();
    let request_type: &str = RequestMethods::get_request_type(request);

    match request_type {
        RequestMethods::GET => {
            // TODO: very very naive implementation, it just sends the second post
            let response = create_successful_response(&json_value["posts"][1].to_string());

            send_response(&stream, &response);
        },

        RequestMethods::POST => {
            let body: String = get_request_body(request);
            let response = create_successful_response(&body);

            send_response(&stream, &response);
        },

        _ => {
            let response = "HTTP/1.1 405\r\nContent-Length: 0\r\n\r\n";
            send_response(&stream, &response);
        }
    }
}

// TODO: very very naive implementation, improve or use dependency
fn get_request_body(request: &str) -> String {
    let split_request: Vec<&str> = request.split("{").collect();

    "{".to_owned() + split_request[1]
}

fn create_successful_response(body: &str) -> String {
    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body)
}

fn send_response(mut stream: &TcpStream, response: &str){
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
