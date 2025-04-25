# EngramAI gRPC API Service

This directory contains examples and documentation for working with the EngramAI gRPC service.

## Building the Server

To build the EngramAI gRPC server, use:

```bash
cargo build --release --features grpc --bin engram_server
```

## Running the Server

To run the server:

```bash
./target/release/engram_server --port 50051 --db-path ./engram_db
```

Options:
- `--port` or `-p`: Server port (default: 50051)
- `--db-path` or `-d`: Database directory path (default: ./engram_db)

## Client Examples

### Rust Client

A simple Rust client example is provided in `client.rs`. To build and run it:

```bash
cargo run --example grpc_client -- --addr "[::1]:50051"
```

### Python Client

A Python client example is provided in `client.py`. Before running, you'll need to generate the Python client code from the protocol buffer definitions:

```bash
# Install the required dependencies
pip install grpcio grpcio-tools

# From the project root, generate the Python code
python -m grpc_tools.protoc -I./proto --python_out=. --grpc_python_out=. proto/engram.proto

# Run the client
python examples/grpc/client.py --addr "localhost:50051"
```

## API Documentation

The EngramAI gRPC service provides the following endpoints:

### Engram Operations
- `CreateEngram`: Create a new engram
- `GetEngram`: Retrieve an engram by ID
- `UpdateEngram`: Update an existing engram
- `DeleteEngram`: Delete an engram
- `ListEngrams`: List engrams with pagination

### Connection Operations
- `CreateConnection`: Create a new connection between engrams
- `GetConnection`: Retrieve a connection by ID
- `UpdateConnection`: Update an existing connection
- `DeleteConnection`: Delete a connection
- `ListConnections`: List connections with filtering options

### Collection Operations
- `CreateCollection`: Create a new collection
- `GetCollection`: Retrieve a collection by ID
- `UpdateCollection`: Update an existing collection
- `DeleteCollection`: Delete a collection
- `ListCollections`: List collections

### Search Operations
- `SearchByText`: Search engrams by text content
- `SearchByVector`: Search engrams by vector similarity
- `SearchHybrid`: Combined text and vector search
- `FindConnections`: Find connections between engrams

### Embedding Operations
- `GenerateEmbedding`: Generate an embedding vector for text

See the protocol buffer definition (`proto/engram.proto`) for detailed request and response structures.

## Authentication

When enabled, authentication is implemented using bearer tokens.

To authenticate:
1. Generate an API key through the EngramAI CLI
2. Include the token in your gRPC metadata with the key `authorization` and value `Bearer <API_KEY>`