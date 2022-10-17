
use std::str;
use std::thread;
use std::net::UdpSocket;

fn main() {

    let socket = match UdpSocket::bind("0.0.0.0:5514") {
        Ok(s) => s,
        Err(e) => panic!("couldn't bind socket: {}", e)
    };
        //Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
    let mut buf = [0; 10];
      // Redeclare `buf` as slice of the received data and send reverse data back to origin
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                thread::spawn(move || {
                    println!("amt: {}", amt);
                    println!("src: {}", src);
                    println!("{}", str::from_utf8(&buf).unwrap_or(""));
                });
            },
            Err(e) => {
                println!("couldn't recieve a datagram: {}", e);
            }
        }
    }
}


