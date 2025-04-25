use engram_lite::schema::{Agent, Collection, Connection, Context, Engram};
use engram_lite::storage::Storage;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;

// Helper to create a unique test directory to avoid conflicts
fn get_test_db_path() -> String {
    let test_id = Uuid::new_v4().to_string();
    format!("./test_db_{}", test_id)
}

// Helper to clean up test directory
fn cleanup_test_db(path: &str) {
    if Path::new(path).exists() {
        let _ = fs::remove_dir_all(path);
    }
}

#[test]
fn test_storage_initialization() {
    let db_path = get_test_db_path();
    
    // Test creating storage
    let storage_result = Storage::new(&db_path);
    assert!(storage_result.is_ok());
    
    // Test creating storage a second time (should reuse)
    let storage_reuse_result = Storage::new(&db_path);
    assert!(storage_reuse_result.is_ok());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_engram_storage() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create engram for testing
    let content = "Test engram content";
    let source = "test_source";
    let confidence = 0.85;
    let engram = Engram::new(content.to_string(), source.to_string(), confidence, None);
    let engram_id = engram.id.clone();
    
    // Test put engram
    storage.put_engram(&engram).expect("Failed to put engram");
    
    // Test get engram
    let retrieved_engram = storage.get_engram(&engram_id).expect("Failed to get engram");
    assert!(retrieved_engram.is_some());
    
    if let Some(retrieved) = retrieved_engram {
        assert_eq!(retrieved.id, engram.id);
        assert_eq!(retrieved.content, engram.content);
        assert_eq!(retrieved.source, engram.source);
        assert_eq!(retrieved.confidence, engram.confidence);
    }
    
    // Test listing engrams
    let engram_list = storage.list_engrams().expect("Failed to list engrams");
    assert!(engram_list.contains(&engram_id));
    
    // Test delete engram
    storage.delete_engram(&engram_id).expect("Failed to delete engram");
    
    // Verify engram is gone
    let deleted_engram = storage.get_engram(&engram_id).expect("Failed to get deleted engram");
    assert!(deleted_engram.is_none());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_connection_storage() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create engrams for connection
    let engram1 = Engram::new("Content 1".to_string(), "source1".to_string(), 0.9, None);
    let engram2 = Engram::new("Content 2".to_string(), "source2".to_string(), 0.8, None);
    
    // Store the engrams
    storage.put_engram(&engram1).expect("Failed to put engram1");
    storage.put_engram(&engram2).expect("Failed to put engram2");
    
    // Create connection
    let connection = Connection::new(
        engram1.id.clone(),
        engram2.id.clone(),
        "supports".to_string(),
        0.75,
        None
    );
    let conn_id = connection.id.clone();
    
    // Test put connection
    storage.put_connection(&connection).expect("Failed to put connection");
    
    // Test get connection
    let retrieved_conn = storage.get_connection(&conn_id).expect("Failed to get connection");
    assert!(retrieved_conn.is_some());
    
    if let Some(retrieved) = retrieved_conn {
        assert_eq!(retrieved.id, connection.id);
        assert_eq!(retrieved.source_id, engram1.id);
        assert_eq!(retrieved.target_id, engram2.id);
        assert_eq!(retrieved.relationship_type, "supports");
        assert_eq!(retrieved.weight, 0.75);
    }
    
    // Test find connections for engram
    let source_conns = storage.find_connections_for_engram(&engram1.id).expect("Failed to find connections");
    assert!(source_conns.contains(&conn_id));
    
    // Test delete connection
    storage.delete_connection(&conn_id).expect("Failed to delete connection");
    
    // Verify connection is gone
    let deleted_conn = storage.get_connection(&conn_id).expect("Failed to get deleted connection");
    assert!(deleted_conn.is_none());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_collection_storage() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create a collection
    let mut collection = Collection::new(
        "Test Collection".to_string(),
        "A test collection".to_string(),
        None
    );
    
    // Create some engrams to add to the collection
    let engram1 = Engram::new("Content 1".to_string(), "source1".to_string(), 0.9, None);
    let engram2 = Engram::new("Content 2".to_string(), "source2".to_string(), 0.8, None);
    
    // Store the engrams
    storage.put_engram(&engram1).expect("Failed to put engram1");
    storage.put_engram(&engram2).expect("Failed to put engram2");
    
    // Add engrams to collection
    collection.add_engram(engram1.id.clone());
    collection.add_engram(engram2.id.clone());
    
    let collection_id = collection.id.clone();
    
    // Test put collection
    storage.put_collection(&collection).expect("Failed to put collection");
    
    // Test get collection
    let retrieved_coll = storage.get_collection(&collection_id).expect("Failed to get collection");
    assert!(retrieved_coll.is_some());
    
    if let Some(retrieved) = retrieved_coll {
        assert_eq!(retrieved.id, collection.id);
        assert_eq!(retrieved.name, "Test Collection");
        assert_eq!(retrieved.description, "A test collection");
        assert_eq!(retrieved.engram_ids.len(), 2);
        assert!(retrieved.engram_ids.contains(&engram1.id));
        assert!(retrieved.engram_ids.contains(&engram2.id));
    }
    
    // Test listing collections
    let collection_list = storage.list_collections().expect("Failed to list collections");
    assert!(collection_list.contains(&collection_id));
    
    // Test delete collection
    storage.delete_collection(&collection_id).expect("Failed to delete collection");
    
    // Verify collection is gone
    let deleted_coll = storage.get_collection(&collection_id).expect("Failed to get deleted collection");
    assert!(deleted_coll.is_none());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_agent_storage() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create capabilities
    let mut capabilities = HashSet::new();
    capabilities.insert("read".to_string());
    capabilities.insert("write".to_string());
    
    // Create an agent
    let mut agent = Agent::new(
        "Test Agent".to_string(),
        "A test agent".to_string(),
        Some(capabilities),
        None
    );
    
    // Create a collection to grant access to
    let collection = Collection::new(
        "Test Collection".to_string(),
        "A test collection".to_string(),
        None
    );
    
    // Store the collection
    storage.put_collection(&collection).expect("Failed to put collection");
    
    // Grant access to the collection
    agent.grant_access(collection.id.clone());
    
    let agent_id = agent.id.clone();
    
    // Test put agent
    storage.put_agent(&agent).expect("Failed to put agent");
    
    // Test get agent
    let retrieved_agent = storage.get_agent(&agent_id).expect("Failed to get agent");
    assert!(retrieved_agent.is_some());
    
    if let Some(retrieved) = retrieved_agent {
        assert_eq!(retrieved.id, agent.id);
        assert_eq!(retrieved.name, "Test Agent");
        assert_eq!(retrieved.description, "A test agent");
        assert_eq!(retrieved.capabilities.len(), 2);
        assert!(retrieved.capabilities.contains("read"));
        assert!(retrieved.capabilities.contains("write"));
        assert_eq!(retrieved.accessible_collections.len(), 1);
        assert!(retrieved.accessible_collections.contains(&collection.id));
    }
    
    // Test listing agents
    let agent_list = storage.list_agents().expect("Failed to list agents");
    assert!(agent_list.contains(&agent_id));
    
    // Test delete agent
    storage.delete_agent(&agent_id).expect("Failed to delete agent");
    
    // Verify agent is gone
    let deleted_agent = storage.get_agent(&agent_id).expect("Failed to get deleted agent");
    assert!(deleted_agent.is_none());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_context_storage() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create a context
    let mut context = Context::new(
        "Test Context".to_string(),
        "A test context".to_string(),
        None
    );
    
    // Create engrams and agents
    let engram = Engram::new("Content".to_string(), "source".to_string(), 0.9, None);
    let agent = Agent::new(
        "Test Agent".to_string(),
        "A test agent".to_string(),
        None,
        None
    );
    
    // Store the engram and agent
    storage.put_engram(&engram).expect("Failed to put engram");
    storage.put_agent(&agent).expect("Failed to put agent");
    
    // Add engram and agent to context
    context.add_engram(engram.id.clone());
    context.add_agent(agent.id.clone());
    
    let context_id = context.id.clone();
    
    // Test put context
    storage.put_context(&context).expect("Failed to put context");
    
    // Test get context
    let retrieved_ctx = storage.get_context(&context_id).expect("Failed to get context");
    assert!(retrieved_ctx.is_some());
    
    if let Some(retrieved) = retrieved_ctx {
        assert_eq!(retrieved.id, context.id);
        assert_eq!(retrieved.name, "Test Context");
        assert_eq!(retrieved.description, "A test context");
        assert_eq!(retrieved.engram_ids.len(), 1);
        assert!(retrieved.engram_ids.contains(&engram.id));
        assert_eq!(retrieved.agent_ids.len(), 1);
        assert!(retrieved.agent_ids.contains(&agent.id));
    }
    
    // Test listing contexts
    let context_list = storage.list_contexts().expect("Failed to list contexts");
    assert!(context_list.contains(&context_id));
    
    // Test delete context
    storage.delete_context(&context_id).expect("Failed to delete context");
    
    // Verify context is gone
    let deleted_ctx = storage.get_context(&context_id).expect("Failed to get deleted context");
    assert!(deleted_ctx.is_none());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_transaction() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Create engrams for testing
    let engram1 = Engram::new("Content 1".to_string(), "source1".to_string(), 0.9, None);
    let engram2 = Engram::new("Content 2".to_string(), "source2".to_string(), 0.8, None);
    let engram1_id = engram1.id.clone();
    let engram2_id = engram2.id.clone();
    
    // Begin a transaction
    let mut txn = storage.begin_transaction();
    
    // Add engrams in the transaction
    txn.put_engram(&engram1).expect("Failed to put engram1 in transaction");
    txn.put_engram(&engram2).expect("Failed to put engram2 in transaction");
    
    // Can't see engrams until transaction is committed
    assert!(storage.get_engram(&engram1_id).expect("Failed to get engram1").is_none());
    assert!(storage.get_engram(&engram2_id).expect("Failed to get engram2").is_none());
    
    // Commit the transaction
    txn.commit().expect("Failed to commit transaction");
    
    // Now engrams should be visible
    assert!(storage.get_engram(&engram1_id).expect("Failed to get engram1").is_some());
    assert!(storage.get_engram(&engram2_id).expect("Failed to get engram2").is_some());
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}

#[test]
fn test_storage_stats() {
    let db_path = get_test_db_path();
    let storage = Storage::new(&db_path).expect("Failed to create storage");
    
    // Initial stats should be empty
    let initial_stats = storage.get_stats().expect("Failed to get stats");
    assert_eq!(initial_stats.engram_count, 0);
    assert_eq!(initial_stats.connection_count, 0);
    assert_eq!(initial_stats.collection_count, 0);
    assert_eq!(initial_stats.agent_count, 0);
    assert_eq!(initial_stats.context_count, 0);
    
    // Add some items
    let engram = Engram::new("Content".to_string(), "source".to_string(), 0.9, None);
    let collection = Collection::new("Test Collection".to_string(), "A test collection".to_string(), None);
    let agent = Agent::new("Test Agent".to_string(), "A test agent".to_string(), None, None);
    let context = Context::new("Test Context".to_string(), "A test context".to_string(), None);
    
    storage.put_engram(&engram).expect("Failed to put engram");
    storage.put_collection(&collection).expect("Failed to put collection");
    storage.put_agent(&agent).expect("Failed to put agent");
    storage.put_context(&context).expect("Failed to put context");
    
    // Create a connection
    let engram2 = Engram::new("Content 2".to_string(), "source2".to_string(), 0.8, None);
    storage.put_engram(&engram2).expect("Failed to put engram2");
    
    let connection = Connection::new(
        engram.id.clone(),
        engram2.id.clone(),
        "supports".to_string(),
        0.75,
        None
    );
    storage.put_connection(&connection).expect("Failed to put connection");
    
    // Get updated stats
    let updated_stats = storage.get_stats().expect("Failed to get stats");
    assert_eq!(updated_stats.engram_count, 2);
    assert_eq!(updated_stats.connection_count, 1);
    assert_eq!(updated_stats.collection_count, 1);
    assert_eq!(updated_stats.agent_count, 1);
    assert_eq!(updated_stats.context_count, 1);
    
    // Clean up test directory
    cleanup_test_db(&db_path);
}