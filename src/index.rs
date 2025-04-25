use crate::error::Result;
use crate::schema::{EngramId, ConnectionId, Collection, Connection, Engram};
use std::collections::{HashMap, HashSet};

/// Efficient indexes for fast relationship traversal
pub struct RelationshipIndex {
    /// Index from source engram ID to outgoing connections
    outgoing_connections: HashMap<EngramId, HashSet<ConnectionId>>,
    
    /// Index from target engram ID to incoming connections
    incoming_connections: HashMap<EngramId, HashSet<ConnectionId>>,
    
    /// Index from relationship type to connections
    relationship_type_index: HashMap<String, HashSet<ConnectionId>>,
    
    /// Index from source to targets (shortcut for faster traversal)
    source_to_targets: HashMap<EngramId, HashSet<EngramId>>,
    
    /// Index from target to sources (shortcut for faster traversal)
    target_to_sources: HashMap<EngramId, HashSet<EngramId>>,
}

impl RelationshipIndex {
    /// Create a new, empty relationship index
    pub fn new() -> Self {
        Self {
            outgoing_connections: HashMap::new(),
            incoming_connections: HashMap::new(),
            relationship_type_index: HashMap::new(),
            source_to_targets: HashMap::new(),
            target_to_sources: HashMap::new(),
        }
    }
    
    /// Add a connection to the index
    pub fn add_connection(&mut self, connection: &Connection) -> Result<()> {
        // Index by source (outgoing)
        self.outgoing_connections
            .entry(connection.source_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Index by target (incoming)
        self.incoming_connections
            .entry(connection.target_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Index by relationship type
        self.relationship_type_index
            .entry(connection.relationship_type.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.id.clone());
        
        // Direct source to target mapping
        self.source_to_targets
            .entry(connection.source_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.target_id.clone());
        
        // Direct target to source mapping
        self.target_to_sources
            .entry(connection.target_id.clone())
            .or_insert_with(HashSet::new)
            .insert(connection.source_id.clone());
        
        Ok(())
    }
    
    /// Remove a connection from the index
    pub fn remove_connection(&mut self, connection: &Connection) -> Result<()> {
        // Remove from source index
        if let Some(connections) = self.outgoing_connections.get_mut(&connection.source_id) {
            connections.remove(&connection.id);
            if connections.is_empty() {
                self.outgoing_connections.remove(&connection.source_id);
            }
        }
        
        // Remove from target index
        if let Some(connections) = self.incoming_connections.get_mut(&connection.target_id) {
            connections.remove(&connection.id);
            if connections.is_empty() {
                self.incoming_connections.remove(&connection.target_id);
            }
        }
        
        // Remove from relationship type index
        if let Some(connections) = self.relationship_type_index.get_mut(&connection.relationship_type) {
            connections.remove(&connection.id);
            if connections.is_empty() {
                self.relationship_type_index.remove(&connection.relationship_type);
            }
        }
        
        // Remove from source to targets mapping
        if let Some(targets) = self.source_to_targets.get_mut(&connection.source_id) {
            targets.remove(&connection.target_id);
            if targets.is_empty() {
                self.source_to_targets.remove(&connection.source_id);
            }
        }
        
        // Remove from target to sources mapping
        if let Some(sources) = self.target_to_sources.get_mut(&connection.target_id) {
            sources.remove(&connection.source_id);
            if sources.is_empty() {
                self.target_to_sources.remove(&connection.target_id);
            }
        }
        
        Ok(())
    }
    
    /// Get all outgoing connections from a source engram
    pub fn get_outgoing_connections(&self, source_id: &EngramId) -> HashSet<ConnectionId> {
        self.outgoing_connections
            .get(source_id)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Get all incoming connections to a target engram
    pub fn get_incoming_connections(&self, target_id: &EngramId) -> HashSet<ConnectionId> {
        self.incoming_connections
            .get(target_id)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Get all connections of a specific relationship type
    pub fn get_connections_by_type(&self, relationship_type: &str) -> HashSet<ConnectionId> {
        self.relationship_type_index
            .get(relationship_type)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Get all target engrams connected from a source
    pub fn get_targets(&self, source_id: &EngramId) -> HashSet<EngramId> {
        self.source_to_targets
            .get(source_id)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Get all source engrams connected to a target
    pub fn get_sources(&self, target_id: &EngramId) -> HashSet<EngramId> {
        self.target_to_sources
            .get(target_id)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find connections that match both source and relationship type
    pub fn find_by_source_and_type(
        &self,
        source_id: &EngramId,
        relationship_type: &str,
    ) -> HashSet<ConnectionId> {
        let by_source = self.get_outgoing_connections(source_id);
        let by_type = self.get_connections_by_type(relationship_type);
        
        // Intersection of the two sets
        by_source.intersection(&by_type).cloned().collect()
    }
    
    /// Find connections that match both target and relationship type
    pub fn find_by_target_and_type(
        &self,
        target_id: &EngramId,
        relationship_type: &str,
    ) -> HashSet<ConnectionId> {
        let by_target = self.get_incoming_connections(target_id);
        let by_type = self.get_connections_by_type(relationship_type);
        
        // Intersection of the two sets
        by_target.intersection(&by_type).cloned().collect()
    }
    
    /// Find all paths between source and target with a maximum depth
    pub fn find_paths(
        &self,
        source_id: &EngramId,
        target_id: &EngramId,
        max_depth: usize,
    ) -> Vec<Vec<EngramId>> {
        let mut paths = Vec::new();
        let mut current_path = vec![source_id.clone()];
        let visited = HashSet::new();
        
        self.dfs_paths(
            source_id,
            target_id,
            max_depth,
            &mut current_path,
            &mut paths,
            &visited,
        );
        
        paths
    }
    
    // Helper method for depth-first search to find paths
    fn dfs_paths(
        &self,
        current: &EngramId,
        target: &EngramId,
        depth_left: usize,
        current_path: &mut Vec<EngramId>,
        all_paths: &mut Vec<Vec<EngramId>>,
        visited: &HashSet<EngramId>,
    ) {
        // Base case: we've reached the target
        if current == target {
            all_paths.push(current_path.clone());
            return;
        }
        
        // Base case: we've reached max depth
        if depth_left == 0 {
            return;
        }
        
        // Get all targets from current node
        if let Some(targets) = self.source_to_targets.get(current) {
            for next in targets {
                // Avoid cycles in the path
                if !current_path.contains(next) {
                    // Add to current path
                    current_path.push(next.clone());
                    
                    // Recursive call
                    self.dfs_paths(
                        next,
                        target,
                        depth_left - 1,
                        current_path,
                        all_paths,
                        visited,
                    );
                    
                    // Backtrack
                    current_path.pop();
                }
            }
        }
    }
}

/// Index for tracking engrams by metadata fields
pub struct MetadataIndex {
    /// Index from metadata key to engrams that have that key
    key_index: HashMap<String, HashSet<EngramId>>,
    
    /// Index from metadata key-value pairs to engrams
    key_value_index: HashMap<(String, String), HashSet<EngramId>>,
}

impl MetadataIndex {
    /// Create a new, empty metadata index
    pub fn new() -> Self {
        Self {
            key_index: HashMap::new(),
            key_value_index: HashMap::new(),
        }
    }
    
    /// Add an engram to the index
    pub fn add_engram(&mut self, engram: &Engram) -> Result<()> {
        for (key, value) in &engram.metadata {
            // Index by key
            self.key_index
                .entry(key.clone())
                .or_insert_with(HashSet::new)
                .insert(engram.id.clone());
            
            // Index by key-value pair (only for string values)
            if let serde_json::Value::String(str_value) = value {
                self.key_value_index
                    .entry((key.clone(), str_value.clone()))
                    .or_insert_with(HashSet::new)
                    .insert(engram.id.clone());
            }
        }
        
        Ok(())
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&mut self, engram: &Engram) -> Result<()> {
        for (key, value) in &engram.metadata {
            // Remove from key index
            if let Some(engrams) = self.key_index.get_mut(key) {
                engrams.remove(&engram.id);
                if engrams.is_empty() {
                    self.key_index.remove(key);
                }
            }
            
            // Remove from key-value index
            if let serde_json::Value::String(str_value) = value {
                let entry = (key.clone(), str_value.clone());
                if let Some(engrams) = self.key_value_index.get_mut(&entry) {
                    engrams.remove(&engram.id);
                    if engrams.is_empty() {
                        self.key_value_index.remove(&entry);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Find engrams with a specific metadata key
    pub fn find_by_key(&self, key: &str) -> HashSet<EngramId> {
        self.key_index
            .get(key)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams with a specific metadata key-value pair
    pub fn find_by_key_value(&self, key: &str, value: &str) -> HashSet<EngramId> {
        let entry = (key.to_string(), value.to_string());
        self.key_value_index
            .get(&entry)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
}

/// Text search index for basic keyword search
pub struct TextIndex {
    /// Maps normalized keywords to engram IDs
    keyword_index: HashMap<String, HashSet<EngramId>>,
    
    /// Maps stemmed words to engram IDs (for more flexible matching)
    stem_index: HashMap<String, HashSet<EngramId>>,
    
    /// Maps engram IDs to the set of keywords it contains
    engram_keywords: HashMap<EngramId, HashSet<String>>,
}

impl TextIndex {
    /// Create a new, empty text index
    pub fn new() -> Self {
        Self {
            keyword_index: HashMap::new(),
            stem_index: HashMap::new(),
            engram_keywords: HashMap::new(),
        }
    }
    
    /// Add an engram to the index
    pub fn add_engram(&mut self, engram: &Engram) -> Result<()> {
        let keywords = Self::extract_keywords(&engram.content);
        self.engram_keywords.insert(engram.id.clone(), keywords.clone());
        
        // Index each keyword
        for keyword in &keywords {
            self.keyword_index
                .entry(keyword.clone())
                .or_insert_with(HashSet::new)
                .insert(engram.id.clone());
            
            // Also index the stemmed version
            let stemmed = Self::stem_word(keyword);
            self.stem_index
                .entry(stemmed)
                .or_insert_with(HashSet::new)
                .insert(engram.id.clone());
        }
        
        Ok(())
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&mut self, engram: &Engram) -> Result<()> {
        if let Some(keywords) = self.engram_keywords.remove(&engram.id) {
            // Remove from keyword index
            for keyword in &keywords {
                if let Some(engrams) = self.keyword_index.get_mut(keyword) {
                    engrams.remove(&engram.id);
                    if engrams.is_empty() {
                        self.keyword_index.remove(keyword);
                    }
                }
                
                // Remove from stem index
                let stemmed = Self::stem_word(keyword);
                if let Some(engrams) = self.stem_index.get_mut(&stemmed) {
                    engrams.remove(&engram.id);
                    if engrams.is_empty() {
                        self.stem_index.remove(&stemmed);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Extract keywords from text content
    fn extract_keywords(text: &str) -> HashSet<String> {
        let mut keywords = HashSet::new();
        
        // Simple tokenization by splitting on whitespace and punctuation
        for word in text
            .split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|s| !s.is_empty())
        {
            // Convert to lowercase for case-insensitive matching
            let normalized = word.to_lowercase();
            if normalized.len() >= 3 {  // Only index words of at least 3 characters
                keywords.insert(normalized);
            }
        }
        
        keywords
    }
    
    /// Very basic stemming function
    /// In a real implementation, you'd want to use a proper stemming algorithm
    fn stem_word(word: &str) -> String {
        let word = word.to_lowercase();
        
        // Very simplified stemming - just handles a few common English suffixes
        if word.ends_with('s') && word.len() > 3 {
            return word[..word.len() - 1].to_string();
        } else if word.ends_with("ing") && word.len() > 5 {
            return word[..word.len() - 3].to_string();
        } else if word.ends_with("ed") && word.len() > 4 {
            return word[..word.len() - 2].to_string();
        }
        
        word
    }
    
    /// Find engrams containing a specific keyword (exact match)
    pub fn find_by_keyword(&self, keyword: &str) -> HashSet<EngramId> {
        let normalized = keyword.to_lowercase();
        self.keyword_index
            .get(&normalized)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams containing a stemmed version of the keyword (more flexible matching)
    pub fn find_by_stem(&self, keyword: &str) -> HashSet<EngramId> {
        let normalized = keyword.to_lowercase();
        let stemmed = Self::stem_word(&normalized);
        
        self.stem_index
            .get(&stemmed)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Search for engrams containing any of the keywords
    pub fn search(&self, query: &str) -> HashSet<EngramId> {
        let keywords = Self::extract_keywords(query);
        let mut results = HashSet::new();
        
        for keyword in keywords {
            // Get results for this keyword
            let keyword_results = self.find_by_keyword(&keyword);
            let stem_results = self.find_by_stem(&keyword);
            
            // Combine both sets
            let mut combined = keyword_results;
            combined.extend(stem_results);
            
            // Add to overall results
            results.extend(combined);
        }
        
        results
    }
    
    /// Search for engrams containing all of the keywords
    pub fn search_all(&self, query: &str) -> HashSet<EngramId> {
        let keywords = Self::extract_keywords(query);
        
        // Start with the entire universe of engrams
        let mut results: Option<HashSet<EngramId>> = None;
        
        for keyword in keywords {
            // Get results for this keyword (exact or stem matches)
            let keyword_results = self.find_by_keyword(&keyword);
            let stem_results = self.find_by_stem(&keyword);
            
            // Combine both sets
            let mut combined = keyword_results;
            combined.extend(stem_results);
            
            // Perform intersection with previous results
            results = match results {
                Some(prev) => Some(prev.intersection(&combined).cloned().collect()),
                None => Some(combined),
            };
            
            // Short-circuit if we have no results
            if let Some(ref res) = results {
                if res.is_empty() {
                    break;
                }
            }
        }
        
        results.unwrap_or_else(HashSet::new)
    }
}

/// Combined search index for efficient querying
pub struct SearchIndex {
    /// Relationship index for traversal
    pub relationship_index: RelationshipIndex,
    
    /// Metadata index for filtering
    pub metadata_index: MetadataIndex,

    /// Text index for keyword search
    pub text_index: TextIndex,
    
    /// Source index for filtering by source
    source_index: HashMap<String, HashSet<EngramId>>,
    
    /// Confidence index for filtering by confidence ranges
    confidence_index: HashMap<u8, HashSet<EngramId>>, // Bucketed by confidence * 10
}

impl SearchIndex {
    /// Create a new, empty search index
    pub fn new() -> Self {
        Self {
            relationship_index: RelationshipIndex::new(),
            metadata_index: MetadataIndex::new(),
            text_index: TextIndex::new(),
            source_index: HashMap::new(),
            confidence_index: HashMap::new(),
        }
    }
    
    /// Add an engram to the index
    pub fn add_engram(&mut self, engram: &Engram) -> Result<()> {
        // Index by metadata
        self.metadata_index.add_engram(engram)?;
        
        // Index by text content
        self.text_index.add_engram(engram)?;
        
        // Index by source
        self.source_index
            .entry(engram.source.clone())
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Index by confidence bucket
        let confidence_bucket = (engram.confidence * 10.0).floor() as u8;
        self.confidence_index
            .entry(confidence_bucket)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        Ok(())
    }
    
    /// Add a connection to the index
    pub fn add_connection(&mut self, connection: &Connection) -> Result<()> {
        self.relationship_index.add_connection(connection)
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&mut self, engram: &Engram) -> Result<()> {
        // Remove from metadata index
        self.metadata_index.remove_engram(engram)?;
        
        // Remove from text index
        self.text_index.remove_engram(engram)?;
        
        // Remove from source index
        if let Some(engrams) = self.source_index.get_mut(&engram.source) {
            engrams.remove(&engram.id);
            if engrams.is_empty() {
                self.source_index.remove(&engram.source);
            }
        }
        
        Ok(())
    }
    
    /// Remove an engram from the index by ID
    pub fn remove_engram_by_id(&mut self, engram_id: &str) -> Result<()> {
        // Since we only have the ID, we may not be able to fully remove from all indexes
        // This is a best-effort method that removes what it can
        
        // Remove from source index
        for (_, engrams) in self.source_index.iter_mut() {
            engrams.remove(engram_id);
        }
        
        // Clean up empty sets in source index
        self.source_index.retain(|_, engrams| !engrams.is_empty());
        
        // Remove from metadata and text indexes
        // We can't remove properly without the full engram, so this is a limitation
        
        // Note: In a real implementation, we would need to fetch the engram first
        // and then use remove_engram, but for now this is a partial implementation
        
        // Return success
        Ok(())
    }
    
    /// Remove a connection from the index
    pub fn remove_connection(&mut self, connection: &Connection) -> Result<()> {
        self.relationship_index.remove_connection(connection)
    }
    
    /// Find engrams by source
    pub fn find_by_source(&self, source: &str) -> HashSet<EngramId> {
        self.source_index
            .get(source)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams by minimum confidence
    pub fn find_by_min_confidence(&self, min_confidence: f64) -> HashSet<EngramId> {
        let min_bucket = (min_confidence * 10.0).floor() as u8;
        let mut result = HashSet::new();
        
        // Combine all buckets at or above the minimum
        for bucket in min_bucket..=10 {
            if let Some(engrams) = self.confidence_index.get(&bucket) {
                result.extend(engrams.iter().cloned());
            }
        }
        
        result
    }
    
    /// Combine multiple search criteria with AND logic
    pub fn search_combined(
        &self,
        text_query: Option<&str>,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_key: Option<&str>,
        metadata_value: Option<&str>,
        exact_match: bool,
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
        
        // Apply metadata key filter if provided
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
        
        final_result.unwrap_or_else(HashSet::new)
    }
}

/// In-memory index for a collection of engrams
pub struct CollectionIndex {
    /// The collection being indexed
    collection_id: String,
    
    /// Set of engram IDs in this collection
    engram_ids: HashSet<EngramId>,
}

impl CollectionIndex {
    /// Create a new index for a collection
    pub fn new(collection: &Collection) -> Self {
        Self {
            collection_id: collection.id.clone(),
            engram_ids: collection.engram_ids.clone(),
        }
    }
    
    /// Add an engram to the collection index
    pub fn add_engram(&mut self, engram_id: &EngramId) {
        self.engram_ids.insert(engram_id.clone());
    }
    
    /// Remove an engram from the collection index
    pub fn remove_engram(&mut self, engram_id: &EngramId) -> bool {
        self.engram_ids.remove(engram_id)
    }
    
    /// Check if an engram is in this collection
    pub fn contains(&self, engram_id: &EngramId) -> bool {
        self.engram_ids.contains(engram_id)
    }
    
    /// Get all engram IDs in this collection
    pub fn get_engram_ids(&self) -> &HashSet<EngramId> {
        &self.engram_ids
    }
    
    /// Get the collection ID
    pub fn get_collection_id(&self) -> &str {
        &self.collection_id
    }
}