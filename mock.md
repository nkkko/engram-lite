# Mock Implementations in EngramAI Lite

This document catalogs all mock, placeholder, and simplified implementations in the EngramAI Lite codebase. These temporary implementations are intended to be replaced with real functionality in future development.

## Embedding Service (`src/embedding.rs`)

- **Pseudo-Embedding Generation** (Lines 407-465)
  - The `embed_text` method generates deterministic pseudo-embeddings based on a hash of the input text when HuggingFace API is not available
  - Comment: "This is a fallback for when the real API is unavailable"
  - Used in production code as a temporary solution or fallback

- ~~**Simplified HNSW Vector Index** (Lines 548-636)~~ ✅ **IMPLEMENTED**
  - The `HnswIndex` now implements a proper HNSW algorithm with:
    - Multi-layer graph structure
    - Hierarchical search paths
    - Priority queue-based nearest neighbor search
    - Efficient node insertion and connection management
  - Follows the original HNSW algorithm described in research papers
  - Includes proper configuration for performance tuning (M parameter, ef_construction, ef_search)

## Vector Search (`src/vector_search.rs`)

- ~~**Missing Embedding Retrieval** (Lines 285-287)~~ ✅ **IMPLEMENTED**
  - The `get_embedding_for_engram` method now properly retrieves embeddings from the HnswIndex
  - Uses the new `get_embedding` method added to HnswIndex
  - Returns actual embeddings or appropriate error messages

- **Dimension Reduction Fallbacks** (Lines 82-92, 113-142)
  - Simplified fallback paths when dimension reduction isn't available
  - Handles missing features gracefully but with reduced functionality

- **Default Search Weights** (Lines 411-414)
  - Hardcoded default weights for hybrid search
  - Future improvement: Make configurable or dynamically calculated

## Web Server (`src/bin/web.rs`)

- ~~**Complete Placeholder Implementation** (Lines 3-16)~~ ✅ **IMPLEMENTED**
  - A fully functional web server has been implemented with:
    - RESTful API for Engrams, Connections, Collections, and Agents
    - HTML dashboard with real-time database statistics
    - JSON-based API responses with proper error handling
    - CORS support and middleware for security
    - Auto-creation of template and static directories

## MCP Server (`src/bin/mcp.rs`)

- **Complete Placeholder Implementation** (Lines 4-16)
  - Entire server is a mock that only prints messages
  - No actual MCP server functionality
  - Comment: "Placeholder for MCP server implementation"
  - Contains `init_config` function (Lines 18-69) that creates example configuration files

## TUI Implementation (`src/bin/engramlt.rs`)

- **Feature-Disabled Mock** (Lines 740-749)
  - Mock implementation when the TUI feature is not enabled
  - Simply prints a message instructing how to build with TUI support
  - Configured with `#[cfg(not(feature = "tui"))]`

## Database Functions (`src/bin/engramlt.rs`, `src/bin/engram_cli.rs`, `src/bin/cli.rs`)

- **Simplified Database Compaction** (Lines 578-603 in `engramlt.rs`)
  - Basic implementation without advanced options
  - Comment: "In a more advanced implementation, we could add options for selective compaction"

## gRPC Service (`src/grpc/service.rs`)

- **Unimplemented Service Methods** (Lines 252-364)
  - Multiple gRPC service methods returning `Err(TonicStatus::unimplemented("Not yet implemented"))`
  - Clear "TODO" comments indicating future implementation
  
- **Missing Search Index Methods** (Lines 151-153, 186-187)
  - Calls to `update_engram` and `remove_engram_by_id` methods that don't exist in the `SearchIndex` implementation

## Test Utilities

- **Test Engram Creation** (`src/index_test.rs` Lines 16-37, `src/graph_test.rs` Lines 10-43)
  - Helper functions that create test data with fixed IDs instead of random UUIDs
  - Used exclusively in tests

- **Mock Storage Reference** (`src/graph_test.rs` Lines 352-358)
  - Comment mentions a mock storage implementation for testing
  - "Currently our unit tests can't use RocksDB storage... we'll test with mock storage"

## Future Development

All mock implementations should be replaced with real functionality according to the project roadmap. Priority should be given to core components like embedding generation and vector search that directly impact system performance and capabilities.