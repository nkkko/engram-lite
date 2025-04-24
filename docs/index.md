# EngramAI Lite

EngramAI Lite is a memory graph storage system designed for AI agents. It provides a structured way to store and retrieve knowledge, mimicking human cognitive processes.

## Features

- **Memory Graph Storage**: Store knowledge in a structured graph with typed, weighted connections
- **Persistent Storage**: RocksDB-based persistence with ACID transaction support
- **Efficient Indexes**: Fast traversal and search capabilities for relationship queries
- **CLI Tool**: Command-line interface for interacting with the memory graph
- **Import/Export**: Backup and restore your memory graph with JSON-based format

## Quick Example

```bash
# Add an engram (atomic unit of knowledge)
engramlt add-engram "The capital of France is Paris";"geography";0.95

# Add another engram
engramlt add-engram "Paris is known for the Eiffel Tower";"landmarks";0.9

# Create a connection between them
engramlt add-connection <engram1-id>;<engram2-id>;"related";0.8

# Query engrams by source
engramlt query geography

# Export your memory graph
engramlt export memory_backup.json
```

## Why EngramAI?

Traditional databases weren't designed for AI memory. EngramAI Lite provides:

- **Cognitive Structure**: Mirrors how humans organize and connect information
- **Relationship-First**: Focuses on connections between knowledge fragments
- **Semantic + Symbolic**: Combines symbolic representations with vector embeddings
- **AI-Optimized**: Designed specifically for agent memory needs

## Installation

```bash
# Clone the repository
git clone https://github.com/nkkko/engram-lite.git
cd engram-lite

# Build from source
cargo build --release

# Run the CLI
./target/release/engramlt
```

For more detailed installation instructions, see the [Installation Guide](getting-started/installation.md).

## Project Status

EngramAI Lite is in active development. Check the [Roadmap](about/roadmap.md) for upcoming features.