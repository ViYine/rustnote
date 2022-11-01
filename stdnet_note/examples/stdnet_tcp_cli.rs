use std::{net::TcpStream, io::{Write, Read}};

fn main() {
    // tcp stream 通过connect 连接到指定的地址
    let mut stream = TcpStream::connect("0.0.0.0:9527").unwrap();
    // write all data to stream
    stream.write_all(b"ping").unwrap();
    // 定义一个缓冲区
    let mut buf = [0u8; 4];
    // 读取指定长度的数据
    stream.read_exact(&mut buf).unwrap();
    println!("Received: {}", String::from_utf8_lossy(&buf));
}
