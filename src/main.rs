pub mod server;

#[tokio::main]
async fn main() -> std::io::Result<()>{
    server::run("127.0.0.1:8172").await
}
