# Roadmap

This document outlines the development roadmap for EngramAI Lite, organized by milestones in order of priority.

## Milestone 1: Core Memory Store (Current)

**Goal:** Create a functional memory store with basic persistence, efficiently storing and retrieving engrams

- Define type schemas for Engram, Connection, Collection, Agent, Context
- Create serialization/deserialization for all types
- Create basic RocksDB storage implementation
- Implement simple key-value operations (put/get engram)
- Add ACID transaction support for data integrity
- Add basic unit tests for core functionality
- Implement petgraph layer for relations between engrams
- Create efficient indexes for relationship traversal
- Support typed, weighted connections with metadata
- Build basic CLI tool with data management commands
- Implement TUI using ratatui
- Deliver fundamental documentation 

**Value:** A working local database for storing structured memory units with persistence

## Milestone 2: Memory Graph & Search

**Goal:** Enable relationship tracking and basic search capabilities

- Implement graph relationships
  - Add relationship storage in RocksDB
  - Create basic graph traversal functions
  - Support typed, weighted connections between engrams
- Build essential search capabilities
  - Implement basic keyword search
  - Add simple metadata filtering
  - Create foundational query interface
- Develop testing infrastructure
  - Set up integration test framework
  - Create benchmark suite for core operations
  - Implement CI pipeline with basic automated tests

**Value:** Ability to connect memories and perform basic searches, enabling simple knowledge graph applications

## Milestone 3: Vector Search & Hybrid Retrieval

**Goal:** Add vector embedding support and advanced retrieval capabilities

- Implement vector infrastructure
  - Add embedding storage to RocksDB schema
  - Integrate HNSW index for vector search
  - Create vector similarity search API
- Build hybrid search capabilities
  - Implement combined keyword + vector search
  - Make vector search, keyword search, and metadata filtering equal citizens
  - Implement BM25 scoring tightly integrated with vector search
  - Add relevance scoring and ranking
  - Create query builder for complex searches
- Create first Python client
  - Implement PyO3 bindings for core functions
  - Create Python API wrapper
  - Add examples of vector search in Python

**Value:** Semantic search capabilities using embeddings alongside keyword search, enabling AI-friendly memory retrieval

## Milestone 4: API Service Layer

**Goal:** Make EngramAI accessible via network API

- Implement gRPC service
  - Define Protocol Buffers for core data types
  - Create essential service endpoints
  - Implement authentication and basic security
  - Design idiomatic API patterns for Rust, Python, Go, and JS clients
- Generate and test client libraries
  - Generate Python client bindings
  - Generate TS client bindings
  - Create comprehensive client examples
- Develop API documentation
  - Create API reference documentation
  - Write endpoint usage examples
  - Document authentication and security
- Build service management
  - Create service configuration system
  - Implement logging and monitoring
  - Add health check endpoints

**Value:** Remote access to memory system, enabling multi-client usage and service-oriented architecture

## Milestone 5: Memory Management (Completed)

**Goal:** Add intelligence to memory management with temporal features and forgetting

- Implement temporal capabilities ✓
  - Add timestamp tracking for all operations ✓
  - Create temporal query operators (before/after) ✓
  - Implement recency-based relevance scoring ✓
- Build forgetting mechanisms ✓
  - Add confidence and importance scoring ✓
  - Implement configurable forgetting policies ✓
  - Create memory pruning and compaction ✓
  - Add TTL support for ephemeral engrams ✓
- Add RocksDB optimizations ✓
  - Implement tiered storage for hot/cold data ✓
  - Optimize for write-heavy workloads ✓
  - Add intelligent compaction policies ✓

**Value:** Intelligent memory management that prioritizes recent and important information, mimicking human memory

## Milestone 6: LLM Integration

**Goal:** Create seamless integration with language models

- Implement LangChain/LlamaIndex adapters
  - Build EngramAI memory class for LangChain
  - Create vector store implementation for LlamaIndex
  - Add memory retrieval augmented generation examples
- Build MCP server basics
  - Implement core MCP server specification
  - Create basic memory tools for MCP
  - Add integration examples with Claude and other MCP clients
  - Build a Remote MCP Server on Cloudflare
- Develop reflection capabilities
  - Create summarization endpoints
  - Implement community detection algorithms
  - Implement memory-based insight generation
  - Develop API hooks for LLM-based summarization

**Value:** Ready-to-use integration with popular LLM frameworks, enabling AI agents to use EngramAI for memory

## Milestone 7: Edge & Cloud Integration

**Goal:** Make EngramAI available on edge computing platforms

- Create TypeScript client
  - Implement TypeScript API wrapper
  - Add client-side validation
  - Create browser-compatible package
- Build Cloudflare Workers integration
  - Develop optimized client for Cloudflare Workers
  - Create D1/KV storage adapter
  - Implement edge-optimized search
- Build Cloudflare AI Agents integration
  - Build demo showing memory-enhanced agents
  - Create edge deployment examples
  - Implement multi-agent memory sharing

**Value:** Ability to use EngramAI on edge computing platforms, enabling memory for serverless AI agents

## Milestone 8: Web User Interface

**Goal:** Create a user-friendly web-based graphical interface for interacting with the EngramAI memory graph

- Design UI architecture and technology stack
- Implement backend component for UI
- Build core UI components and layout
- Implement data browsing and detail views
- Implement data manipulation forms
- Add query and command execution interface
- Integrate interactive graph visualization
- Add UI launch mechanisms
- Develop comprehensive UI documentation

**Value:** Provides an intuitive visual way for users to interact with, explore, and manage their memory graphs

## Milestone 9: Performance & Scalability

**Goal:** Enhance system to handle large-scale deployments

- Implement caching layer
  - Add in-memory cache for hot data
  - Create cache invalidation system
  - Implement configurable caching policies
- Optimize for scale
  - Add parallel query execution
  - Implement connection pooling
  - Create query optimization layer
- Add distributed capabilities
  - Implement basic sharding strategy
  - Create replication support
  - Add cluster coordination

**Value:** Ability to scale to large memory graphs with billions of nodes and high query throughput

## Milestone 10: Production Readiness

**Goal:** Prepare for enterprise adoption with security, monitoring, and administration

- Enhance security
  - Implement at-rest encryption
  - Add comprehensive audit logging
  - Create multi-tenant access controls
- Build monitoring system
  - Add detailed metrics collection
  - Create monitoring dashboards
  - Implement alerting
- Develop administration tools
  - Create admin CLI
  - Build web-based admin interface
  - Add backup/restore system

**Value:** Enterprise-ready memory system with the security, monitoring, and administration tools needed for production use

## Future Explorations

**Goal:** Research directions for future development

- Advanced graph algorithms
  - Implement graph embeddings
  - Add knowledge distillation
  - Create semantic network analysis
- Multi-modal engrams
  - Support image embeddings
  - Add audio and video processing
  - Implement cross-modal retrieval
- Federated memory
  - Create federated storage protocol
  - Implement privacy-preserving sharing
  - Build decentralized memory graph

**Value:** Exploration of cutting-edge memory capabilities for future versions