use std::net::SocketAddr;
use proto::{KeyGenerator, KeyGeneratorServer};
use tonic::{
    transport::{self, Server},
    Response, Status,
};

#[derive(Debug, Default, Clone)]
pub struct KeyGeneratorService;

#[tonic::async_trait]
impl KeyGenerator for KeyGeneratorService {
    async fn generate(
        &self,
        request: tonic::Request<proto::KeyGenerateRequest>,
    ) -> Result<Response<proto::KeyGenerateResponse>, Status> {
        todo!()
    }
}

pub fn server(
    addr: SocketAddr,
) -> impl Future<Output = Result<(), transport::Error>> {
    Server::builder()
        .add_service(KeyGeneratorServer::new(KeyGeneratorService))
        .serve(addr)
}
