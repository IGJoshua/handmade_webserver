use std::io::prelude::*;
use std::net::{TcpStream, TcpListener};
use std::collections::HashMap;
use html::*;
use http::*;

pub mod html;
pub mod http;

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let html = Html {
        root_node: HtmlDom::Html(Lang::En, vec![
            HtmlDom::Head(vec![
                HtmlDom::Meta(Some(Charset::Utf8), vec![]),
                HtmlDom::Title(vec![
                    HtmlDom::Text(String::from("Hello!"))])]),
            HtmlDom::Body(vec![
                HtmlDom::H1(vec![
                    HtmlDom::Text(String::from("Hello!"))]),
                HtmlDom::P(vec![
                    HtmlDom::Text(String::from("Hi from Rust!"))])])])};

    let mut buffer = [0; 512];

    let len = stream.read(&mut buffer).unwrap();

    let request: Request = String::from_utf8_lossy(&buffer[..len]).parse().unwrap();
    let response = Response {
        code: 200,
        message: String::from("OK"),
        headers: HashMap::new(),
        body: html.to_string(),
    }.to_string();

    stream.write(&response.into_bytes()).unwrap();

    println!("{}", request);
}
