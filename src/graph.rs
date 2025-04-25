use crate::error::{EngramError, Result};
use crate::schema::{
    Agent, AgentId, Collection, CollectionId, Connection, ConnectionId, Context, ContextId, Engram,
    EngramId,
};
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;
use petgraph::visit::EdgeRef;
use std::collections::{HashMap, HashSet};

/// Node types in the memory graph
#[derive(Debug, Clone)]
pub enum Node {
    Engram(Engram),
    Collection(Collection),
    Agent(Agent),
    Context(Context),
}

/// Edge types in the memory graph
#[derive(Debug, Clone)]
pub enum Edge {
    Connection(Connection),
    Contains, // Collection/Context contains Engram
    HasAccess, // Agent has access to Collection
    Participates, // Agent participates in Context
}

/// In-memory graph representation of the EngramAI knowledge structure
pub struct MemoryGraph {
    /// The graph structure
    graph: DiGraph<Node, Edge>,
    
    /// Mapping from Engram IDs to graph node indices
    engram_indices: HashMap<EngramId, NodeIndex>,
    
    /// Mapping from Collection IDs to graph node indices
    collection_indices: HashMap<CollectionId, NodeIndex>,
    
    /// Mapping from Agent IDs to graph node indices
    agent_indices: HashMap<AgentId, NodeIndex>,
    
    /// Mapping from Context IDs to graph node indices
    context_indices: HashMap<ContextId, NodeIndex>,
    
    /// Mapping from Connection IDs to graph edge indices
    connection_indices: HashMap<ConnectionId, petgraph::graph::EdgeIndex>,
}

#[allow(dead_code)]
impl MemoryGraph {
    /// Create a new, empty memory graph
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            engram_indices: HashMap::new(),
            collection_indices: HashMap::new(),
            agent_indices: HashMap::new(),
            context_indices: HashMap::new(),
            connection_indices: HashMap::new(),
        }
    }

    /// Add an engram to the graph
    pub fn add_engram(&mut self, engram: Engram) -> Result<EngramId> {
        // Create a node in the graph
        let node_idx = self.graph.add_node(Node::Engram(engram.clone()));
        
        // Store the mapping from ID to node index
        self.engram_indices.insert(engram.id.clone(), node_idx);
        
        Ok(engram.id)
    }

    /// Add a connection between two engrams
    pub fn add_connection(&mut self, connection: Connection) -> Result<ConnectionId> {
        // Verify that both source and target engrams exist
        let source_idx = self.engram_indices.get(&connection.source_id).ok_or_else(|| {
            EngramError::NotFound(format!("Source engram not found: {}", connection.source_id))
        })?;
        
        let target_idx = self.engram_indices.get(&connection.target_id).ok_or_else(|| {
            EngramError::NotFound(format!("Target engram not found: {}", connection.target_id))
        })?;
        
        // Create an edge in the graph
        let edge_idx = self.graph.add_edge(*source_idx, *target_idx, Edge::Connection(connection.clone()));
        
        // Store the mapping from ID to edge index
        self.connection_indices.insert(connection.id.clone(), edge_idx);
        
        Ok(connection.id)
    }

    /// Add a collection to the graph
    pub fn add_collection(&mut self, collection: Collection) -> Result<CollectionId> {
        // Create a node in the graph
        let node_idx = self.graph.add_node(Node::Collection(collection.clone()));
        
        // Store the mapping from ID to node index
        self.collection_indices.insert(collection.id.clone(), node_idx);
        
        // Connect collection to its engrams
        for engram_id in &collection.engram_ids {
            if let Some(engram_idx) = self.engram_indices.get(engram_id) {
                self.graph.add_edge(node_idx, *engram_idx, Edge::Contains);
            }
        }
        
        Ok(collection.id)
    }

    /// Add an agent to the graph
    pub fn add_agent(&mut self, agent: Agent) -> Result<AgentId> {
        // Create a node in the graph
        let node_idx = self.graph.add_node(Node::Agent(agent.clone()));
        
        // Store the mapping from ID to node index
        self.agent_indices.insert(agent.id.clone(), node_idx);
        
        // Connect agent to its accessible collections
        for collection_id in &agent.accessible_collections {
            if let Some(collection_idx) = self.collection_indices.get(collection_id) {
                self.graph.add_edge(node_idx, *collection_idx, Edge::HasAccess);
            }
        }
        
        Ok(agent.id)
    }

    /// Add a context to the graph
    pub fn add_context(&mut self, context: Context) -> Result<ContextId> {
        // Create a node in the graph
        let node_idx = self.graph.add_node(Node::Context(context.clone()));
        
        // Store the mapping from ID to node index
        self.context_indices.insert(context.id.clone(), node_idx);
        
        // Connect context to its engrams
        for engram_id in &context.engram_ids {
            if let Some(engram_idx) = self.engram_indices.get(engram_id) {
                self.graph.add_edge(node_idx, *engram_idx, Edge::Contains);
            }
        }
        
        // Connect context to its agents
        for agent_id in &context.agent_ids {
            if let Some(agent_idx) = self.agent_indices.get(agent_id) {
                // Context includes agent
                self.graph.add_edge(node_idx, *agent_idx, Edge::Contains);
                
                // Agent participates in context
                self.graph.add_edge(*agent_idx, node_idx, Edge::Participates);
            }
        }
        
        Ok(context.id)
    }

    /// Retrieve an engram by ID
    pub fn get_engram(&self, id: &EngramId) -> Result<Option<Engram>> {
        let idx = match self.engram_indices.get(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        
        match &self.graph[*idx] {
            Node::Engram(engram) => Ok(Some(engram.clone())),
            _ => Err(EngramError::InvalidId(format!("ID {} is not an engram", id))),
        }
    }

    /// Retrieve a connection by ID
    pub fn get_connection(&self, id: &ConnectionId) -> Result<Option<Connection>> {
        let edge_idx = match self.connection_indices.get(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        
        match &self.graph[*edge_idx] {
            Edge::Connection(connection) => Ok(Some(connection.clone())),
            _ => Err(EngramError::InvalidId(format!("ID {} is not a connection", id))),
        }
    }

    /// Retrieve a collection by ID
    pub fn get_collection(&self, id: &CollectionId) -> Result<Option<Collection>> {
        let idx = match self.collection_indices.get(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        
        match &self.graph[*idx] {
            Node::Collection(collection) => Ok(Some(collection.clone())),
            _ => Err(EngramError::InvalidId(format!("ID {} is not a collection", id))),
        }
    }

    /// Retrieve an agent by ID
    pub fn get_agent(&self, id: &AgentId) -> Result<Option<Agent>> {
        let idx = match self.agent_indices.get(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        
        match &self.graph[*idx] {
            Node::Agent(agent) => Ok(Some(agent.clone())),
            _ => Err(EngramError::InvalidId(format!("ID {} is not an agent", id))),
        }
    }

    /// Retrieve a context by ID
    pub fn get_context(&self, id: &ContextId) -> Result<Option<Context>> {
        let idx = match self.context_indices.get(id) {
            Some(idx) => idx,
            None => return Ok(None),
        };
        
        match &self.graph[*idx] {
            Node::Context(context) => Ok(Some(context.clone())),
            _ => Err(EngramError::InvalidId(format!("ID {} is not a context", id))),
        }
    }

    /// Get all connections between two engrams
    pub fn get_connections_between(
        &self,
        source_id: &EngramId,
        target_id: &EngramId,
    ) -> Result<Vec<Connection>> {
        let source_idx = match self.engram_indices.get(source_id) {
            Some(idx) => *idx,
            None => return Ok(Vec::new()),
        };
        
        let target_idx = match self.engram_indices.get(target_id) {
            Some(idx) => *idx,
            None => return Ok(Vec::new()),
        };
        
        let mut connections = Vec::new();
        
        // Iterate over edges from source to target
        for edge_idx in self.graph.edges_connecting(source_idx, target_idx) {
            if let Edge::Connection(connection) = &self.graph[edge_idx.id()] {
                connections.push(connection.clone());
            }
        }
        
        Ok(connections)
    }

    /// Get engrams from a specific source
    pub fn get_engrams_by_source(&self, source: &str) -> Result<Vec<Engram>> {
        let mut engrams = Vec::new();
        
        for (_, &idx) in &self.engram_indices {
            if let Node::Engram(engram) = &self.graph[idx] {
                if engram.source == source {
                    engrams.push(engram.clone());
                }
            }
        }
        
        Ok(engrams)
    }

    /// Get engrams with confidence above or equal to the minimum
    pub fn get_engrams_by_confidence(&self, min_confidence: f64) -> Result<Vec<Engram>> {
        let mut engrams = Vec::new();
        
        for (_, &idx) in &self.engram_indices {
            if let Node::Engram(engram) = &self.graph[idx] {
                if engram.confidence >= min_confidence {
                    engrams.push(engram.clone());
                }
            }
        }
        
        Ok(engrams)
    }

    /// Get the most recent engrams
    pub fn get_recent_engrams(&self, count: usize) -> Result<Vec<Engram>> {
        let mut engrams: Vec<_> = self
            .engram_indices
            .iter()
            .filter_map(|(_, &idx)| {
                if let Node::Engram(engram) = &self.graph[idx] {
                    Some(engram.clone())
                } else {
                    None
                }
            })
            .collect();
        
        // Sort by timestamp (newest first)
        engrams.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        
        // Take the first `count` elements
        if engrams.len() > count {
            engrams.truncate(count);
        }
        
        Ok(engrams)
    }

    /// Get all engrams an agent can access through collections
    pub fn get_agent_accessible_engrams(&self, agent_id: &AgentId) -> Result<Vec<Engram>> {
        let agent_idx = match self.agent_indices.get(agent_id) {
            Some(idx) => *idx,
            None => return Ok(Vec::new()),
        };
        
        let mut accessible_engrams = HashSet::new();
        
        // Find all collections the agent has access to
        for edge in self.graph.edges_directed(agent_idx, Direction::Outgoing) {
            if let Edge::HasAccess = &self.graph[edge.id()] {
                let collection_idx = edge.target();
                
                // Find all engrams in the collection
                for edge in self.graph.edges_directed(collection_idx, Direction::Outgoing) {
                    if let Edge::Contains = &self.graph[edge.id()] {
                        let engram_idx = edge.target();
                        if let Node::Engram(engram) = &self.graph[engram_idx] {
                            accessible_engrams.insert(engram.id.clone());
                        }
                    }
                }
            }
        }
        
        // Get actual engram objects
        let engrams = accessible_engrams
            .iter()
            .filter_map(|id| {
                if let Ok(Some(engram)) = self.get_engram(id) {
                    Some(engram)
                } else {
                    None
                }
            })
            .collect();
        
        Ok(engrams)
    }

    /// Get all engrams in a context
    pub fn get_context_engrams(&self, context_id: &ContextId) -> Result<Vec<Engram>> {
        let context_idx = match self.context_indices.get(context_id) {
            Some(idx) => *idx,
            None => return Ok(Vec::new()),
        };
        
        let mut engrams = Vec::new();
        
        // Find all engrams in the context
        for edge in self.graph.edges_directed(context_idx, Direction::Outgoing) {
            if let Edge::Contains = &self.graph[edge.id()] {
                let node_idx = edge.target();
                if let Node::Engram(engram) = &self.graph[node_idx] {
                    engrams.push(engram.clone());
                }
            }
        }
        
        Ok(engrams)
    }

    /// Get all agents in a context
    pub fn get_agents_in_context(&self, context_id: &ContextId) -> Result<Vec<Agent>> {
        let context_idx = match self.context_indices.get(context_id) {
            Some(idx) => *idx,
            None => return Ok(Vec::new()),
        };
        
        let mut agents = Vec::new();
        
        // Find all agents in the context
        for edge in self.graph.edges_directed(context_idx, Direction::Outgoing) {
            if let Edge::Contains = &self.graph[edge.id()] {
                let node_idx = edge.target();
                if let Node::Agent(agent) = &self.graph[node_idx] {
                    agents.push(agent.clone());
                }
            }
        }
        
        Ok(agents)
    }

    /// Add an engram to a collection
    pub fn add_engram_to_collection(
        &mut self,
        engram_id: &EngramId,
        collection_id: &CollectionId,
    ) -> Result<bool> {
        // Get node indices
        let engram_idx = match self.engram_indices.get(engram_id) {
            Some(idx) => *idx,
            None => return Err(EngramError::NotFound(format!("Engram not found: {}", engram_id))),
        };
        
        let collection_idx = match self.collection_indices.get(collection_id) {
            Some(idx) => *idx,
            None => {
                return Err(EngramError::NotFound(format!(
                    "Collection not found: {}",
                    collection_id
                )))
            }
        };
        
        // Update the collection
        if let Node::Collection(collection) = &mut self.graph[collection_idx] {
            collection.add_engram(engram_id.clone());
        }
        
        // Add edge in graph
        self.graph.add_edge(collection_idx, engram_idx, Edge::Contains);
        
        Ok(true)
    }

    /// Add an engram to a context
    pub fn add_engram_to_context(
        &mut self,
        engram_id: &EngramId,
        context_id: &ContextId,
    ) -> Result<bool> {
        // Get node indices
        let engram_idx = match self.engram_indices.get(engram_id) {
            Some(idx) => *idx,
            None => return Err(EngramError::NotFound(format!("Engram not found: {}", engram_id))),
        };
        
        let context_idx = match self.context_indices.get(context_id) {
            Some(idx) => *idx,
            None => {
                return Err(EngramError::NotFound(format!(
                    "Context not found: {}",
                    context_id
                )))
            }
        };
        
        // Update the context
        if let Node::Context(context) = &mut self.graph[context_idx] {
            context.add_engram(engram_id.clone());
        }
        
        // Add edge in graph
        self.graph.add_edge(context_idx, engram_idx, Edge::Contains);
        
        Ok(true)
    }

    /// Add an agent to a context
    pub fn add_agent_to_context(
        &mut self,
        agent_id: &AgentId,
        context_id: &ContextId,
    ) -> Result<bool> {
        // Get node indices
        let agent_idx = match self.agent_indices.get(agent_id) {
            Some(idx) => *idx,
            None => return Err(EngramError::NotFound(format!("Agent not found: {}", agent_id))),
        };
        
        let context_idx = match self.context_indices.get(context_id) {
            Some(idx) => *idx,
            None => {
                return Err(EngramError::NotFound(format!(
                    "Context not found: {}",
                    context_id
                )))
            }
        };
        
        // Update the context
        if let Node::Context(context) = &mut self.graph[context_idx] {
            context.add_agent(agent_id.clone());
        }
        
        // Add edges in graph (bidirectional)
        self.graph.add_edge(context_idx, agent_idx, Edge::Contains);
        self.graph.add_edge(agent_idx, context_idx, Edge::Participates);
        
        Ok(true)
    }
}