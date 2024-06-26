use crate::RespFrame;
use anyhow::Result;
use tokio::net::TcpStream;

#[allow(dead_code)]
async fn stream_handler(_stream: TcpStream) -> Result<()> {
    todo!()
}

#[allow(dead_code)]
async fn request_handler() -> Result<RespFrame> {
    todo!()
}
