use std::{
    fs::read_to_string,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4242").unwrap();
    println!("Listening on 127.0.0.1:4242");
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("New connection: {}", stream.peer_addr().unwrap());
        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let lines = read_lines(&stream);
    println!("Req: {lines:#?}");
    send_response(&mut stream);
}

fn read_lines(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

fn send_response(stream: &mut TcpStream) {
    let status_line = "HTTP/1.1 200 OK";
    let body = read_to_string("hello.html").unwrap();
    let length = body.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{body}");

    stream.write_all(response.as_bytes()).unwrap();
}
