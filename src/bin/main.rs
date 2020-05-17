use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use simpleserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    if buffer.starts_with(get) {
        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\n\r\n{}", contents);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
