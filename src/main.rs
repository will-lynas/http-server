use std::{
    io::{BufRead, BufReader},
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

fn handle_stream(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let lines: Vec<_> = buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Req: {lines:#?}");
}
