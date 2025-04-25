use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum EngramError {
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Connection error: Source or target engram does not exist")]
    ConnectionError,
    
    #[error("Access denied: {0}")]
    AccessDenied(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] rocksdb::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
    
    #[error("Computation error: {0}")]
    ComputationError(String),
    
    #[error("Invalid state: {0}")]
    InvalidState(String),
    
    #[error("Concurrency error: {0}")]
    ConcurrencyError(String),
    
    #[error("Generic error: {0}")]
    Generic(String),
}

pub type Result<T> = std::result::Result<T, EngramError>;