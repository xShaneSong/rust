use std::net::TcpStream;
use std::io::{Read, Write};

fn main() {
    let mut _stream = TcpStream::connect("localhost:3000").unwrap_or_else(|e| {
        eprintln!("Failed to connect to server: {}", e);
        std::process::exit(1);
    });
    println!("Connected to server at localhost:3000");

    _stream.write(b"Hello, server!").unwrap_or_else(|e| {
        eprintln!("Failed to write to stream: {}", e);
        std::process::exit(1);
    });

    let mut buffer = [0; 1024];
    let bytes_read = _stream.read(&mut buffer).unwrap_or_else(|e| {
        eprintln!("Failed to read from stream: {}", e);
        std::process::exit(1);
    });
    
    // 只处理实际读取到的字节数，并去除换行符等空白字符
    let received_data = str::from_utf8(&buffer[..bytes_read])
        .unwrap()
        .trim(); // trim() 会去除开头和结尾的空白字符（包括 \n, \r, 空格等）
    
    println!("Received data: {}", received_data);
}