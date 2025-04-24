use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

pub type EngramId = String;
pub type ConnectionId = String;
pub type CollectionId = String;
pub type AgentId = String;
pub type ContextId = String;
pub type Metadata = HashMap<String, serde_json::Value>;

/// Atomic unit of knowledge/memory with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engram {
    /// Unique identifier for the engram
    pub id: EngramId,
    
    /// The actual knowledge/information content
    pub content: String,
    
    /// When this engram was created
    pub timestamp: DateTime<Utc>,
    
    /// Where this knowledge came from
    pub source: String,
    
    /// Certainty score between 0.0 and 1.0
    pub confidence: f64,
    
    /// Additional custom metadata
    pub metadata: Metadata,
}

impl Engram {
    pub fn new(content: String, source: String, confidence: f64, metadata: Option<Metadata>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content,
            timestamp: Utc::now(),
            source,
            confidence,
            metadata: metadata.unwrap_or_default(),
        }
    }
}

/// Typed relationship between engrams with strength/weight
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    /// Unique identifier for the connection
    pub id: ConnectionId,
    
    /// ID of the source engram
    pub source_id: EngramId,
    
    /// ID of the target engram
    pub target_id: EngramId,
    
    /// Type of relationship (e.g., "causes", "supports", "contradicts")
    pub relationship_type: String,
    
    /// Strength of the connection (0.0 to 1.0)
    pub weight: f64,
    
    /// Additional custom metadata
    pub metadata: Metadata,
}

impl Connection {
    pub fn new(
        source_id: EngramId,
        target_id: EngramId,
        relationship_type: String,
        weight: f64,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            source_id,
            target_id,
            relationship_type,
            weight,
            metadata: metadata.unwrap_or_default(),
        }
    }
}

/// Named grouping of engrams for organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    /// Unique identifier for the collection
    pub id: CollectionId,
    
    /// Name of the collection
    pub name: String,
    
    /// Description of what this collection represents
    pub description: String,
    
    /// Set of engram IDs in this collection
    pub engram_ids: HashSet<EngramId>,
    
    /// Additional custom metadata
    pub metadata: Metadata,
}

impl Collection {
    pub fn new(name: String, description: String, metadata: Option<Metadata>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            engram_ids: HashSet::new(),
            metadata: metadata.unwrap_or_default(),
        }
    }
    
    pub fn add_engram(&mut self, engram_id: EngramId) {
        self.engram_ids.insert(engram_id);
    }
    
    pub fn remove_engram(&mut self, engram_id: &EngramId) -> bool {
        self.engram_ids.remove(engram_id)
    }
}

/// Entity with access controls and capabilities within the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// Unique identifier for the agent
    pub id: AgentId,
    
    /// Name of the agent
    pub name: String,
    
    /// Description of the agent's role/purpose
    pub description: String,
    
    /// Set of capabilities this agent has
    pub capabilities: HashSet<String>,
    
    /// Collection IDs this agent can access
    pub accessible_collections: HashSet<CollectionId>,
    
    /// Additional custom metadata
    pub metadata: Metadata,
}

impl Agent {
    pub fn new(
        name: String,
        description: String,
        capabilities: Option<HashSet<String>>,
        metadata: Option<Metadata>,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            capabilities: capabilities.unwrap_or_default(),
            accessible_collections: HashSet::new(),
            metadata: metadata.unwrap_or_default(),
        }
    }
    
    pub fn grant_access(&mut self, collection_id: CollectionId) {
        self.accessible_collections.insert(collection_id);
    }
    
    pub fn revoke_access(&mut self, collection_id: &CollectionId) -> bool {
        self.accessible_collections.remove(collection_id)
    }
    
    pub fn has_access(&self, collection_id: &CollectionId) -> bool {
        self.accessible_collections.contains(collection_id)
    }
}

/// Shareable environment with relevant engrams for agent collaboration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    /// Unique identifier for the context
    pub id: ContextId,
    
    /// Name of the context
    pub name: String,
    
    /// Description of what this context represents
    pub description: String,
    
    /// Set of engram IDs in this context
    pub engram_ids: HashSet<EngramId>,
    
    /// Set of agent IDs with access to this context
    pub agent_ids: HashSet<AgentId>,
    
    /// Additional custom metadata
    pub metadata: Metadata,
}

impl Context {
    pub fn new(name: String, description: String, metadata: Option<Metadata>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            engram_ids: HashSet::new(),
            agent_ids: HashSet::new(),
            metadata: metadata.unwrap_or_default(),
        }
    }
    
    pub fn add_engram(&mut self, engram_id: EngramId) {
        self.engram_ids.insert(engram_id);
    }
    
    pub fn remove_engram(&mut self, engram_id: &EngramId) -> bool {
        self.engram_ids.remove(engram_id)
    }
    
    pub fn add_agent(&mut self, agent_id: AgentId) {
        self.agent_ids.insert(agent_id);
    }
    
    pub fn remove_agent(&mut self, agent_id: &AgentId) -> bool {
        self.agent_ids.remove(agent_id)
    }
}