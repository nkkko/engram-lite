# Report: Utilizing Engram-Lite as a Memory and Runtime for Pydantic.ai Multi-Agent Workflows

## 1. Overview of Components

### Engram-Lite

Engram-Lite is a memory graph storage system designed for AI agents with a focus on:
- Structured knowledge representation via engrams, connections, collections, and contexts
- Persistent RocksDB-based storage with ACID transactions
- Graph-based relations using petgraph for fast traversals
- Efficient indexing for relationship queries
- gRPC API for remote access
- Vector embedding capabilities for semantic search

### Pydantic.ai

Pydantic.ai is a Python framework for developing structured AI agents that provides:
- Model-agnostic support for multiple LLM providers (OpenAI, Anthropic, etc.)
- Structured responses using Pydantic models
- Dependency injection for configuring agents
- Tool registration for AI model function calls
- Asynchronous and synchronous execution

## 2. Integration Architecture

A comprehensive integration between Engram-Lite and pydantic.ai could use the following architecture:

### 2.1 Engram-Lite as Memory Layer

Engram-Lite can serve as a persistent, structured memory system for Pydantic.ai agents through:

1. gRPC Interface: The existing gRPC server (src/grpc/server.rs) provides a language-agnostic API for Python agents to interact with Engram-Lite.
2. Python Client Library: Creating a specialized Python client that wraps the gRPC interface with Pydantic models:

```python
from pydanticai import Agent, Dependency, Tool
from pydantic import BaseModel
from typing import List, Optional, Dict, Any

# Pydantic models mirroring Engram-Lite structures
class EngramModel(BaseModel):
    id: Optional[str] = None
    content: str
    source: str
    confidence: float
    metadata: Dict[str, Any] = {}

class ConnectionModel(BaseModel):
    source_id: str
    target_id: str
    relationship_type: str
    weight: float
    metadata: Dict[str, Any] = {}

# Engram-Lite client as a dependency
class EngramClient:
    def __init__(self, host: str = "localhost", port: int = 50051):
        # Initialize gRPC client connection to Engram-Lite
        # ...

    async def store_memory(self, content: str, source: str, confidence: float, metadata: Dict[str, Any] = {}) -> str:
        # Store memory in Engram-Lite via gRPC
        # ...

    async def retrieve_related(self, engram_id: str, max_depth: int = 1) -> List[EngramModel]:
        # Retrieve related memories via gRPC
        # ...

# Register as a Pydantic.ai dependency
engram_memory = Dependency(lambda: EngramClient())
```

### 2.2 Engram-Lite as Agent Runtime

Engram-Lite can serve as a runtime environment for multi-agent coordination using its context mechanism:

1. Agent State Management: Agents' state can be stored in Engram's metadata structures
2. Context-Based Collaboration: Using Engram-Lite's Context entity to manage shared knowledge spaces between agents:

```python
from pydanticai import Agent, Tool, run_agent
from contextlib import asynccontextmanager

# Define agent with Engram-Lite memory access
@Agent.define
def research_agent(query: str, engram_memory: EngramClient = engram_memory):
    """A research agent that searches for information and stores results in memory."""

    @Tool.define
    async def store_finding(content: str, confidence: float):
        """Store a research finding in memory."""
        engram_id = await engram_memory.store_memory(
            content=content,
            source="research_agent",
            confidence=confidence
        )
        return f"Stored finding with ID: {engram_id}"

    @Tool.define
    async def create_context(name: str, description: str):
        """Create a shared context for collaboration."""
        context_id = await engram_memory.create_context(name, description)
        return context_id

    # Main execution with LLM
    return f"Researching: {query}"

# Define synthesis agent that works with research agent's findings
@Agent.define
def synthesis_agent(context_id: str, engram_memory: EngramClient = engram_memory):
    """An agent that synthesizes information from a shared context."""

    @Tool.define
    async def get_context_memories(context_id: str):
        """Get all memories from a context."""
        memories = await engram_memory.get_context_engrams(context_id)
        return memories

    # Main execution with LLM
    return "Synthesizing findings..."
```

### 2.3 Workflow Orchestration

Engram-Lite's graph structure can be used to orchestrate multi-agent workflows by representing the workflow steps as engrams and connections:

```python
async def run_research_workflow(query: str):
    # Initialize memory client
    memory = EngramClient()

    # Create workflow context
    context_id = await memory.create_context(
        name=f"Research: {query}",
        description=f"Research workflow for query: {query}"
    )

    # Step 1: Research agent gathers information
    research_results = await run_agent(research_agent, query=query)

    # Store results in context
    for finding in research_results.findings:
        engram_id = await memory.store_memory(
            content=finding.content,
            source="research_agent",
            confidence=finding.confidence
        )
        await memory.add_engram_to_context(engram_id, context_id)

    # Step 2: Synthesis agent processes findings
    synthesis = await run_agent(synthesis_agent, context_id=context_id)

    # Store synthesis in context
    synthesis_id = await memory.store_memory(
        content=synthesis.summary,
        source="synthesis_agent",
        confidence=0.9
    )

    # Connect the synthesis to all findings
    for finding_id in context_engram_ids:
        await memory.create_connection(
            source_id=finding_id,
            target_id=synthesis_id,
            relationship_type="contributes_to",
            weight=0.8
        )

    return synthesis.summary
```

## 3. Key Integration Features

### 3.1 Shared Memory Access

Using Engram-Lite's context functionality, multiple agents can share access to the same memory space:

```python
# Create a shared context
context_id = await engram_client.create_context(
    name="Collaborative Research",
    description="Shared research environment for multiple agents"
)

# Add agent to context
await engram_client.add_agent_to_context(agent_id, context_id)

# Access context-specific memory
context_engrams = await engram_client.get_context_engrams(context_id)
```

### 3.2 Agent Communication Through Memory

Agents can communicate indirectly through memory structures rather than direct message passing:

```python
# Agent A stores knowledge
engram_id = await engram_client.store_memory(
    content="The sky is blue because of Rayleigh scattering.",
    source="physics_agent",
    confidence=0.95,
    metadata={"type": "explanation", "topic": "optics"}
)

# Agent B queries for relevant knowledge
optics_engrams = await engram_client.search_combined(
    text=None,
    source=None,
    metadata_key="topic",
    metadata_value="optics",
    exact_match=True
)
```

### 3.3 Graph-Based Reasoning

Leveraging Engram-Lite's graph structure for multi-step reasoning processes:

```python
# Find connected knowledge to build reasoning chains
traversal_result = await engram_client.find_connected_engrams(
    engram_id=starting_point_id,
    max_depth=3,
    relationship_type="causes"
)

# Generate causal chain based on connected engrams
causal_chain = [engram.content for engram in traversal_result.engrams]
```

### 3.4 Vector Search for Semantic Lookup

Using Engram-Lite's vector embedding capabilities for semantic memory access:

```python
@Tool.define
async def semantic_search(query: str, limit: int = 5):
    """Search for semantically similar memories."""
    results = await engram_client.search_by_vector(
        text=query,  # Will be converted to embedding
        limit=limit
    )
    return [result.engram for result in results]
```

## 4. Implementation Guide

### 4.1 Setting Up Engram-Lite as a Service

1. Build and run Engram-Lite as a gRPC server:
```bash
cargo build --release --features="grpc"
./target/release/engram_server --db-path /path/to/database --host 0.0.0.0 --port 50051
```

2. Create a Python client wrapper using gRPC Python libraries that connects to the service:
```python
import grpc
from engram_pb2 import *
from engram_pb2_grpc import EngramServiceStub

class EngramLiteClient:
    def __init__(self, host="localhost", port=50051):
        self.channel = grpc.insecure_channel(f"{host}:{port}")
        self.stub = EngramServiceStub(self.channel)

    def create_engram(self, content, source, confidence=0.9, metadata=None):
        request = CreateEngramRequest(
            content=content,
            source=source,
            confidence=confidence,
            metadata=metadata or {}
        )
        response = self.stub.CreateEngram(request)
        return response.engram.id
```

### 4.2 Integrating with Pydantic.ai Agents

1. Create a dependency provider for Pydantic.ai:
```python
from pydanticai import Dependency, Agent

# Create Engram-Lite client as a dependency
engram_db = Dependency(lambda: EngramLiteClient())

# Define agent with memory dependencies
@Agent.define
def memory_agent(
    engram_db: EngramLiteClient = engram_db
):
    """An agent with access to external memory."""
    # Agent implementation...
```

2. Create tools for memory operations:
```python
@Tool.define
async def store_memory(content: str, source: str, confidence: float = 0.9):
    """Store a new memory."""
    return await engram_db.create_engram(content, source, confidence)

@Tool.define
async def recall_relevant(query: str, limit: int = 5):
    """Recall relevant memories based on a query."""
    response = await engram_db.search_by_text(
        query=query,
        limit=limit
    )
    return [r.engram for r in response.results]
```

### 4.3 Creating Multi-Agent Runtime

1. Use Engram-Lite's Context entity to create shared runtime environments:
```python
async def create_agent_runtime(agents):
    # Create a context for agents to share
    context_id = await engram_db.create_context(
        name="Agent Runtime",
        description="Shared memory context for agent runtime"
    )

    # Register agents in context
    for agent in agents:
        agent_id = await engram_db.create_agent(
            name=agent.name,
            description=agent.description
        )
        await engram_db.add_agent_to_context(agent_id, context_id)

    return context_id
```

2. Implement runtime execution that leverages shared context:
```python
async def run_multi_agent_workflow(workflow_context_id, task_description):
    # Get agents in context
    agents = await engram_db.get_agents_in_context(workflow_context_id)

    # Create task engram
    task_id = await engram_db.create_engram(
        content=task_description,
        source="workflow_engine",
        confidence=1.0,
        metadata={"type": "task"}
    )
    await engram_db.add_engram_to_context(task_id, workflow_context_id)

    # Execute each agent in sequence (or parallel as needed)
    results = []
    for agent_info in agents:
        # Instantiate agent from registry
        agent = agent_registry.get(agent_info.name)

        # Run agent with context access
        result = await run_agent(
            agent,
            task_id=task_id,
            context_id=workflow_context_id
        )

        # Store result in context
        result_id = await engram_db.create_engram(
            content=result.output,
            source=agent_info.name,
            confidence=result.confidence
        )
        await engram_db.add_engram_to_context(result_id, workflow_context_id)

        # Connect result to task
        await engram_db.create_connection(
            source_id=task_id,
            target_id=result_id,
            relationship_type="result",
            weight=1.0
        )

        results.append(result)

    return results
```

## 5. Advantages of the Integration

1. Persistence and Reliability: Engram-Lite's RocksDB-based storage provides ACID transactions, ensuring agent memory persistence.
2. Graph-Based Knowledge: Complex relationships can be represented and traversed efficiently.
3. Semantic Search: Vector embeddings enable concept-level search beyond exact keyword matching.
4. Multi-Agent Coordination: Context mechanics provide a natural mechanism for agent collaboration.
5. Structured Memory: Type-safe, schema-based memory architecture fits well with Pydantic.ai's model-driven approach.
6. Access Control: Engram-Lite's agent and collection permissions can restrict memory access appropriately.
7. Scalability: Separation of memory from computation allows for more complex workflows.

## 6. Conclusion

Engram-Lite provides an ideal memory and runtime infrastructure for pydantic.ai-based multi-agent systems by offering:

1. A persistent, structured knowledge storage system
2. A graph-based model for representing relationships between knowledge units
3. Context-based collaboration mechanisms for multi-agent interaction
4. A high-performance gRPC API compatible with Python clients
5. Vector embedding support for semantic memory operations

By integrating Engram-Lite with pydantic.ai, developers can create sophisticated multi-agent workflows where:
- Knowledge persists beyond individual agent sessions
- Agents can discover and build upon each others' knowledge
- Complex reasoning chains can be represented graphically
- Agents can coordinate through structured contexts
- Memory access can be controlled based on agent roles

This integration separates memory concerns from agent implementation, allowing for more modular, maintainable, and scalable multi-agent systems.