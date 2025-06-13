use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(socket_addr: &'a str) -> Self {
        let connection_listener = TcpListener::bind(socket_addr).unwrap();
        println!("Running HTTP server on {}", socket_addr);

        for stream in connection_listener.incoming() {
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut read_buffer = [0; 1024];
            stream.read(&mut read_buffer).unwrap();

            let req: HttpRequest = String::from_utf8(read_buffer.to_vec())
                .unwrap()
                .into();
            Router::route(req, &mut stream);
        }
    }
}