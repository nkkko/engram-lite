use crate::error::{EngramError, Result};
use crate::schema::{
    Agent, AgentId, Collection, CollectionId, Connection, ConnectionId, Context, ContextId, Engram,
    EngramId,
};
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

/// Column family names
const CF_ENGRAMS: &str = "engrams";
const CF_CONNECTIONS: &str = "connections";
const CF_COLLECTIONS: &str = "collections";
const CF_AGENTS: &str = "agents";
const CF_CONTEXTS: &str = "contexts";
const CF_METADATA: &str = "metadata";

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
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;
        
        let mut connection_ids = HashSet::new();
        let iter = self.db.iterator_cf(cf, IteratorMode::Start);
        
        for result in iter {
            let (key, value) = result.map_err(|e| EngramError::StorageError(e.to_string()))?;
            
            // Extract ID from key (remove the prefix)
            if key.starts_with(CONNECTION_PREFIX) {
                let id_bytes = &key[CONNECTION_PREFIX.len()..];
                let id = String::from_utf8_lossy(id_bytes).to_string();
                
                // Deserialize the connection to check if it involves the engram
                let connection: Connection = Self::deserialize(&value)?;
                
                if connection.source_id == *engram_id || connection.target_id == *engram_id {
                    connection_ids.insert(id);
                }
            }
        }
        
        Ok(connection_ids)
    }

    /// Helper method to serialize an object to JSON bytes
    fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
        serde_json::to_vec(value).map_err(|e| EngramError::SerializationError(e.to_string()))
    }

    /// Helper method to deserialize JSON bytes to an object
    fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
        serde_json::from_slice(bytes).map_err(|e| EngramError::SerializationError(e.to_string()))
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

        self.db
            .put_cf(cf, key, value)
            .map_err(|e| EngramError::StorageError(e.to_string()))
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
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Self::create_key(CONNECTION_PREFIX, id);

        self.db
            .delete_cf(cf, key)
            .map_err(|e| EngramError::StorageError(e.to_string()))
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
pub struct Transaction<'a> {
    batch: WriteBatch,
    db: &'a DB,
}

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
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Storage::create_key(CONNECTION_PREFIX, &connection.id);
        let value = Storage::serialize(connection)?;

        self.batch.put_cf(cf, key, value);
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
    pub fn delete_connection(&mut self, id: &ConnectionId) -> Result<()> {
        let cf = self.db.cf_handle(CF_CONNECTIONS).ok_or_else(|| {
            EngramError::StorageError(format!("Column family {} not found", CF_CONNECTIONS))
        })?;

        let key = Storage::create_key(CONNECTION_PREFIX, id);
        self.batch.delete_cf(cf, key);
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