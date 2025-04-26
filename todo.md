# TODO Style Guide

## Format Rules

- Use `[ ]` for incomplete tasks: `- [ ] Task description`
- Use `[x]` for completed tasks: `- [x] Task description`
- All tasks must have a priority marker: `[P0]`, `[P1]`, `[P2]`, `[P3]`, or `[NO]`
  - `[P0]`: Critical - Must be done immediately
  - `[P1]`: High - Required for next milestone
  - `[P2]`: Medium - Important but not blocking
  - `[P3]`: Low - Nice to have
  - `[NO]`: Should not be worked on - Intentionally deprioritized

## Task Description Style
- Begin with an action verb in imperative form (e.g., "Add", "Implement", "Create")
- Be specific and concise
- Include acceptance criteria when appropriate
- For complex tasks, add sub-tasks with indentation

## Organization
- Group tasks by feature area or component
- Use headings (## for major sections, ### for subsections)
- Order tasks within sections by priority
- Move completed tasks to a "Completed" section with date

## Examples
```
- [x] [P1] Add basic vector search functionality (completed 2023-04-01)
- [ ] [P0] Fix memory leak in vector index when handling large embeddings
- [ ] [P1] Implement transaction support
  - [ ] Add begin_transaction() API
  - [ ] Add rollback functionality
  - [ ] Add commit functionality
- [ ] [NO] Port entire codebase to JavaScript/Node.js - Sticking with Rust for core performance
```

---

# Project Tasks

## Completed
- [x] [P1] Prototype the API & schema in Python

## Milestone 1: Core Memory Store [P0]
**Goal:** Create a functional memory store with basic persistence, efficiently storing and retrieving engrams

- [x] [P0] Implement minimal Rust core
  - [x] Define type schemas for Engram, Connection, Collection, Agent, Context
  - [x] Create serialization/deserialization for all types
  - [x] Create basic RocksDB storage implementation
  - [x] Implement simple key-value operations (put/get engram)
  - [x] Add ACID transaction support for data integrity
  - [x] Add basic unit tests for core functionality
  - [x] Implement petgraph layer for relations between engrams
  - [x] Create efficient indexes for relationship traversal
  - [x] Support typed, weighted connections with metadata
- [x] [P0] Build basic CLI tool
  - [x] Create simple command-line interface
  - [x] Implement commands for adding/retrieving engrams
  - [x] Add export/import functionality for data backup
  - [x] Create commands for data maintenance
  - [x] Implement TUI using ratatui (latest v0.29.0)
- [x] [P0] Deliver fundamental documentation using Material for MkDocs (pip install mkdocs-material)
  - [x] Document data model and storage approach
  - [x] Create README with setup instructions
  - [x] Add CLI usage examples
  - [x] Add TUI usage documentation
  - [x] Document storage schema design decisions

**Value:** A working local database for storing structured memory units with persistence

## Milestone 2: Memory Graph & Search [P0]
**Goal:** Enable relationship tracking and basic search capabilities

- [x] [P0] Implement graph relationships
  - [x] Add relationship storage in RocksDB
  - [x] Create basic graph traversal functions
  - [x] Support typed, weighted connections between engrams
- [x] [P0] Build essential search capabilities
  - [x] Implement basic keyword search
  - [x] Add simple metadata filtering
  - [x] Create foundational query interface
- [x] [P0] Develop testing infrastructure
  - [x] Set up integration test framework
  - [x] Create benchmark suite for core operations
  - [x] Implement CI pipeline with basic automated tests
  - [x] Add comprehensive unit tests for core functionality
    - [x] Create schema tests for data model entities
    - [x] Implement storage tests for persistence operations
    - [x] Add graph tests for relationship operations
    - [x] Create vector search tests for embedding functionality

**Value:** Ability to connect memories and perform basic searches, enabling simple knowledge graph applications

## Milestone 3: Vector Search & Hybrid Retrieval [P1]
**Goal:** Add vector embedding support and advanced retrieval capabilities

- [x] [P1] Implement vector infrastructure
  - [x] Add embedding storage to RocksDB schema
  - [x] Integrate HNSW index for vector search
  - [x] Create vector similarity search API
  - [x] Create EmbeddingService with multi-model support
    - [x] Implement base embedding service with common functionality
    - [x] Add model selection capabilities
    - [x] Support dimensions and other model-specific parameters
    - [x] Implement caching for embeddings
    - [x] Add dimensionality reduction for storage optimization
      - [x] Implement PCA using linfa for optimal variance preservation
      - [x] Add random projection for faster dimensionality reduction
      - [x] Create truncation method for simple dimension reduction
    - [x] Implement incremental indexing for large collections
- [x] [P1] Implement model types for semantic search
  - [x] [P1] E5 Multilingual Large Instruct (Default) - intfloat/multilingual-e5-large-instruct
    - [x] Support high-quality embeddings (1024 dimensions)
    - [x] Enable multilingual support (100+ languages)
    - [x] Implement instruction-based embedding generation
  - [x] [P2] GTE Modern BERT Base - Alibaba-NLP/gte-modernbert-base
    - [x] Support modern semantics (768 dimensions)
    - [x] Optimize for semantic search and similarity tasks
  - [x] [P2] Jina Embeddings V3 - jinaai/jina-embeddings-v3
    - [x] Implement state-of-the-art NLP support (768 dimensions)
    - [x] Enable cross-lingual capabilities
  - [x] [P2] Custom model support
    - [x] Allow specifying custom model by name
    - [x] Auto-detect dimensions and model properties
    - [ ] Support local models where possible
- [x] [P1] Build hybrid search capabilities
  - [x] Implement combined keyword + vector search
  - [x] Make vector search, keyword search, and metadata filtering equal citizens
  - [x] Implement BM25 scoring tightly integrated with vector search
  - [x] Add relevance scoring and ranking
    - [x] Create multi-factor relevance scoring system
    - [x] Implement configurable relevance weights
    - [ ] Add contextual boosting for query terms
    - [ ] Build dynamic reranking based on context
  - [x] Create query builder for complex searches
    - [ ] Implement semantic-aware query parsing
    - [ ] Add support for nested boolean expressions
    - [ ] Create query expansion with related terms
    - [x] Build semantic similarity threshold controls
- [x] [P1] Create first Python client
  - [x] Implement PyO3 bindings for core functions
  - [x] Create Python API wrapper
  - [x] Add examples of vector search in Python
  - [x] Implement EmbeddingService Python equivalents
    - [x] Add with_model_type method (Enum-based selection)
    - [x] Add with_model method (String-based custom model selection)
    - [x] Ensure compatibility with Rust implementation
- [ ] [P1] Fix mock implementations
  - [x] Replace placeholder embedding generation in embedding.rs with real implementation
  - [ ] [P2] Implement local embedding models support for offline usage
    - [ ] Add support for loading and running E5, GTE, and Jina models locally
    - [ ] Implement efficient inference using ONNX runtime
    - [ ] Create model caching and download mechanisms
    - [ ] Ensure model-specific parameters are properly handled
  - [x] Implement HNSW algorithm properly instead of linear search
  - [x] Add real vector_search.rs implementation for get_embedding_for_engram
  - [x] Fix web server placeholder implementation
  - [ ] Fix MCP server placeholder implementation
  - [ ] See mock.md for a complete list of mock implementations to address

**Value:** Semantic search capabilities using embeddings alongside keyword search, enabling AI-friendly memory retrieval

## Milestone 4: API Service Layer [P1] ✅
**Goal:** Make EngramAI accessible via network API

- [x] [P1] Implement gRPC service
  - [x] Define Protocol Buffers for core data types
  - [x] Create essential service endpoints
  - [ ] Implement authentication and basic security
  - [x] Design idiomatic API patterns for Rust, Python, Go, and JS clients
- [x] [P1] Generate and test client libraries
  - [x] Generate Python client bindings
  - [x] Generate TS client bindings
  - [x] Create comprehensive client examples
- [x] [P1] Develop API documentation
  - [x] Create API reference documentation
  - [x] Write endpoint usage examples
  - [ ] Document authentication and security
  - [ ] [P1] Generate OpenAPI/Swagger specification
    - [ ] Define OpenAPI schema for all API endpoints
    - [ ] Add request/response examples
    - [ ] Implement interactive API documentation UI
- [x] [P1] Build service management
  - [x] Create service configuration system
  - [x] Implement logging and monitoring
  - [x] Add health check endpoints

**Value:** Remote access to memory system, enabling multi-client usage and service-oriented architecture

## Milestone 5: Memory Management [P1]
**Goal:** Add intelligence to memory management with temporal features and forgetting

- [ ] [P1] Implement temporal capabilities
  - [ ] Add timestamp tracking for all operations
  - [ ] Create temporal query operators (before/after)
  - [ ] Implement recency-based relevance scoring
  - [ ] Add temporal sequence detection
    - [ ] Build temporal chain identification algorithm
    - [ ] Implement cause-effect relationship detection
    - [ ] Create temporal reasoning API for sequence analysis
  - [ ] Develop time-aware retrieval mechanisms
    - [ ] Implement time window filtering with contextual relevance
    - [ ] Add decay functions for temporal relevance
    - [ ] Create time-sensitive connection weighting
- [ ] [P1] Build forgetting mechanisms
  - [ ] Add confidence and importance scoring
    - [ ] Implement centrality-based importance calculation
    - [ ] Create access frequency tracking for importance
    - [ ] Add multi-factor importance scoring algorithm
  - [ ] Implement configurable forgetting policies
    - [ ] Create age-based forgetting strategy
    - [ ] Add importance-threshold forgetting strategy
    - [ ] Implement hybrid forgetting policy framework
  - [ ] Create memory pruning and compaction
    - [ ] Build memory graph health analysis tools
    - [ ] Implement automated pruning recommendations
    - [ ] Add manual pruning interface with safeguards
  - [ ] Add TTL support for ephemeral engrams
- [ ] [P1] Add RocksDB optimizations
  - [ ] Implement tiered storage for hot/cold data
  - [ ] Optimize for write-heavy workloads
  - [ ] Add intelligent compaction policies

**Value:** Intelligent memory management that prioritizes recent and important information, mimicking human memory

## Milestone 6: LLM Integration [P1]
**Goal:** Create seamless integration with language models

- [ ] [P1] Implement LangChain/LlamaIndex adapters
  - [ ] Build EngramAI memory class for LangChain
  - [ ] Create vector store implementation for LlamaIndex
  - [ ] Add memory retrieval augmented generation examples
  - [ ] Implement connection generation from LLM reasoning
  - [ ] Add engram enrichment with LLM-generated metadata
- [x] [P1] Improve developer experience
  - [x] Add demo data generation with realistic multi-agent scenario
  - [x] Create web UI template pages for browsing demo data
  - [ ] Create interactive tutorials for common usage patterns
  - [ ] Add visual graph exploration tools
- [ ] [P1] Build MCP server basics (mcp-full-llms.txt)
  - [ ] Implement core MCP server specification
  - [ ] Create basic memory tools for MCP (add_engram, get_engram, query, engram_relate, memory_summarize)
  - [ ] Add integration examples with Claude and other MCP clients
  - [ ] Build a Remote MCP Server on Cloudflare (cf-remote-mcp.txt)
  - [ ] Create content validation mechanisms using LLMs
- [ ] [P1] Develop reflection capabilities
  - [ ] Create summarization endpoints
  - [ ] Implement community detection algorithms
    - [ ] Add Louvain method for community identification
    - [ ] Implement hierarchical community clustering
    - [ ] Create visualization tools for communities
  - [ ] Implement memory-based insight generation
    - [ ] Add pattern recognition for recurring knowledge
    - [ ] Create automated tagging based on content
    - [ ] Build contradiction detection between engrams
  - [ ] Develop API hooks for LLM-based summarization
    - [ ] Implement memory recontextualization with LLMs
    - [ ] Add collaborative answer synthesis with multiple engrams
    - [ ] Create automated knowledge graph enhancement

**Value:** Ready-to-use integration with popular LLM frameworks, enabling AI agents to use EngramAI for memory

## Milestone 7: Edge & Cloud Integration [P2]
**Goal:** Make EngramAI available on edge computing platforms

- [ ] [P2] Create TypeScript client
  - [ ] Implement TypeScript API wrapper
  - [ ] Add client-side validation
  - [ ] Create browser-compatible package
- [ ] [P2] Build Cloudflare Workers integration (cf-dev-llms-full.txt)
  - [ ] Develop optimized client for Cloudflare Workers
  - [ ] Create D1/KV storage adapter
  - [ ] Implement edge-optimized search
- [ ] [P2] Build Cloudflare AI Agents integration (agents-llms.txt)
  - [ ] Build demo showing memory-enhanced agents
  - [ ] Create edge deployment examples
  - [ ] Implement multi-agent memory sharing

**Value:** Ability to use EngramAI on edge computing platforms, enabling memory for serverless AI agents

## Milestone 8: Web User Interface [P1]
**Goal:** Create a user-friendly web-based graphical interface for interacting with the EngramAI memory graph, similar to the DuckDB CLI UI experience.

- [x] [P1] Design UI architecture and technology stack
  - [x] Evaluate options: Embedded Rust HTTP server vs. separate web server application.
  - [x] Choose frontend framework (using simple HTML/CSS with Tera templates).
  - [x] Define communication protocol (REST over HTTP) to interact with API.
  - [x] Setup UI development environment.
- [x] [P1] Implement backend component for UI
  - [x] Add minimal embedded HTTP server in Rust with Actix-Web.
  - [x] Create necessary API endpoints for UI data retrieval.
- [x] [P1] Build core UI components and layout
  - [x] Create main application structure and navigation.
  - [x] Implement reusable components for displaying engrams, connections, etc.
  - [x] Design basic styling and theme.
- [x] [P1] Implement data browsing and detail views
  - [x] Create pages/views for listing Engrams.
  - [x] Create pages/views for listing Connections.
  - [x] Create pages/views for listing Collections and Agents.
  - [x] Implement detail panel for viewing collection contents.
- [ ] [P1] Implement data manipulation forms
  - [ ] Build forms for adding new Engrams (content, source, confidence, metadata).
  - [ ] Build forms for adding new Connections (source, target, type, weight, metadata).
  - [ ] Implement forms for editing and deleting data items.
- [ ] [P1] Add query and command execution interface
  - [ ] Create a persistent input area for typing commands or executing queries.
  - [ ] Implement sending commands/queries to the backend via API.
  - [ ] Display results in a user-friendly format (structured tables for query results).
- [x] [P1] Integrate interactive graph visualization
  - [x] Choose and integrate a JavaScript graph visualization library (Cytoscape.js).
  - [x] Implement fetching graph data subsets for visualization.
  - [x] Display a basic interactive graph visualization allowing pan, zoom, and node click.
- [x] [P1] Add UI launch mechanisms
  - [x] Implement a CLI command (`engramlt web`) that starts the UI backend component.
  - [x] Add configuration options (port number) via CLI flags.
- [x] [P1] Develop comprehensive UI documentation
  - [x] Create a user guide specific to the web UI.
  - [x] Document installation, launching, and basic usage.
  - [ ] Explain how to perform common tasks (add data, search, view graph).

**Value:** Provides an intuitive visual way for users to interact with, explore, and manage their memory graphs without relying solely on the CLI or raw API calls. Lowers the barrier to entry and makes complex data structures more understandable.


## Milestone 9: Performance & Scalability [P2]
**Goal:** Enhance system to handle large-scale deployments

- [ ] [P2] Implement caching layer
  - [ ] Add in-memory cache for hot data
  - [ ] Create cache invalidation system
  - [ ] Implement configurable caching policies
- [ ] [P2] Optimize for scale
  - [ ] Add parallel query execution
  - [ ] Implement connection pooling
  - [ ] Create query optimization layer
  - [ ] Implement advanced graph algorithms
    - [ ] Add PageRank for engram importance scoring
    - [ ] Implement betweenness centrality for identifying bridge nodes
    - [ ] Create eigenvector centrality for finding influential engrams
    - [ ] Build path optimization algorithms (shortest path, max-flow)
    - [ ] Add subgraph extraction with defined boundary conditions
  - [ ] Implement advanced graph traversal capabilities
    - [ ] Create multi-criteria path finding
    - [ ] Add relevance-weighted traversal algorithms
    - [ ] Implement context-aware path exploration
    - [ ] Build recursive query capabilities with cycle detection
    - [ ] Develop traversal pattern matching (path templates)
- [ ] [P2] Add distributed capabilities
  - [ ] Implement basic sharding strategy
  - [ ] Create replication support
  - [ ] Add cluster coordination
- [ ] [P2] Add knowledge graph analysis capabilities
  - [ ] Implement graph embeddings for node representation
  - [ ] Create knowledge distillation algorithms
  - [ ] Add semantic network analysis tools
  - [ ] Build abstraction layer for concept hierarchies
  - [ ] Implement inference mechanisms for implicit connections

**Value:** Ability to scale to large memory graphs with billions of nodes and high query throughput, with advanced algorithms for knowledge analysis and traversal

## Milestone 10: Production Readiness [P3]
**Goal:** Prepare for enterprise adoption with security, monitoring, and administration

- [ ] [P3] Enhance security
  - [ ] Implement at-rest encryption
  - [ ] Add comprehensive audit logging
  - [ ] Create multi-tenant access controls
- [ ] [P3] Build monitoring system
  - [ ] Add detailed metrics collection
  - [ ] Create monitoring dashboards
  - [ ] Implement alerting
- [ ] [P3] Develop administration tools
  - [ ] Create admin CLI
  - [ ] Build web-based admin interface
  - [ ] Add backup/restore system

**Value:** Enterprise-ready memory system with the security, monitoring, and administration tools needed for production use

## Future Explorations [P3]
**Goal:** Research directions for future development

- [ ] [P3] Advanced graph algorithms
  - [ ] Implement graph embeddings
  - [ ] Add knowledge distillation
  - [ ] Create semantic network analysis
- [ ] [P3] Multi-modal engrams
  - [ ] Support image embeddings
  - [ ] Add audio and video processing
  - [ ] Implement cross-modal retrieval
- [ ] [P3] Federated memory
  - [ ] Create federated storage protocol
  - [ ] Implement privacy-preserving sharing
  - [ ] Build decentralized memory graph
- [ ] [P2] LLM-friendly documentation files
  - [ ] Generate .llms.txt files during build process for core components
  - [ ] Create structured format for API documentation in llms-txt format
  - [ ] Add automated extraction from code comments to llms-txt
  - [ ] Implement contextual linking between related components
- [ ] [P1] WebAssembly Runtime for Agent Integration
  - [ ] Implement Wasmtime integration for running agent code directly in Engram-Lite
  - [ ] Create host function interfaces for memory, query, and graph operations
  - [ ] Develop framework bridges for Pydantic.ai and Mastra.ai
  - [ ] Add context-based collaboration for multi-agent workflows
  - [ ] Implement transaction support for atomic agent operations
  - [ ] See detailed specification in [specs/wasm-runtime.md](specs/wasm-runtime.md)

**Value:** Exploration of cutting-edge memory capabilities for future versions and improved LLM contextual understanding