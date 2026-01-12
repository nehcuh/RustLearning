use kv::network::Server;
use kv::service::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Service::default();
    let server = Server::new("127.0.0.1:9876", service);
    server.run().await?;
    Ok(())
}
