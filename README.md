# EngramAI Lite

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

EngramAI Lite is a memory graph storage system designed for AI agents. It provides a structured way to store and retrieve knowledge, mimicking human cognitive processes.

## Features

- **Memory Graph Storage**: Store knowledge in a structured graph with typed, weighted connections
- **Persistent Storage**: RocksDB-based persistence with ACID transaction support
- **Efficient Indexes**: Fast traversal and search capabilities for relationship queries
- **CLI Tool**: Command-line interface for interacting with the memory graph
- **Web UI**: Browser-based interface for exploring the memory graph
- **Demo Data**: Realistic multi-agent collaboration scenario with sample data
- **Import/Export**: Backup and restore your memory graph with JSON-based format

## Core Components

- **Engram**: Atomic unit of knowledge with metadata (confidence, timestamp, source)
- **Connection**: Typed relationship between engrams with strength/weight
- **Collection**: Named grouping of engrams for organization
- **Agent**: Entity with capabilities and access controls
- **Context**: Shareable environment for agent collaboration

## Quick Start

### Installation

```bash
# Clone the repository
git clone https://github.com/nkkko/engram-lite.git
cd engram-lite

# Build from source
cargo build --release
```

### Usage

```bash
# Run the CLI
./target/release/engramlt

# With custom database path
./target/release/engramlt --db-path /path/to/database

# Populate with demo data
./target/release/engramlt demo --db-path /path/to/database

# Start web UI
./target/release/engramlt web --db-path /path/to/database
```

### Example Commands

```bash
# Add an engram
> add-engram The capital of France is Paris;geography;0.95
Engram added with ID: 3a7c9f8e-1234-5678-90ab-cdef01234567

# Add another engram
> add-engram Paris is known for the Eiffel Tower;landmarks;0.9
Engram added with ID: 5e8f7d6c-1234-5678-90ab-cdef01234567

# Create a connection between them
> add-connection 3a7c9f8e-1234-5678-90ab-cdef01234567;5e8f7d6c-1234-5678-90ab-cdef01234567;related;0.8
Connection added with ID: 7b2d1e9c-1234-5678-90ab-cdef01234567

# List all engrams
> list-engrams
Engrams:
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The capital of France is Paris (source: geography, confidence: 0.95)
  [5e8f7d6c-1234-5678-90ab-cdef01234567] Paris is known for the Eiffel Tower (source: landmarks, confidence: 0.9)

# Query engrams by source
> query geography
Engrams from source 'geography':
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The capital of France is Paris (confidence: 0.95)
```

## Architecture

EngramAI Lite consists of several core components:

### 1. Data Model

Core type definitions that represent knowledge units, connections, and organizational structures.

### 2. Storage Layer

RocksDB-based persistent storage with column families for different entity types and ACID transaction support.

### 3. Graph Engine

In-memory graph representation using `petgraph` for fast traversal and query operations.

### 4. Indexing System

Specialized indexes for efficient queries and traversals:
- RelationshipIndex: Optimized for traversing connections between engrams
- MetadataIndex: Fast lookup of engrams by metadata fields
- SearchIndex: Combined index for efficient search across multiple dimensions

### 5. CLI Interface

Command-line interface for interacting with the memory graph.

### 6. Web UI & Demo Data

Browser-based interface for exploring the memory graph with pre-populated demo data showcasing a multi-agent collaboration scenario.

## Documentation

For detailed documentation, see:

- [Installation Guide](docs/getting-started/installation.md)
- [Quickstart Guide](docs/getting-started/quickstart.md)
- [CLI Reference](docs/usage/cli.md)
- [Web UI Guide](docs/usage/tui.md)
- [Data Model](docs/design/data-model.md)
- [Storage Design](docs/design/storage.md)
- [Graph Engine](docs/design/graph-engine.md)
- [Indexing System](docs/design/indexing.md)

## Roadmap

See the [Roadmap](docs/about/roadmap.md) for information about upcoming features and milestones.

## Contributing

Contributions are welcome! See the [Contributing Guide](docs/about/contributing.md) for more information.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.