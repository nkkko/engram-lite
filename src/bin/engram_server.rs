#[cfg(feature = "grpc")]
use engram_lite::{
    Storage, SearchIndex, VectorIndex, EmbeddingService, GrpcServer,
    utils::{load_env_from_file, get_anthropic_api_key},
};

use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;

#[cfg(feature = "grpc")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    load_env_from_file(".env");
    
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    let mut db_path = PathBuf::from("./engram_db");
    let mut addr = "[::1]:50051".parse::<SocketAddr>().unwrap();
    
    // Process command-line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--db-path" | "-d" => {
                if i + 1 < args.len() {
                    db_path = PathBuf::from(&args[i + 1]);
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --db-path");
                    std::process::exit(1);
                }
            }
            "--port" | "-p" => {
                if i + 1 < args.len() {
                    let port: u16 = args[i + 1].parse().unwrap_or_else(|_| {
                        eprintln!("Error: Invalid port number");
                        std::process::exit(1);
                    });
                    addr = SocketAddr::from(([0, 0, 0, 0], port));
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --port");
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                println!("EngramAI gRPC Server");
                println!("Usage: engram_server [OPTIONS]");
                println!("Options:");
                println!("  -d, --db-path PATH    Set the database directory (default: ./engram_db)");
                println!("  -p, --port PORT       Set the server port (default: 50051)");
                println!("  -h, --help            Show this help message");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error: Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
    }
    
    // Create the storage instance
    let storage = Arc::new(Storage::new(&db_path)?);
    
    // Create indices
    let search_index = Arc::new(SearchIndex::new());
    
    // Create embedding service
    let embedding_service = Arc::new(EmbeddingService::new());
    
    // Create vector index
    let vector_index = Arc::new(VectorIndex::with_embedding_service(embedding_service.clone()));
    
    // Create and run the gRPC server
    let server = GrpcServer::new(
        addr,
        storage,
        search_index,
        vector_index,
        embedding_service,
    );
    
    println!("EngramAI gRPC Server");
    println!("Database path: {}", db_path.display());
    println!("Listening on: {}", addr);
    
    server.run().await?;
    
    Ok(())
}

#[cfg(not(feature = "grpc"))]
fn main() {
    eprintln!("Error: This binary requires the 'grpc' feature to be enabled.");
    eprintln!("Please rebuild with: cargo build --features grpc");
    std::process::exit(1);
}