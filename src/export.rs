use crate::error::{EngramError, Result};
use crate::schema::{Agent, Collection, Connection, Context, Engram};
use crate::storage::Storage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

/// Struct representing the exported data format
#[derive(Serialize, Deserialize)]
pub struct ExportData {
    /// Version of the export format
    pub version: String,
    
    /// Map of engram IDs to engrams
    pub engrams: HashMap<String, Engram>,
    
    /// Map of connection IDs to connections
    pub connections: HashMap<String, Connection>,
    
    /// Map of collection IDs to collections
    pub collections: HashMap<String, Collection>,
    
    /// Map of agent IDs to agents
    pub agents: HashMap<String, Agent>,
    
    /// Map of context IDs to contexts
    pub contexts: HashMap<String, Context>,
}

impl ExportData {
    /// Create a new, empty export data structure
    pub fn new() -> Self {
        Self {
            version: "1.0".to_string(),
            engrams: HashMap::new(),
            connections: HashMap::new(),
            collections: HashMap::new(),
            agents: HashMap::new(),
            contexts: HashMap::new(),
        }
    }
}

/// Export data from storage to a file
pub fn export_to_file(storage: &Storage, file_path: &Path) -> Result<()> {
    let mut export_data = ExportData::new();
    
    // Read all engrams from storage
    let engram_ids = storage.list_engrams()?;
    for id in engram_ids {
        if let Some(engram) = storage.get_engram(&id)? {
            export_data.engrams.insert(id, engram);
        }
    }
    
    // Read all connections from storage
    let connection_ids = storage.list_connections()?;
    for id in connection_ids {
        if let Some(connection) = storage.get_connection(&id)? {
            export_data.connections.insert(id, connection);
        }
    }
    
    // Read all collections from storage
    let collection_ids = storage.list_collections()?;
    for id in collection_ids {
        if let Some(collection) = storage.get_collection(&id)? {
            export_data.collections.insert(id, collection);
        }
    }
    
    // Read all agents from storage
    let agent_ids = storage.list_agents()?;
    for id in agent_ids {
        if let Some(agent) = storage.get_agent(&id)? {
            export_data.agents.insert(id, agent);
        }
    }
    
    // Read all contexts from storage
    let context_ids = storage.list_contexts()?;
    for id in context_ids {
        if let Some(context) = storage.get_context(&id)? {
            export_data.contexts.insert(id, context);
        }
    }
    
    // Write to file
    let file = File::create(file_path).map_err(|e| {
        EngramError::StorageError(format!("Failed to create export file: {}", e))
    })?;
    
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &export_data).map_err(|e| {
        EngramError::SerializationError(format!("Failed to serialize export data: {}", e))
    })?;
    
    Ok(())
}

/// Import data from a file into storage
pub fn import_from_file(storage: &Storage, file_path: &Path) -> Result<()> {
    // Read from file
    let file = File::open(file_path).map_err(|e| {
        EngramError::StorageError(format!("Failed to open import file: {}", e))
    })?;
    
    let reader = BufReader::new(file);
    let export_data: ExportData = serde_json::from_reader(reader).map_err(|e| {
        EngramError::SerializationError(format!("Failed to deserialize import data: {}", e))
    })?;
    
    // Start a transaction
    let mut transaction = storage.begin_transaction();
    
    // Store all engrams
    for (_, engram) in export_data.engrams {
        transaction.put_engram(&engram)?;
    }
    
    // Store all connections
    for (_, connection) in export_data.connections {
        transaction.put_connection(&connection)?;
    }
    
    // Store all collections
    for (_, collection) in export_data.collections {
        transaction.put_collection(&collection)?;
    }
    
    // Store all agents
    for (_, agent) in export_data.agents {
        transaction.put_agent(&agent)?;
    }
    
    // Store all contexts
    for (_, context) in export_data.contexts {
        transaction.put_context(&context)?;
    }
    
    // Commit the transaction
    transaction.commit()?;
    
    Ok(())
}

/// Export only a subset of data (e.g., a specific collection) to a file
pub fn export_collection_to_file(
    storage: &Storage,
    collection_id: &str,
    file_path: &Path,
) -> Result<()> {
    let mut export_data = ExportData::new();
    
    // Get the collection
    let collection = match storage.get_collection(&collection_id.to_string())? {
        Some(c) => c,
        None => {
            return Err(EngramError::NotFound(format!(
                "Collection not found: {}",
                collection_id
            )))
        }
    };
    
    // Add the collection to export data
    export_data
        .collections
        .insert(collection_id.to_string(), collection.clone());
    
    // Get all engrams in the collection
    for engram_id in &collection.engram_ids {
        if let Some(engram) = storage.get_engram(engram_id)? {
            export_data.engrams.insert(engram_id.clone(), engram);
            
            // Get connections related to this engram
            let connection_ids = storage.find_connections_for_engram(engram_id)?;
            for conn_id in connection_ids {
                if let Some(connection) = storage.get_connection(&conn_id)? {
                    // Only include connections between engrams in this collection
                    if collection.engram_ids.contains(&connection.source_id)
                        && collection.engram_ids.contains(&connection.target_id)
                    {
                        export_data.connections.insert(conn_id, connection);
                    }
                }
            }
        }
    }
    
    // Write to file
    let file = File::create(file_path).map_err(|e| {
        EngramError::StorageError(format!("Failed to create export file: {}", e))
    })?;
    
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &export_data).map_err(|e| {
        EngramError::SerializationError(format!("Failed to serialize export data: {}", e))
    })?;
    
    Ok(())
}

/// Import a partial export (e.g., a specific collection) into storage
pub fn import_partial_from_file(storage: &Storage, file_path: &Path) -> Result<()> {
    // Read from file
    let file = File::open(file_path).map_err(|e| {
        EngramError::StorageError(format!("Failed to open import file: {}", e))
    })?;
    
    let reader = BufReader::new(file);
    let export_data: ExportData = serde_json::from_reader(reader).map_err(|e| {
        EngramError::SerializationError(format!("Failed to deserialize import data: {}", e))
    })?;
    
    // Start a transaction
    let mut transaction = storage.begin_transaction();
    
    // Store all engrams
    for (_, engram) in export_data.engrams {
        transaction.put_engram(&engram)?;
    }
    
    // Store all connections
    for (_, connection) in export_data.connections {
        transaction.put_connection(&connection)?;
    }
    
    // Store all collections
    for (_, collection) in export_data.collections {
        transaction.put_collection(&collection)?;
    }
    
    // Commit the transaction
    transaction.commit()?;
    
    Ok(())
}