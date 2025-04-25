use engram_lite::schema::{Agent, Collection, Connection, Context, Engram, Metadata};
use std::collections::{HashMap, HashSet};

#[test]
fn test_engram_creation() {
    // Create basic engram
    let content = "Test engram content";
    let source = "test_source";
    let confidence = 0.85;
    
    let engram = Engram::new(content.to_string(), source.to_string(), confidence, None);
    
    // Test that values are correctly set
    assert_eq!(engram.content, content);
    assert_eq!(engram.source, source);
    assert_eq!(engram.confidence, confidence);
    assert_eq!(engram.metadata.len(), 0);
    
    // ID should have been generated automatically
    assert!(!engram.id.is_empty());
    
    // Timestamp should be set to current time
    assert!(engram.timestamp <= chrono::Utc::now());
    
    // Create engram with metadata
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    metadata.insert("key2".to_string(), serde_json::Value::Number(serde_json::Number::from(42)));
    
    let engram_with_metadata = Engram::new(
        content.to_string(), 
        source.to_string(), 
        confidence, 
        Some(metadata.clone())
    );
    
    // Test that metadata is correctly set
    assert_eq!(engram_with_metadata.metadata.len(), 2);
    assert_eq!(
        engram_with_metadata.metadata.get("key1").unwrap().as_str().unwrap(),
        "value1"
    );
    assert_eq!(
        engram_with_metadata.metadata.get("key2").unwrap().as_u64().unwrap(),
        42
    );
}

#[test]
fn test_connection_creation() {
    // Create basic connection
    let source_id = "source_engram_id";
    let target_id = "target_engram_id";
    let relationship_type = "supports";
    let weight = 0.75;
    
    let connection = Connection::new(
        source_id.to_string(),
        target_id.to_string(),
        relationship_type.to_string(),
        weight,
        None
    );
    
    // Test that values are correctly set
    assert_eq!(connection.source_id, source_id);
    assert_eq!(connection.target_id, target_id);
    assert_eq!(connection.relationship_type, relationship_type);
    assert_eq!(connection.weight, weight);
    assert_eq!(connection.metadata.len(), 0);
    
    // ID should have been generated automatically
    assert!(!connection.id.is_empty());
    
    // Create connection with metadata
    let mut metadata = HashMap::new();
    metadata.insert("key1".to_string(), serde_json::Value::String("value1".to_string()));
    
    let connection_with_metadata = Connection::new(
        source_id.to_string(),
        target_id.to_string(),
        relationship_type.to_string(),
        weight,
        Some(metadata.clone())
    );
    
    // Test that metadata is correctly set
    assert_eq!(connection_with_metadata.metadata.len(), 1);
    assert_eq!(
        connection_with_metadata.metadata.get("key1").unwrap().as_str().unwrap(),
        "value1"
    );
}

#[test]
fn test_collection_creation_and_manipulation() {
    // Create basic collection
    let name = "Test Collection";
    let description = "A test collection";
    
    let mut collection = Collection::new(name.to_string(), description.to_string(), None);
    
    // Test that values are correctly set
    assert_eq!(collection.name, name);
    assert_eq!(collection.description, description);
    assert_eq!(collection.engram_ids.len(), 0);
    assert_eq!(collection.metadata.len(), 0);
    
    // ID should have been generated automatically
    assert!(!collection.id.is_empty());
    
    // Add engrams to collection
    let engram_id1 = "engram_id_1";
    let engram_id2 = "engram_id_2";
    
    collection.add_engram(engram_id1.to_string());
    collection.add_engram(engram_id2.to_string());
    
    // Verify engrams were added
    assert_eq!(collection.engram_ids.len(), 2);
    assert!(collection.engram_ids.contains(&engram_id1.to_string()));
    assert!(collection.engram_ids.contains(&engram_id2.to_string()));
    
    // Remove an engram
    let removed = collection.remove_engram(&engram_id1.to_string());
    assert!(removed);
    
    // Verify engram was removed
    assert_eq!(collection.engram_ids.len(), 1);
    assert!(!collection.engram_ids.contains(&engram_id1.to_string()));
    assert!(collection.engram_ids.contains(&engram_id2.to_string()));
    
    // Try to remove an engram that doesn't exist
    let removed = collection.remove_engram(&"nonexistent_id".to_string());
    assert!(!removed);
}

#[test]
fn test_agent_creation_and_access_control() {
    // Create basic agent
    let name = "Test Agent";
    let description = "A test agent";
    
    let mut capabilities = HashSet::new();
    capabilities.insert("read".to_string());
    capabilities.insert("write".to_string());
    
    let mut agent = Agent::new(
        name.to_string(),
        description.to_string(),
        Some(capabilities.clone()),
        None
    );
    
    // Test that values are correctly set
    assert_eq!(agent.name, name);
    assert_eq!(agent.description, description);
    assert_eq!(agent.capabilities.len(), 2);
    assert!(agent.capabilities.contains("read"));
    assert!(agent.capabilities.contains("write"));
    assert_eq!(agent.accessible_collections.len(), 0);
    assert_eq!(agent.metadata.len(), 0);
    
    // ID should have been generated automatically
    assert!(!agent.id.is_empty());
    
    // Add access to collections
    let collection_id1 = "collection_id_1";
    let collection_id2 = "collection_id_2";
    
    agent.grant_access(collection_id1.to_string());
    agent.grant_access(collection_id2.to_string());
    
    // Verify access was granted
    assert_eq!(agent.accessible_collections.len(), 2);
    assert!(agent.has_access(&collection_id1.to_string()));
    assert!(agent.has_access(&collection_id2.to_string()));
    
    // Revoke access
    let revoked = agent.revoke_access(&collection_id1.to_string());
    assert!(revoked);
    
    // Verify access was revoked
    assert_eq!(agent.accessible_collections.len(), 1);
    assert!(!agent.has_access(&collection_id1.to_string()));
    assert!(agent.has_access(&collection_id2.to_string()));
    
    // Try to revoke access for a collection that doesn't exist
    let revoked = agent.revoke_access(&"nonexistent_id".to_string());
    assert!(!revoked);
}

#[test]
fn test_context_creation_and_manipulation() {
    // Create basic context
    let name = "Test Context";
    let description = "A test context";
    
    let mut context = Context::new(name.to_string(), description.to_string(), None);
    
    // Test that values are correctly set
    assert_eq!(context.name, name);
    assert_eq!(context.description, description);
    assert_eq!(context.engram_ids.len(), 0);
    assert_eq!(context.agent_ids.len(), 0);
    assert_eq!(context.metadata.len(), 0);
    
    // ID should have been generated automatically
    assert!(!context.id.is_empty());
    
    // Add engrams to context
    let engram_id1 = "engram_id_1";
    let engram_id2 = "engram_id_2";
    
    context.add_engram(engram_id1.to_string());
    context.add_engram(engram_id2.to_string());
    
    // Verify engrams were added
    assert_eq!(context.engram_ids.len(), 2);
    assert!(context.engram_ids.contains(&engram_id1.to_string()));
    assert!(context.engram_ids.contains(&engram_id2.to_string()));
    
    // Add agents to context
    let agent_id1 = "agent_id_1";
    let agent_id2 = "agent_id_2";
    
    context.add_agent(agent_id1.to_string());
    context.add_agent(agent_id2.to_string());
    
    // Verify agents were added
    assert_eq!(context.agent_ids.len(), 2);
    assert!(context.agent_ids.contains(&agent_id1.to_string()));
    assert!(context.agent_ids.contains(&agent_id2.to_string()));
    
    // Remove an engram
    let removed = context.remove_engram(&engram_id1.to_string());
    assert!(removed);
    
    // Verify engram was removed
    assert_eq!(context.engram_ids.len(), 1);
    assert!(!context.engram_ids.contains(&engram_id1.to_string()));
    assert!(context.engram_ids.contains(&engram_id2.to_string()));
    
    // Remove an agent
    let removed = context.remove_agent(&agent_id1.to_string());
    assert!(removed);
    
    // Verify agent was removed
    assert_eq!(context.agent_ids.len(), 1);
    assert!(!context.agent_ids.contains(&agent_id1.to_string()));
    assert!(context.agent_ids.contains(&agent_id2.to_string()));
}