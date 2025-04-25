use engram_lite::graph::MemoryGraph;
use engram_lite::schema::{Agent, Collection, Connection, Context, Engram};
use std::collections::HashSet;

// Helper function to create test engrams
fn create_test_engrams() -> (Engram, Engram, Engram) {
    let engram1 = Engram::new(
        "Test engram 1 content".to_string(),
        "source1".to_string(),
        0.9,
        None,
    );

    let engram2 = Engram::new(
        "Test engram 2 content".to_string(),
        "source2".to_string(),
        0.8,
        None,
    );

    let engram3 = Engram::new(
        "Test engram 3 content".to_string(),
        "source1".to_string(), // Same source as engram1
        0.7,
        None,
    );

    (engram1, engram2, engram3)
}

#[test]
fn test_graph_basic_operations() {
    let mut graph = MemoryGraph::new();
    let (engram1, engram2, engram3) = create_test_engrams();

    // Add engrams
    let id1 = graph.add_engram(engram1.clone()).expect("Failed to add engram1");
    let id2 = graph.add_engram(engram2.clone()).expect("Failed to add engram2");
    let id3 = graph.add_engram(engram3.clone()).expect("Failed to add engram3");

    // Verify IDs match
    assert_eq!(id1, engram1.id);
    assert_eq!(id2, engram2.id);
    assert_eq!(id3, engram3.id);

    // Test getting engram by ID
    let retrieved_engram = graph.get_engram(&id1).expect("Failed to get engram");
    assert!(retrieved_engram.is_some());
    if let Some(e) = retrieved_engram {
        assert_eq!(e.id, engram1.id);
        assert_eq!(e.content, engram1.content);
    }

    // Test connections between engrams
    let connection = Connection::new(
        id1.clone(),
        id2.clone(),
        "supports".to_string(),
        0.75,
        None,
    );
    let connection_id = graph.add_connection(connection.clone()).expect("Failed to add connection");
    assert_eq!(connection_id, connection.id);

    // Test getting connections
    let connections = graph.get_connections_for_engram(&id1).expect("Failed to get connections");
    assert_eq!(connections.len(), 1);
    assert_eq!(connections[0].id, connection.id);

    // Test getting connections by type
    let typed_connections = graph
        .get_connections_by_type("supports".to_string())
        .expect("Failed to get connections by type");
    assert_eq!(typed_connections.len(), 1);
    assert_eq!(typed_connections[0].id, connection.id);

    // Test getting engrams by source
    let source_engrams = graph
        .get_engrams_by_source("source1")
        .expect("Failed to get engrams by source");
    assert_eq!(source_engrams.len(), 2);
    assert!(source_engrams.iter().any(|e| e.id == id1));
    assert!(source_engrams.iter().any(|e| e.id == id3));

    // Test getting engrams by confidence
    let high_confidence_engrams = graph
        .get_engrams_by_confidence(0.8)
        .expect("Failed to get engrams by confidence");
    assert_eq!(high_confidence_engrams.len(), 2);
    assert!(high_confidence_engrams.iter().any(|e| e.id == id1));
    assert!(high_confidence_engrams.iter().any(|e| e.id == id2));

    // Test getting recent engrams
    let recent_engrams = graph.get_recent_engrams(2).expect("Failed to get recent engrams");
    assert_eq!(recent_engrams.len(), 2);
}

#[test]
fn test_graph_collection_operations() {
    let mut graph = MemoryGraph::new();
    let (engram1, engram2, _) = create_test_engrams();

    // Add engrams
    let id1 = graph.add_engram(engram1.clone()).expect("Failed to add engram1");
    let id2 = graph.add_engram(engram2.clone()).expect("Failed to add engram2");

    // Create a collection
    let mut collection = Collection::new(
        "Test Collection".to_string(),
        "A test collection".to_string(),
        None,
    );
    let collection_id = collection.id.clone();

    // Add collection to graph
    graph.add_collection(collection.clone()).expect("Failed to add collection");

    // Add engrams to collection
    graph
        .add_engram_to_collection(&id1, &collection_id)
        .expect("Failed to add engram to collection");
    graph
        .add_engram_to_collection(&id2, &collection_id)
        .expect("Failed to add engram to collection");

    // Test getting collection
    let retrieved_collection = graph.get_collection(&collection_id).expect("Failed to get collection");
    assert!(retrieved_collection.is_some());
    if let Some(c) = retrieved_collection {
        assert_eq!(c.id, collection_id);
        assert_eq!(c.engram_ids.len(), 2);
    }

    // Test getting engrams in collection
    let collection_engrams = graph
        .get_engrams_in_collection(&collection_id)
        .expect("Failed to get engrams in collection");
    assert_eq!(collection_engrams.len(), 2);
    assert!(collection_engrams.iter().any(|e| e.id == id1));
    assert!(collection_engrams.iter().any(|e| e.id == id2));
}

#[test]
fn test_graph_agent_operations() {
    let mut graph = MemoryGraph::new();
    
    // Create a collection
    let collection = Collection::new(
        "Test Collection".to_string(),
        "A test collection".to_string(),
        None,
    );
    let collection_id = collection.id.clone();
    
    // Add collection to graph
    graph.add_collection(collection.clone()).expect("Failed to add collection");
    
    // Create an agent
    let mut capabilities = HashSet::new();
    capabilities.insert("read".to_string());
    capabilities.insert("write".to_string());
    
    let mut agent = Agent::new(
        "Test Agent".to_string(),
        "A test agent".to_string(),
        Some(capabilities),
        None,
    );
    
    // Grant collection access
    agent.grant_access(collection_id.clone());
    let agent_id = agent.id.clone();
    
    // Add agent to graph
    graph.add_agent(agent.clone()).expect("Failed to add agent");
    
    // Test getting agent
    let retrieved_agent = graph.get_agent(&agent_id).expect("Failed to get agent");
    assert!(retrieved_agent.is_some());
    if let Some(a) = retrieved_agent {
        assert_eq!(a.id, agent_id);
        assert_eq!(a.accessible_collections.len(), 1);
        assert!(a.accessible_collections.contains(&collection_id));
    }
    
    // Test getting collections accessible to agent
    let accessible_collections = graph
        .get_accessible_collections(&agent_id)
        .expect("Failed to get accessible collections");
    assert_eq!(accessible_collections.len(), 1);
    assert_eq!(accessible_collections[0].id, collection_id);
}

#[test]
fn test_graph_context_operations() {
    let mut graph = MemoryGraph::new();
    let (engram1, engram2, _) = create_test_engrams();
    
    // Add engrams
    let id1 = graph.add_engram(engram1.clone()).expect("Failed to add engram1");
    let id2 = graph.add_engram(engram2.clone()).expect("Failed to add engram2");
    
    // Create an agent
    let agent = Agent::new(
        "Test Agent".to_string(),
        "A test agent".to_string(),
        None,
        None,
    );
    let agent_id = agent.id.clone();
    
    // Add agent to graph
    graph.add_agent(agent.clone()).expect("Failed to add agent");
    
    // Create a context
    let mut context = Context::new(
        "Test Context".to_string(),
        "A test context".to_string(),
        None,
    );
    
    // Add engrams and agent to context
    context.add_engram(id1.clone());
    context.add_engram(id2.clone());
    context.add_agent(agent_id.clone());
    let context_id = context.id.clone();
    
    // Add context to graph
    graph.add_context(context.clone()).expect("Failed to add context");
    
    // Test getting context
    let retrieved_context = graph.get_context(&context_id).expect("Failed to get context");
    assert!(retrieved_context.is_some());
    if let Some(c) = retrieved_context {
        assert_eq!(c.id, context_id);
        assert_eq!(c.engram_ids.len(), 2);
        assert_eq!(c.agent_ids.len(), 1);
    }
    
    // Test getting engrams in context
    let context_engrams = graph
        .get_engrams_in_context(&context_id)
        .expect("Failed to get engrams in context");
    assert_eq!(context_engrams.len(), 2);
    assert!(context_engrams.iter().any(|e| e.id == id1));
    assert!(context_engrams.iter().any(|e| e.id == id2));
    
    // Test getting contexts for agent
    let agent_contexts = graph
        .get_contexts_for_agent(&agent_id)
        .expect("Failed to get contexts for agent");
    assert_eq!(agent_contexts.len(), 1);
    assert_eq!(agent_contexts[0].id, context_id);
}

#[test]
fn test_path_finding_between_engrams() {
    let mut graph = MemoryGraph::new();
    let (engram1, engram2, engram3) = create_test_engrams();
    
    // Add engrams
    let id1 = graph.add_engram(engram1.clone()).expect("Failed to add engram1");
    let id2 = graph.add_engram(engram2.clone()).expect("Failed to add engram2");
    let id3 = graph.add_engram(engram3.clone()).expect("Failed to add engram3");
    
    // Create connections between engrams
    let connection1 = Connection::new(
        id1.clone(),
        id2.clone(),
        "causes".to_string(),
        0.9,
        None,
    );
    
    let connection2 = Connection::new(
        id2.clone(),
        id3.clone(),
        "supports".to_string(),
        0.8,
        None,
    );
    
    // Add connections
    graph.add_connection(connection1.clone()).expect("Failed to add connection1");
    graph.add_connection(connection2.clone()).expect("Failed to add connection2");
    
    // Test finding path between engrams
    let path = graph
        .find_path_between_engrams(&id1, &id3)
        .expect("Failed to find path between engrams");
    
    // Path should include both connections
    assert_eq!(path.len(), 2);
    assert_eq!(path[0].source_id, id1);
    assert_eq!(path[0].target_id, id2);
    assert_eq!(path[1].source_id, id2);
    assert_eq!(path[1].target_id, id3);
}