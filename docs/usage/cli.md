# CLI Reference

EngramAI Lite provides a comprehensive command-line interface (CLI) for interacting with the memory graph. This document describes all available commands and their usage.

## Starting the CLI

```bash
# Start with default database location
engramlt

# Specify a custom database path
engramlt --db-path /path/to/database
```

## Command Categories

The CLI commands are organized into the following categories:

- Creation Commands: Add new data to the memory graph
- Retrieval Commands: Get specific data from the memory graph
- List Commands: Show multiple items of a specific type
- Relationship Commands: Manage connections between items
- Data Maintenance Commands: Maintain and optimize the database
- Import/Export Commands: Backup and restore data
- System Commands: Basic system operations

## Creation Commands

### add-engram

Adds a new engram (atomic unit of knowledge) to the memory graph.

```
> add-engram <content>;<source>;<confidence>
```

Example:
```
> add-engram The capital of France is Paris;geography;0.95
Engram added with ID: 3a7c9f8e-1234-5678-90ab-cdef01234567
```

Parameters:
- `content`: The knowledge/information content
- `source`: Where this knowledge came from
- `confidence`: A value between 0.0 and 1.0 indicating certainty

### add-connection

Creates a typed, weighted connection between two engrams.

```
> add-connection <source-id>;<target-id>;<type>;<weight>
```

Example:
```
> add-connection 3a7c9f8e-1234-5678-90ab-cdef01234567;5e8f7d6c-1234-5678-90ab-cdef01234567;relates;0.8
Connection added with ID: 7b2d1e9c-1234-5678-90ab-cdef01234567
```

Parameters:
- `source-id`: ID of the source engram
- `target-id`: ID of the target engram
- `type`: The relationship type (e.g., "causes", "supports", "contradicts")
- `weight`: Strength of the connection between 0.0 and 1.0

### create-collection

Creates a new collection for organizing engrams.

```
> create-collection <name>;<description>
```

Example:
```
> create-collection Geography;Knowledge about places and locations
Collection created with ID: 9c4b8a7f-1234-5678-90ab-cdef01234567
```

Parameters:
- `name`: Name of the collection
- `description`: Description of what this collection represents

### create-agent

Creates a new agent entity in the system.

```
> create-agent <name>;<description>
```

Example:
```
> create-agent Geography_Assistant;Helps with geography questions
Agent created with ID: b1a2c3d4-1234-5678-90ab-cdef01234567
```

Parameters:
- `name`: Name of the agent
- `description`: Description of the agent's role/purpose

## Retrieval Commands

### get-engram

Retrieves a specific engram by ID.

```
> get-engram <id>
```

Example:
```
> get-engram 3a7c9f8e-1234-5678-90ab-cdef01234567
ID: 3a7c9f8e-1234-5678-90ab-cdef01234567
Content: The capital of France is Paris
Source: geography
Confidence: 0.95
Timestamp: 2023-06-15T14:30:22Z
```

Parameters:
- `id`: The unique identifier of the engram

### query

Queries engrams by source.

```
> query <source>
```

Example:
```
> query geography
Engrams from source 'geography':
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The capital of France is Paris (confidence: 0.95)
  [5e8f7d6c-1234-5678-90ab-cdef01234567] The Amazon River is in South America (confidence: 0.95)
```

Parameters:
- `source`: The source to filter by

### filter-by-source

Alternative way to filter engrams by source.

```
> filter-by-source <source>
```

Parameters:
- `source`: The source to filter by

### filter-by-confidence

Filters engrams by minimum confidence level.

```
> filter-by-confidence <value>
```

Example:
```
> filter-by-confidence 0.9
Engrams with confidence ≥ 0.9:
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The capital of France is Paris (confidence: 0.95, source: geography)
  [5e8f7d6c-1234-5678-90ab-cdef01234567] The Amazon River is in South America (confidence: 0.95, source: geography)
```

Parameters:
- `value`: Minimum confidence value (between 0.0 and 1.0)

## List Commands

### list-engrams

Lists all engrams in the memory graph (up to 100, sorted by recency).

```
> list-engrams
```

### list-collections

Lists all collections in the memory graph.

```
> list-collections
```

Example:
```
> list-collections
Collections:
  [9c4b8a7f-1234-5678-90ab-cdef01234567] Geography - Knowledge about places and locations (5 engrams)
  [e7f8d9c0-1234-5678-90ab-cdef01234567] History - Historical events and facts (12 engrams)
```

### list-agents

Lists all agents in the memory graph.

```
> list-agents
```

Example:
```
> list-agents
Agents:
  [b1a2c3d4-1234-5678-90ab-cdef01234567] Geography_Assistant - Helps with geography questions (access to 1 collections)
```

### list-contexts

Lists all contexts in the memory graph.

```
> list-contexts
```

Example:
```
> list-contexts
Contexts:
  [f1e2d3c4-1234-5678-90ab-cdef01234567] Study_Session - Context for geography education (7 engrams, 2 agents)
```

## Relationship Commands

### add-to-collection

Adds an engram to a collection.

```
> add-to-collection <engram-id>;<collection-id>
```

Example:
```
> add-to-collection 3a7c9f8e-1234-5678-90ab-cdef01234567;9c4b8a7f-1234-5678-90ab-cdef01234567
Engram added to collection
```

Parameters:
- `engram-id`: ID of the engram to add
- `collection-id`: ID of the collection to add to

### grant-access

Grants an agent access to a collection.

```
> grant-access <agent-id>;<collection-id>
```

Example:
```
> grant-access b1a2c3d4-1234-5678-90ab-cdef01234567;9c4b8a7f-1234-5678-90ab-cdef01234567
Access granted
```

Parameters:
- `agent-id`: ID of the agent
- `collection-id`: ID of the collection to grant access to

## Data Maintenance Commands

### delete-engram

Deletes an engram and all its connections.

```
> delete-engram <id>
```

Example:
```
> delete-engram 3a7c9f8e-1234-5678-90ab-cdef01234567
Deleted engram with ID: 3a7c9f8e-1234-5678-90ab-cdef01234567
Also deleted 2 related connections
```

Parameters:
- `id`: ID of the engram to delete

### delete-connection

Deletes a connection between engrams.

```
> delete-connection <id>
```

Parameters:
- `id`: ID of the connection to delete

### delete-collection

Deletes a collection (does not delete the engrams in it).

```
> delete-collection <id>
```

Parameters:
- `id`: ID of the collection to delete

### delete-agent

Deletes an agent.

```
> delete-agent <id>
```

Parameters:
- `id`: ID of the agent to delete

### delete-context

Deletes a context.

```
> delete-context <id>
```

Parameters:
- `id`: ID of the context to delete

### stats

Shows system statistics.

```
> stats
```

Example:
```
> stats
EngramAI System Statistics:
  Engrams:     24
  Connections: 37
  Collections: 3
  Agents:      2
  Contexts:    1
```

### compact

Compacts the database to reclaim space and optimize performance.

```
> compact
```

Example:
```
> compact
Compacting database...
  Compacted column family: engrams
  Compacted column family: connections
  Compacted column family: collections
  Compacted column family: agents
  Compacted column family: contexts
  Compacted column family: metadata
Database compaction completed
```

### refresh

Reloads the memory graph from storage.

```
> refresh
```

## Import/Export Commands

### export

Exports data to a JSON file.

```
> export <file-path>
```

Export a specific collection:
```
> export <file-path>;<collection-id>
```

Examples:
```
> export memory_backup.json
Exporting all data to memory_backup.json
Data exported successfully

> export geography_backup.json;9c4b8a7f-1234-5678-90ab-cdef01234567
Exporting collection 9c4b8a7f-1234-5678-90ab-cdef01234567 to geography_backup.json
Collection exported successfully
```

Parameters:
- `file-path`: Path to save the exported file
- `collection-id` (optional): ID of the collection to export

### import

Imports data from a JSON file.

```
> import <file-path>
```

Example:
```
> import memory_backup.json
Importing data from memory_backup.json
Data imported successfully
Refreshing memory graph...
```

Parameters:
- `file-path`: Path to the file to import

## System Commands

### help

Shows available commands and their descriptions.

```
> help
```

### exit / quit

Exits the program.

```
> exit
```

or

```
> quit
```