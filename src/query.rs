use crate::error::{EngramError, Result};
use crate::schema::{Engram, EngramId, Connection, ConnectionId};
use crate::storage::Storage;
use crate::index::SearchIndex;
use std::collections::HashSet;

/// Represents filter conditions for querying engrams
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
}

/// Represents types of relationship queries
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
pub struct RelationshipQuery {
    /// The engram ID to start from
    pub engram_id: EngramId,
    
    /// The type of relationship query
    pub query_type: RelationshipQueryType,
}

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
pub struct QueryEngine<'a> {
    /// The storage backend
    storage: &'a Storage,
    
    /// The search index
    index: &'a SearchIndex,
}

impl<'a> QueryEngine<'a> {
    /// Create a new query engine
    pub fn new(storage: &'a Storage, index: &'a SearchIndex) -> Self {
        Self { storage, index }
    }
    
    /// Execute an engram query and return matching engrams
    pub fn query_engrams(&self, query: &EngramQuery) -> Result<Vec<Engram>> {
        // Execute the query against the index
        let ids = self.index.search_combined(
            query.text.as_deref(),
            query.source.as_deref(),
            query.min_confidence,
            query.metadata_key.as_deref(),
            query.metadata_value.as_deref(),
            query.exact_match,
        );
        
        // Fetch the matching engrams
        let mut engrams = Vec::new();
        for id in ids {
            if let Some(engram) = self.storage.get_engram(&id)? {
                engrams.push(engram);
            }
        }
        
        // Sort by confidence (highest first)
        engrams.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap());
        
        // Apply limit if specified
        if let Some(limit) = query.limit {
            if engrams.len() > limit {
                engrams.truncate(limit);
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
pub struct QueryService<'a> {
    /// The query engine
    query_engine: QueryEngine<'a>,
    
    /// The traversal engine
    traversal_engine: TraversalEngine<'a>,
}

impl<'a> QueryService<'a> {
    /// Get a reference to the query engine
    pub fn get_query_engine(&self) -> &QueryEngine<'a> {
        &self.query_engine
    }
    
    /// Create a new query service
    pub fn new(storage: &'a Storage, index: &'a SearchIndex) -> Self {
        Self {
            query_engine: QueryEngine::new(storage, index),
            traversal_engine: TraversalEngine::new(storage, index),
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
        
        if let Some(l) = limit {
            query = query.with_limit(l);
        }
        
        self.query_engine.query_engrams(&query)
    }
}