use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind address");

    socket.connect("127.0.0.1:3000").expect("Could not connect to server");

    socket.send("Hello: send data".as_bytes()).expect("Could not write data");
}