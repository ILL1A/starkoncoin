use tonic::{transport::Server, Request, Response, Status};

use blockchain_service::blockchain_server::{Blockchain, BlockchainServer};
use blockchain_service::{TransactionRequest, TransactionResponse};

use crate::node::Node;
use std::sync::Arc;

pub mod blockchain_service {
  tonic::include_proto!("blockchain");
}

#[derive(Default)]
pub struct BlockchainService{}

#[tonic::async_trait]
impl Blockchain for BlockchainService {
  async fn send_transaction(
    &self,
    request: Request<TransactionRequest>
  ) -> Result<Response<TransactionResponse>, Status> {
    println!("Received request: {:?}", request);
    let response = TransactionResponse {
      message: "Transaction received".into(),
    };
    Ok(Response::new(response))
  }
}

impl Node {
  pub async fn run_server(self: Arc<Box<Self>>) -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = format!("{}:{}", self.options.addr, self.options.port).parse().unwrap();
    let svc = BlockchainService::default();
  
    println!("Blockchain server listening on {}", addr);

    Server::builder()
      .add_service(BlockchainServer::new(svc))
      .serve(addr)
      .await?;

    println!("Blockchain server stopped");
  
    Ok(())
  }
}
