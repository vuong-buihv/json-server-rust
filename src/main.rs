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

const IP_ADDRESS: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

fn main() {
    let args = Args::parse();

    let port = args.port.unwrap_or(DEFAULT_PORT.to_string());

    let listener = TcpListener::bind(format!("{}:{}", IP_ADDRESS, port)).unwrap();
    println!("\tServer started at http://{}:{}", IP_ADDRESS, port);

    let json_content = fs::read_to_string(args.json_filename).unwrap();
    for stream in listener.incoming() {
        handle_connection(stream.unwrap(), &json_content);
    }
}

fn handle_connection(mut stream: TcpStream, json_content: &String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request: &str = from_utf8(&buffer).unwrap();
    let request_type: &str = RequestMethods::get_request_type(request);

    match request_type {
        RequestMethods::GET => {
            let response = create_successful_response(json_content);

            send_response(&stream, &response);
        },

        RequestMethods::POST => {
            let body: String = get_request_body(request);
            let response = create_successful_response(&body);

            send_response(&stream, &response);
        },

        _ => {
            let response = "HTTP/1.1 405\r\nContent-Length: \r\n\r\n";
            send_response(&stream, &response);
        }
    }
}

// TODO: very veru naive implementation, improve or use dependency
fn get_request_body(request: &str) -> String {
    let split_request: Vec<&str> = request.split("{").collect();

    return "{".to_owned() + split_request[1];
}

fn create_successful_response(body: &str) -> String {
    return format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", body.len(), body);
}

fn send_response(mut stream: &TcpStream, response: &str){
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
