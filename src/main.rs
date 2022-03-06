use std::{fs, io};
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process;
use std::str::from_utf8;
use std::sync::Arc;

use clap::Parser;
use serde_json::Value;

use args::Args;
use request_methods::RequestMethods;
use thread_pool::ThreadPool;

mod request_methods;
mod args;
mod thread_pool;

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: &str = "8080";

fn main() {
    let args = Args::parse();

    let host = args.host.unwrap_or(DEFAULT_HOST.to_string());
    let port = args.port.unwrap_or(DEFAULT_PORT.to_string());

    let json_content = match fs::read_to_string(&args.json_filename) {
        Ok(json_content) => json_content,
        Err(_) => {
            println!("\tFailed to read the JSON file");
            process::exit(0);
        }
    };

    let json_value: Value = match serde_json::from_str(&*json_content) {
        Ok(json_value) => json_value,
        Err(_) => {
            println!("\tFailed to parse the JSON file");
            process::exit(0);
        }
    };
    let json_value = Arc::new(json_value);

    let listener = match TcpListener::bind(format!("{}:{}", DEFAULT_HOST, port)) {
        Ok(listener) => {
            println!("\tServer started at http://{}:{}", host, port);
            listener
        }
        Err(_) => {
            println!("\tServer failed to start");
            process::exit(0);
        }
    };

    let pool = ThreadPool::new(4);

    for stream_result in listener.incoming() {
        let stream = match stream_result {
            Ok(tcp_stream) => tcp_stream,
            Err(_) => continue,
        };
        let arc_json_value = Arc::clone(&json_value);
        pool.execute(move || {
            handle_connection(stream, arc_json_value).unwrap_or_default();
        });
    }
}

fn handle_connection(mut stream: TcpStream, json_value: Arc<Value>) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let request_result: Result<&str, Error> = match from_utf8(&buffer) {
        Ok(request) => Ok(request),
        Err(e) => Err(io::Error::new(ErrorKind::Other, e))
    };
    let request = request_result?;
    let request_type: &str = RequestMethods::get_request_type(request);

    match request_type {
        RequestMethods::GET => {
            // TODO: very very naive implementation, it just sends the second post
            let response = create_successful_response(&json_value["posts"][1].to_string());

            send_response(&stream, &response)
        }

        RequestMethods::POST => {
            let body: String = get_request_body(request);
            let response = create_successful_response(&body);

            send_response(&stream, &response)
        }

        _ => {
            let response = "HTTP/1.1 405\r\nContent-Length: 0\r\n\r\n";
            send_response(&stream, &response)
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

fn send_response(mut stream: &TcpStream, response: &str) -> io::Result<()> {
    stream.write(response.as_bytes())?;
    stream.flush()
}
