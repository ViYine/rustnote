use anyhow::Result;
use bytes::Bytes;
use futures::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

#[tokio::main]
async fn main() -> Result<()> {
    // 通过bind 创建一个 listener
    let listener = TcpListener::bind("0.0.0.0:9527").await?;

    loop {
        // accept 会阻塞当前线程，直到有新的连接进来，返回一个新的socket
        let (stream, addr) = listener.accept().await?;
        println!("Connection from {}", addr);
        // tokio frame wrap
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
        // therad spawn 会创建一个新的线程，处理新的连接
        tokio::spawn(async move {
            // get read data from steam
            // 从stream中读取数据 需要use futures::{SinkExt, StreamExt};
            while let Some(Ok(data)) = stream.next().await {
                println!("Received: {}", String::from_utf8_lossy(&data));
                // write all data to back
                stream.send(Bytes::from("pong")).await.unwrap();
            }
        });
    }
}
