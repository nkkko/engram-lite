use crate::error::EngramError;
use tonic::Status;

#[cfg(feature = "grpc")]
pub fn to_status(err: EngramError) -> Status {
    match err {
        EngramError::NotFound(msg) => Status::not_found(msg),
        EngramError::InvalidId(msg) => Status::invalid_argument(msg),
        EngramError::ConnectionError => Status::failed_precondition("Source or target engram does not exist"),
        EngramError::AccessDenied(msg) => Status::permission_denied(msg),
        EngramError::StorageError(msg) => Status::internal(format!("Storage error: {}", msg)),
        EngramError::SerializationError(msg) => Status::internal(format!("Serialization error: {}", msg)),
        EngramError::DatabaseError(e) => Status::internal(format!("Database error: {}", e)),
        EngramError::JsonError(e) => Status::internal(format!("JSON error: {}", e)),
        EngramError::IoError(e) => Status::internal(format!("I/O error: {}", e)),
        EngramError::TransactionError(msg) => Status::aborted(format!("Transaction error: {}", msg)),
        EngramError::InvalidOperation(msg) => Status::invalid_argument(msg),
        EngramError::NotImplemented(msg) => Status::unimplemented(msg),
        EngramError::ComputationError(msg) => Status::internal(format!("Computation error: {}", msg)),
        EngramError::InvalidState(msg) => Status::failed_precondition(msg),
        EngramError::ConcurrencyError(msg) => Status::internal(format!("Concurrency error: {}", msg)),
    }
}