for i in 0..num_cpus {
    tokio::spawn(async move {
        let mut buf_udp = [0u8; MAX_PACKET_LEN];
        let udp_sock = new_udp_reuseport(local_addr);
        udp_sock.connect(addr).await.unwrap();

        loop {
            if Ok(size) = udp_sock.recv(&mut buf_udp) {
                println!("{:?} bytes received" size);
            }   
        }   
    }   
}




