#[cfg(test)]
mod tests {
    use crate::index::{RelationshipIndex, MetadataIndex, SearchIndex};
    use crate::schema::{Connection, Engram};
    
    use serde_json::json;

    fn create_test_engram(id: &str, content: &str, source: &str, confidence: f64) -> Engram {
        let mut engram = Engram::new(
            content.to_string(),
            source.to_string(),
            confidence,
            None,
        );
        // Override the UUID with a fixed ID for testing
        engram.id = id.to_string();
        engram
    }

    fn create_test_connection(
        id: &str,
        source_id: &str,
        target_id: &str,
        relationship_type: &str,
        weight: f64,
    ) -> Connection {
        let mut connection = Connection::new(
            source_id.to_string(),
            target_id.to_string(),
            relationship_type.to_string(),
            weight,
            None,
        );
        // Override the UUID with a fixed ID for testing
        connection.id = id.to_string();
        connection
    }

    #[test]
    fn test_relationship_index() {
        let mut index = RelationshipIndex::new();
        
        // Create test connections
        let conn1 = create_test_connection("conn1", "e1", "e2", "causes", 0.8);
        let conn2 = create_test_connection("conn2", "e1", "e3", "supports", 0.9);
        let conn3 = create_test_connection("conn3", "e2", "e4", "causes", 0.7);
        
        // Add connections to index
        index.add_connection(&conn1).unwrap();
        index.add_connection(&conn2).unwrap();
        index.add_connection(&conn3).unwrap();
        
        // Test outgoing connections
        let outgoing_e1 = index.get_outgoing_connections(&"e1".to_string());
        assert_eq!(outgoing_e1.len(), 2);
        assert!(outgoing_e1.contains(&"conn1".to_string()));
        assert!(outgoing_e1.contains(&"conn2".to_string()));
        
        // Test incoming connections
        let incoming_e3 = index.get_incoming_connections(&"e3".to_string());
        assert_eq!(incoming_e3.len(), 1);
        assert!(incoming_e3.contains(&"conn2".to_string()));
        
        // Test by relationship type
        let causes_conns = index.get_connections_by_type("causes");
        assert_eq!(causes_conns.len(), 2);
        assert!(causes_conns.contains(&"conn1".to_string()));
        assert!(causes_conns.contains(&"conn3".to_string()));
        
        // Test targets
        let targets_e1 = index.get_targets(&"e1".to_string());
        assert_eq!(targets_e1.len(), 2);
        assert!(targets_e1.contains(&"e2".to_string()));
        assert!(targets_e1.contains(&"e3".to_string()));
        
        // Test combined queries
        let e1_causes = index.find_by_source_and_type(&"e1".to_string(), "causes");
        assert_eq!(e1_causes.len(), 1);
        assert!(e1_causes.contains(&"conn1".to_string()));
        
        // Test path finding
        let paths = index.find_paths(&"e1".to_string(), &"e4".to_string(), 2);
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0], vec!["e1".to_string(), "e2".to_string(), "e4".to_string()]);
        
        // Test removing a connection
        index.remove_connection(&conn1).unwrap();
        let outgoing_e1_after = index.get_outgoing_connections(&"e1".to_string());
        assert_eq!(outgoing_e1_after.len(), 1);
        assert!(outgoing_e1_after.contains(&"conn2".to_string()));
    }
    
    #[test]
    fn test_metadata_index() {
        let mut index = MetadataIndex::new();
        
        // Create test engrams with metadata
        let mut engram1 = create_test_engram("e1", "Test content 1", "source1", 0.8);
        engram1.metadata.insert("topic".to_string(), json!("climate"));
        engram1.metadata.insert("tags".to_string(), json!(["important", "verified"]));
        
        let mut engram2 = create_test_engram("e2", "Test content 2", "source2", 0.9);
        engram2.metadata.insert("topic".to_string(), json!("science"));
        engram2.metadata.insert("tags".to_string(), json!(["important"]));
        
        let mut engram3 = create_test_engram("e3", "Test content 3", "source1", 0.7);
        engram3.metadata.insert("topic".to_string(), json!("climate"));
        engram3.metadata.insert("verified".to_string(), json!(true));
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Test find by key
        let topic_engrams = index.find_by_key("topic");
        assert_eq!(topic_engrams.len(), 3);
        
        let verified_engrams = index.find_by_key("verified");
        assert_eq!(verified_engrams.len(), 1);
        assert!(verified_engrams.contains(&"e3".to_string()));
        
        // Test find by key-value
        let climate_engrams = index.find_by_key_value("topic", "climate");
        assert_eq!(climate_engrams.len(), 2);
        assert!(climate_engrams.contains(&"e1".to_string()));
        assert!(climate_engrams.contains(&"e3".to_string()));
        
        // Test removing an engram
        index.remove_engram(&engram1).unwrap();
        let climate_engrams_after = index.find_by_key_value("topic", "climate");
        assert_eq!(climate_engrams_after.len(), 1);
        assert!(climate_engrams_after.contains(&"e3".to_string()));
    }
    
    #[test]
    fn test_search_index() {
        let mut index = SearchIndex::new();
        
        // Create test engrams
        let mut engram1 = create_test_engram("e1", "Climate facts", "research", 0.9);
        engram1.metadata.insert("topic".to_string(), json!("climate"));
        
        let mut engram2 = create_test_engram("e2", "Solar energy", "observation", 0.8);
        engram2.metadata.insert("topic".to_string(), json!("energy"));
        
        let mut engram3 = create_test_engram("e3", "Weather patterns", "research", 0.7);
        engram3.metadata.insert("topic".to_string(), json!("climate"));
        
        // Create connections
        let conn1 = create_test_connection("conn1", "e1", "e2", "related_to", 0.8);
        let conn2 = create_test_connection("conn2", "e2", "e3", "influences", 0.9);
        
        // Add to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        index.add_connection(&conn1).unwrap();
        index.add_connection(&conn2).unwrap();
        
        // Test source searching
        let research_engrams = index.find_by_source("research");
        assert_eq!(research_engrams.len(), 2);
        assert!(research_engrams.contains(&"e1".to_string()));
        assert!(research_engrams.contains(&"e3".to_string()));
        
        // Test confidence searching
        let high_confidence = index.find_by_min_confidence(0.8);
        assert_eq!(high_confidence.len(), 2);
        assert!(high_confidence.contains(&"e1".to_string()));
        assert!(high_confidence.contains(&"e2".to_string()));
        
        // Test combined searching
        let combined_results = index.search_combined(
            Some("climate"), // text to search for in content
            Some("research"), // source filter
            Some(0.8),      // min confidence
            Some("topic"),  // metadata key
            Some("climate"), // metadata value
            false,          // exact_match set to false
        );
        assert_eq!(combined_results.len(), 1);
        assert!(combined_results.contains(&"e1".to_string()));
        
        // Test relationship traversal
        let targets_e2 = index.relationship_index.get_targets(&"e2".to_string());
        assert_eq!(targets_e2.len(), 1);
        assert!(targets_e2.contains(&"e3".to_string()));
        
        // Test metadata filtering
        let climate_engrams = index.metadata_index.find_by_key_value("topic", "climate");
        assert_eq!(climate_engrams.len(), 2);
        assert!(climate_engrams.contains(&"e1".to_string()));
        assert!(climate_engrams.contains(&"e3".to_string()));
    }
}