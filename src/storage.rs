use crate::error::{EngramError, Result};
use crate::schema::{
    Agent, AgentId, Collection, CollectionId, Connection, ConnectionId, Context, ContextId, Engram,
    EngramId,
};
// Forward declare the Embedding struct to avoid circular dependency
// We don't need to import the embedding module here, as we'll define our own Embedding struct

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Embedding {
    pub vector: Vec<f32>,
    pub model: String,
    pub dimensions: usize,
    pub metadata: std::collections::HashMap<String, String>,
}

// We'll implement conversion utilities as methods instead of using the From trait
// to avoid circular dependencies

#[allow(dead_code)]
impl Embedding {
    /// Create from vector, model, and dimensions
    pub fn create(vector: Vec<f32>, model: String, dimensions: usize, metadata: std::collections::HashMap<String, String>) -> Self {
        Self {
            vector,
            model,
            dimensions,
            metadata,
        }
    }
}
use rocksdb::{ColumnFamilyDescriptor, Options, DB, WriteBatch, IteratorMode};
use serde::{de::DeserializeOwned, Serialize};
use std::path::Path;
use std::collections::HashSet;

/// Storage prefix keys for different entity types
const ENGRAM_PREFIX: &[u8] = b"engram:";
const CONNECTION_PREFIX: &[u8] = b"connection:";
const COLLECTION_PREFIX: &[u8] = b"collection:";
const AGENT_PREFIX: &[u8] = b"agent:";
const CONTEXT_PREFIX: &[u8] = b"context:";

// Additional prefixes for indexes
const SOURCE_CONNECTION_PREFIX: &[u8] = b"source_conn:";
const TARGET_CONNECTION_PREFIX: &[u8] = b"target_conn:";
const RELATION_TYPE_PREFIX: &[u8] = b"rel_type:";

// Embedding prefixes
#[allow(dead_code)]
const EMBEDDING_PREFIX: &[u8] = b"embedding:";
#[allow(dead_code)]
const REDUCED_EMBEDDING_PREFIX: &[u8] = b"reduced_embedding:";

/// Column family names
const CF_ENGRAMS: &str = "engrams";
const CF_CONNECTIONS: &str = "connections";
const CF_COLLECTIONS: &str = "collections";
const CF_AGENTS: &str = "agents";
const CF_CONTEXTS: &str = "contexts";
const CF_METADATA: &str = "metadata";
const CF_RELATIONSHIPS: &str = "relationships"; // For storing relationship indexes
const CF_EMBEDDINGS: &str = "embeddings"; // For storing vector embeddings

/// Database statistics structure
#[derive(Debug, Clone, serde::Serialize)]
pub struct StorageStats {
    pub engram_count: usize,
    pub connection_count: usize,
    pub collection_count: usize,
    pub agent_count: usize,
    pub context_count: usize, 
    pub db_size_mb: f64,
}

/// RocksDB-based storage implementation for EngramAI
pub struct Storage {
    pub db: DB,
}

impl Storage {
    /// Creates a new Storage instance with the specified path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        // Define column families
        let cf_names = vec![
            CF_ENGRAMS,
            CF_CONNECTIONS,
            CF_COLLECTIONS,
            CF_AGENTS,
            CF_CONTEXTS,
            CF_METADATA,
            CF_RELATIONSHIPS,
            CF_EMBEDDINGS,
        ];

        let cf_descriptors: Vec<_> = cf_names
            .iter()
            .map(|name| {
                let mut cf_opts = Options::default();
                cf_opts.set_max_write_buffer_number(16);
                ColumnFamilyDescriptor::new(*name, cf_opts)
            })
            .collect();

        // Open database with all column families
        let db = DB::open_cf_descriptors(&opts, path, cf_descriptors)
            .map_err(|e| EngramError::StorageError(e.to_string()))?;

        Ok(Self { db })
    }
    
    /// List all engram IDs in the database
    pub fn list_engrams(&self) -> Result<Vec<String>> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;
        
        let mut engram_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(ENGRAM_PREFIX) {
                let id_bytes = &key[ENGRAM_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                engram_ids.push(id);
            }
        }
        
        Ok(engram_ids)
    }
    
    /// List all connection IDs in the database
    pub fn list_connections(&self) -> Result<Vec<String>> {
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;
        
        let mut connection_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(CONNECTION_PREFIX) {
                let id_bytes = &key[CONNECTION_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                connection_ids.push(id);
            }
        }
        
        Ok(connection_ids)
    }
    
    /// List all collection IDs in the database
    pub fn list_collections(&self) -> Result<Vec<String>> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;
        
        let mut collection_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(COLLECTION_PREFIX) {
                let id_bytes = &key[COLLECTION_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                collection_ids.push(id);
            }
        }
        
        Ok(collection_ids)
    }
    
    /// List all agent IDs in the database
    pub fn list_agents(&self) -> Result<Vec<String>> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;
        
        let mut agent_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(AGENT_PREFIX) {
                let id_bytes = &key[AGENT_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                agent_ids.push(id);
            }
        }
        
        Ok(agent_ids)
    }
    
    /// List all context IDs in the database
    pub fn list_contexts(&self) -> Result<Vec<String>> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;
        
        let mut context_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(CONTEXT_PREFIX) {
                let id_bytes = &key[CONTEXT_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                context_ids.push(id);
            }
        }
        
        Ok(context_ids)
    }
    
    /// Find all connections related to a specific engram (either as source or target)
    pub fn find_connections_for_engram(&self, engram_id: &EngramId) -> Result<HashSet<ConnectionId>> {
        // Get outgoing and incoming connections from the relationship index
        let mut connection_ids = HashSet::new();
        
        // Add outgoing connections
        connection_ids.extend(self.find_outgoing_connections(engram_id)?);
        
        // Add incoming connections
        connection_ids.extend(self.find_incoming_connections(engram_id)?);
        
        Ok(connection_ids)
    }
    
    /// Find all outgoing connections from a source engram
    pub fn find_outgoing_connections(&self, source_id: &EngramId) -> Result<HashSet<ConnectionId>> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        let mut connection_ids = HashSet::new();
        
        // Create the prefix for the source engram
        let prefix = [SOURCE_CONNECTION_PREFIX, source_id.as_bytes()].concat();
        
        // Iterate through keys with this prefix
        let iter = self.db.prefix_iterator_cf(cf, &prefix);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract connection ID from the key
            // Key format: source_conn:{source_id}:{connection_id}
            let parts: Vec<&[u8]> = key.split(|&b| b == b':').collect();
            if parts.len() >= 3 {
                let connection_id = String::from_utf8_lossy(parts[2]).to_string();
                connection_ids.insert(connection_id);
            }
        }
        
        Ok(connection_ids)
    }
    
    /// Find all incoming connections to a target engram
    pub fn find_incoming_connections(&self, target_id: &EngramId) -> Result<HashSet<ConnectionId>> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        let mut connection_ids = HashSet::new();
        
        // Create the prefix for the target engram
        let prefix = [TARGET_CONNECTION_PREFIX, target_id.as_bytes()].concat();
        
        // Iterate through keys with this prefix
        let iter = self.db.prefix_iterator_cf(cf, &prefix);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract connection ID from the key
            // Key format: target_conn:{target_id}:{connection_id}
            let parts: Vec<&[u8]> = key.split(|&b| b == b':').collect();
            if parts.len() >= 3 {
                let connection_id = String::from_utf8_lossy(parts[2]).to_string();
                connection_ids.insert(connection_id);
            }
        }
        
        Ok(connection_ids)
    }
    
    /// Find all connections with a specific relationship type
    pub fn find_connections_by_type(&self, relationship_type: &str) -> Result<HashSet<ConnectionId>> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        let mut connection_ids = HashSet::new();
        
        // Create the prefix for the relationship type
        let prefix = [RELATION_TYPE_PREFIX, relationship_type.as_bytes()].concat();
        
        // Iterate through keys with this prefix
        let iter = self.db.prefix_iterator_cf(cf, &prefix);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract connection ID from the key
            // Key format: rel_type:{relationship_type}:{connection_id}
            let parts: Vec<&[u8]> = key.split(|&b| b == b':').collect();
            if parts.len() >= 3 {
                let connection_id = String::from_utf8_lossy(parts[2]).to_string();
                connection_ids.insert(connection_id);
            }
        }
        
        Ok(connection_ids)
    }
    
    //
    // Embedding Operations
    //
    
    /// Store an embedding for an engram
    pub fn put_embedding(&self, engram_id: &EngramId, embedding: &Embedding) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = engram_id.as_bytes();
        let value = Self::serialize(embedding)?;
        
        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }
    
    /// Retrieve an embedding for an engram
    pub fn get_embedding(&self, engram_id: &EngramId) -> Result<Option<Embedding>> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = engram_id.as_bytes();
        
        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }
    
    /// Store a reduced (dimensionality-reduced) embedding for an engram
    pub fn put_reduced_embedding(&self, engram_id: &EngramId, embedding: &Embedding) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = [REDUCED_EMBEDDING_PREFIX, engram_id.as_bytes()].concat();
        let value = Self::serialize(embedding)?;
        
        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    /// Retrieve a reduced embedding for an engram
    pub fn get_reduced_embedding(&self, engram_id: &EngramId) -> Result<Option<Embedding>> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = [REDUCED_EMBEDDING_PREFIX, engram_id.as_bytes()].concat();
        
        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }
    
    /// Delete a reduced embedding for an engram
    pub fn delete_reduced_embedding(&self, engram_id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = [REDUCED_EMBEDDING_PREFIX, engram_id.as_bytes()].concat();
        
        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }
    
    /// Delete an embedding for an engram
    pub fn delete_embedding(&self, engram_id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = engram_id.as_bytes();
        
        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }
    
    /// List all embeddings in the database
    pub fn list_embeddings(&self) -> Result<Vec<EngramId>> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let mut engram_ids = Vec::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, _) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            let id = String::from_utf8_lossy(&key).to_string();
            engram_ids.push(id);
        }
        
        Ok(engram_ids)
    }

    /// Helper method to serialize an object to JSON bytes
    fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
        serde_json::to_vec(value).map_err(|e| EngramError::SerializationError(e.to_string()))
    }

    /// Helper method to deserialize JSON bytes to an object
    fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| EngramError::SerializationError(e.to_string()))
    }
    
    /// Get database statistics
    pub fn get_stats(&self) -> Result<StorageStats> {
        // Count items in each collection
        let engram_count = self.list_engrams()?.len();
        let connection_count = self.list_connections()?.len();
        let collection_count = self.list_collections()?.len();
        let agent_count = self.list_agents()?.len();
        let context_count = self.list_contexts()?.len();
        
        // Estimate database size (this is an approximation)
        // For a more accurate measure, we would need file system operations
        let db_size_mb = 0.0; // Placeholder - could use another approach for actual size
        
        Ok(StorageStats {
            engram_count,
            connection_count,
            collection_count,
            agent_count,
            context_count,
            db_size_mb,
        })
    }

    /// Creates a key with the appropriate prefix
    fn create_key(prefix: &[u8], id: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(prefix.len() + id.len());
        key.extend_from_slice(prefix);
        key.extend_from_slice(id.as_bytes());
        key
    }

    //
    // Engram Operations
    //

    /// Stores an engram in the database
    pub fn put_engram(&self, engram: &Engram) -> Result<()> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;

        let key = Self::create_key(ENGRAM_PREFIX, &engram.id);
        let value = Self::serialize(engram)?;

        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    /// Retrieves an engram from the database by ID
    pub fn get_engram(&self, id: &EngramId) -> Result<Option<Engram>> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;

        let key = Self::create_key(ENGRAM_PREFIX, id);

        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Deletes an engram from the database by ID
    pub fn delete_engram(&self, id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;

        let key = Self::create_key(ENGRAM_PREFIX, id);

        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    //
    // Connection Operations
    //

    /// Stores a connection in the database
    pub fn put_connection(&self, connection: &Connection) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Self::create_key(CONNECTION_PREFIX, &connection.id);
        let value = Self::serialize(connection)?;

        // Store the main connection record
        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        // Also store relationship indexes for faster traversal
        self.index_connection(connection)?;
        
        Ok(())
    }
    
    /// Store relationship indexes for a connection
    fn index_connection(&self, connection: &Connection) -> Result<()> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        // Index by source engram
        let source_key = Self::create_relationship_key(
            SOURCE_CONNECTION_PREFIX, 
            &connection.source_id, 
            &connection.id
        );
        self.db
            .put_cf(cf, source_key, vec![])
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        // Index by target engram
        let target_key = Self::create_relationship_key(
            TARGET_CONNECTION_PREFIX, 
            &connection.target_id, 
            &connection.id
        );
        self.db
            .put_cf(cf, target_key, vec![])
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        // Index by relationship type
        let rel_type_key = Self::create_relationship_key(
            RELATION_TYPE_PREFIX, 
            &connection.relationship_type, 
            &connection.id
        );
        self.db
            .put_cf(cf, rel_type_key, vec![])
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        Ok(())
    }
    
    /// Helper to create relationship index keys
    fn create_relationship_key(prefix: &[u8], entity_id: &str, connection_id: &str) -> Vec<u8> {
        let mut key = Vec::with_capacity(prefix.len() + entity_id.len() + 1 + connection_id.len());
        key.extend_from_slice(prefix);
        key.extend_from_slice(entity_id.as_bytes());
        key.push(b':');
        key.extend_from_slice(connection_id.as_bytes());
        key
    }

    /// Retrieves a connection from the database by ID
    pub fn get_connection(&self, id: &ConnectionId) -> Result<Option<Connection>> {
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Self::create_key(CONNECTION_PREFIX, id);

        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Deletes a connection from the database by ID
    pub fn delete_connection(&self, id: &ConnectionId) -> Result<()> {
        // First get the connection to remove indexes
        let connection = self.get_connection(id)?;
        
        if let Some(connection) = connection {
            // First delete relationship indexes
            self.delete_relationship_indexes(&connection)?;
            
            // Then delete the main connection record
            let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
                EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
            })?;

            let key = Self::create_key(CONNECTION_PREFIX, id);

            self.db
                .delete_cf(cf, key)
                .map_err(|e| EngramError::StorageError(e.to_string()))?;
        }
        
        Ok(())
    }
    
    /// Delete relationship indexes for a connection
    fn delete_relationship_indexes(&self, connection: &Connection) -> Result<()> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        // Delete source index
        let source_key = Self::create_relationship_key(
            SOURCE_CONNECTION_PREFIX, 
            &connection.source_id, 
            &connection.id
        );
        self.db
            .delete_cf(cf, source_key)
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        // Delete target index
        let target_key = Self::create_relationship_key(
            TARGET_CONNECTION_PREFIX, 
            &connection.target_id, 
            &connection.id
        );
        self.db
            .delete_cf(cf, target_key)
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        // Delete relationship type index
        let rel_type_key = Self::create_relationship_key(
            RELATION_TYPE_PREFIX, 
            &connection.relationship_type, 
            &connection.id
        );
        self.db
            .delete_cf(cf, rel_type_key)
            .map_err(|e| EngramError::StorageError(e.to_string()))?;
        
        Ok(())
    }

    //
    // Collection Operations
    //

    /// Stores a collection in the database
    pub fn put_collection(&self, collection: &Collection) -> Result<()> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;

        let key = Self::create_key(COLLECTION_PREFIX, &collection.id);
        let value = Self::serialize(collection)?;

        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    /// Retrieves a collection from the database by ID
    pub fn get_collection(&self, id: &CollectionId) -> Result<Option<Collection>> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;

        let key = Self::create_key(COLLECTION_PREFIX, id);

        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Deletes a collection from the database by ID
    pub fn delete_collection(&self, id: &CollectionId) -> Result<()> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;

        let key = Self::create_key(COLLECTION_PREFIX, id);

        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    //
    // Agent Operations
    //

    /// Stores an agent in the database
    pub fn put_agent(&self, agent: &Agent) -> Result<()> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;

        let key = Self::create_key(AGENT_PREFIX, &agent.id);
        let value = Self::serialize(agent)?;

        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    /// Retrieves an agent from the database by ID
    pub fn get_agent(&self, id: &AgentId) -> Result<Option<Agent>> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;

        let key = Self::create_key(AGENT_PREFIX, id);

        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Deletes an agent from the database by ID
    pub fn delete_agent(&self, id: &AgentId) -> Result<()> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;

        let key = Self::create_key(AGENT_PREFIX, id);

        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    //
    // Context Operations
    //

    /// Stores a context in the database
    pub fn put_context(&self, context: &Context) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;

        let key = Self::create_key(CONTEXT_PREFIX, &context.id);
        let value = Self::serialize(context)?;

        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    /// Retrieves a context from the database by ID
    pub fn get_context(&self, id: &ContextId) -> Result<Option<Context>> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;

        let key = Self::create_key(CONTEXT_PREFIX, id);

        match self.db.get_cf(cf, key)? {
            Some(bytes) => Ok(Some(Self::deserialize(&bytes)?)),
            None => Ok(None),
        }
    }

    /// Deletes a context from the database by ID
    pub fn delete_context(&self, id: &ContextId) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;

        let key = Self::create_key(CONTEXT_PREFIX, id);

        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
    }

    //
    // Transaction Support
    //

    /// Begin a transaction (batch operations)
    pub fn begin_transaction(&self) -> Transaction {
        Transaction {
            batch: WriteBatch::default(),
            db: &self.db,
        }
    }
}

/// Represents a transaction in RocksDB (using WriteBatch)
#[allow(dead_code)]
pub struct Transaction<'a> {
    batch: WriteBatch,
    db: &'a DB,
}

#[allow(dead_code)]
impl<'a> Transaction<'a> {
    /// Add an engram to the transaction
    pub fn put_engram(&mut self, engram: &Engram) -> Result<()> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;

        let key = Storage::create_key(ENGRAM_PREFIX, &engram.id);
        let value = Storage::serialize(engram)?;

        self.batch.put_cf(cf, key, value);
        Ok(())
    }

    /// Add a connection to the transaction
    pub fn put_connection(&mut self, connection: &Connection) -> Result<()> {
        // Add the main connection record
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Storage::create_key(CONNECTION_PREFIX, &connection.id);
        let value = Storage::serialize(connection)?;

        self.batch.put_cf(cf, key, value);
        
        // Add relationship indexes
        self.index_connection(connection)?;
        
        Ok(())
    }
    
    /// Add relationship indexes for a connection to the transaction
    fn index_connection(&mut self, connection: &Connection) -> Result<()> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        // Index by source engram
        let source_key = Storage::create_relationship_key(
            SOURCE_CONNECTION_PREFIX, 
            &connection.source_id, 
            &connection.id
        );
        self.batch.put_cf(cf, source_key, vec![]);
        
        // Index by target engram
        let target_key = Storage::create_relationship_key(
            TARGET_CONNECTION_PREFIX, 
            &connection.target_id, 
            &connection.id
        );
        self.batch.put_cf(cf, target_key, vec![]);
        
        // Index by relationship type
        let rel_type_key = Storage::create_relationship_key(
            RELATION_TYPE_PREFIX, 
            &connection.relationship_type, 
            &connection.id
        );
        self.batch.put_cf(cf, rel_type_key, vec![]);
        
        Ok(())
    }

    /// Add a collection to the transaction
    pub fn put_collection(&mut self, collection: &Collection) -> Result<()> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;

        let key = Storage::create_key(COLLECTION_PREFIX, &collection.id);
        let value = Storage::serialize(collection)?;

        self.batch.put_cf(cf, key, value);
        Ok(())
    }

    /// Add an agent to the transaction
    pub fn put_agent(&mut self, agent: &Agent) -> Result<()> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;

        let key = Storage::create_key(AGENT_PREFIX, &agent.id);
        let value = Storage::serialize(agent)?;

        self.batch.put_cf(cf, key, value);
        Ok(())
    }

    /// Add a context to the transaction
    pub fn put_context(&mut self, context: &Context) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;

        let key = Storage::create_key(CONTEXT_PREFIX, &context.id);
        let value = Storage::serialize(context)?;

        self.batch.put_cf(cf, key, value);
        Ok(())
    }

    /// Delete an engram in the transaction
    pub fn delete_engram(&mut self, id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_ENGRAMS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_ENGRAMS))
        })?;

        let key = Storage::create_key(ENGRAM_PREFIX, id);
        self.batch.delete_cf(cf, key);
        Ok(())
    }

    /// Delete a connection in the transaction
    pub fn delete_connection(&mut self, id: &ConnectionId, connection: Option<&Connection>) -> Result<()> {
        // Delete the main connection record
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Storage::create_key(CONNECTION_PREFIX, id);
        self.batch.delete_cf(cf, key);
        
        // Delete relationship indexes if connection is provided
        if let Some(conn) = connection {
            self.delete_relationship_indexes(conn)?;
        }
        
        Ok(())
    }
    
    /// Delete relationship indexes for a connection in the transaction
    fn delete_relationship_indexes(&mut self, connection: &Connection) -> Result<()> {
        let cf = self.db.cf_handle(CF_RELATIONSHIPS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_RELATIONSHIPS))
        })?;
        
        // Delete source index
        let source_key = Storage::create_relationship_key(
            SOURCE_CONNECTION_PREFIX, 
            &connection.source_id, 
            &connection.id
        );
        self.batch.delete_cf(cf, source_key);
        
        // Delete target index
        let target_key = Storage::create_relationship_key(
            TARGET_CONNECTION_PREFIX, 
            &connection.target_id, 
            &connection.id
        );
        self.batch.delete_cf(cf, target_key);
        
        // Delete relationship type index
        let rel_type_key = Storage::create_relationship_key(
            RELATION_TYPE_PREFIX, 
            &connection.relationship_type, 
            &connection.id
        );
        self.batch.delete_cf(cf, rel_type_key);
        
        Ok(())
    }

    /// Delete a collection in the transaction
    pub fn delete_collection(&mut self, id: &CollectionId) -> Result<()> {
        let cf = self.db.cf_handle(CF_COLLECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_COLLECTIONS))
        })?;

        let key = Storage::create_key(COLLECTION_PREFIX, id);
        self.batch.delete_cf(cf, key);
        Ok(())
    }

    /// Delete an agent in the transaction
    pub fn delete_agent(&mut self, id: &AgentId) -> Result<()> {
        let cf = self.db.cf_handle(CF_AGENTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_AGENTS))
        })?;

        let key = Storage::create_key(AGENT_PREFIX, id);
        self.batch.delete_cf(cf, key);
        Ok(())
    }

    /// Delete a context in the transaction
    pub fn delete_context(&mut self, id: &ContextId) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONTEXTS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONTEXTS))
        })?;

        let key = Storage::create_key(CONTEXT_PREFIX, id);
        self.batch.delete_cf(cf, key);
        Ok(())
    }
    
    /// Add an embedding to the transaction
    pub fn put_embedding(&mut self, engram_id: &EngramId, embedding: &Embedding) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = engram_id.as_bytes();
        let value = Storage::serialize(embedding)?;
        
        self.batch.put_cf(cf, key, value);
        Ok(())
    }
    
    /// Add a reduced embedding to the transaction
    pub fn put_reduced_embedding(&mut self, engram_id: &EngramId, embedding: &Embedding) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = [REDUCED_EMBEDDING_PREFIX, engram_id.as_bytes()].concat();
        let value = Storage::serialize(embedding)?;
        
        self.batch.put_cf(cf, key, value);
        Ok(())
    }
    
    /// Delete a reduced embedding in the transaction
    pub fn delete_reduced_embedding(&mut self, engram_id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = [REDUCED_EMBEDDING_PREFIX, engram_id.as_bytes()].concat();
        self.batch.delete_cf(cf, key);
        Ok(())
    }

    /// Delete an embedding in the transaction
    pub fn delete_embedding(&mut self, engram_id: &EngramId) -> Result<()> {
        let cf = self.db.cf_handle(CF_EMBEDDINGS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_EMBEDDINGS))
        })?;
        
        let key = engram_id.as_bytes();
        self.batch.delete_cf(cf, key);
        Ok(())
    }

    /// Commit the transaction (apply all operations)
    pub fn commit(self) -> Result<()> {
        self.db
            .write(self.batch)
            .map_err(|e| EngramError::TransactionError(e.to_string()))
    }

    /// Abort the transaction (discard all operations)
    pub fn abort(self) {
        // Drop the batch without writing it
    }
}