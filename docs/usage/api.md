# API Reference

> **Note**: The EngramAI Lite API service is planned for Milestone 4 and is not yet implemented in the current release.

EngramAI Lite will provide a gRPC-based API service for remote access to the memory graph. This document outlines the planned API design.

## Planned API Endpoints

### Engram Operations

- `CreateEngram(CreateEngramRequest) returns (CreateEngramResponse)`
- `GetEngram(GetEngramRequest) returns (GetEngramResponse)`
- `UpdateEngram(UpdateEngramRequest) returns (UpdateEngramResponse)`
- `DeleteEngram(DeleteEngramRequest) returns (DeleteEngramResponse)`
- `ListEngrams(ListEngramsRequest) returns (ListEngramsResponse)`

### Connection Operations

- `CreateConnection(CreateConnectionRequest) returns (CreateConnectionResponse)`
- `GetConnection(GetConnectionRequest) returns (GetConnectionResponse)`
- `UpdateConnection(UpdateConnectionRequest) returns (UpdateConnectionResponse)`
- `DeleteConnection(DeleteConnectionRequest) returns (DeleteConnectionResponse)`
- `ListConnections(ListConnectionsRequest) returns (ListConnectionsResponse)`

### Collection Operations

- `CreateCollection(CreateCollectionRequest) returns (CreateCollectionResponse)`
- `GetCollection(GetCollectionRequest) returns (GetCollectionResponse)`
- `UpdateCollection(UpdateCollectionRequest) returns (UpdateCollectionResponse)`
- `DeleteCollection(DeleteCollectionRequest) returns (DeleteCollectionResponse)`
- `ListCollections(ListCollectionsRequest) returns (ListCollectionsResponse)`
- `AddToCollection(AddToCollectionRequest) returns (AddToCollectionResponse)`
- `RemoveFromCollection(RemoveFromCollectionRequest) returns (RemoveFromCollectionResponse)`

### Agent Operations

- `CreateAgent(CreateAgentRequest) returns (CreateAgentResponse)`
- `GetAgent(GetAgentRequest) returns (GetAgentResponse)`
- `UpdateAgent(UpdateAgentRequest) returns (UpdateAgentResponse)`
- `DeleteAgent(DeleteAgentRequest) returns (DeleteAgentResponse)`
- `ListAgents(ListAgentsRequest) returns (ListAgentsResponse)`
- `GrantAccess(GrantAccessRequest) returns (GrantAccessResponse)`
- `RevokeAccess(RevokeAccessRequest) returns (RevokeAccessResponse)`

### Context Operations

- `CreateContext(CreateContextRequest) returns (CreateContextResponse)`
- `GetContext(GetContextRequest) returns (GetContextResponse)`
- `UpdateContext(UpdateContextRequest) returns (UpdateContextResponse)`
- `DeleteContext(DeleteContextRequest) returns (DeleteContextResponse)`
- `ListContexts(ListContextsRequest) returns (ListContextsResponse)`
- `AddToContext(AddToContextRequest) returns (AddToContextResponse)`
- `RemoveFromContext(RemoveFromContextRequest) returns (RemoveFromContextResponse)`

### Search Operations

- `QueryBySource(QueryBySourceRequest) returns (QueryBySourceResponse)`
- `QueryByConfidence(QueryByConfidenceRequest) returns (QueryByConfidenceResponse)`
- `QueryByMetadata(QueryByMetadataRequest) returns (QueryByMetadataResponse)`
- `Search(SearchRequest) returns (SearchResponse)`

### Import/Export Operations

- `ExportData(ExportDataRequest) returns (ExportDataResponse)`
- `ImportData(ImportDataRequest) returns (ImportDataResponse)`

### Management Operations

- `GetStats(GetStatsRequest) returns (GetStatsResponse)`
- `CompactDatabase(CompactDatabaseRequest) returns (CompactDatabaseResponse)`
- `Health(HealthRequest) returns (HealthResponse)`

## Authentication

The API will support multiple authentication methods:

- API key authentication
- OAuth 2.0
- mTLS (mutual TLS)

## Client Libraries

Once implemented, client libraries will be available for:

- Python
- TypeScript/JavaScript
- Go
- Rust

## Usage Example (Future)

When the API is available, usage will look something like this:

```python
from engramai import EngramClient

# Connect to the EngramAI service
client = EngramClient("localhost:50051", api_key="your_api_key")

# Create an engram
engram = client.create_engram(
    content="The capital of France is Paris",
    source="geography",
    confidence=0.95
)

# Create another engram
another_engram = client.create_engram(
    content="Paris is known for the Eiffel Tower",
    source="landmarks",
    confidence=0.9
)

# Create a connection between them
connection = client.create_connection(
    source_id=engram.id,
    target_id=another_engram.id,
    relationship_type="related",
    weight=0.8
)

# Query engrams by source
results = client.query_by_source("geography")
for engram in results:
    print(f"{engram.id}: {engram.content} (confidence: {engram.confidence})")
```

## Stay Tuned

The API service is planned for Milestone 4. Check back for updates in future releases!