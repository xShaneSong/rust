use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // 绑定到本地地址和端口
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap_or_else(|e| {
        eprintln!("Failed to bind to address: {}", e);
        std::process::exit(1);
    });
    println!("Server is listening on {}", listener.local_addr().unwrap());
    // 接受传入的连接
    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        println!("Incoming connection: {:?}", _stream);

        let mut buffer = [0; 1024];
        _stream.read(&mut buffer)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to read from stream: {}", e);
                    std::process::exit(1);
                });
        _stream.write(&mut buffer)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to write to stream: {}", e);
                    std::process::exit(1);
                });
        println!("Received data: {:?}", String::from_utf8_lossy(&buffer));
    }
}