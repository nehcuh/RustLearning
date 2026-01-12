use crate::codec::ProstCodec;
use crate::pb::abi::{CommandRequest, CommandResponse};
use crate::service::Service;
use futures::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_util::codec::Framed;

/// 网络服务器
pub struct Server {
    addr: String,
    service: Arc<Service>,
}

impl Server {
    pub fn new(addr: impl Into<String>, service: Service) -> Self {
        Self {
            addr: addr.into(),
            service: Arc::new(service),
        }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New connection from {}", addr);

            let service = self.service.clone();
            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, service).await {
                    eprintln!("Connection error: {}", e);
                }
            });
        }
    }

    async fn handle_connection(
        stream: tokio::net::TcpStream,
        service: Arc<Service>,
    ) -> anyhow::Result<()> {
        let mut framed = Framed::new(
            stream,
            ProstCodec::<CommandResponse, CommandRequest>::default(),
        );

        while let Some(result) = framed.next().await {
            match result {
                Ok(cmd) => {
                    println!("Received command: {:?}", cmd);
                    let response = service.execute(cmd);
                    framed.send(response).await?;
                }
                Err(e) => {
                    eprintln!("Error decoding request: {}", e);
                    return Err(anyhow::anyhow!("Decode error: {}", e));
                }
            }
        }

        Ok(())
    }
}

/// 网络客户端
pub struct Client {
    addr: String,
}

impl Client {
    pub fn new(addr: impl Into<String>) -> Self {
        Self { addr: addr.into() }
    }

    pub async fn connect(
        &self,
    ) -> anyhow::Result<Framed<tokio::net::TcpStream, ProstCodec<CommandRequest, CommandResponse>>>
    {
        let stream = tokio::net::TcpStream::connect(&self.addr).await?;
        Ok(Framed::new(
            stream,
            ProstCodec::<CommandRequest, CommandResponse>::default(),
        ))
    }

    pub async fn send_command(&self, cmd: CommandRequest) -> anyhow::Result<CommandResponse> {
        let mut framed = self.connect().await?;
        framed.send(cmd).await?;

        match framed.next().await {
            Some(Ok(response)) => Ok(response),
            Some(Err(e)) => Err(anyhow::anyhow!("Response error: {}", e)),
            None => Err(anyhow::anyhow!("Connection closed")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::abi::CommandRequest;
    use crate::pb::abi::value::Value;

    #[tokio::test]
    async fn test_server_client() {
        let addr = "127.0.0.1:9877";
        let service = Service::default();

        let server = Server::new(addr, service);
        tokio::spawn(async move {
            if let Err(e) = server.run().await {
                eprintln!("Server error: {}", e);
            }
        });

        // 给服务器一些时间启动
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let client = Client::new(addr);
        let cmd = CommandRequest::new_hset("test", "key1", Value::String("value1".to_string()));

        let response = client.send_command(cmd).await;
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.status, 200);
    }
}
