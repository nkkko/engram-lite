#[cfg(test)]
mod tests {
    use crate::schema::{Agent, Collection, Connection, Context, Engram};
    use std::collections::HashSet;

    #[test]
    fn test_engram_creation() {
        let engram = Engram::new(
            "Test content".to_string(),
            "test_source".to_string(),
            0.9,
            None,
        );

        assert_eq!(engram.content, "Test content");
        assert_eq!(engram.source, "test_source");
        assert_eq!(engram.confidence, 0.9);
        assert!(engram.metadata.is_empty());
    }

    #[test]
    fn test_connection_creation() {
        let connection = Connection::new(
            "source_id".to_string(),
            "target_id".to_string(),
            "causes".to_string(),
            0.8,
            None,
        );

        assert_eq!(connection.source_id, "source_id");
        assert_eq!(connection.target_id, "target_id");
        assert_eq!(connection.relationship_type, "causes");
        assert_eq!(connection.weight, 0.8);
        assert!(connection.metadata.is_empty());
    }

    #[test]
    fn test_collection_operations() {
        let mut collection = Collection::new(
            "Test Collection".to_string(),
            "A test collection".to_string(),
            None,
        );

        assert_eq!(collection.name, "Test Collection");
        assert_eq!(collection.description, "A test collection");
        assert!(collection.engram_ids.is_empty());

        // Add engrams
        collection.add_engram("engram1".to_string());
        collection.add_engram("engram2".to_string());
        assert_eq!(collection.engram_ids.len(), 2);
        assert!(collection.engram_ids.contains("engram1"));
        assert!(collection.engram_ids.contains("engram2"));

        // Remove engram
        assert!(collection.remove_engram(&"engram1".to_string()));
        assert_eq!(collection.engram_ids.len(), 1);
        assert!(!collection.engram_ids.contains("engram1"));
        assert!(collection.engram_ids.contains("engram2"));
    }

    #[test]
    fn test_agent_operations() {
        let mut capabilities = HashSet::new();
        capabilities.insert("query".to_string());
        capabilities.insert("analyze".to_string());

        let mut agent = Agent::new(
            "Test Agent".to_string(),
            "A test agent".to_string(),
            Some(capabilities),
            None,
        );

        assert_eq!(agent.name, "Test Agent");
        assert_eq!(agent.description, "A test agent");
        assert_eq!(agent.capabilities.len(), 2);
        assert!(agent.capabilities.contains("query"));
        assert!(agent.capabilities.contains("analyze"));
        assert!(agent.accessible_collections.is_empty());

        // Grant access
        agent.grant_access("collection1".to_string());
        agent.grant_access("collection2".to_string());
        assert_eq!(agent.accessible_collections.len(), 2);
        assert!(agent.has_access(&"collection1".to_string()));
        assert!(agent.has_access(&"collection2".to_string()));

        // Revoke access
        assert!(agent.revoke_access(&"collection1".to_string()));
        assert_eq!(agent.accessible_collections.len(), 1);
        assert!(!agent.has_access(&"collection1".to_string()));
        assert!(agent.has_access(&"collection2".to_string()));
    }

    #[test]
    fn test_context_operations() {
        let mut context = Context::new(
            "Test Context".to_string(),
            "A test context".to_string(),
            None,
        );

        assert_eq!(context.name, "Test Context");
        assert_eq!(context.description, "A test context");
        assert!(context.engram_ids.is_empty());
        assert!(context.agent_ids.is_empty());

        // Add engrams
        context.add_engram("engram1".to_string());
        context.add_engram("engram2".to_string());
        assert_eq!(context.engram_ids.len(), 2);
        assert!(context.engram_ids.contains("engram1"));
        assert!(context.engram_ids.contains("engram2"));

        // Add agents
        context.add_agent("agent1".to_string());
        context.add_agent("agent2".to_string());
        assert_eq!(context.agent_ids.len(), 2);
        assert!(context.agent_ids.contains("agent1"));
        assert!(context.agent_ids.contains("agent2"));

        // Remove engram and agent
        assert!(context.remove_engram(&"engram1".to_string()));
        assert!(context.remove_agent(&"agent1".to_string()));
        assert_eq!(context.engram_ids.len(), 1);
        assert_eq!(context.agent_ids.len(), 1);
        assert!(!context.engram_ids.contains("engram1"));
        assert!(context.engram_ids.contains("engram2"));
        assert!(!context.agent_ids.contains("agent1"));
        assert!(context.agent_ids.contains("agent2"));
    }
}