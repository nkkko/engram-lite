# Changelog

All notable changes to EngramAI Lite will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Vector embedding support for engrams
- HNSW index for fast vector search
- Dimension reduction for optimizing embeddings
- Enhanced relationship indexing in RocksDB
- Python client bindings with PyO3

### Changed
- Improved performance of graph traversal
- Enhanced query builders with more options

### Fixed
- Resolved memory leaks in long-running operations
- Fixed issue with transaction rollback

## [0.3.0] - 2023-04-28

### Added
- Vector Search & Hybrid Retrieval capabilities
- Multiple embedding model support
- Hybrid search combining vectors and keywords
- BM25 scoring integration
- Python client library

### Changed
- Completely redesigned storage layer for vector support
- Enhanced query interface for more complex searches

### Fixed
- Performance issues with large graphs
- Memory consumption during batch operations

## [0.2.0] - 2023-03-15

### Added
- Memory Graph implementation with petgraph
- Basic search capabilities
- Typed, weighted connections
- Relationship traversal
- CLI and TUI interface

### Changed
- Switched from in-memory to RocksDB for persistence
- Updated interface for better developer experience

### Fixed
- Data loss issues during export/import
- Concurrent access problems

## [0.1.0] - 2023-02-01

### Added
- Initial release
- Core data model (Engram, Connection, Collection, Agent, Context)
- Basic storage with RocksDB
- Transaction support
- Command line tools for data manipulation
- Documentation

[Unreleased]: https://github.com/yourusername/engram-lite/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/yourusername/engram-lite/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/yourusername/engram-lite/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/yourusername/engram-lite/releases/tag/v0.1.0