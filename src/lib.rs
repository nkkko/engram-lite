pub mod schema;
pub mod storage;
pub mod graph;
pub mod error;
pub mod utils;
pub mod index;
pub mod export;
pub mod query;
pub mod embedding;
pub mod vector_search;
pub mod dimension_reduction;
pub mod demo;
#[cfg(feature = "grpc")]
pub mod grpc;
#[cfg(feature = "python")]
pub mod python;
#[cfg(test)]
mod schema_test;
#[cfg(test)]
mod index_test;
#[cfg(test)]
mod graph_test;

// Re-export core types for convenience
pub use schema::{Agent, Collection, Connection, Context, Engram};
pub use storage::Storage;
pub use graph::MemoryGraph;
pub use error::{EngramError, Result};
pub use utils::{load_env_from_file, get_anthropic_api_key};
pub use index::{RelationshipIndex, MetadataIndex, SearchIndex, CollectionIndex, TextIndex};
pub use export::{export_to_file, import_from_file, export_collection_to_file, import_partial_from_file, ExportData};
pub use query::{EngramQuery, RelationshipQuery, QueryEngine, TraversalEngine, QueryService, TraversalResult};
pub use embedding::{Embedding, EmbeddingModel, EmbeddingService, EmbeddingCache, HnswIndex};
pub use vector_search::{VectorIndex, VectorQuery, HybridQuery, HybridSearchEngine, HybridSearchResult, CombinationMethod};
pub use dimension_reduction::{DimensionReducer, ReductionMethod};
pub use demo::populate_demo_data;

#[cfg(feature = "grpc")]
pub use grpc::server::GrpcServer;