use tonic::transport::Channel;
use tonic::{Request, Response};

// When generated, import the proto types
// use engram_v1::engram_service_client::EngramServiceClient;
// use engram_v1::{CreateEngramRequest, CreateEngramResponse, GetEngramRequest, GetEngramResponse};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    
    let mut server_addr = "[::1]:50051";
    
    // Process command-line arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--addr" | "-a" => {
                if i + 1 < args.len() {
                    server_addr = &args[i + 1];
                    i += 2;
                } else {
                    eprintln!("Error: Missing value for --addr");
                    std::process::exit(1);
                }
            }
            "--help" | "-h" => {
                println!("EngramAI gRPC Client Example");
                println!("Usage: client [OPTIONS]");
                println!("Options:");
                println!("  -a, --addr ADDRESS    Set the server address (default: [::1]:50051)");
                println!("  -h, --help            Show this help message");
                std::process::exit(0);
            }
            _ => {
                eprintln!("Error: Unknown option: {}", args[i]);
                std::process::exit(1);
            }
        }
    }
    
    println!("EngramAI gRPC Client Example");
    println!("Connecting to server at: {}", server_addr);
    
    // Create a channel to the server
    let channel = Channel::from_shared(format!("http://{}", server_addr))?
        .connect()
        .await?;
    
    // To use the client, uncomment and modify the code below once you have generated the proto types
    /*
    // Create client
    let mut client = EngramServiceClient::new(channel);
    
    // Create an engram
    let create_request = Request::new(CreateEngramRequest {
        content: "This is a test engram created via gRPC".to_string(),
        source: "gRPC client example".to_string(),
        confidence: 0.95,
        metadata: None,
    });
    
    let create_response = client.create_engram(create_request).await?;
    let engram = create_response.into_inner().engram.unwrap();
    
    println!("Created engram with ID: {}", engram.id);
    println!("Content: {}", engram.content);
    
    // Retrieve the engram
    let get_request = Request::new(GetEngramRequest {
        id: engram.id.clone(),
    });
    
    let get_response = client.get_engram(get_request).await?;
    let retrieved_engram = get_response.into_inner().engram.unwrap();
    
    println!("Retrieved engram with ID: {}", retrieved_engram.id);
    println!("Content: {}", retrieved_engram.content);
    println!("Source: {}", retrieved_engram.source);
    println!("Confidence: {}", retrieved_engram.confidence);
    println!("Created at: {:?}", retrieved_engram.created_at);
    */
    
    println!("Client executed successfully");
    
    Ok(())
}