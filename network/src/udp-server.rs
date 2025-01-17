use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:3000").expect("Could not bind address");
    let mut buffer = [0; 1024];

    loop {
        let socket_new = socket.try_clone().expect("Could not clone socket");
        match socket_new.recv_from(&mut buffer) {
            Ok((number_of_bytes, src_addr)) => {

                thread::spawn(move || {
                    let received = str::from_utf8(&buffer[..number_of_bytes])
                        .expect("Could not convert buffer to string");
                    println!("Received from {}: {}", src_addr, received);

                    let response = format!("Hello, {}", received);
                    let bytes = response.as_bytes();
                    socket_new
                        .send_to(bytes, &src_addr)
                        .expect("Could not write response");
                });
            }
            Err(e) => {
                eprintln!("Could not receive a datagram: {}", e);
            }
        }
    }
}
