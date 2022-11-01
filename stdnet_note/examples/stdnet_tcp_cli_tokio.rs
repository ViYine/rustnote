use bytes::Bytes;
use tokio::net::TcpStream;
use anyhow::Result;
use tokio_util::codec::Framed;
use tokio_util::codec::LengthDelimitedCodec;
use futures::{SinkExt, StreamExt};


#[tokio::main]
async fn main() -> Result<()> {
    // tcp stream 通过connect 连接到指定的地址
    let stream = TcpStream::connect("0.0.0.0:9527").await?;
    // write all data to stream
    let mut stream = Framed::new(stream, LengthDelimitedCodec::new());
    stream.send(Bytes::from("ping")).await?;
    if let Some(Ok(data)) = stream.next().await {
        println!("Received: {}", String::from_utf8_lossy(&data));
        // write all data to back
        // stream.send(Bytes::from("pong")).await?;
    }
    Ok(())
}
