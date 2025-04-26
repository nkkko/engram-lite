use crate::error::{EngramError, Result};
use crate::schema::{Engram, EngramId, Connection, ConnectionId};
use crate::storage::Storage;
use crate::index::SearchIndex;
use std::collections::HashSet;

/// Represents filter conditions for querying engrams
#[allow(dead_code)]
pub struct EngramQuery {
    /// Optional text search query
    pub text: Option<String>,
    
    /// Optional minimum confidence level
    pub min_confidence: Option<f64>,
    
    /// Optional source filter
    pub source: Option<String>,
    
    /// Optional metadata key filter
    pub metadata_key: Option<String>,
    
    /// Optional metadata value filter (only used if metadata_key is set)
    pub metadata_value: Option<String>,
    
    /// Whether text search should match all terms (true) or any term (false)
    pub exact_match: bool,
    
    /// Maximum number of results to return
    pub limit: Option<usize>,
    
    /// Optional filter for engrams created before this timestamp
    pub before: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Optional filter for engrams created after this timestamp
    pub after: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Optional filter for engrams created in a specific year
    pub year: Option<i32>,
    
    /// Optional filter for engrams created in a specific month (requires year to be set)
    pub month: Option<u32>,
    
    /// Optional filter for engrams created on a specific day (requires year and month to be set)
    pub day: Option<u32>,
    
    /// Sort by recency (true = newest first, false = oldest first)
    pub sort_by_recency: bool,
}

impl EngramQuery {
    /// Create a new empty query
    pub fn new() -> Self {
        Self {
            text: None,
            min_confidence: None,
            source: None,
            metadata_key: None,
            metadata_value: None,
            exact_match: false,
            limit: None,
            before: None,
            after: None,
            year: None,
            month: None,
            day: None,
            sort_by_recency: true,
        }
    }
    
    /// Set text search query
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }
    
    /// Set minimum confidence level
    pub fn with_min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = Some(confidence);
        self
    }
    
    /// Set source filter
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
    
    /// Set metadata key filter
    pub fn with_metadata_key(mut self, key: impl Into<String>) -> Self {
        self.metadata_key = Some(key.into());
        self
    }
    
    /// Set metadata key-value filter
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata_key = Some(key.into());
        self.metadata_value = Some(value.into());
        self
    }
    
    /// Set exact match flag for text search
    pub fn with_exact_match(mut self, exact: bool) -> Self {
        self.exact_match = exact;
        self
    }
    
    /// Set result limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }
    
    /// Set filter for engrams created before a specific timestamp
    pub fn with_before(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.before = Some(timestamp);
        self
    }
    
    /// Set filter for engrams created after a specific timestamp
    pub fn with_after(mut self, timestamp: chrono::DateTime<chrono::Utc>) -> Self {
        self.after = Some(timestamp);
        self
    }
    
    /// Set filter for engrams created between two timestamps
    pub fn with_time_range(mut self, start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Self {
        self.after = Some(start);
        self.before = Some(end);
        self
    }
    
    /// Set filter for engrams created in a specific year
    pub fn with_year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }
    
    /// Set filter for engrams created in a specific month
    pub fn with_month(mut self, year: i32, month: u32) -> Self {
        self.year = Some(year);
        self.month = Some(month);
        self
    }
    
    /// Set filter for engrams created on a specific day
    pub fn with_day(mut self, year: i32, month: u32, day: u32) -> Self {
        self.year = Some(year);
        self.month = Some(month);
        self.day = Some(day);
        self
    }
    
    /// Set sort order by recency (true = newest first, false = oldest first)
    pub fn with_sort_by_recency(mut self, newest_first: bool) -> Self {
        self.sort_by_recency = newest_first;
        self
    }
}

/// Represents types of relationship queries
#[allow(dead_code)]
pub enum RelationshipQueryType {
    /// Query for outgoing connections from a source engram
    Outgoing,
    
    /// Query for incoming connections to a target engram
    Incoming,
    
    /// Query for both incoming and outgoing connections
    Both,
    
    /// Query for connections with a specific relationship type
    RelationshipType(String),
    
    /// Query for paths between two engrams
    Path {
        /// Target engram ID
        target_id: EngramId,
        /// Maximum path length
        max_depth: usize,
    },
}

/// Represents a query for relationships
#[allow(dead_code)]
pub struct RelationshipQuery {
    /// The engram ID to start from
    pub engram_id: EngramId,
    
    /// The type of relationship query
    pub query_type: RelationshipQueryType,
}

#[allow(dead_code)]
impl RelationshipQuery {
    /// Create a new query for outgoing connections from the specified engram
    pub fn outgoing(engram_id: EngramId) -> Self {
        Self {
            engram_id,
            query_type: RelationshipQueryType::Outgoing,
        }
    }
    
    /// Create a new query for incoming connections to the specified engram
    pub fn incoming(engram_id: EngramId) -> Self {
        Self {
            engram_id,
            query_type: RelationshipQueryType::Incoming,
        }
    }
    
    /// Create a new query for both incoming and outgoing connections for the specified engram
    pub fn both(engram_id: EngramId) -> Self {
        Self {
            engram_id,
            query_type: RelationshipQueryType::Both,
        }
    }
    
    /// Create a new query for connections with a specific relationship type
    pub fn by_type(engram_id: EngramId, relationship_type: impl Into<String>) -> Self {
        Self {
            engram_id,
            query_type: RelationshipQueryType::RelationshipType(relationship_type.into()),
        }
    }
    
    /// Create a new query for paths between source and target engrams
    pub fn path(source_id: EngramId, target_id: EngramId, max_depth: usize) -> Self {
        Self {
            engram_id: source_id,
            query_type: RelationshipQueryType::Path {
                target_id,
                max_depth,
            },
        }
    }
}

/// Engine for executing queries against the memory graph
#[allow(dead_code)]
pub struct QueryEngine<'a> {
    /// The storage backend
    storage: &'a Storage,
    
    /// The search index
    index: &'a SearchIndex,
    
    /// The current forgetting policy
    forgetting_policy: Option<crate::index::ForgettingPolicy>,
}

#[allow(dead_code)]
impl<'a> QueryEngine<'a> {
    /// Create a new query engine
    pub fn new(storage: &'a Storage, index: &'a SearchIndex) -> Self {
        Self { 
            storage, 
            index,
            forgetting_policy: None,
        }
    }
    
    /// Set the forgetting policy
    pub fn set_forgetting_policy(&mut self, policy: Option<crate::index::ForgettingPolicy>) {
        self.forgetting_policy = policy;
    }
    
    /// Get the current forgetting policy
    pub fn get_forgetting_policy(&self) -> Option<&crate::index::ForgettingPolicy> {
        self.forgetting_policy.as_ref()
    }
    
    /// Record an access to an engram
    pub fn record_access(&self, id: &EngramId) -> Result<()> {
        // Record access in the index
        self.index.record_access(id)?;
        
        // Retrieve the engram to update
        if let Some(mut engram) = self.storage.get_engram(id)? {
            // Update access count and timestamp
            engram.record_access();
            
            // Store the updated engram
            self.storage.put_engram(&engram)?;
        }
        
        Ok(())
    }
    
    /// Update importance score for an engram
    pub fn update_importance(&self, id: &EngramId, importance: f64) -> Result<()> {
        // Update importance in the index
        self.index.update_importance(id, importance)?;
        
        // Retrieve the engram to update
        if let Some(mut engram) = self.storage.get_engram(id)? {
            // Update importance score
            engram.set_importance(importance);
            
            // Store the updated engram
            self.storage.put_engram(&engram)?;
        }
        
        Ok(())
    }
    
    /// Set TTL for an engram
    pub fn set_ttl(&self, id: &EngramId, ttl: Option<u64>) -> Result<()> {
        // Update TTL in the index
        self.index.set_ttl(id, ttl)?;
        
        // Retrieve the engram to update
        if let Some(mut engram) = self.storage.get_engram(id)? {
            // Update TTL
            if let Some(seconds) = ttl {
                engram.set_ttl(seconds);
            } else {
                engram.clear_ttl();
            }
            
            // Store the updated engram
            self.storage.put_engram(&engram)?;
        }
        
        Ok(())
    }
    
    /// Get forgetting candidates based on the current policy
    pub fn get_forgetting_candidates(&self) -> Result<Vec<Engram>> {
        if let Some(policy) = &self.forgetting_policy {
            // Get candidate IDs based on the policy
            let candidate_ids = policy.get_forgetting_candidates(self.index);
            
            // Fetch the full engram objects
            let mut candidates = Vec::new();
            for id in candidate_ids {
                if let Some(engram) = self.storage.get_engram(&id)? {
                    candidates.push(engram);
                }
            }
            
            Ok(candidates)
        } else {
            Ok(Vec::new()) // No policy, no candidates
        }
    }
    
    /// Apply forgetting by removing the engrams selected by the policy
    pub fn apply_forgetting(&self) -> Result<usize> {
        // Get forgetting candidates
        let candidates = self.get_forgetting_candidates()?;
        
        // Count of successfully forgotten engrams
        let mut forgotten_count = 0;
        
        // Remove each candidate
        for engram in candidates {
            if self.storage.delete_engram(&engram.id).is_ok() {
                forgotten_count += 1;
            }
        }
        
        Ok(forgotten_count)
    }
    
    /// Execute an engram query and return matching engrams
    pub fn query_engrams(&self, query: &EngramQuery) -> Result<Vec<Engram>> {
        let mut engram_ids = HashSet::new();
        
        // Process basic search parameters using the combined search
        engram_ids = self.index.search_combined(
            query.text.as_deref(),
            query.source.as_deref(),
            query.min_confidence,
            query.metadata_key.as_deref(),
            query.metadata_value.as_deref(),
            query.exact_match,
            query.before.as_ref(),
            query.after.as_ref(),
        );
        
        // Process additional temporal filters if not already covered by before/after
        if query.before.is_none() && query.after.is_none() {
            // Apply year filter if specified
            if let Some(year) = query.year {
                let year_results = self.index.find_by_year(year);
                
                // If we have existing results, intersect them
                if !engram_ids.is_empty() {
                    engram_ids = engram_ids.intersection(&year_results).cloned().collect();
                } else {
                    engram_ids = year_results;
                }
                
                // If we also have month filter
                if let Some(month) = query.month {
                    let month_results = self.index.find_by_month(year, month);
                    engram_ids = engram_ids.intersection(&month_results).cloned().collect();
                    
                    // If we also have day filter
                    if let Some(day) = query.day {
                        let day_results = self.index.find_by_day(year, month, day);
                        engram_ids = engram_ids.intersection(&day_results).cloned().collect();
                    }
                }
            }
        }
        
        // If we have no results from filtering, return empty
        if engram_ids.is_empty() && query.text.is_some() || query.source.is_some() 
            || query.min_confidence.is_some() || query.metadata_key.is_some() 
            || query.before.is_some() || query.after.is_some() || query.year.is_some() {
            return Ok(Vec::new());
        }
        
        // If no filtering was applied, get all engrams
        if engram_ids.is_empty() {
            // Just use a few recent engrams to avoid overwhelming the response
            return Ok(self.get_most_recent_engrams(100)?);
        }
        
        // Fetch the matching engrams
        let mut engrams = Vec::new();
        for id in engram_ids {
            if let Some(engram) = self.storage.get_engram(&id)? {
                engrams.push(engram);
            }
        }
        
        // Sort by time or confidence
        if query.sort_by_recency {
            // Sort by timestamp (newest first or oldest first)
            if query.sort_by_recency {
                engrams.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
            } else {
                engrams.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            }
        } else {
            // Sort by confidence (highest first)
            engrams.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        }
        
        // Apply limit if specified
        if let Some(limit) = query.limit {
            if engrams.len() > limit {
                engrams.truncate(limit);
            }
        }
        
        Ok(engrams)
    }
    
    /// Get the most recent engrams
    fn get_most_recent_engrams(&self, count: usize) -> Result<Vec<Engram>> {
        let engram_ids = self.index.get_most_recent(count);
        let mut engrams = Vec::new();
        
        for id in engram_ids {
            if let Some(engram) = self.storage.get_engram(&id)? {
                engrams.push(engram);
            }
        }
        
        Ok(engrams)
    }
    
    /// Execute a relationship query and return matching connections
    pub fn query_relationships(&self, query: &RelationshipQuery) -> Result<Vec<Connection>> {
        let connection_ids = match &query.query_type {
            RelationshipQueryType::Outgoing => {
                self.index.relationship_index.get_outgoing_connections(&query.engram_id)
            }
            RelationshipQueryType::Incoming => {
                self.index.relationship_index.get_incoming_connections(&query.engram_id)
            }
            RelationshipQueryType::Both => {
                let mut ids = self.index.relationship_index.get_outgoing_connections(&query.engram_id);
                ids.extend(self.index.relationship_index.get_incoming_connections(&query.engram_id));
                ids
            }
            RelationshipQueryType::RelationshipType(rel_type) => {
                self.index.relationship_index.find_by_source_and_type(&query.engram_id, rel_type)
            }
            RelationshipQueryType::Path { target_id, max_depth } => {
                let paths = self.index.relationship_index.find_paths(&query.engram_id, target_id, *max_depth);
                let mut connection_ids = HashSet::new();
                
                // For each path
                for path in paths {
                    // For each consecutive pair of nodes in the path
                    for i in 0..path.len() - 1 {
                        let source_id = &path[i];
                        let target_id = &path[i + 1];
                        
                        // Find connections between these nodes
                        for connection in self.get_connections_between(source_id, target_id)? {
                            connection_ids.insert(connection.id);
                        }
                    }
                }
                
                connection_ids
            }
        };
        
        let mut connections = Vec::new();
        
        for id in connection_ids {
            if let Some(connection) = self.storage.get_connection(&id)? {
                connections.push(connection);
            }
        }
        
        // Sort by weight (highest first)
        connections.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());
        
        Ok(connections)
    }
    
    /// Helper method to get connections between two engrams
    fn get_connections_between(&self, source_id: &EngramId, target_id: &EngramId) -> Result<Vec<Connection>> {
        let connection_ids = self.storage.find_connections_for_engram(source_id)?;
        
        let mut connections = Vec::new();
        
        for id in connection_ids {
            if let Some(connection) = self.storage.get_connection(&id)? {
                if connection.source_id == *source_id && connection.target_id == *target_id {
                    connections.push(connection);
                }
            }
        }
        
        Ok(connections)
    }
}

/// Result of a graph traversal operation
pub struct TraversalResult {
    /// The engrams found during traversal
    pub engrams: Vec<Engram>,
    
    /// The connections traversed
    pub connections: Vec<Connection>,
}

/// Engine for graph traversal operations
pub struct TraversalEngine<'a> {
    /// The storage backend
    storage: &'a Storage,
    
    /// The search index
    index: &'a SearchIndex,
}

impl<'a> TraversalEngine<'a> {
    /// Create a new traversal engine
    pub fn new(storage: &'a Storage, index: &'a SearchIndex) -> Self {
        Self { storage, index }
    }
    
    /// Find all engrams connected to the specified engram, up to max_depth
    pub fn find_connected_engrams(
        &self,
        engram_id: &EngramId,
        max_depth: usize,
        relationship_type: Option<&str>,
    ) -> Result<TraversalResult> {
        let mut visited_engrams = HashSet::new();
        let mut visited_connections = HashSet::new();
        
        self.traverse_outgoing(
            engram_id,
            max_depth,
            relationship_type,
            &mut visited_engrams,
            &mut visited_connections,
        )?;
        
        let mut engrams = Vec::new();
        let mut connections = Vec::new();
        
        for id in visited_engrams {
            if let Some(engram) = self.storage.get_engram(&id)? {
                engrams.push(engram);
            }
        }
        
        for id in visited_connections {
            if let Some(connection) = self.storage.get_connection(&id)? {
                connections.push(connection);
            }
        }
        
        Ok(TraversalResult {
            engrams,
            connections,
        })
    }
    
    /// Recursive helper method for traversing outgoing connections
    fn traverse_outgoing(
        &self,
        engram_id: &EngramId,
        depth_left: usize,
        relationship_type: Option<&str>,
        visited_engrams: &mut HashSet<EngramId>,
        visited_connections: &mut HashSet<ConnectionId>,
    ) -> Result<()> {
        // Mark this engram as visited
        visited_engrams.insert(engram_id.clone());
        
        // Base case: we've reached max depth
        if depth_left == 0 {
            return Ok(());
        }
        
        // Get outgoing connections
        let connections = if let Some(rel_type) = relationship_type {
            self.index.relationship_index.find_by_source_and_type(engram_id, rel_type)
        } else {
            self.index.relationship_index.get_outgoing_connections(engram_id)
        };
        
        // Process each connection
        for connection_id in connections {
            if visited_connections.contains(&connection_id) {
                continue;
            }
            
            visited_connections.insert(connection_id.clone());
            
            if let Some(connection) = self.storage.get_connection(&connection_id)? {
                if !visited_engrams.contains(&connection.target_id) {
                    // Recursive call
                    self.traverse_outgoing(
                        &connection.target_id,
                        depth_left - 1,
                        relationship_type,
                        visited_engrams,
                        visited_connections,
                    )?;
                }
            }
        }
        
        Ok(())
    }
}

/// A higher-level interface for performing queries and traversals
#[allow(dead_code)]
pub struct QueryService<'a> {
    /// The query engine
    query_engine: QueryEngine<'a>,
    
    /// The traversal engine
    traversal_engine: TraversalEngine<'a>,
}

#[allow(dead_code)]
impl<'a> QueryService<'a> {
    /// Get a reference to the query engine
    pub fn get_query_engine(&self) -> &QueryEngine<'a> {
        &self.query_engine
    }
    
    /// Get a mutable reference to the query engine
    pub fn get_query_engine_mut(&mut self) -> &mut QueryEngine<'a> {
        &mut self.query_engine
    }
    
    /// Create a new query service
    pub fn new(storage: &'a Storage, index: &'a SearchIndex) -> Self {
        Self {
            query_engine: QueryEngine::new(storage, index),
            traversal_engine: TraversalEngine::new(storage, index),
        }
    }
    
    /// Set the forgetting policy
    pub fn set_forgetting_policy(&mut self, policy: Option<crate::index::ForgettingPolicy>) {
        self.query_engine.set_forgetting_policy(policy);
    }
    
    /// Get the current forgetting policy
    pub fn get_forgetting_policy(&self) -> Option<&crate::index::ForgettingPolicy> {
        self.query_engine.get_forgetting_policy()
    }
    
    /// Record an access to an engram
    pub fn record_engram_access(&self, id: &EngramId) -> Result<()> {
        self.query_engine.record_access(id)
    }
    
    /// Update importance score for an engram
    pub fn update_engram_importance(&self, id: &EngramId, importance: f64) -> Result<()> {
        self.query_engine.update_importance(id, importance)
    }
    
    /// Set TTL for an engram
    pub fn set_engram_ttl(&self, id: &EngramId, ttl_seconds: Option<u64>) -> Result<()> {
        self.query_engine.set_ttl(id, ttl_seconds)
    }
    
    /// Get forgetting candidates based on the current policy
    pub fn get_forgetting_candidates(&self) -> Result<Vec<Engram>> {
        self.query_engine.get_forgetting_candidates()
    }
    
    /// Apply forgetting by removing the engrams selected by the policy
    pub fn apply_forgetting(&self) -> Result<usize> {
        self.query_engine.apply_forgetting()
    }
    
    /// Calculate importance score based on node centrality
    pub fn calculate_importance_by_centrality(&self, id: &EngramId) -> Result<f64> {
        // Get incoming and outgoing connections
        let incoming = self.find_all_connections(id)?
            .into_iter()
            .filter(|conn| &conn.target_id == id)
            .count() as f64;
        
        let outgoing = self.find_all_connections(id)?
            .into_iter()
            .filter(|conn| &conn.source_id == id)
            .count() as f64;
        
        // Simple centrality score - more connections = more important
        // Normalize to 0.0-1.0 range using a logarithmic scale
        let connection_count = incoming + outgoing;
        let importance = if connection_count > 0.0 {
            (1.0 + connection_count.ln().max(0.0) / 5.0).min(1.0)
        } else {
            0.2 // Base importance for isolated engrams
        };
        
        // Update the importance
        self.query_engine.update_importance(id, importance)?;
        
        Ok(importance)
    }
    
    /// Create an age-based forgetting policy
    pub fn create_age_based_policy(&self, max_age_days: u32, max_items: usize) -> crate::index::ForgettingPolicy {
        crate::index::ForgettingPolicy::AgeBased {
            max_age_seconds: max_age_days as u64 * 86400, // Convert days to seconds
            max_items,
        }
    }
    
    /// Create an importance-threshold forgetting policy
    pub fn create_importance_threshold_policy(&self, max_importance: f64, max_items: usize) -> crate::index::ForgettingPolicy {
        crate::index::ForgettingPolicy::ImportanceThreshold {
            max_importance,
            max_items,
        }
    }
    
    /// Create an access-frequency forgetting policy
    pub fn create_access_frequency_policy(&self, max_access_count: u32, min_idle_days: u32, max_items: usize) -> crate::index::ForgettingPolicy {
        crate::index::ForgettingPolicy::AccessFrequency {
            max_access_count,
            min_idle_seconds: min_idle_days as u64 * 86400, // Convert days to seconds
            max_items,
        }
    }
    
    /// Create a hybrid forgetting policy
    pub fn create_hybrid_policy(&self, max_importance: f64, max_access_count: u32, min_idle_days: u32, max_items: usize) -> crate::index::ForgettingPolicy {
        crate::index::ForgettingPolicy::Hybrid {
            max_importance,
            max_access_count,
            min_idle_seconds: min_idle_days as u64 * 86400, // Convert days to seconds
            max_items,
        }
    }
    
    /// Create a TTL-based expiration policy
    pub fn create_ttl_policy(&self, max_items: usize) -> crate::index::ForgettingPolicy {
        crate::index::ForgettingPolicy::TTLExpiration {
            max_items,
        }
    }
    
    /// Search for engrams by text content
    pub fn search_by_text(&self, query: &str, exact_match: bool, limit: Option<usize>) -> Result<Vec<Engram>> {
        let query = EngramQuery::new()
            .with_text(query)
            .with_exact_match(exact_match);
        
        let query = if let Some(limit) = limit {
            query.with_limit(limit)
        } else {
            query
        };
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams by source
    pub fn search_by_source(&self, source: &str, limit: Option<usize>) -> Result<Vec<Engram>> {
        let query = EngramQuery::new().with_source(source);
        
        let query = if let Some(limit) = limit {
            query.with_limit(limit)
        } else {
            query
        };
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams by minimum confidence level
    pub fn search_by_confidence(&self, min_confidence: f64, limit: Option<usize>) -> Result<Vec<Engram>> {
        let query = EngramQuery::new().with_min_confidence(min_confidence);
        
        let query = if let Some(limit) = limit {
            query.with_limit(limit)
        } else {
            query
        };
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams by metadata
    pub fn search_by_metadata(
        &self,
        key: &str,
        value: Option<&str>,
        limit: Option<usize>,
    ) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new();
        
        if let Some(val) = value {
            query = query.with_metadata(key, val);
        } else {
            query = query.with_metadata_key(key);
        }
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Get the most recent engrams
    pub fn get_recent_engrams(&self, limit: usize) -> Result<Vec<Engram>> {
        let query = EngramQuery::new()
            .with_sort_by_recency(true)
            .with_limit(limit);
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created before a specific timestamp
    pub fn search_before_time(
        &self, 
        timestamp: chrono::DateTime<chrono::Utc>,
        limit: Option<usize>
    ) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_before(timestamp);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created after a specific timestamp
    pub fn search_after_time(
        &self, 
        timestamp: chrono::DateTime<chrono::Utc>,
        limit: Option<usize>
    ) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_after(timestamp);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created between two timestamps
    pub fn search_between_times(
        &self,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
        limit: Option<usize>
    ) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_time_range(start, end);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created in a specific year
    pub fn search_by_year(&self, year: i32, limit: Option<usize>) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_year(year);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created in a specific month
    pub fn search_by_month(&self, year: i32, month: u32, limit: Option<usize>) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_month(year, month);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Search for engrams created on a specific day
    pub fn search_by_day(&self, year: i32, month: u32, day: u32, limit: Option<usize>) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_day(year, month, day);
        
        if let Some(limit) = limit {
            query = query.with_limit(limit);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Find connections between two engrams
    pub fn find_connections(
        &self,
        source_id: &EngramId,
        target_id: &EngramId,
    ) -> Result<Vec<Connection>> {
        // Get the storage reference through query_engine
        let storage = &self.query_engine.storage;
        
        // First check if both engrams exist
        if storage.get_engram(source_id)?.is_none() {
            return Err(EngramError::NotFound(format!("Source engram not found: {}", source_id)));
        }
        
        if storage.get_engram(target_id)?.is_none() {
            return Err(EngramError::NotFound(format!("Target engram not found: {}", target_id)));
        }
        
        // Get all connections from source to target
        self.query_engine.get_connections_between(source_id, target_id)
    }
    
    /// Find all connections for an engram
    pub fn find_all_connections(&self, engram_id: &EngramId) -> Result<Vec<Connection>> {
        let query = RelationshipQuery::both(engram_id.clone());
        self.query_engine.query_relationships(&query)
    }
    
    /// Find paths between two engrams
    pub fn find_paths(
        &self,
        source_id: &EngramId,
        target_id: &EngramId,
        max_depth: usize,
    ) -> Result<Vec<(Vec<Engram>, Vec<Connection>)>> {
        // Get the storage and index references through query_engine
        let storage = &self.query_engine.storage;
        let index = &self.query_engine.index;
        
        // First check if both engrams exist
        if storage.get_engram(source_id)?.is_none() {
            return Err(EngramError::NotFound(format!("Source engram not found: {}", source_id)));
        }
        
        if storage.get_engram(target_id)?.is_none() {
            return Err(EngramError::NotFound(format!("Target engram not found: {}", target_id)));
        }
        
        // Find all paths between source and target
        let paths = index.relationship_index.find_paths(source_id, target_id, max_depth);
        
        let mut result = Vec::new();
        
        for path in paths {
            let mut engrams = Vec::new();
            let mut connections = Vec::new();
            
            // Collect all engrams in the path
            for id in &path {
                if let Some(engram) = storage.get_engram(id)? {
                    engrams.push(engram);
                }
            }
            
            // Collect all connections in the path
            for i in 0..path.len() - 1 {
                let source = &path[i];
                let target = &path[i + 1];
                
                for connection in self.query_engine.get_connections_between(source, target)? {
                    connections.push(connection);
                }
            }
            
            result.push((engrams, connections));
        }
        
        Ok(result)
    }
    
    /// Find all engrams connected to the specified engram
    pub fn find_connected_engrams(
        &self,
        engram_id: &EngramId,
        max_depth: usize,
        relationship_type: Option<&str>,
    ) -> Result<TraversalResult> {
        self.traversal_engine
            .find_connected_engrams(engram_id, max_depth, relationship_type)
    }
    
    /// Combined search with multiple criteria
    pub fn search_combined(
        &self,
        text: Option<&str>,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_key: Option<&str>,
        metadata_value: Option<&str>,
        exact_match: bool,
        before: Option<chrono::DateTime<chrono::Utc>>,
        after: Option<chrono::DateTime<chrono::Utc>>,
        year: Option<i32>,
        month: Option<u32>,
        day: Option<u32>,
        sort_by_recency: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Engram>> {
        let mut query = EngramQuery::new().with_exact_match(exact_match);
        
        if let Some(t) = text {
            query = query.with_text(t);
        }
        
        if let Some(s) = source {
            query = query.with_source(s);
        }
        
        if let Some(c) = min_confidence {
            query = query.with_min_confidence(c);
        }
        
        if let Some(k) = metadata_key {
            if let Some(v) = metadata_value {
                query = query.with_metadata(k, v);
            } else {
                query = query.with_metadata_key(k);
            }
        }
        
        // Apply temporal filters
        if let Some(b) = before {
            query = query.with_before(b);
        }
        
        if let Some(a) = after {
            query = query.with_after(a);
        }
        
        // Apply year/month/day filters if before/after not specified
        if before.is_none() && after.is_none() {
            if let Some(y) = year {
                query = query.with_year(y);
                
                if let Some(m) = month {
                    query = query.with_month(y, m);
                    
                    if let Some(d) = day {
                        query = query.with_day(y, m, d);
                    }
                }
            }
        }
        
        query = query.with_sort_by_recency(sort_by_recency);
        
        if let Some(l) = limit {
            query = query.with_limit(l);
        }
        
        self.query_engine.query_engrams(&query)
    }
    
    /// Simplified combined search (backward compatibility)
    pub fn search_combined_legacy(
        &self,
        text: Option<&str>,
        source: Option<&str>,
        min_confidence: Option<f64>,
        metadata_key: Option<&str>,
        metadata_value: Option<&str>,
        exact_match: bool,
        limit: Option<usize>,
    ) -> Result<Vec<Engram>> {
        self.search_combined(
            text,
            source,
            min_confidence,
            metadata_key,
            metadata_value,
            exact_match,
            None,
            None,
            None,
            None,
            None,
            false,
            limit,
        )
    }
}