# Indexing System

EngramAI Lite includes specialized indexes for efficient queries and traversals. This document explains the design and implementation of the indexing system.

## Index Architecture

The indexing system is composed of several specialized indexes:

1. **RelationshipIndex**: Optimized for traversing connections between engrams
2. **MetadataIndex**: Fast lookup of engrams by metadata fields
3. **TextIndex**: Fast keyword search across engram content
4. **TemporalIndex**: Time-based organization and retrieval of engrams
5. **ImportanceIndex**: Tracks importance scores, access patterns, and TTL
6. **SearchIndex**: Combined index for efficient search across multiple dimensions
7. **CollectionIndex**: Optimized for collection membership queries

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
    pub text_index: TextIndex,
    pub temporal_index: TemporalIndex,
    pub importance_index: ImportanceIndex,
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
    
    /// Perform a comprehensive search across multiple dimensions
    pub fn search_combined(
        &self,
        text_query: Option<&str>,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_key: Option<&str>,
        metadata_value: Option<&str>,
        exact_match: bool,
        before_time: Option<&chrono::DateTime<chrono::Utc>>,
        after_time: Option<&chrono::DateTime<chrono::Utc>>,
        min_importance: Option<f64>,
        min_access_count: Option<u32>,
    ) -> HashSet<EngramId> {
        let mut final_result: Option<HashSet<EngramId>> = None;
        
        // Apply text search if provided
        if let Some(query) = text_query {
            let text_results = if exact_match {
                self.text_index.search_all(query)
            } else {
                self.text_index.search(query)
            };
            
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&text_results).cloned().collect(),
                None => text_results,
            });
        }
        
        // Apply source filter if provided
        if let Some(source_str) = source {
            let source_results = self.find_by_source(source_str);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&source_results).cloned().collect(),
                None => source_results,
            });
        }
        
        // Apply confidence filter if provided
        if let Some(conf) = min_confidence {
            let conf_results = self.find_by_min_confidence(conf);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&conf_results).cloned().collect(),
                None => conf_results,
            });
        }
        
        // Apply metadata filters if provided
        if let Some(key) = metadata_key {
            let key_results = if let Some(value) = metadata_value {
                self.metadata_index.find_by_key_value(key, value)
            } else {
                self.metadata_index.find_by_key(key)
            };
            
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&key_results).cloned().collect(),
                None => key_results,
            });
        }
        
        // Apply temporal filters if provided
        if let Some(time) = before_time {
            let time_results = self.find_by_before_timestamp(time);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&time_results).cloned().collect(),
                None => time_results,
            });
        }
        
        if let Some(time) = after_time {
            let time_results = self.find_by_after_timestamp(time);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&time_results).cloned().collect(),
                None => time_results,
            });
        }
        
        // Apply importance filter if provided
        if let Some(importance) = min_importance {
            let importance_results = self.find_by_min_importance(importance);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&importance_results).cloned().collect(),
                None => importance_results,
            });
        }
        
        // Apply access count filter if provided
        if let Some(count) = min_access_count {
            let access_results = self.find_by_min_access_count(count);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&access_results).cloned().collect(),
                None => access_results,
            });
        }
        
        final_result.unwrap_or_else(HashSet::new)
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
| TextIndex | Keyword search | O(K) where K is number of keywords | O(W × E) where W is total words |
| TemporalIndex | Time-based queries | O(1) for fixed time periods | O(E) with constant factor overhead |
| ImportanceIndex | Importance/access queries | O(1) for threshold queries | O(E) with overhead for sorting |
| SearchIndex | Combined search | O(min(result set sizes)) | O(E) with overhead for all indexes |
| CollectionIndex | Get engrams in collection | O(1) | O(E + C) where C is number of collections |

## TemporalIndex

The `TemporalIndex` enables efficient time-based queries and recency-based sorting:

```rust
pub struct TemporalIndex {
    /// Engrams indexed by year
    year_index: HashMap<i32, HashSet<EngramId>>,
    
    /// Engrams indexed by year-month (format: YYYYMM as i32)
    month_index: HashMap<i32, HashSet<EngramId>>,
    
    /// Engrams indexed by year-month-day (format: YYYYMMDD as i32)
    day_index: HashMap<i32, HashSet<EngramId>>,
    
    /// Engrams indexed by hour buckets (24 buckets, 0-23)
    hour_index: HashMap<u8, HashSet<EngramId>>,
    
    /// Sorted list of engram IDs by recency (most recent first)
    recency_list: Vec<EngramId>,
    
    /// Map of engram IDs to their timestamp for quick access
    timestamp_map: HashMap<EngramId, chrono::DateTime<chrono::Utc>>,
}
```

### Key Operations

```rust
impl TemporalIndex {
    /// Add an engram to the index
    pub fn add_engram(&mut self, engram: &Engram) -> Result<()> {
        let timestamp = engram.timestamp;
        
        // Extract time components
        let year = timestamp.year();
        let month = timestamp.month() as i32;
        let day = timestamp.day() as i32;
        let hour = timestamp.hour() as u8;
        
        // Index by year
        self.year_index
            .entry(year)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Index by year-month
        let year_month = year * 100 + month;
        self.month_index
            .entry(year_month)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Index by year-month-day
        let year_month_day = year_month * 100 + day;
        self.day_index
            .entry(year_month_day)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Index by hour
        self.hour_index
            .entry(hour)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Store timestamp and update recency list
        self.timestamp_map.insert(engram.id.clone(), timestamp);
        self.update_recency_list(engram.id.clone(), timestamp);
        
        Ok(())
    }
    
    /// Find engrams created on a specific day
    pub fn find_by_day(&self, year: i32, month: u32, day: u32) -> HashSet<EngramId> {
        let year_month = year * 100 + month as i32;
        let year_month_day = year_month * 100 + day as i32;
        
        self.day_index
            .get(&year_month_day)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams created after a specific timestamp
    pub fn find_after(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        let mut result = HashSet::new();
        
        for (id, ts) in &self.timestamp_map {
            if ts > timestamp {
                result.insert(id.clone());
            }
        }
        
        result
    }
    
    /// Get most recent engrams
    pub fn get_most_recent(&self, count: usize) -> Vec<EngramId> {
        self.recency_list.iter().take(count).cloned().collect()
    }
}
```

## ImportanceIndex

The `ImportanceIndex` tracks importance scores, access patterns, and TTL-based expiration:

```rust
pub struct ImportanceIndex {
    /// Engrams indexed by importance buckets (0.0-1.0 in 0.1 increments)
    importance_buckets: HashMap<u8, HashSet<EngramId>>,
    
    /// Engrams sorted by importance score (most important first)
    importance_sorted: Vec<(EngramId, f64)>,
    
    /// Engrams indexed by access frequency buckets
    access_buckets: HashMap<u8, HashSet<EngramId>>,
    
    /// Engrams sorted by access recency (most recently accessed first)
    recency_sorted: Vec<(EngramId, chrono::DateTime<chrono::Utc>)>,
    
    /// Map from engram ID to importance score for quick lookup
    importance_map: HashMap<EngramId, f64>,
    
    /// Map from engram ID to access count for quick lookup
    access_count_map: HashMap<EngramId, u32>,
    
    /// Map from engram ID to last access time for quick lookup
    last_accessed_map: HashMap<EngramId, chrono::DateTime<chrono::Utc>>,
    
    /// Map from engram ID to TTL information (expiration timestamp)
    ttl_map: HashMap<EngramId, Option<u64>>,
}
```

### Key Operations

```rust
impl ImportanceIndex {
    /// Record an access to an engram
    pub fn record_access(&mut self, id: &EngramId) -> Result<()> {
        // Get current access count
        let old_count = self.access_count_map.get(id).cloned().unwrap_or(0);
        let new_count = old_count + 1;
        
        // Update access count buckets if needed
        let old_bucket = (old_count / 10) as u8;
        let new_bucket = (new_count / 10) as u8;
        
        if old_bucket != new_bucket {
            // Remove from old bucket
            if let Some(engrams) = self.access_buckets.get_mut(&old_bucket) {
                engrams.remove(id);
            }
            
            // Add to new bucket
            self.access_buckets
                .entry(new_bucket)
                .or_insert_with(HashSet::new)
                .insert(id.clone());
        }
        
        // Update access count map
        self.access_count_map.insert(id.clone(), new_count);
        
        // Update last accessed time
        let now = chrono::Utc::now();
        self.last_accessed_map.insert(id.clone(), now);
        
        // Update recency sorted list
        self.update_recency_list(id.clone(), now);
        
        Ok(())
    }
    
    /// Update importance score for an engram
    pub fn update_importance(&mut self, id: &EngramId, importance: f64) -> Result<()> {
        // Ensure importance is within valid range
        let importance = importance.max(0.0).min(1.0);
        
        // Update importance map
        self.importance_map.insert(id.clone(), importance);
        
        // Update importance buckets
        let bucket = (importance * 10.0).floor() as u8;
        
        // First remove from any existing bucket
        for bucket_idx in 0..=10 {
            if let Some(engrams) = self.importance_buckets.get_mut(&bucket_idx) {
                engrams.remove(id);
            }
        }
        
        // Add to the correct bucket
        self.importance_buckets
            .entry(bucket)
            .or_insert_with(HashSet::new)
            .insert(id.clone());
        
        // Update importance_sorted list
        self.update_importance_sorted(id.clone(), importance);
        
        Ok(())
    }
    
    /// Get engrams by minimum importance
    pub fn find_by_min_importance(&self, min_importance: f64) -> HashSet<EngramId> {
        let min_bucket = (min_importance * 10.0).floor() as u8;
        let mut result = HashSet::new();
        
        for bucket in min_bucket..=10 {
            if let Some(engrams) = self.importance_buckets.get(&bucket) {
                result.extend(engrams.iter().cloned());
            }
        }
        
        result
    }
    
    /// Get expired engrams based on TTL
    pub fn get_expired_engrams(&self) -> HashSet<EngramId> {
        let now = chrono::Utc::now();
        let mut result = HashSet::new();
        
        for (id, ttl_opt) in &self.ttl_map {
            if let Some(ttl) = ttl_opt {
                if let Some(last_accessed) = self.last_accessed_map.get(id) {
                    let elapsed = now.signed_duration_since(*last_accessed).num_seconds() as u64;
                    if elapsed > *ttl {
                        result.insert(id.clone());
                    }
                }
            }
        }
        
        result
    }
    
    /// Get forgetting candidates based on importance, access count, and recency
    pub fn get_forgetting_candidates(
        &self,
        max_importance: f64,
        max_access_count: u32,
        older_than: &chrono::DateTime<chrono::Utc>,
        limit: usize
    ) -> Vec<EngramId> {
        // Find engrams that match all criteria
        let low_importance = self.importance_sorted.iter()
            .rev() // Start from least important
            .filter(|(_, imp)| *imp <= max_importance)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        let low_access_count = self.access_count_map.iter()
            .filter(|(_, count)| **count <= max_access_count)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        let old_access = self.last_accessed_map.iter()
            .filter(|(_, time)| *time < older_than)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        // Find intersection of all three sets
        let mut candidates = low_importance.intersection(&low_access_count).cloned().collect::<HashSet<_>>();
        candidates = candidates.intersection(&old_access).cloned().collect();
        
        // Sort candidates by importance (lowest first)
        let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
        candidates_vec.truncate(limit); // Limit the number of candidates
        
        candidates_vec
    }
}
```

## ForgettingPolicy

The `ForgettingPolicy` enum defines different strategies for memory pruning:

```rust
pub enum ForgettingPolicy {
    /// Forget engrams based on age
    AgeBased {
        /// Maximum age in seconds before considering for forgetting
        max_age_seconds: u64,
        /// Maximum number of engrams to forget in one operation
        max_items: usize,
    },
    
    /// Forget engrams based on importance threshold
    ImportanceThreshold {
        /// Maximum importance score for forgetting candidates (0.0-1.0)
        max_importance: f64,
        /// Maximum number of engrams to forget in one operation
        max_items: usize,
    },
    
    /// Forget engrams based on access frequency
    AccessFrequency {
        /// Maximum access count for forgetting candidates
        max_access_count: u32,
        /// Minimum time since last access (in seconds)
        min_idle_seconds: u64,
        /// Maximum number of engrams to forget in one operation
        max_items: usize,
    },
    
    /// Hybrid policy combining importance, access frequency, and age
    Hybrid {
        /// Maximum importance score for forgetting candidates (0.0-1.0)
        max_importance: f64,
        /// Maximum access count for forgetting candidates
        max_access_count: u32,
        /// Minimum time since last access (in seconds)
        min_idle_seconds: u64,
        /// Maximum number of engrams to forget in one operation
        max_items: usize,
    },
    
    /// Time-to-live (TTL) based expiration
    TTLExpiration {
        /// Maximum number of engrams to forget in one operation
        max_items: usize,
    },
}
```

## Future Index Enhancements

Future enhancements to the indexing system may include:

1. **Vector Indexes**: HNSW or other ANN indexes for embedding search (in progress)
2. **Full-Text Indexes**: BM25 or similar text search capabilities
3. **Persistent Indexes**: Store indexes in RocksDB for persistence
4. **Incremental Updates**: Optimize index updates for large-scale changes
5. **Distributed Indexes**: Support for partitioned/sharded indexes