# Comparison: EngramAI Lite vs. Zep/Graphiti

Based on an analysis of the EngramAI Lite codebase and the Zep/Graphiti paper, here's a comprehensive comparison of these knowledge graph systems for AI agent memory:

## 1. Core Architecture

### EngramAI Lite
- Written in Rust with Python bindings
- Uses petgraph as its core graph engine
- RocksDB for persistent storage
- Multiple specialized indexes for different query types
- Memory management inspired by human cognitive processes

### Zep/Graphiti
- Knowledge graph engine built on Neo4j (uses Lucene)
- Hierarchical three-tier graph structure: episode, semantic entity, and community subgraphs
- Bi-temporal model for tracking both chronological and ingestion timelines
- LLM-based entity extraction, relationship detection, and community formation

## 2. Data Model

### EngramAI Lite
- **Engram**: Atomic unit of knowledge with metadata (confidence, timestamp, source, importance)
- **Connection**: Typed relationship between engrams with strength/weight
- **Collection**: Named grouping of engrams for organization
- **Agent**: Entity with capabilities and access controls
- **Context**: Shareable environment for agent collaboration

### Zep/Graphiti
- **Episode**: Raw input data (messages, text, JSON)
- **Entity**: Semantic entities extracted from episodes and resolved with existing graph entities
- **Fact/Edge**: Relationships between entities with temporal validity information
- **Community**: High-level clusters of strongly connected entities with summarizations

## 3. Temporal Awareness

### EngramAI Lite
- **TemporalIndex**: Multi-granular time buckets (year/month/day/hour)
- Recency list for recently accessed engrams
- Time range queries (before, after, between)
- Access tracking with timestamps

### Zep/Graphiti
- **Bi-temporal model** with two timelines:
  - T timeline: Chronological ordering of events
  - T' timeline: Transactional order of data ingestion
- Four timestamps per edge: t'created, t'expired (system times) and t_valid, t_invalid (factual validity)
- Can extract relative temporal information from messages (e.g., "next Thursday")

## 4. Entity/Knowledge Extraction

### EngramAI Lite
- Manual engram creation and connection (focus on storage and retrieval)
- No built-in LLM-based extraction (though this could be implemented externally)

### Zep/Graphiti
- Automated entity extraction using LLMs with context-aware processing
- Entity resolution to avoid duplicates
- Fact extraction between entities
- Reflection technique inspired by "reflexion" to minimize hallucinations

## 5. Memory Management

### EngramAI Lite
- **Importance scoring** based on centrality, access frequency, and recency
- **Forgetting mechanisms** with multiple policies:
  - Age-based
  - Importance threshold
  - Access frequency
  - Hybrid approaches
  - TTL-based expiration
- Access tracking and importance recalculation

### Zep/Graphiti
- **Edge invalidation** for contradictory information
- **Dynamic community updates** through label propagation algorithm
- Historical tracking of relationship evolution
- No explicit forgetting mechanisms mentioned in the paper

## 6. Search and Retrieval

### EngramAI Lite
- Multiple specialized indexes (text, metadata, relationship, temporal)
- Vector embedding support with HNSW index
- Hybrid search combining vectors and keywords
- BM25 scoring integration
- Path finding between engrams
- Relationship-based traversal

### Zep/Graphiti
- Three search functions:
  - Cosine semantic similarity (vector embeddings)
  - Okapi BM25 full-text search
  - Breadth-first graph search
- Multiple reranking strategies:
  - Reciprocal Rank Fusion (RRF)
  - Maximal Marginal Relevance (MMR)
  - Graph-based episode-mentions
  - Node distance
  - Cross-encoder LLMs

## 7. Benchmarks and Performance

### EngramAI Lite
- No published benchmark results, but focuses on performance optimization
- RocksDB for persistence with indexing for fast retrieval
- In-memory graph representation for fast traversal

### Zep/Graphiti
- Outperforms MemGPT in Deep Memory Retrieval (DMR) benchmark (94.8% vs 93.4%)
- Evaluated on LongMemEval benchmark with complex temporal reasoning tasks
- Achieves accuracy improvements of up to 18.5% while reducing response latency by 90%
- Particularly strong in enterprise-critical tasks like cross-session information synthesis

## 8. Community Structure

### EngramAI Lite
- **Collection**: Named grouping of engrams (similar to traditional categorization)
- No automatic community detection mentioned

### Zep/Graphiti
- **Community nodes**: Automatically detected clusters of strongly connected entities
- Uses label propagation algorithm with dynamic extension
- Communities contain high-level summarizations of clusters
- Enables global understanding of the domain

## 9. Implementation and Integration

### EngramAI Lite
- Rust implementation with Python bindings via PyO3
- CLI and web UI interfaces
- GRPC service for remote access
- Plans for LangChain/LlamaIndex adapters (mentioned in roadmap)

### Zep/Graphiti
- Production system with APIs for integration
- Hosted service in AWS
- Optimized for real-world enterprise applications
- Graph search API with formatting for LLM context

## 10. Unique Strengths

### EngramAI Lite
- Strong focus on memory management principles (importance, forgetting, TTL)
- Comprehensive indexing system for various query types
- Built-in access control through Agent and Context concepts
- Pure Rust implementation with potential performance benefits

### Zep/Graphiti
- Hierarchical organization (episodes → facts → entities → communities)
- Dynamic, non-lossy knowledge graph that represents an evolving world
- Automatic entity and fact extraction using LLMs
- Temporal contradiction resolution
- Proven performance in benchmarks against other memory systems

## Conclusion

Both EngramAI Lite and Zep/Graphiti represent advanced approaches to knowledge graphs for AI agent memory, but with different focuses:

1. EngramAI Lite is a more general-purpose memory graph system with strong emphasis on memory management principles like importance scoring and forgetting mechanisms. It provides a comprehensive data model and indexing system but requires manual knowledge entry.

2. Zep/Graphiti is specifically optimized for conversational memory with automated entity and relationship extraction using LLMs. Its bi-temporal model and edge invalidation make it particularly strong for handling evolving information and contradictions.

For applications requiring automated extraction from conversations with complex temporal reasoning, Zep/Graphiti appears more advanced. For applications requiring fine-grained control over memory importance and forgetting with custom knowledge structures, EngramAI Lite may be more suitable.

The hierarchical community structure in Zep/Graphiti represents a significant advantage for understanding the global structure of knowledge, while EngramAI Lite's comprehensive indexing and query capabilities may offer more flexibility for certain applications.