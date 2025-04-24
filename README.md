# EngramAI Memory Graph System

EngramAI is a knowledge memory graph system for AI agents, designed to store and retrieve information in a structured way that mimics human cognitive processes. The system organizes knowledge as "engrams" (knowledge units) connected in a semantic network with typed relationships, supporting context-aware memory retrieval and multi-agent collaboration.

## Features

- **Core Memory Types**
  - **Engram**: Atomic unit of knowledge with metadata (confidence, timestamp, source)
  - **Connection**: Typed relationship between engrams with strength/weight
  - **Collection**: Named grouping of engrams for organization
  - **Agent**: Entity with capabilities and access controls
  - **Context**: Shareable environment with relevant engrams for agent collaboration

- **Storage & Retrieval**
  - Persistent storage using RocksDB
  - In-memory graph representation using petgraph
  - Fast key-value operations for engram storage and retrieval
  - ACID transaction support for data integrity

- **Access Control & Collaboration**
  - Fine-grained permissions for collections and contexts
  - Multi-agent collaboration through shared contexts
  - Typed, weighted connections with metadata

## Getting Started

### Prerequisites

- Rust (latest stable)
- Cargo

### Installation

```bash
# Clone the repository
git clone https://github.com/nkkko/engram-lite.git
cd engram-lite

# Build the project
cargo build --release
```

### Usage

#### Run the Demo

```bash
cargo run --release
```

#### Run the CLI

```bash
cargo run --release --bin engram_cli
```

CLI commands:
- `help` - Show available commands
- `add-engram <content>;<source>;<confidence>` - Add a new engram
- `get-engram <id>` - Get engram by ID
- `add-connection <source-id>;<target-id>;<type>;<weight>` - Add a connection
- `create-collection <name>;<description>` - Create a new collection
- `add-to-collection <engram-id>;<collection-id>` - Add engram to collection
- `create-agent <name>;<description>` - Create a new agent
- `grant-access <agent-id>;<collection-id>` - Grant collection access to agent
- `query <source>` - Query engrams by source
- `list-engrams` - List all engrams
- `export <file-path>` - Export data to JSON
- `import <file-path>` - Import data from JSON

## Architecture

EngramAI consists of several core components:

1. **Schema**: Core data types representing knowledge units, connections, and organizational structures
2. **Storage**: RocksDB-based persistent storage with transaction support
3. **Graph**: In-memory graph representation for fast traversal and queries
4. **CLI**: Command-line interface for interacting with the system

## Future Development

- Vector embeddings for semantic search
- Graph-based retrieval optimization
- Forgetting mechanisms and abstraction policies
- Temporal reasoning and time-based queries
- Large-scale performance optimizations

## License

This project is licensed under the MIT License - see the LICENSE file for details.