# EngramAI Lite Tests

This directory contains integration tests for the EngramAI Lite project. These tests verify that the components of the system work together correctly in various scenarios.

## Test Organization

1. **Schema Tests** (`schema_test.rs`)
   - Tests for data model entities (Engram, Connection, Collection, Agent, Context)
   - Validates entity creation, property access, and basic operations
   - Tests metadata handling and relationship modeling

2. **Storage Tests** (`storage_test.rs`)
   - Tests for RocksDB-based persistence layer
   - Validates CRUD operations for all entity types
   - Tests transactions, concurrent access, and database statistics
   - Ensures proper cleanup of test directories

3. **Graph Tests** (`graph_test.rs`)
   - Tests for the in-memory graph representation (MemoryGraph)
   - Validates node and edge creation, traversal, and path finding
   - Tests relationship operations between different entity types
   - Verifies graph queries and filters

4. **Vector Search Tests** (`vector_search_test.rs`)
   - Tests for vector embedding and similarity search functionality
   - Validates HNSW index creation, addition, and search operations
   - Tests hybrid search combining vector and keyword matching
   - Handles API key availability gracefully

5. **Embedding Tests** (`embedding_test.rs`)
   - Tests for embedding service functionality
   - Validates embedding generation with various models
   - Tests batch embedding processing
   - Tests embedding normalization and similarity calculations

## Running Tests

Run the entire test suite with:

```bash
cargo test
```

Run a specific test file with:

```bash
cargo test --test schema_test
cargo test --test storage_test
cargo test --test graph_test
cargo test --test vector_search_test
cargo test --test embedding_test
```

Run a specific test with:

```bash
cargo test --test schema_test test_engram_creation
```

Note that some tests require API keys to be set in the environment or in an `.env` file in the project root. Tests that require external API access are designed to skip gracefully when keys are not available.

## Writing New Tests

When writing new tests, follow these guidelines:

1. Create separate test files for different components
2. Use descriptive test names with the `test_` prefix
3. Handle external dependencies gracefully with optional tests
4. Clean up test resources (e.g., temporary database directories)
5. Use helper functions for common test setup
6. Include tests for both success and failure cases
7. Test edge cases and error handling