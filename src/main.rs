#![warn(clippy::pedantic)]

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
    let request = Request::parse(lines);
    let response = if request.resource == "/" && request.method == Method::Get {
        Response {
            status_code: StatusCode::Ok,
            body: read_to_string("hello.html").unwrap(),
        }
    } else {
        Response {
            status_code: StatusCode::NotFound,
            body: "Not Found!".into(),
        }
    };
    response.send(&mut stream);
}

fn read_lines(stream: &TcpStream) -> Vec<String> {
    let buf_reader = BufReader::new(stream);
    buf_reader
        .lines()
        .map(|res| res.unwrap())
        .take_while(|line| !line.is_empty())
        .collect()
}

struct Request {
    resource: String,
    method: Method,
}

impl Request {
    fn parse(lines: Vec<String>) -> Self {
        let first = lines.into_iter().next().unwrap();
        let resource = first.split_whitespace().nth(1).unwrap();
        Self {
            resource: resource.into(),
            method: Method::Get,
        }
    }
}

#[derive(PartialEq)]
enum Method {
    Get,
}

struct Response {
    status_code: StatusCode,
    body: String,
}

impl Response {
    fn send(&self, stream: &mut TcpStream) {
        let status_line = format!(
            "HTTP/1.1 {} {}",
            self.status_code.num(),
            self.status_code.text()
        );
        let length = self.body.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{}",
            self.body
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}

enum StatusCode {
    Ok,
    NotFound,
}

impl StatusCode {
    fn num(&self) -> u32 {
        match self {
            Self::Ok => 200,
            Self::NotFound => 404,
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Ok => "OK",
            Self::NotFound => "Not Found",
        }
        .into()
    }
}
