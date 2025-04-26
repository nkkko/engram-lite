use crate::error::Result;
use crate::schema::{EngramId, ConnectionId, Collection, Connection, Engram};
use std::collections::{HashMap, HashSet};

/// Efficient indexes for fast relationship traversal
#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
#[allow(dead_code)]
pub struct TextIndex {
    /// Maps normalized keywords to engram IDs
    keyword_index: HashMap<String, HashSet<EngramId>>,
    
    /// Maps stemmed words to engram IDs (for more flexible matching)
    stem_index: HashMap<String, HashSet<EngramId>>,
    
    /// Maps engram IDs to the set of keywords it contains
    engram_keywords: HashMap<EngramId, HashSet<String>>,
}

#[allow(dead_code)]
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

/// Index for tracking engrams by time periods
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

#[allow(dead_code)]
impl TemporalIndex {
    /// Create a new, empty temporal index
    pub fn new() -> Self {
        Self {
            year_index: HashMap::new(),
            month_index: HashMap::new(),
            day_index: HashMap::new(),
            hour_index: HashMap::new(),
            recency_list: Vec::new(),
            timestamp_map: HashMap::new(),
        }
    }
    
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
        
        // Index by year-month (YYYYMM format)
        let year_month = year * 100 + month;
        self.month_index
            .entry(year_month)
            .or_insert_with(HashSet::new)
            .insert(engram.id.clone());
        
        // Index by year-month-day (YYYYMMDD format)
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
        
        // Store timestamp for quick access
        self.timestamp_map.insert(engram.id.clone(), timestamp);
        
        // Find position to insert in recency list (binary search)
        match self.recency_list.binary_search_by(|id| {
            self.timestamp_map
                .get(id)
                .unwrap()
                .cmp(&timestamp)
                .reverse() // Reverse for most recent first
        }) {
            Ok(pos) => self.recency_list.insert(pos, engram.id.clone()),
            Err(pos) => self.recency_list.insert(pos, engram.id.clone()),
        }
        
        Ok(())
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&mut self, engram: &Engram) -> Result<()> {
        let id = &engram.id;
        
        // Remove from timestamp map
        if let Some(timestamp) = self.timestamp_map.remove(id) {
            // Extract time components
            let year = timestamp.year();
            let month = timestamp.month() as i32;
            let day = timestamp.day() as i32;
            let hour = timestamp.hour() as u8;
            
            // Remove from year index
            if let Some(engrams) = self.year_index.get_mut(&year) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.year_index.remove(&year);
                }
            }
            
            // Remove from month index
            let year_month = year * 100 + month;
            if let Some(engrams) = self.month_index.get_mut(&year_month) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.month_index.remove(&year_month);
                }
            }
            
            // Remove from day index
            let year_month_day = year_month * 100 + day;
            if let Some(engrams) = self.day_index.get_mut(&year_month_day) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.day_index.remove(&year_month_day);
                }
            }
            
            // Remove from hour index
            if let Some(engrams) = self.hour_index.get_mut(&hour) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.hour_index.remove(&hour);
                }
            }
            
            // Remove from recency list
            if let Some(pos) = self.recency_list.iter().position(|x| x == id) {
                self.recency_list.remove(pos);
            }
        }
        
        Ok(())
    }
    
    /// Find engrams created in a specific year
    pub fn find_by_year(&self, year: i32) -> HashSet<EngramId> {
        self.year_index
            .get(&year)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams created in a specific month (year and month)
    pub fn find_by_month(&self, year: i32, month: u32) -> HashSet<EngramId> {
        let year_month = year * 100 + month as i32;
        self.month_index
            .get(&year_month)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams created on a specific day (year, month, and day)
    pub fn find_by_day(&self, year: i32, month: u32, day: u32) -> HashSet<EngramId> {
        let year_month = year * 100 + month as i32;
        let year_month_day = year_month * 100 + day as i32;
        self.day_index
            .get(&year_month_day)
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams created during a specific hour of the day (0-23)
    pub fn find_by_hour(&self, hour: u32) -> HashSet<EngramId> {
        if hour > 23 {
            return HashSet::new();
        }
        
        self.hour_index
            .get(&(hour as u8))
            .cloned()
            .unwrap_or_else(HashSet::new)
    }
    
    /// Find engrams created before a specific timestamp
    pub fn find_before(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        let mut result = HashSet::new();
        
        for (id, ts) in &self.timestamp_map {
            if ts < timestamp {
                result.insert(id.clone());
            }
        }
        
        result
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
    
    /// Find engrams created between two timestamps
    pub fn find_between(
        &self, 
        start: &chrono::DateTime<chrono::Utc>, 
        end: &chrono::DateTime<chrono::Utc>
    ) -> HashSet<EngramId> {
        let mut result = HashSet::new();
        
        for (id, ts) in &self.timestamp_map {
            if ts >= start && ts <= end {
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

/// Index for tracking engram importance and managing forgetting
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

#[allow(dead_code)]
impl ImportanceIndex {
    /// Create a new, empty importance index
    pub fn new() -> Self {
        Self {
            importance_buckets: HashMap::new(),
            importance_sorted: Vec::new(),
            access_buckets: HashMap::new(),
            recency_sorted: Vec::new(),
            importance_map: HashMap::new(),
            access_count_map: HashMap::new(),
            last_accessed_map: HashMap::new(),
            ttl_map: HashMap::new(),
        }
    }
    
    /// Add an engram to the index
    pub fn add_engram(&mut self, engram: &Engram) -> Result<()> {
        let id = &engram.id;
        
        // Add to importance buckets
        let importance_bucket = (engram.importance * 10.0).floor() as u8;
        self.importance_buckets
            .entry(importance_bucket)
            .or_insert_with(HashSet::new)
            .insert(id.clone());
        
        // Add to importance sorted list
        match self.importance_sorted.binary_search_by(|(_, imp)| {
            imp.partial_cmp(&engram.importance).unwrap().reverse() // Reverse for most important first
        }) {
            Ok(pos) => self.importance_sorted.insert(pos, (id.clone(), engram.importance)),
            Err(pos) => self.importance_sorted.insert(pos, (id.clone(), engram.importance)),
        }
        
        // Add to access buckets
        let access_bucket = if engram.access_count > 100 {
            10 // Cap at bucket 10 for very frequent access
        } else {
            (engram.access_count / 10) as u8 // 0-9 buckets for 0-99 accesses
        };
        self.access_buckets
            .entry(access_bucket)
            .or_insert_with(HashSet::new)
            .insert(id.clone());
        
        // Add to recency sorted list
        match self.recency_sorted.binary_search_by(|(_, time)| {
            time.cmp(&engram.last_accessed).reverse() // Reverse for most recent first
        }) {
            Ok(pos) => self.recency_sorted.insert(pos, (id.clone(), engram.last_accessed)),
            Err(pos) => self.recency_sorted.insert(pos, (id.clone(), engram.last_accessed)),
        }
        
        // Update maps for quick lookup
        self.importance_map.insert(id.clone(), engram.importance);
        self.access_count_map.insert(id.clone(), engram.access_count);
        self.last_accessed_map.insert(id.clone(), engram.last_accessed);
        self.ttl_map.insert(id.clone(), engram.ttl);
        
        Ok(())
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&mut self, engram: &Engram) -> Result<()> {
        let id = &engram.id;
        
        // Remove from importance map and buckets
        if let Some(importance) = self.importance_map.remove(id) {
            let bucket = (importance * 10.0).floor() as u8;
            if let Some(engrams) = self.importance_buckets.get_mut(&bucket) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.importance_buckets.remove(&bucket);
                }
            }
        }
        
        // Remove from access count map and buckets
        if let Some(access_count) = self.access_count_map.remove(id) {
            let bucket = if access_count > 100 {
                10 // Cap at bucket 10 for very frequent access
            } else {
                (access_count / 10) as u8 // 0-9 buckets for 0-99 accesses
            };
            if let Some(engrams) = self.access_buckets.get_mut(&bucket) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.access_buckets.remove(&bucket);
                }
            }
        }
        
        // Remove from last accessed map
        self.last_accessed_map.remove(id);
        
        // Remove from TTL map
        self.ttl_map.remove(id);
        
        // Remove from sorted lists
        if let Some(pos) = self.importance_sorted.iter().position(|(i, _)| i == id) {
            self.importance_sorted.remove(pos);
        }
        
        if let Some(pos) = self.recency_sorted.iter().position(|(i, _)| i == id) {
            self.recency_sorted.remove(pos);
        }
        
        Ok(())
    }
    
    /// Update an engram's importance score
    pub fn update_importance(&mut self, id: &EngramId, new_importance: f64) -> Result<()> {
        // Ensure importance is within valid range
        let new_importance = new_importance.max(0.0).min(1.0);
        
        // Get old importance bucket
        let old_importance = self.importance_map.get(id).cloned().unwrap_or(0.5);
        let old_bucket = (old_importance * 10.0).floor() as u8;
        
        // Get new importance bucket
        let new_bucket = (new_importance * 10.0).floor() as u8;
        
        // Update importance buckets if the bucket has changed
        if old_bucket != new_bucket {
            // Remove from old bucket
            if let Some(engrams) = self.importance_buckets.get_mut(&old_bucket) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.importance_buckets.remove(&old_bucket);
                }
            }
            
            // Add to new bucket
            self.importance_buckets
                .entry(new_bucket)
                .or_insert_with(HashSet::new)
                .insert(id.clone());
        }
        
        // Update importance map
        self.importance_map.insert(id.clone(), new_importance);
        
        // Update importance sorted list
        if let Some(pos) = self.importance_sorted.iter().position(|(i, _)| i == id) {
            self.importance_sorted.remove(pos);
            
            // Insert at new position
            match self.importance_sorted.binary_search_by(|(_, imp)| {
                imp.partial_cmp(&new_importance).unwrap().reverse() // Reverse for most important first
            }) {
                Ok(pos) => self.importance_sorted.insert(pos, (id.clone(), new_importance)),
                Err(pos) => self.importance_sorted.insert(pos, (id.clone(), new_importance)),
            }
        }
        
        Ok(())
    }
    
    /// Record an access to an engram
    pub fn record_access(&mut self, id: &EngramId) -> Result<()> {
        // Get current access count
        let old_count = self.access_count_map.get(id).cloned().unwrap_or(0);
        let new_count = old_count + 1;
        
        // Get old and new access buckets
        let old_bucket = if old_count > 100 {
            10
        } else {
            (old_count / 10) as u8
        };
        
        let new_bucket = if new_count > 100 {
            10
        } else {
            (new_count / 10) as u8
        };
        
        // Update access buckets if the bucket has changed
        if old_bucket != new_bucket {
            // Remove from old bucket
            if let Some(engrams) = self.access_buckets.get_mut(&old_bucket) {
                engrams.remove(id);
                if engrams.is_empty() {
                    self.access_buckets.remove(&old_bucket);
                }
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
        if let Some(pos) = self.recency_sorted.iter().position(|(i, _)| i == id) {
            self.recency_sorted.remove(pos);
            
            // Insert at the beginning (most recent)
            self.recency_sorted.insert(0, (id.clone(), now));
        }
        
        Ok(())
    }
    
    /// Set or update TTL for an engram
    pub fn set_ttl(&mut self, id: &EngramId, ttl: Option<u64>) -> Result<()> {
        self.ttl_map.insert(id.clone(), ttl);
        Ok(())
    }
    
    /// Get engrams by minimum importance
    pub fn find_by_min_importance(&self, min_importance: f64) -> HashSet<EngramId> {
        let min_bucket = (min_importance * 10.0).floor() as u8;
        let mut result = HashSet::new();
        
        // Combine all buckets at or above the minimum
        for bucket in min_bucket..=10 {
            if let Some(engrams) = self.importance_buckets.get(&bucket) {
                result.extend(engrams.iter().cloned());
            }
        }
        
        result
    }
    
    /// Get engrams by minimum access count
    pub fn find_by_min_access_count(&self, min_count: u32) -> HashSet<EngramId> {
        let mut result = HashSet::new();
        
        // Calculate minimum bucket
        let min_bucket = if min_count > 100 {
            10
        } else {
            (min_count / 10) as u8
        };
        
        // Combine all buckets at or above the minimum
        for bucket in min_bucket..=10 {
            if let Some(engrams) = self.access_buckets.get(&bucket) {
                result.extend(engrams.iter().cloned());
            }
        }
        
        result
    }
    
    /// Get engrams by access recency
    pub fn find_by_last_accessed_after(&self, time: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        let mut result = HashSet::new();
        
        for (id, last_accessed) in &self.recency_sorted {
            if last_accessed >= time {
                result.insert(id.clone());
            } else {
                // Since the list is sorted by recency, we can break early
                break;
            }
        }
        
        result
    }
    
    /// Get engrams sorted by importance (most important first)
    pub fn get_most_important(&self, count: usize) -> Vec<EngramId> {
        self.importance_sorted.iter().take(count).map(|(id, _)| id.clone()).collect()
    }
    
    /// Get expired engrams based on TTL
    pub fn get_expired_engrams(&self) -> HashSet<EngramId> {
        let now = chrono::Utc::now();
        let mut result = HashSet::new();
        
        // Check each engram with a TTL
        for (id, ttl_opt) in &self.ttl_map {
            if let Some(ttl) = ttl_opt {
                // Calculate creation time from the timestamp map
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
    
    /// Calculate forgetting candidates based on importance, access count, and recency
    pub fn get_forgetting_candidates(
        &self,
        max_importance: f64,
        max_access_count: u32,
        older_than: &chrono::DateTime<chrono::Utc>,
        limit: usize
    ) -> Vec<EngramId> {
        let mut candidates = HashSet::new();
        
        // Get engrams with low importance
        let low_importance = self.importance_sorted.iter()
            .rev() // Start from least important
            .filter(|(_, imp)| *imp <= max_importance)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        // Get engrams with low access count
        let low_access_count = self.access_count_map.iter()
            .filter(|(_, count)| **count <= max_access_count)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        // Get engrams that haven't been accessed recently
        let old_access = self.last_accessed_map.iter()
            .filter(|(_, time)| *time < older_than)
            .map(|(id, _)| id.clone())
            .collect::<HashSet<_>>();
        
        // Find intersection of all three sets
        candidates = low_importance.intersection(&low_access_count).cloned().collect();
        candidates = candidates.intersection(&old_access).cloned().collect();
        
        // Sort candidates by importance (least important first)
        let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
        candidates_vec.sort_by(|a, b| {
            let a_imp = self.importance_map.get(a).unwrap_or(&0.5);
            let b_imp = self.importance_map.get(b).unwrap_or(&0.5);
            a_imp.partial_cmp(b_imp).unwrap()
        });
        
        // Limit the number of candidates
        candidates_vec.truncate(limit);
        
        candidates_vec
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
    
    /// Temporal index for time-based operations
    pub temporal_index: TemporalIndex,
    
    /// Importance index for importance scoring and forgetting
    pub importance_index: ImportanceIndex,
    
    /// Source index for filtering by source
    source_index: HashMap<String, HashSet<EngramId>>,
    
    /// Confidence index for filtering by confidence ranges
    confidence_index: HashMap<u8, HashSet<EngramId>>, // Bucketed by confidence * 10
}

#[allow(dead_code)]
impl SearchIndex {
    /// Create a new, empty search index
    pub fn new() -> Self {
        Self {
            relationship_index: RelationshipIndex::new(),
            metadata_index: MetadataIndex::new(),
            text_index: TextIndex::new(),
            temporal_index: TemporalIndex::new(),
            importance_index: ImportanceIndex::new(),
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
        
        // Index by temporal properties
        self.temporal_index.add_engram(engram)?;
        
        // Index by importance, access count, and TTL
        self.importance_index.add_engram(engram)?;
        
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
        
        // Remove from temporal index
        self.temporal_index.remove_engram(engram)?;
        
        // Remove from importance index
        self.importance_index.remove_engram(engram)?;
        
        // Remove from source index
        if let Some(engrams) = self.source_index.get_mut(&engram.source) {
            engrams.remove(&engram.id);
            if engrams.is_empty() {
                self.source_index.remove(&engram.source);
            }
        }
        
        // Remove from confidence index
        let confidence_bucket = (engram.confidence * 10.0).floor() as u8;
        if let Some(engrams) = self.confidence_index.get_mut(&confidence_bucket) {
            engrams.remove(&engram.id);
            if engrams.is_empty() {
                self.confidence_index.remove(&confidence_bucket);
            }
        }
        
        Ok(())
    }
    
    /// Record an access to an engram
    pub fn record_access(&mut self, id: &EngramId) -> Result<()> {
        self.importance_index.record_access(id)
    }
    
    /// Update an engram's importance score
    pub fn update_importance(&mut self, id: &EngramId, importance: f64) -> Result<()> {
        self.importance_index.update_importance(id, importance)
    }
    
    /// Set or update TTL for an engram
    pub fn set_ttl(&mut self, id: &EngramId, ttl: Option<u64>) -> Result<()> {
        self.importance_index.set_ttl(id, ttl)
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
    
    /// Find engrams by minimum importance score
    pub fn find_by_min_importance(&self, min_importance: f64) -> HashSet<EngramId> {
        self.importance_index.find_by_min_importance(min_importance)
    }
    
    /// Find engrams by minimum access count
    pub fn find_by_min_access_count(&self, min_count: u32) -> HashSet<EngramId> {
        self.importance_index.find_by_min_access_count(min_count)
    }
    
    /// Find engrams accessed after a specific time
    pub fn find_by_last_accessed_after(&self, time: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        self.importance_index.find_by_last_accessed_after(time)
    }
    
    /// Get the most important engrams
    pub fn get_most_important(&self, count: usize) -> Vec<EngramId> {
        self.importance_index.get_most_important(count)
    }
    
    /// Get expired engrams based on TTL
    pub fn get_expired_engrams(&self) -> HashSet<EngramId> {
        self.importance_index.get_expired_engrams()
    }
    
    /// Get forgetting candidates based on importance, access frequency, and recency
    pub fn get_forgetting_candidates(
        &self,
        max_importance: f64,
        max_access_count: u32,
        older_than: &chrono::DateTime<chrono::Utc>,
        limit: usize
    ) -> Vec<EngramId> {
        self.importance_index.get_forgetting_candidates(
            max_importance,
            max_access_count,
            older_than,
            limit
        )
    }
    
    /// Find engrams created before a specific timestamp
    pub fn find_by_before_timestamp(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        self.temporal_index.find_before(timestamp)
    }
    
    /// Find engrams created after a specific timestamp
    pub fn find_by_after_timestamp(&self, timestamp: &chrono::DateTime<chrono::Utc>) -> HashSet<EngramId> {
        self.temporal_index.find_after(timestamp)
    }
    
    /// Find engrams created between two timestamps
    pub fn find_by_timestamp_range(
        &self,
        start: &chrono::DateTime<chrono::Utc>,
        end: &chrono::DateTime<chrono::Utc>,
    ) -> HashSet<EngramId> {
        self.temporal_index.find_between(start, end)
    }
    
    /// Find engrams created in a specific year
    pub fn find_by_year(&self, year: i32) -> HashSet<EngramId> {
        self.temporal_index.find_by_year(year)
    }
    
    /// Find engrams created in a specific month
    pub fn find_by_month(&self, year: i32, month: u32) -> HashSet<EngramId> {
        self.temporal_index.find_by_month(year, month)
    }
    
    /// Find engrams created on a specific day
    pub fn find_by_day(&self, year: i32, month: u32, day: u32) -> HashSet<EngramId> {
        self.temporal_index.find_by_day(year, month, day)
    }
    
    /// Find engrams created during a specific hour
    pub fn find_by_hour(&self, hour: u32) -> HashSet<EngramId> {
        self.temporal_index.find_by_hour(hour)
    }
    
    /// Get most recent engrams
    pub fn get_most_recent(&self, count: usize) -> Vec<EngramId> {
        self.temporal_index.get_most_recent(count)
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
        before_time: Option<&chrono::DateTime<chrono::Utc>>,
        after_time: Option<&chrono::DateTime<chrono::Utc>>,
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
        
        // Apply before time filter if provided
        if let Some(time) = before_time {
            let time_results = self.find_by_before_timestamp(time);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&time_results).cloned().collect(),
                None => time_results,
            });
        }
        
        // Apply after time filter if provided
        if let Some(time) = after_time {
            let time_results = self.find_by_after_timestamp(time);
            final_result = Some(match final_result {
                Some(existing) => existing.intersection(&time_results).cloned().collect(),
                None => time_results,
            });
        }
        
        final_result.unwrap_or_else(HashSet::new)
    }
    
    /// Original search_combined method for backward compatibility
    pub fn search_combined_legacy(
        &self,
        text_query: Option<&str>,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_key: Option<&str>,
        metadata_value: Option<&str>,
        exact_match: bool,
    ) -> HashSet<EngramId> {
        self.search_combined(
            text_query,
            source,
            min_confidence,
            metadata_key,
            metadata_value,
            exact_match,
            None,
            None,
        )
    }
}

/// Forgetting policy for memory pruning
#[derive(Debug, Clone)]
pub enum ForgettingPolicy {
    /// Forget engrams based on age
    AgeBased {
        /// Maximum age in seconds before considering forgetting
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

impl ForgettingPolicy {
    /// Execute the forgetting policy on the given index and return engrams to forget
    pub fn get_forgetting_candidates(&self, index: &SearchIndex) -> Vec<EngramId> {
        match self {
            Self::AgeBased { max_age_seconds, max_items } => {
                // Calculate timestamp threshold
                let threshold = chrono::Utc::now() - chrono::Duration::seconds(*max_age_seconds as i64);
                
                // Get engrams created before threshold
                let candidates = index.find_by_before_timestamp(&threshold);
                
                // Sort by age (oldest first) and limit
                let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
                candidates_vec.truncate(*max_items);
                candidates_vec
            },
            
            Self::ImportanceThreshold { max_importance, max_items } => {
                // Get engrams with importance below threshold
                let candidates = index.find_by_min_importance(*max_importance);
                
                // Limit number of candidates
                let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
                candidates_vec.truncate(*max_items);
                candidates_vec
            },
            
            Self::AccessFrequency { max_access_count, min_idle_seconds, max_items } => {
                // Calculate access time threshold
                let threshold = chrono::Utc::now() - chrono::Duration::seconds(*min_idle_seconds as i64);
                
                // Get engrams with low access count and not accessed recently
                let infrequent = index.find_by_min_access_count(*max_access_count);
                let old_access = index.find_by_last_accessed_after(&threshold);
                
                // Find intersection
                let candidates: HashSet<_> = infrequent.difference(&old_access).cloned().collect();
                
                // Limit number of candidates
                let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
                candidates_vec.truncate(*max_items);
                candidates_vec
            },
            
            Self::Hybrid { max_importance, max_access_count, min_idle_seconds, max_items } => {
                // Calculate access time threshold
                let threshold = chrono::Utc::now() - chrono::Duration::seconds(*min_idle_seconds as i64);
                
                // Get forgetting candidates using the combined criteria
                index.get_forgetting_candidates(*max_importance, *max_access_count, &threshold, *max_items)
            },
            
            Self::TTLExpiration { max_items } => {
                // Get expired engrams based on TTL
                let candidates = index.get_expired_engrams();
                
                // Limit number of candidates
                let mut candidates_vec: Vec<_> = candidates.into_iter().collect();
                candidates_vec.truncate(*max_items);
                candidates_vec
            },
        }
    }
}

/// In-memory index for a collection of engrams
#[allow(dead_code)]
pub struct CollectionIndex {
    /// The collection being indexed
    collection_id: String,
    
    /// Set of engram IDs in this collection
    engram_ids: HashSet<EngramId>,
}

#[allow(dead_code)]
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