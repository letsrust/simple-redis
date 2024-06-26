use anyhow::Result;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let addr = "0.0.0.0:6379";
    info!("Simple-Redis-Server is listening on {}", addr);
    let listener = TcpListener::bind(addr).await?;

    loop {
        let (_socket, raddr) = listener.accept().await?;
        tokio::spawn(async move {
            // if let Err(e) = crate::connection::process(socket).await {
            //  info!("connection error: {:?}", e)
            // }
            info!("connection from: {:?}", raddr);
        });
    }
}
