pub mod service;
pub mod conversion;
pub mod error;
pub mod server;

#[cfg(feature = "grpc")]
pub mod proto {
    tonic::include_proto!("engram.v1");
}