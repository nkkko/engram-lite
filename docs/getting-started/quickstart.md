# Quickstart Guide

This guide will help you get started with EngramAI Lite, covering the essential operations to begin storing and retrieving knowledge.

## Starting the CLI

The primary way to interact with EngramAI Lite is through the command-line interface:

```bash
# Start with default database location (./engram_db)
engramlt

# Or specify a custom database location
engramlt --db-path /path/to/database
```

## Basic Operations

Once you start the CLI, you can use the following commands to perform basic operations:

### Adding Engrams

Engrams are the atomic units of knowledge in the system:

```
> add-engram The sky is blue;observation;0.9
Engram added with ID: 3a7c9f8e-1234-5678-90ab-cdef01234567
```

The format is: `add-engram <content>;<source>;<confidence>`

- **content**: The knowledge/information to store
- **source**: Where this knowledge came from (e.g., observation, book, website)
- **confidence**: A value between 0.0 and 1.0 indicating certainty

### Creating Connections

Connect engrams to build a knowledge graph:

```
> add-connection 3a7c9f8e-1234-5678-90ab-cdef01234567;5e8f7d6c-1234-5678-90ab-cdef01234567;related;0.8
Connection added with ID: 7b2d1e9c-1234-5678-90ab-cdef01234567
```

The format is: `add-connection <source-engram-id>;<target-engram-id>;<relationship-type>;<weight>`

- **source-engram-id**: ID of the first engram
- **target-engram-id**: ID of the second engram
- **relationship-type**: The type of relationship (e.g., "causes", "relates", "contradicts")
- **weight**: Strength of the connection between 0.0 and 1.0

### Creating Collections

Collections help organize engrams into meaningful groups:

```
> create-collection Weather;Daily weather observations
Collection created with ID: 9c4b8a7f-1234-5678-90ab-cdef01234567
```

The format is: `create-collection <name>;<description>`

### Adding Engrams to Collections

```
> add-to-collection 3a7c9f8e-1234-5678-90ab-cdef01234567;9c4b8a7f-1234-5678-90ab-cdef01234567
Engram added to collection
```

The format is: `add-to-collection <engram-id>;<collection-id>`

## Querying and Retrieving Data

### Retrieving Engrams

Get a specific engram by ID:

```
> get-engram 3a7c9f8e-1234-5678-90ab-cdef01234567
ID: 3a7c9f8e-1234-5678-90ab-cdef01234567
Content: The sky is blue
Source: observation
Confidence: 0.9
Timestamp: 2023-06-15T14:30:22Z
```

### Listing Engrams

```
> list-engrams
Engrams:
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The sky is blue (source: observation, confidence: 0.9)
  [5e8f7d6c-1234-5678-90ab-cdef01234567] Rain forms when water vapor condenses (source: science, confidence: 0.95)
```

### Querying by Source

```
> query observation
Engrams from source 'observation':
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The sky is blue (confidence: 0.9)
```

### Filtering by Confidence

```
> filter-by-confidence 0.9
Engrams with confidence ≥ 0.9:
  [3a7c9f8e-1234-5678-90ab-cdef01234567] The sky is blue (confidence: 0.9, source: observation)
  [5e8f7d6c-1234-5678-90ab-cdef01234567] Rain forms when water vapor condenses (confidence: 0.95, source: science)
```

## Data Maintenance

### System Statistics

```
> stats
EngramAI System Statistics:
  Engrams:     24
  Connections: 37
  Collections: 3
  Agents:      2
  Contexts:    1
```

### Database Compaction

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

## Import and Export

### Exporting Data

```
> export memory_backup.json
Exporting all data to memory_backup.json
Data exported successfully
```

### Exporting a Specific Collection

```
> export weather_backup.json;9c4b8a7f-1234-5678-90ab-cdef01234567
Exporting collection 9c4b8a7f-1234-5678-90ab-cdef01234567 to weather_backup.json
Collection exported successfully
```

### Importing Data

```
> import memory_backup.json
Importing data from memory_backup.json
Data imported successfully
Refreshing memory graph...
```

## Exiting the CLI

```
> exit
```

or

```
> quit
```

## Next Steps

Now that you're familiar with the basic operations, check out the [CLI Documentation](../usage/cli.md) for a complete reference of all available commands.