# Indexing System

EngramAI Lite includes specialized indexes for efficient queries and traversals. This document explains the design and implementation of the indexing system.

## Index Architecture

The indexing system is composed of several specialized indexes:

1. **RelationshipIndex**: Optimized for traversing connections between engrams
2. **MetadataIndex**: Fast lookup of engrams by metadata fields
3. **SearchIndex**: Combined index for efficient search across multiple dimensions
4. **CollectionIndex**: Optimized for collection membership queries

## RelationshipIndex

The `RelationshipIndex` enables fast traversal of connections between engrams:

```rust
pub struct RelationshipIndex {
    /// Maps engram IDs to their outgoing connections
    outgoing_connections: HashMap<EngramId, HashSet<ConnectionId>>,
    
    /// Maps engram IDs to their incoming connections
    incoming_connections: HashMap<EngramId, HashSet<ConnectionId>>,
    
    /// Maps relationship types to connections of that type
    relationship_type_index: HashMap<String, HashSet<ConnectionId>>,
    
    /// Maps source engrams to their target engrams
    source_to_targets: HashMap<EngramId, HashSet<EngramId>>,
    
    /// Maps target engrams to their source engrams
    target_to_sources: HashMap<EngramId, HashSet<EngramId>>,
}
```

### Key Operations

```rust
impl RelationshipIndex {
    /// Add a connection to the index
    pub fn add_connection(&mut self, connection: &Connection) {
        // Update outgoing connections
        self.outgoing_connections
            .entry(connection.source_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Update incoming connections
        self.incoming_connections
            .entry(connection.target_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Update relationship type index
        self.relationship_type_index
            .entry(connection.relationship_type.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Update source-to-targets mapping
        self.source_to_targets
            .entry(connection.source_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.target_id.clone());
        
        // Update target-to-sources mapping
        self.target_to_sources
            .entry(connection.target_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.source_id.clone());
    }
    
    /// Get all connections from a source engram
    pub fn get_outgoing_connections(&self, source_id: &EngramId) -> HashSet<ConnectionId> {
        self.outgoing_connections
            .get(source_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all connections to a target engram
    pub fn get_incoming_connections(&self, target_id: &EngramId) -> HashSet<ConnectionId> {
        self.incoming_connections
            .get(target_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all connections of a specific relationship type
    pub fn get_connections_by_type(&self, relationship_type: &str) -> HashSet<ConnectionId> {
        self.relationship_type_index
            .get(relationship_type)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all target engrams for a source engram
    pub fn get_targets_for_source(&self, source_id: &EngramId) -> HashSet<EngramId> {
        self.source_to_targets
            .get(source_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all source engrams for a target engram
    pub fn get_sources_for_target(&self, target_id: &EngramId) -> HashSet<EngramId> {
        self.target_to_sources
            .get(target_id)
            .cloned()
            .unwrap_or_default()
    }
}
```

## MetadataIndex

The `MetadataIndex` enables fast lookup of engrams by metadata fields:

```rust
pub struct MetadataIndex {
    /// Maps metadata keys to a mapping of values to engram IDs
    metadata_index: HashMap<String, HashMap<String, HashSet<EngramId>>>,
}
```

### Key Operations

```rust
impl MetadataIndex {
    /// Add an engram's metadata to the index
    pub fn add_engram(&mut self, engram: &Engram) {
        for (key, value) in &engram.metadata {
            // Convert value to string for indexing
            let value_str = value.to_string();
            
            // Update the index
            self.metadata_index
                .entry(key.clone())
                .or_insert_with(HashMap::new)
                .entry(value_str)
                .or_insert_with(HashSet::new)
                .insert(engram.id.clone());
        }
    }
    
    /// Get engrams by metadata key and value
    pub fn get_engrams_by_metadata(
        &self,
        key: &str,
        value: &str,
    ) -> HashSet<EngramId> {
        self.metadata_index
            .get(key)
            .and_then(|value_map| value_map.get(value))
            .cloned()
            .unwrap_or_default()
    }
}
```

## SearchIndex

The `SearchIndex` combines multiple indexes for comprehensive search:

```rust
pub struct SearchIndex {
    pub relationship_index: RelationshipIndex,
    pub metadata_index: MetadataIndex,
    source_index: HashMap<String, HashSet<EngramId>>,
    confidence_index: HashMap<u8, HashSet<EngramId>>,
}
```

### Key Operations

```rust
impl SearchIndex {
    /// Add an engram to all indexes
    pub fn add_engram(&mut self, engram: &Engram) {
        // Add to metadata index
        self.metadata_index.add_engram(engram);
        
        // Add to source index
        self.source_index
            .entry(engram.source.clone())
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Add to confidence index (bucketized for range queries)
        let confidence_bucket = (engram.confidence * 10.0) as u8;
        self.confidence_index
            .entry(confidence_bucket)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
    }
    
    /// Add a connection to the relationship index
    pub fn add_connection(&mut self, connection: &Connection) {
        self.relationship_index.add_connection(connection);
    }
    
    /// Search for engrams by source
    pub fn search_by_source(&self, source: &str) -> HashSet<EngramId> {
        self.source_index
            .get(source)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Search for engrams by minimum confidence
    pub fn search_by_min_confidence(&self, min_confidence: f64) -> HashSet<EngramId> {
        let min_bucket = (min_confidence * 10.0) as u8;
        
        // Collect all engrams with confidence >= min_bucket
        let mut result = HashSet::new();
        for bucket in min_bucket..=10 {
            if let Some(engrams) = self.confidence_index.get(&bucket) {
                result.extend(engrams.iter().cloned());
            }
        }
        
        result
    }
    
    /// Perform a combined search across multiple dimensions
    pub fn combined_search(
        &self,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_filters: &[(String, String)],
    ) -> HashSet<EngramId> {
        // Start with all engrams if no source filter
        let mut result = match source {
            Some(src) => self.search_by_source(src),
            None => HashSet::new(), // Will be populated by first filter or return empty
        };
        
        // Apply confidence filter if specified
        if let Some(min_conf) = min_confidence {
            let confidence_matches = self.search_by_min_confidence(min_conf);
            
            // If result is empty, use confidence matches as starting point
            if result.is_empty() {
                result = confidence_matches;
            } else {
                // Otherwise, intersect with current results
                result = result.intersection(&confidence_matches).cloned().collect();
            }
        }
        
        // Apply metadata filters
        for (key, value) in metadata_filters {
            let metadata_matches = self.metadata_index.get_engrams_by_metadata(key, value);
            
            // If result is empty, use metadata matches as starting point
            if result.is_empty() {
                result = metadata_matches;
            } else {
                // Otherwise, intersect with current results
                result = result.intersection(&metadata_matches).cloned().collect();
            }
        }
        
        result
    }
}
```

## CollectionIndex

The `CollectionIndex` optimizes collection membership queries:

```rust
pub struct CollectionIndex {
    /// Maps collection IDs to engram IDs
    collection_to_engrams: HashMap<CollectionId, HashSet<EngramId>>,
    
    /// Maps engram IDs to collection IDs
    engram_to_collections: HashMap<EngramId, HashSet<CollectionId>>,
}
```

### Key Operations

```rust
impl CollectionIndex {
    /// Add a collection to the index
    pub fn add_collection(&mut self, collection: &Collection) {
        // Initialize collection entry if not exists
        let engrams = self.collection_to_engrams
            .entry(collection.id.clone())
            .or_insert_with(HashSet::new);
        
        // Add all engrams in the collection
        for engram_id in &collection.engram_ids {
            engrams.insert(engram_id.clone());
            
            // Update reverse mapping
            self.engram_to_collections
                .entry(engram_id.clone())
                .or_insert_with(HashSet::new)
                .insert(collection.id.clone());
        }
    }
    
    /// Get all engrams in a collection
    pub fn get_engrams_in_collection(&self, collection_id: &CollectionId) -> HashSet<EngramId> {
        self.collection_to_engrams
            .get(collection_id)
            .cloned()
            .unwrap_or_default()
    }
    
    /// Get all collections containing an engram
    pub fn get_collections_for_engram(&self, engram_id: &EngramId) -> HashSet<CollectionId> {
        self.engram_to_collections
            .get(engram_id)
            .cloned()
            .unwrap_or_default()
    }
}
```

## Index Synchronization

Indexes are kept in sync with the main data store through the following mechanisms:

1. **Write-Through**: Updates to entities update the indexes immediately
2. **Bulk Loading**: Indexes can be rebuilt from storage for recovery or initialization
3. **Transactional Consistency**: Index updates are part of the same transaction as data updates

## Performance Characteristics

| Index Type | Operation | Time Complexity | Space Complexity |
|------------|-----------|-----------------|------------------|
| RelationshipIndex | Get connections | O(1) | O(E) where E is number of connections |
| MetadataIndex | Query by metadata | O(1) | O(M × E) where M is number of metadata fields |
| SearchIndex | Combined search | O(min(result set sizes)) | O(E) |
| CollectionIndex | Get engrams in collection | O(1) | O(E + C) where C is number of collections |

## Future Index Enhancements

Future enhancements to the indexing system may include:

1. **Vector Indexes**: HNSW or other ANN indexes for embedding search
2. **Temporal Indexes**: Optimize time-based queries
3. **Full-Text Indexes**: BM25 or similar text search capabilities
4. **Persistent Indexes**: Store indexes in RocksDB for persistence
5. **Incremental Updates**: Optimize index updates for large-scale changes