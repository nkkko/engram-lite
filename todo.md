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

- [ ] [P0] Implement minimal Rust core
  - [ ] Define type schemas for Engram, Connection, Collection, Agent, Context
  - [ ] Create serialization/deserialization for all types
  - [ ] Create basic RocksDB storage implementation
  - [ ] Implement simple key-value operations (put/get engram)
  - [ ] Add ACID transaction support for data integrity
  - [ ] Add basic unit tests for core functionality
  - [ ] Implement petgraph layer for relations between engrams
  - [ ] Create efficient indexes for relationship traversal
  - [ ] Support typed, weighted connections with metadata
- [ ] [P0] Build basic CLI tool
  - [ ] Create simple command-line interface
  - [ ] Implement commands for adding/retrieving engrams
  - [ ] Add export/import functionality for data backup
  - [ ] Create commands for data maintenance
  - [ ] Implement TUI using ratatui (latest v0.29.0)
- [ ] [P0] Deliver fundamental documentation using Material for MkDocs (pip install mkdocs-material)
  - [ ] Document data model and storage approach
  - [ ] Create README with setup instructions
  - [ ] Add CLI usage examples
  - [ ] Document storage schema design decisions

**Value:** A working local database for storing structured memory units with persistence

## Milestone 2: Memory Graph & Search [P0]
**Goal:** Enable relationship tracking and basic search capabilities

- [ ] [P0] Implement graph relationships
  - [ ] Add relationship storage in RocksDB
  - [ ] Create basic graph traversal functions
  - [ ] Support typed, weighted connections between engrams
- [ ] [P0] Build essential search capabilities
  - [ ] Implement basic keyword search
  - [ ] Add simple metadata filtering
  - [ ] Create foundational query interface
- [ ] [P0] Develop testing infrastructure
  - [ ] Set up integration test framework
  - [ ] Create benchmark suite for core operations
  - [ ] Implement CI pipeline with basic automated tests

**Value:** Ability to connect memories and perform basic searches, enabling simple knowledge graph applications

## Milestone 3: Vector Search & Hybrid Retrieval [P1]
**Goal:** Add vector embedding support and advanced retrieval capabilities

- [ ] [P1] Implement vector infrastructure
  - [ ] Add embedding storage to RocksDB schema
  - [ ] Integrate HNSW index for vector search
  - [ ] Create vector similarity search API
- [ ] [P1] Build hybrid search capabilities
  - [ ] Implement combined keyword + vector search
  - [ ] Make vector search, keyword search, and metadata filtering equal citizens
  - [ ] Implement BM25 scoring tightly integrated with vector search
  - [ ] Add relevance scoring and ranking
  - [ ] Create query builder for complex searches
- [ ] [P1] Create first Python client
  - [ ] Implement PyO3 bindings for core functions
  - [ ] Create Python API wrapper
  - [ ] Add examples of vector search in Python

**Value:** Semantic search capabilities using embeddings alongside keyword search, enabling AI-friendly memory retrieval

## Milestone 4: API Service Layer [P1]
**Goal:** Make EngramAI accessible via network API

- [ ] [P1] Implement gRPC service
  - [ ] Define Protocol Buffers for core data types
  - [ ] Create essential service endpoints
  - [ ] Implement authentication and basic security
  - [ ] Design idiomatic API patterns for Rust, Python, Go, and JS clients
- [ ] [P1] Generate and test client libraries
  - [ ] Generate Python client bindings
  - [ ] Generate TS client bindings
  - [ ] Create comprehensive client examples
- [ ] [P1] Develop API documentation
  - [ ] Create API reference documentation
  - [ ] Write endpoint usage examples
  - [ ] Document authentication and security
- [ ] [P1] Build service management
  - [ ] Create service configuration system
  - [ ] Implement logging and monitoring
  - [ ] Add health check endpoints

**Value:** Remote access to memory system, enabling multi-client usage and service-oriented architecture

## Milestone 5: Memory Management [P1]
**Goal:** Add intelligence to memory management with temporal features and forgetting

- [ ] [P1] Implement temporal capabilities
  - [ ] Add timestamp tracking for all operations
  - [ ] Create temporal query operators (before/after)
  - [ ] Implement recency-based relevance scoring
- [ ] [P1] Build forgetting mechanisms
  - [ ] Add confidence and importance scoring
  - [ ] Implement configurable forgetting policies
  - [ ] Create memory pruning and compaction
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
- [ ] [P1] Build MCP server basics (mcp-full-llms.txt)
  - [ ] Implement core MCP server specification
  - [ ] Create basic memory tools for MCP (add_engram, get_engram, query, engram_relate, memory_summarize)
  - [ ] Add integration examples with Claude and other MCP clients
  - [ ] Build a Remote MCP Server on Cloudflare (cf-remote-mcp.txt)
- [ ] [P1] Develop reflection capabilities
  - [ ] Create summarization endpoints
  - [ ] Implement community detection algorithms
  - [ ] Implement memory-based insight generation
  - [ ] Develop API hooks for LLM-based summarization

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

- [ ] [P1] Design UI architecture and technology stack
  - [ ] Evaluate options: Embedded Rust HTTP server vs. separate web server application.
  - [ ] Choose frontend framework (e.g., React, Vue, Svelte) and related tooling.
  - [ ] Define communication protocol (e.g., REST over HTTP, WebSockets, gRPC-web) to interact with M4 API or embedded server.
  - [ ] Setup frontend development environment.
- [ ] [P1] Implement backend component for UI (if not fully relying on M4 API)
  - [ ] Add minimal embedded HTTP server in Rust or build a small companion web server app.
  - [ ] Create or consume necessary API endpoints for UI data retrieval and manipulation.
- [ ] [P1] Build core UI components and layout
  - [ ] Create main application structure and navigation.
  - [ ] Implement reusable components (tables, forms, buttons, graph canvas).
  - [ ] Design basic styling and theme.
- [ ] [P1] Implement data browsing and detail views
  - [ ] Create pages/views for listing/searching Engrams.
  - [ ] Create pages/views for listing/searching Connections.
  - [ ] Create pages/views for listing/searching Collections, Agents, and Contexts.
  - [ ] Implement detail panels for inspecting individual data items with all their attributes.
- [ ] [P1] Implement data manipulation forms
  - [ ] Build forms for adding new Engrams (content, source, confidence, metadata).
  - [ ] Build forms for adding new Connections (source, target, type, weight, metadata).
  - [ ] Implement forms for editing and deleting data items.
- [ ] [P1] Add query and command execution interface
  - [ ] Create a persistent input area for typing commands or executing queries (leveraging M2/M3 search/query capabilities).
  - [ ] Implement sending commands/queries to the backend via API.
  - [ ] Display results in a user-friendly format (structured tables for query results, plain text for command output).
- [ ] [P1] Integrate interactive graph visualization
  - [ ] Choose and integrate a JavaScript graph visualization library (e.g., Cytoscape.js, D3.js, vis.js).
  - [ ] Implement fetching graph data subsets for visualization (e.g., connections around a specific engram, a collection's contents).
  - [ ] Display a basic interactive graph visualization allowing pan, zoom, and node click.
- [ ] [P1] Add UI launch mechanisms
  - [ ] Implement a CLI command (e.g., `engramai ui` or `engramai --ui`) that starts the UI backend component and opens the default web browser to the UI URL.
  - [ ] Add configuration options (e.g., port number) via CLI flags or config file.
- [ ] [P1] Develop comprehensive UI documentation
  - [ ] Create a user guide specific to the web UI.
  - [ ] Document installation, launching, and basic usage.
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
- [ ] [P2] Add distributed capabilities
  - [ ] Implement basic sharding strategy
  - [ ] Create replication support
  - [ ] Add cluster coordination

**Value:** Ability to scale to large memory graphs with billions of nodes and high query throughput

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

**Value:** Exploration of cutting-edge memory capabilities for future versions