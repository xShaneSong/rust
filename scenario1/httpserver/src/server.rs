use super::router::Router;
use http::httprequest::HttpRequest;
use std::io::prelude::*;
use std::net::{TcpListener};
use std::str;

// HTTP 服务器结构体
// HTTP Server struct
pub struct Server<'a> {
    socket_addr: &'a str, // 服务器绑定的套接字地址
}

impl<'a> Server<'a> {
    // 创建新的服务器实例
    // Create a new server instance
    pub fn new(socket_addr: &'a str) -> Self {
        Server { socket_addr }
    }
    
    // 启动并运行 HTTP 服务器
    // Start and run the HTTP server
    pub fn run(&self) {
        println!("Starting HTTP server...");
        
        // 绑定到指定的套接字地址并创建监听器
        // Bind to the specified socket address and create a listener
        let connection_listener = TcpListener::bind(self.socket_addr).unwrap();
        println!("Running HTTP server on {}", self.socket_addr);

        // 持续监听传入的连接
        // Continuously listen for incoming connections
        for stream in connection_listener.incoming() {
            // 获取连接流
            // Get the connection stream
            let mut stream = stream.unwrap();
            println!("Connection established");

            // 创建读取缓冲区
            // Create a read buffer
            let mut read_buffer = [0; 1024];
            
            // 从流中读取数据到缓冲区
            // Read data from the stream into the buffer
            stream.read(&mut read_buffer).unwrap();

            // 将字节缓冲区转换为字符串，然后转换为 HttpRequest
            // Convert byte buffer to string, then to HttpRequest
            let req: HttpRequest = String::from_utf8(read_buffer.to_vec())
                .unwrap()  // 将 Vec<u8> 转换为 String
                .into();   // 将 String 转换为 HttpRequest
            
            // 使用路由器处理请求并发送响应
            // Use router to handle the request and send response
            Router::route(req, &mut stream);
        }
    }
}