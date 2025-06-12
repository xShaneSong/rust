use std::net::TcpListener;

fn main() {
    // 绑定到本地地址和端口
    let listener = TcpListener::bind("0.0.0.0:3000").unwrap_or_else(|e| {
        eprintln!("Failed to bind to address: {}", e);
        std::process::exit(1);
    });
    println!("Server is listening on {}", listener.local_addr().unwrap());
    // 接受传入的连接
    for stream in listener.incoming() {
        let _stream = stream.as_ref().map_err(|e| {
            eprintln!("Error accepting connection: {}", e);
            e
        });
        println!("Incoming connection: {:?}", _stream);
    }
}