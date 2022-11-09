use std::{
    io::{Read, Write},
    net::TcpListener,
    thread,
};

fn main() {
    // 通过bind 创建一个 listener
    let listener = TcpListener::bind("0.0.0.0:9527").unwrap();

    loop {
        // accept 会阻塞当前线程，直到有新的连接进来，返回一个新的socket
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Connection from {}", addr);
        // therad spawn 会创建一个新的线程，处理新的连接
        thread::spawn(move || {
            // 定义一个缓冲区
            let mut buf = [0u8; 4];
            // read_ecact 需要知道读取的数据的长度
            stream.read_exact(&mut buf).unwrap();
            println!("Received: {}", String::from_utf8_lossy(&buf));
            // write all data to back
            stream.write_all(b"pong").unwrap();
        });
    }
}
