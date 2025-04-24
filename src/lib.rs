pub mod schema;
pub mod storage;
pub mod graph;
pub mod error;
pub mod utils;
pub mod index;
pub mod export;
#[cfg(test)]
mod schema_test;
#[cfg(test)]
mod index_test;

// Re-export core types for convenience
pub use schema::{Agent, Collection, Connection, Context, Engram};
pub use storage::Storage;
pub use graph::MemoryGraph;
pub use error::{EngramError, Result};
pub use utils::{load_env_from_file, get_anthropic_api_key};
pub use index::{RelationshipIndex, MetadataIndex, SearchIndex, CollectionIndex};
pub use export::{export_to_file, import_from_file, export_collection_to_file, import_partial_from_file, ExportData};