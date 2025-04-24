# Graph Engine

EngramAI Lite uses a specialized graph engine built on `petgraph` to represent and traverse the memory graph. This document explains the design and implementation of the graph engine.

## Graph Architecture

The graph engine is structured in layers:

1. **petgraph Core**: Low-level directed graph implementation
2. **Memory Graph Layer**: High-level operations and domain-specific traversals
3. **Index Layer**: Efficient lookups and specialized queries

## Graph Structure

The core graph structure is a directed graph (`DiGraph`) from `petgraph` that contains nodes and edges:

### Nodes

Nodes can be one of four entity types, represented as an enum:

```rust
pub enum Node {
    Engram(Engram),
    Collection(Collection),
    Agent(Agent),
    Context(Context),
}
```

### Edges

Edges represent relationships between nodes, also using an enum:

```rust
pub enum Edge {
    Connection(Connection),  // Engram to Engram connection
    Contains,                // Collection/Context contains Engram
    HasAccess,               // Agent has access to Collection
    Participates,            // Agent participates in Context
}
```

## MemoryGraph Implementation

The `MemoryGraph` struct encapsulates the graph and provides high-level operations:

```rust
pub struct MemoryGraph {
    /// The graph structure
    graph: DiGraph<Node, Edge>,
    
    /// Mapping from entity IDs to graph node indices
    engram_indices: HashMap<EngramId, NodeIndex>,
    collection_indices: HashMap<CollectionId, NodeIndex>,
    agent_indices: HashMap<AgentId, NodeIndex>,
    context_indices: HashMap<ContextId, NodeIndex>,
    
    /// Mapping from Connection IDs to graph edge indices
    connection_indices: HashMap<ConnectionId, EdgeIndex>,
}
```

## Key Operations

### Adding Entities

The graph engine provides methods to add various entities to the graph:

```rust
// Add an engram
pub fn add_engram(&mut self, engram: Engram) -> Result<EngramId> {
    let node_idx = self.graph.add_node(Node::Engram(engram.clone()));
    self.engram_indices.insert(engram.id.clone(), node_idx);
    Ok(engram.id)
}

// Add a connection between engrams
pub fn add_connection(&mut self, connection: Connection) -> Result<ConnectionId> {
    let source_idx = self.engram_indices.get(&connection.source_id)
        .ok_or_else(|| EngramError::NotFound(format!("Source engram not found")))?;
    
    let target_idx = self.engram_indices.get(&connection.target_id)
        .ok_or_else(|| EngramError::NotFound(format!("Target engram not found")))?;
    
    let edge_idx = self.graph.add_edge(
        *source_idx, 
        *target_idx, 
        Edge::Connection(connection.clone())
    );
    
    self.connection_indices.insert(connection.id.clone(), edge_idx);
    
    Ok(connection.id)
}

// Similar methods for other entity types
```

### Traversal and Queries

The graph engine provides various traversal and query methods:

```rust
// Get connections between two engrams
pub fn get_connections_between(
    &self,
    source_id: &EngramId,
    target_id: &EngramId,
) -> Result<Vec<Connection>> {
    // Implementation using petgraph's edges_connecting method
}

// Get engrams by source
pub fn get_engrams_by_source(&self, source: &str) -> Result<Vec<Engram>> {
    // Implementation filtering engrams by source
}

// Get engrams by confidence threshold
pub fn get_engrams_by_confidence(&self, min_confidence: f64) -> Result<Vec<Engram>> {
    // Implementation filtering engrams by minimum confidence
}

// Get recent engrams
pub fn get_recent_engrams(&self, count: usize) -> Result<Vec<Engram>> {
    // Implementation sorting by timestamp and limiting to count
}

// Get engrams accessible to an agent
pub fn get_agent_accessible_engrams(&self, agent_id: &AgentId) -> Result<Vec<Engram>> {
    // Implementation traversing agent → collections → engrams
}

// Get engrams in a context
pub fn get_context_engrams(&self, context_id: &ContextId) -> Result<Vec<Engram>> {
    // Implementation traversing context → engrams
}
```

## Advanced Graph Operations

The graph engine supports more complex operations:

### Adding Engrams to Collections

```rust
pub fn add_engram_to_collection(
    &mut self,
    engram_id: &EngramId,
    collection_id: &CollectionId,
) -> Result<bool> {
    // Get node indices
    let engram_idx = self.engram_indices.get(engram_id)
        .ok_or_else(|| EngramError::NotFound(format!("Engram not found")))?;
    
    let collection_idx = self.collection_indices.get(collection_id)
        .ok_or_else(|| EngramError::NotFound(format!("Collection not found")))?;
    
    // Update the collection node
    if let Node::Collection(collection) = &mut self.graph[*collection_idx] {
        collection.add_engram(engram_id.clone());
    }
    
    // Add edge in graph
    self.graph.add_edge(*collection_idx, *engram_idx, Edge::Contains);
    
    Ok(true)
}
```

### Adding Agents to Contexts

```rust
pub fn add_agent_to_context(
    &mut self,
    agent_id: &AgentId,
    context_id: &ContextId,
) -> Result<bool> {
    // Get node indices
    let agent_idx = self.agent_indices.get(agent_id)
        .ok_or_else(|| EngramError::NotFound(format!("Agent not found")))?;
    
    let context_idx = self.context_indices.get(context_id)
        .ok_or_else(|| EngramError::NotFound(format!("Context not found")))?;
    
    // Update the context node
    if let Node::Context(context) = &mut self.graph[*context_idx] {
        context.add_agent(agent_id.clone());
    }
    
    // Add bidirectional edges
    self.graph.add_edge(*context_idx, *agent_idx, Edge::Contains);
    self.graph.add_edge(*agent_idx, *context_idx, Edge::Participates);
    
    Ok(true)
}
```

## Graph Algorithms

The graph engine leverages `petgraph`'s algorithms for traversal and analysis:

1. **Directed Traversal**: Find paths between engrams
2. **Neighbor Queries**: Find adjacent engrams
3. **Connectivity Analysis**: Determine connected components

Example of directed traversal:

```rust
// Using petgraph::Direction for traversal
for edge in self.graph.edges_directed(node_idx, Direction::Outgoing) {
    let target_idx = edge.target();
    // Process the target node
}
```

## Integration with Storage

The graph engine works closely with the storage layer:

1. The storage layer persists all entities
2. The graph engine provides an in-memory representation for fast queries
3. Changes made to the graph are persisted via the storage layer
4. The graph can be refreshed from storage to maintain consistency

```rust
// Helper method to reload the memory graph from storage
pub fn refresh_memory_graph(&mut self) -> Result<()> {
    // Create a new memory graph
    self.memory_graph = MemoryGraph::new();
    
    // Load engrams
    let engram_ids = self.storage.list_engrams()?;
    for id in &engram_ids {
        if let Some(engram) = self.storage.get_engram(id)? {
            self.memory_graph.add_engram(engram)?;
        }
    }
    
    // Load other entity types...
    
    Ok(())
}
```

## Performance Considerations

The graph engine includes several optimizations:

1. **In-Memory Representation**: Fast traversal and query operations
2. **Index HashMaps**: O(1) lookup from entity IDs to graph indices
3. **Directed Graph**: Efficient traversal in both directions
4. **Enum Nodes/Edges**: Type-safe representation of different entities

## Future Graph Enhancements

Future enhancements to the graph engine may include:

1. **Graph Algorithms**: Centrality measures, community detection
2. **Path Finding**: Shortest path between engrams
3. **Graph Embeddings**: Node2Vec or similar embeddings
4. **Subgraph Extraction**: Extract relevant subgraphs for analysis
5. **Temporal Graph**: Track changes over time