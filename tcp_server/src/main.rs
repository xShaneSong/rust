use std::net::{SocketAddr, TcpListener};
use std::io::{Read, Write};

fn handle_client(mut stream: std::net::TcpStream) {
    // 处理客户端连接
    // 在这里可以读取请求并发送响应
    println!("New client connected: {:?}", stream.peer_addr());

    stream.read(&mut [0; 1024])
            .unwrap_or_else(|e| {
                eprintln!("Failed to read from stream: {}", e);
            });

    let message = b"Hello from the server!";
    stream.write_all(message).unwrap_or_else(|e| {
        eprintln!("Failed to write to stream: {}", e);
    });
    println!("Response sent to client.");
}

fn main() {
    // let addrs = [
    //     SocketAddr::from(([0, 0, 0, 0], 8080)),
    //     SocketAddr::from(([0, 0, 0, 0], 8443)),
    // ];

    let addrs = SocketAddr::from(([0, 0, 0, 0], 8080)).to_string();
    let listener = TcpListener::bind(addrs).unwrap_or_else(|e| {
        // let listener = TcpListener::bind(&addrs[..]).unwrap_or_else(|e| {
        eprintln!("Failed to bind to addresses: {}", e);
        std::process::exit(1);
    });

    println!("local_addr: {:?}", listener.local_addr());
    println!("ttl: {:?}", listener.ttl());

    // blocking
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // non-blocking
    // use std::io;
    // listener
    //     .set_nonblocking(true)
    //     .expect("Cannot set non-blocking");

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(s) => {
    //             // do something with the TcpStream
    //             handle_client(s);
    //         }
    //         Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    //             // wait until network socket is ready, typically implemented
    //             // via platform-specific APIs such as epoll or IOCP
    //             // wait_for_fd();
    //             continue;
    //         }
    //         Err(e) => panic!("encountered IO error: {e}"),
    //     }
    // }
}
