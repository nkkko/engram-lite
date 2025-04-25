#[cfg(feature = "grpc")]
use crate::grpc::proto::{
    engram_service_server::EngramService, CreateCollectionRequest, CreateCollectionResponse,
    CreateConnectionRequest, CreateConnectionResponse, CreateEngramRequest, CreateEngramResponse,
    DeleteCollectionRequest, DeleteCollectionResponse, DeleteConnectionRequest,
    DeleteConnectionResponse, DeleteEngramRequest, DeleteEngramResponse, FindConnectionsRequest,
    FindConnectionsResponse, GenerateEmbeddingRequest, GenerateEmbeddingResponse, GetCollectionRequest,
    GetCollectionResponse, GetConnectionRequest, GetConnectionResponse, GetEngramRequest,
    GetEngramResponse, ListCollectionsRequest, ListCollectionsResponse, ListConnectionsRequest,
    ListConnectionsResponse, ListEngramsRequest, ListEngramsResponse, SearchByTextRequest,
    SearchByVectorRequest, SearchHybridRequest, SearchResponse, Status, UpdateCollectionRequest,
    UpdateCollectionResponse, UpdateConnectionRequest, UpdateConnectionResponse, UpdateEngramRequest,
    UpdateEngramResponse,
};
use crate::storage::Storage;
use crate::embedding::{EmbeddingService, EmbeddingModel};
use crate::index::SearchIndex;
use crate::vector_search::VectorIndex;
use crate::error::Result as EngramResult;
use crate::grpc::conversion;
use crate::grpc::error::to_status;

use std::sync::Arc;
use tonic::{Request, Response, Status as TonicStatus};

#[cfg(feature = "grpc")]
pub struct EngramServiceImpl {
    storage: Arc<Storage>,
    search_index: Arc<SearchIndex>,
    vector_index: Arc<VectorIndex>,
    embedding_service: Arc<EmbeddingService>,
}

#[cfg(feature = "grpc")]
impl EngramServiceImpl {
    pub fn new(
        storage: Arc<Storage>,
        search_index: Arc<SearchIndex>,
        vector_index: Arc<VectorIndex>,
        embedding_service: Arc<EmbeddingService>,
    ) -> Self {
        Self {
            storage,
            search_index,
            vector_index,
            embedding_service,
        }
    }

    fn create_success_status() -> Status {
        Status {
            code: 0, // Success
            message: "OK".to_string(),
        }
    }
}

#[cfg(feature = "grpc")]
#[tonic::async_trait]
impl EngramService for EngramServiceImpl {
    async fn create_engram(
        &self,
        request: Request<CreateEngramRequest>,
    ) -> Result<Response<CreateEngramResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Convert request to domain model
        let engram = conversion::create_engram_request_to_engram(req)?;
        
        // Store the engram
        match self.storage.put_engram(&engram) {
            Ok(_) => {
                // Add to search index
                if let Err(err) = self.search_index.add_engram(&engram) {
                    return Err(to_status(err));
                }
                
                // Add to vector index
                if let Err(err) = self.vector_index.add_engram(&engram) {
                    return Err(to_status(err));
                }
                
                // Create response
                let engram_proto = conversion::engram_to_proto(&engram)?;
                let response = CreateEngramResponse {
                    status: Some(Self::create_success_status()),
                    engram: Some(engram_proto),
                };
                
                Ok(Response::new(response))
            }
            Err(err) => Err(to_status(err)),
        }
    }

    async fn get_engram(
        &self,
        request: Request<GetEngramRequest>,
    ) -> Result<Response<GetEngramResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Get the engram
        match self.storage.get_engram(&req.id) {
            Ok(Some(engram)) => {
                // Convert to proto
                let engram_proto = conversion::engram_to_proto(&engram)?;
                let response = GetEngramResponse {
                    status: Some(Self::create_success_status()),
                    engram: Some(engram_proto),
                };
                
                Ok(Response::new(response))
            }
            Ok(None) => {
                Err(TonicStatus::not_found(format!("Engram with ID {} not found", req.id)))
            }
            Err(err) => Err(to_status(err)),
        }
    }

    async fn update_engram(
        &self,
        request: Request<UpdateEngramRequest>,
    ) -> Result<Response<UpdateEngramResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Check if engram exists
        let engram_id = req.id.clone();
        match self.storage.get_engram(&engram_id) {
            Ok(Some(mut engram)) => {
                // Update engram fields
                if !req.content.is_empty() {
                    engram.content = req.content;
                }
                if !req.source.is_empty() {
                    engram.source = req.source;
                }
                if req.confidence > 0.0 {
                    engram.confidence = req.confidence;
                }
                if let Some(metadata) = req.metadata {
                    engram.metadata = conversion::struct_to_serde_value(&metadata)?;
                }
                
                // Store updated engram
                if let Err(err) = self.storage.put_engram(&engram) {
                    return Err(to_status(err));
                }
                
                // Update search index
                if let Err(err) = self.search_index.add_engram(&engram) {
                    return Err(to_status(err));
                }
                
                // Update vector index (remove and add)
                if let Err(err) = self.vector_index.remove_engram(&engram.id) {
                    return Err(to_status(err));
                }
                if let Err(err) = self.vector_index.add_engram(&engram) {
                    return Err(to_status(err));
                }
                
                // Create response
                let engram_proto = conversion::engram_to_proto(&engram)?;
                let response = UpdateEngramResponse {
                    status: Some(Self::create_success_status()),
                    engram: Some(engram_proto),
                };
                
                Ok(Response::new(response))
            }
            Ok(None) => {
                Err(TonicStatus::not_found(format!("Engram with ID {} not found", engram_id)))
            }
            Err(err) => Err(to_status(err)),
        }
    }

    async fn delete_engram(
        &self,
        request: Request<DeleteEngramRequest>,
    ) -> Result<Response<DeleteEngramResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Delete from search index
        if let Err(err) = self.search_index.remove_engram_by_id(&req.id) {
            return Err(to_status(err));
        }
        
        // Delete from vector index
        if let Err(err) = self.vector_index.remove_engram(&req.id) {
            return Err(to_status(err));
        }
        
        // Delete from storage
        match self.storage.delete_engram(&req.id) {
            Ok(_) => {
                let response = DeleteEngramResponse {
                    status: Some(Self::create_success_status()),
                    deleted: true,
                };
                
                Ok(Response::new(response))
            }
            Err(err) => Err(to_status(err)),
        }
    }

    async fn list_engrams(
        &self,
        request: Request<ListEngramsRequest>,
    ) -> Result<Response<ListEngramsResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Get engrams (using page size and token for pagination)
        let page_size = if req.page_size > 0 { req.page_size as usize } else { 50 };
        
        // TODO: Implement proper pagination with page token
        match self.storage.list_engrams() {
            Ok(engram_ids) => {
                let mut engrams = Vec::new();
                
                // Load each engram
                for engram_id in engram_ids.iter().take(page_size) {
                    if let Ok(Some(engram)) = self.storage.get_engram(engram_id) {
                        if let Ok(engram_proto) = conversion::engram_to_proto(&engram) {
                            engrams.push(engram_proto);
                        }
                    }
                }
                
                // Create next page token (just a placeholder for now)
                let next_page_token = if engram_ids.len() > page_size {
                    "more".to_string()
                } else {
                    "".to_string()
                };
                
                let response = ListEngramsResponse {
                    status: Some(Self::create_success_status()),
                    engrams,
                    next_page_token,
                };
                
                Ok(Response::new(response))
            }
            Err(err) => Err(to_status(err)),
        }
    }

    // Connection operations (these would be implemented similarly to the engram operations)
    async fn create_connection(
        &self,
        _request: Request<CreateConnectionRequest>,
    ) -> Result<Response<CreateConnectionResponse>, TonicStatus> {
        // TODO: Implement connection creation
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn get_connection(
        &self,
        _request: Request<GetConnectionRequest>,
    ) -> Result<Response<GetConnectionResponse>, TonicStatus> {
        // TODO: Implement connection retrieval
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn update_connection(
        &self,
        _request: Request<UpdateConnectionRequest>,
    ) -> Result<Response<UpdateConnectionResponse>, TonicStatus> {
        // TODO: Implement connection update
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn delete_connection(
        &self,
        _request: Request<DeleteConnectionRequest>,
    ) -> Result<Response<DeleteConnectionResponse>, TonicStatus> {
        // TODO: Implement connection deletion
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn list_connections(
        &self,
        _request: Request<ListConnectionsRequest>,
    ) -> Result<Response<ListConnectionsResponse>, TonicStatus> {
        // TODO: Implement connection listing
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    // Collection operations
    async fn create_collection(
        &self,
        _request: Request<CreateCollectionRequest>,
    ) -> Result<Response<CreateCollectionResponse>, TonicStatus> {
        // TODO: Implement collection creation
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn get_collection(
        &self,
        _request: Request<GetCollectionRequest>,
    ) -> Result<Response<GetCollectionResponse>, TonicStatus> {
        // TODO: Implement collection retrieval
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn update_collection(
        &self,
        _request: Request<UpdateCollectionRequest>,
    ) -> Result<Response<UpdateCollectionResponse>, TonicStatus> {
        // TODO: Implement collection update
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn delete_collection(
        &self,
        _request: Request<DeleteCollectionRequest>,
    ) -> Result<Response<DeleteCollectionResponse>, TonicStatus> {
        // TODO: Implement collection deletion
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn list_collections(
        &self,
        _request: Request<ListCollectionsRequest>,
    ) -> Result<Response<ListCollectionsResponse>, TonicStatus> {
        // TODO: Implement collection listing
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    // Search operations
    async fn search_by_text(
        &self,
        _request: Request<SearchByTextRequest>,
    ) -> Result<Response<SearchResponse>, TonicStatus> {
        // TODO: Implement text search
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn search_by_vector(
        &self,
        _request: Request<SearchByVectorRequest>,
    ) -> Result<Response<SearchResponse>, TonicStatus> {
        // TODO: Implement vector search
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn search_hybrid(
        &self,
        _request: Request<SearchHybridRequest>,
    ) -> Result<Response<SearchResponse>, TonicStatus> {
        // TODO: Implement hybrid search
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn find_connections(
        &self,
        _request: Request<FindConnectionsRequest>,
    ) -> Result<Response<FindConnectionsResponse>, TonicStatus> {
        // TODO: Implement connection search
        Err(TonicStatus::unimplemented("Not yet implemented"))
    }

    async fn generate_embedding(
        &self,
        request: Request<GenerateEmbeddingRequest>,
    ) -> Result<Response<GenerateEmbeddingResponse>, TonicStatus> {
        let req = request.into_inner();
        
        // Use the embedding service to generate an embedding
        let mut embedding_service = self.embedding_service.clone();
        
        // Apply request options
        if let Some(model) = conversion::proto_to_embedding_model(req.model) {
            // Create new embedding service with specified model type
            embedding_service = Arc::new(EmbeddingService::with_model_type(model));
        }
        
        // Create a new EmbeddingService with the desired options
        let model = embedding_service.get_model_type().unwrap_or(EmbeddingModel::E5MultilingualLargeInstruct);
        let mut builder = EmbeddingService::builder().model_type(model);
        
        // Apply normalization if requested
        if req.normalize {
            builder = builder.normalize(true);
        }
        
        // Apply instruction prefix if requested
        if req.use_instruction_prefix {
            builder = builder.use_instruction_prefix(true);
        }
        
        // Build the new embedding service
        embedding_service = Arc::new(builder.build());
        
        // Generate the embedding
        match embedding_service.embed_text(&req.text) {
            Ok(embedding) => {
                // Convert to proto
                let embedding_proto = conversion::embedding_to_proto(&embedding)?;
                let response = GenerateEmbeddingResponse {
                    status: Some(Self::create_success_status()),
                    embedding: Some(embedding_proto),
                };
                
                Ok(Response::new(response))
            }
            Err(err) => Err(to_status(err)),
        }
    }
}