use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn run(addr: &str) -> std::io::Result<()> {
    let listener = TcpListener::bind(addr).await?;

    println!("rkvdb listening on {}", addr);

    loop {
        let (socket, peer_addr) = listener.accept().await?;

        println!("client connected: {peer_addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("connection error: {e}");
            }
        });
    }
}

async fn handle_connection(mut socket: TcpStream) -> std::io::Result<()> {
    let mut buf = [0_u8; 1024];

    loop {
        let n = socket.read(&mut buf).await?;

        if n == 0 {
            return Ok(());
        }

        println!("received: {:?}", String::from_utf8_lossy(&buf[..n]));
        socket.write_all(&buf[..n]).await?;
    }
}