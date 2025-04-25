#[cfg(feature = "grpc")]
use crate::grpc::proto::engram_service_server::EngramServiceServer;
use crate::grpc::service::EngramServiceImpl;
use crate::storage::Storage;
use crate::embedding::EmbeddingService;
use crate::index::SearchIndex;
use crate::vector_search::VectorIndex;
use crate::error::Result;

use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;

#[cfg(feature = "grpc")]
pub struct GrpcServer {
    addr: SocketAddr,
    storage: Arc<Storage>,
    search_index: Arc<SearchIndex>,
    vector_index: Arc<VectorIndex>,
    embedding_service: Arc<EmbeddingService>,
}

#[cfg(feature = "grpc")]
impl GrpcServer {
    pub fn new(
        addr: SocketAddr,
        storage: Arc<Storage>,
        search_index: Arc<SearchIndex>,
        vector_index: Arc<VectorIndex>,
        embedding_service: Arc<EmbeddingService>,
    ) -> Self {
        Self {
            addr,
            storage,
            search_index,
            vector_index,
            embedding_service,
        }
    }
    
    pub async fn run(&self) -> Result<()> {
        let service = EngramServiceImpl::new(
            self.storage.clone(),
            self.search_index.clone(),
            self.vector_index.clone(),
            self.embedding_service.clone(),
        );
        
        let engram_service = EngramServiceServer::new(service);
        
        println!("Starting gRPC server on {}", self.addr);
        
        Server::builder()
            .add_service(engram_service)
            .serve(self.addr)
            .await
            .map_err(|e| crate::error::EngramError::Generic(format!("gRPC server error: {}", e)))
    }
}