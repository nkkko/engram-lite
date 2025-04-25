#[cfg(test)]
mod tests {
    use crate::graph::MemoryGraph;
    use crate::schema::{Engram, Connection, Collection, Agent, Context};
    use crate::index::SearchIndex;
    use crate::query::{QueryService, EngramQuery, RelationshipQuery};
    use std::collections::HashMap;

    /// Helper function to create test engrams
    fn create_test_engrams() -> Vec<Engram> {
        vec![
            Engram::new(
                "The capital of France is Paris".to_string(),
                "geography".to_string(),
                0.95,
                Some(HashMap::from([("category".to_string(), serde_json::json!("cities"))])),
            ),
            Engram::new(
                "Paris is known for the Eiffel Tower".to_string(),
                "landmarks".to_string(),
                0.9,
                Some(HashMap::from([("category".to_string(), serde_json::json!("monuments"))])),
            ),
            Engram::new(
                "The Eiffel Tower was built in 1889".to_string(),
                "history".to_string(),
                0.85,
                Some(HashMap::from([("category".to_string(), serde_json::json!("monuments"))])),
            ),
            Engram::new(
                "France is a country in Western Europe".to_string(),
                "geography".to_string(),
                0.98,
                Some(HashMap::from([("category".to_string(), serde_json::json!("countries"))])),
            ),
            Engram::new(
                "The European Union was formed in 1993".to_string(),
                "history".to_string(),
                0.80,
                Some(HashMap::from([("category".to_string(), serde_json::json!("organizations"))])),
            ),
        ]
    }

    #[test]
    fn test_memory_graph_basic() {
        let mut graph = MemoryGraph::new();
        
        // Create test engrams
        let engrams = create_test_engrams();
        
        // Add engrams to graph
        for engram in &engrams {
            graph.add_engram(engram.clone()).unwrap();
        }
        
        // Verify we can retrieve engrams
        for engram in &engrams {
            let retrieved = graph.get_engram(&engram.id).unwrap().unwrap();
            assert_eq!(retrieved.id, engram.id);
            assert_eq!(retrieved.content, engram.content);
        }
        
        // Create connections
        let connection1 = Connection::new(
            engrams[0].id.clone(), // France -> Paris
            engrams[1].id.clone(),
            "contains".to_string(),
            0.9,
            None,
        );
        
        let connection2 = Connection::new(
            engrams[1].id.clone(), // Paris -> Eiffel Tower
            engrams[2].id.clone(),
            "has_landmark".to_string(),
            0.85,
            None,
        );
        
        let connection3 = Connection::new(
            engrams[3].id.clone(), // France -> EU
            engrams[4].id.clone(),
            "is_member_of".to_string(),
            0.9,
            None,
        );
        
        // Add connections to graph
        graph.add_connection(connection1.clone()).unwrap();
        graph.add_connection(connection2.clone()).unwrap();
        graph.add_connection(connection3.clone()).unwrap();
        
        // Verify we can retrieve connections
        let retrieved1 = graph.get_connection(&connection1.id).unwrap().unwrap();
        assert_eq!(retrieved1.id, connection1.id);
        assert_eq!(retrieved1.source_id, connection1.source_id);
        assert_eq!(retrieved1.target_id, connection1.target_id);
        
        // Get connections between France and Paris
        let connections = graph.get_connections_between(&engrams[0].id, &engrams[1].id).unwrap();
        assert_eq!(connections.len(), 1);
        assert_eq!(connections[0].id, connection1.id);
        
        // Test source-based retrieval
        let geography_engrams = graph.get_engrams_by_source("geography").unwrap();
        assert_eq!(geography_engrams.len(), 2);
        
        // Test confidence-based retrieval
        let high_confidence_engrams = graph.get_engrams_by_confidence(0.9).unwrap();
        assert_eq!(high_confidence_engrams.len(), 3);
    }
    
    #[test]
    fn test_graph_collections_and_contexts() {
        let mut graph = MemoryGraph::new();
        
        // Create test engrams
        let engrams = create_test_engrams();
        
        // Add engrams to graph
        for engram in &engrams {
            graph.add_engram(engram.clone()).unwrap();
        }
        
        // Create a collection
        let mut collection = Collection::new(
            "Geography Collection".to_string(),
            "Collection of geography-related engrams".to_string(),
            None,
        );
        
        // Add engrams to collection
        collection.add_engram(engrams[0].id.clone()); // France
        collection.add_engram(engrams[3].id.clone()); // Paris
        
        // Add collection to graph
        graph.add_collection(collection.clone()).unwrap();
        
        // Create an agent
        let mut agent = Agent::new(
            "Geography Bot".to_string(),
            "Agent specialized in geography".to_string(),
            Some(["query".to_string(), "update".to_string()].into()),
            None,
        );
        
        // Give agent access to collection
        agent.grant_access(collection.id.clone());
        
        // Add agent to graph
        graph.add_agent(agent.clone()).unwrap();
        
        // Create a context
        let mut context = Context::new(
            "Europe Study".to_string(),
            "Context for studying European geography".to_string(),
            None,
        );
        
        // Add engrams to context
        context.add_engram(engrams[0].id.clone()); // France
        context.add_engram(engrams[3].id.clone()); // Paris
        context.add_engram(engrams[4].id.clone()); // EU
        
        // Add agent to context
        context.add_agent(agent.id.clone());
        
        // Add context to graph
        graph.add_context(context.clone()).unwrap();
        
        // Verify we can retrieve collection
        let retrieved_collection = graph.get_collection(&collection.id).unwrap().unwrap();
        assert_eq!(retrieved_collection.id, collection.id);
        assert_eq!(retrieved_collection.name, collection.name);
        assert_eq!(retrieved_collection.engram_ids.len(), 2);
        
        // Verify we can retrieve agent
        let retrieved_agent = graph.get_agent(&agent.id).unwrap().unwrap();
        assert_eq!(retrieved_agent.id, agent.id);
        assert_eq!(retrieved_agent.name, agent.name);
        assert_eq!(retrieved_agent.accessible_collections.len(), 1);
        
        // Verify we can retrieve context
        let retrieved_context = graph.get_context(&context.id).unwrap().unwrap();
        assert_eq!(retrieved_context.id, context.id);
        assert_eq!(retrieved_context.name, context.name);
        assert_eq!(retrieved_context.engram_ids.len(), 3);
        assert_eq!(retrieved_context.agent_ids.len(), 1);
        
        // Test getting context engrams
        let context_engrams = graph.get_context_engrams(&context.id).unwrap();
        assert_eq!(context_engrams.len(), 3);
        
        // Test getting agents in context
        let context_agents = graph.get_agents_in_context(&context.id).unwrap();
        assert_eq!(context_agents.len(), 1);
        assert_eq!(context_agents[0].id, agent.id);
        
        // Test getting agent accessible engrams
        let agent_engrams = graph.get_agent_accessible_engrams(&agent.id).unwrap();
        assert_eq!(agent_engrams.len(), 2);
    }
    
    #[test]
    fn test_search_index() {
        // Create a search index
        let mut index = SearchIndex::new();
        
        // Create test engrams
        let engrams = create_test_engrams();
        
        // Add engrams to index
        for engram in &engrams {
            index.add_engram(engram).unwrap();
        }
        
        // Create connections
        let connection1 = Connection::new(
            engrams[0].id.clone(), // France -> Paris
            engrams[1].id.clone(),
            "contains".to_string(),
            0.9,
            None,
        );
        
        let connection2 = Connection::new(
            engrams[1].id.clone(), // Paris -> Eiffel Tower
            engrams[2].id.clone(),
            "has_landmark".to_string(),
            0.85,
            None,
        );
        
        let connection3 = Connection::new(
            engrams[3].id.clone(), // France -> EU
            engrams[4].id.clone(),
            "is_member_of".to_string(),
            0.9,
            None,
        );
        
        // Add connections to index
        index.add_connection(&connection1).unwrap();
        index.add_connection(&connection2).unwrap();
        index.add_connection(&connection3).unwrap();
        
        // Test text search
        let paris_results = index.text_index.search("Paris");
        assert_eq!(paris_results.len(), 2); // Should match both engrams mentioning Paris
        
        let tower_results = index.text_index.search("Tower");
        assert_eq!(tower_results.len(), 2); // Should match both engrams mentioning Eiffel Tower
        
        // Test exact match search
        let france_europe_results = index.text_index.search_all("France Europe");
        assert_eq!(france_europe_results.len(), 1); // Should only match the engram containing both terms
        
        // Test source search
        let geography_results = index.find_by_source("geography");
        assert_eq!(geography_results.len(), 2);
        
        // Test confidence search
        let high_confidence_results = index.find_by_min_confidence(0.9);
        assert_eq!(high_confidence_results.len(), 3);
        
        // Test metadata search
        let monuments_results = index.metadata_index.find_by_key_value("category", "monuments");
        assert_eq!(monuments_results.len(), 2);
        
        // Test combined search
        let combined_results = index.search_combined(
            Some("Paris"),
            Some("landmarks"),
            None,
            Some("category"),
            Some("monuments"),
            false,
        );
        assert_eq!(combined_results.len(), 1); // Should match only the Paris landmarks engram
        
        // Test relationship traversal
        let targets = index.relationship_index.get_targets(&engrams[0].id);
        assert_eq!(targets.len(), 1);
        assert!(targets.contains(&engrams[1].id));
        
        let sources = index.relationship_index.get_sources(&engrams[1].id);
        assert_eq!(sources.len(), 1);
        assert!(sources.contains(&engrams[0].id));
        
        // Test path finding
        let paths = index.relationship_index.find_paths(&engrams[0].id, &engrams[2].id, 3);
        assert_eq!(paths.len(), 1); // Should find one path from France to Eiffel Tower
        assert_eq!(paths[0].len(), 3); // The path should have 3 nodes: France -> Paris -> Eiffel Tower
    }
    
    #[test]
    fn test_query_service() {
        // Create test engrams
        let engrams = create_test_engrams();
        
        // Create a memory graph and add engrams
        let mut graph = MemoryGraph::new();
        for engram in &engrams {
            graph.add_engram(engram.clone()).unwrap();
        }
        
        // Create connections
        let connection1 = Connection::new(
            engrams[0].id.clone(), // France -> Paris
            engrams[1].id.clone(),
            "contains".to_string(),
            0.9,
            None,
        );
        
        let connection2 = Connection::new(
            engrams[1].id.clone(), // Paris -> Eiffel Tower
            engrams[2].id.clone(),
            "has_landmark".to_string(),
            0.85,
            None,
        );
        
        let connection3 = Connection::new(
            engrams[3].id.clone(), // France -> EU
            engrams[4].id.clone(),
            "is_member_of".to_string(),
            0.9,
            None,
        );
        
        // Add connections to graph
        graph.add_connection(connection1.clone()).unwrap();
        graph.add_connection(connection2.clone()).unwrap();
        graph.add_connection(connection3.clone()).unwrap();
        
        // Create a search index
        let mut index = SearchIndex::new();
        
        // Add engrams to index
        for engram in &engrams {
            index.add_engram(engram).unwrap();
        }
        
        // Add connections to index
        index.add_connection(&connection1).unwrap();
        index.add_connection(&connection2).unwrap();
        index.add_connection(&connection3).unwrap();
        
        // Currently our unit tests can't use RocksDB storage
        // In a real integration test, we would use a temporary RocksDB instance
        // For now, we'll test with mock storage
        
        // Create a query service with the search index
        // In a real test, we would use QueryService::new(storage, &index)
        // TODO: Create a mock storage implementation for testing
    }
}