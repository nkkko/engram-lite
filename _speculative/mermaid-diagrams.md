# Mermaid Diagrams for Engram-Lite & Pydantic.ai Integration

## 1. Component Overview

```mermaid
classDiagram
    class EngramLite {
        +RocksDB storage
        +gRPC API
        +Graph relations
        +Vector embeddings
        +ACID transactions
    }

    class Pydantic_ai {
        +LLM integration
        +Structured responses
        +Dependency injection
        +Tool registration
        +Async execution
    }

    class EngramComponents {
        +Engrams
        +Connections
        +Collections
        +Contexts
    }

    class PydanticComponents {
        +Agents
        +Tools
        +Dependencies
        +Models
    }

    EngramLite --> EngramComponents
    Pydantic_ai --> PydanticComponents
```

## 2. Integration Architecture

```mermaid
flowchart TB
    subgraph "Pydantic.ai"
        A[Agents] --> T[Tools]
        A --> D[Dependencies]
        A --> M[Pydantic Models]
    end

    subgraph "Engram-Lite"
        E[Engram Storage] --> G[Graph Relations]
        E --> V[Vector Embeddings]
        E --> C[Contexts]
    end

    D <--> P[Python Client]
    P <--> gRPC[gRPC Interface]
    gRPC <--> E

    style gRPC fill:#f9f,stroke:#333
```

## 3. Memory Layer Integration

```mermaid
sequenceDiagram
    participant Agent
    participant EngramClient
    participant gRPC
    participant EngramLite

    Agent->>EngramClient: store_memory(content, source, confidence)
    EngramClient->>gRPC: CreateEngram request
    gRPC->>EngramLite: Process request
    EngramLite->>EngramLite: Store in RocksDB
    EngramLite-->>gRPC: Return engram_id
    gRPC-->>EngramClient: Return response
    EngramClient-->>Agent: Return engram_id

    Agent->>EngramClient: retrieve_related(engram_id)
    EngramClient->>gRPC: GetRelatedEngrams request
    gRPC->>EngramLite: Traverse graph
    EngramLite-->>gRPC: Return connected engrams
    gRPC-->>EngramClient: Return results
    EngramClient-->>Agent: Return engram models
```

## 4. Multi-Agent Workflow Architecture

```mermaid
flowchart TD
    subgraph "Workflow"
        Task[Task Definition]
        RC[Research Context]

        subgraph "Research_Agent"
            RT[Research Tools]
            RM[Research Memory]
        end

        subgraph "Synthesis_Agent"
            ST[Synthesis Tools]
            SM[Synthesis Memory]
        end

        Task --> RC
        RC --> Research_Agent
        RC --> Synthesis_Agent
        Research_Agent --> RC
        RC --> Synthesis_Agent
    end

    subgraph "Engram-Lite"
        Engrams
        Connections
        Contexts
    end

    RC <--> Contexts
    RM <--> Engrams
    SM <--> Engrams
    RM <--> Connections
    SM <--> Connections
```

## 5. Agent Runtime Using Contexts

```mermaid
graph TD
    subgraph "Context: Research Project"
        E1[Engram: Task Definition]
        E2[Engram: Research Finding 1]
        E3[Engram: Research Finding 2]
        E4[Engram: Synthesis]

        Agent1[Research Agent]
        Agent2[Synthesis Agent]

        E1 -->|created_by| Workflow
        E2 -->|created_by| Agent1
        E3 -->|created_by| Agent1
        E4 -->|created_by| Agent2

        E2 -->|contributes_to| E4
        E3 -->|contributes_to| E4
    end

    Workflow -->|creates| E1
    Agent1 -->|reads| E1
    Agent1 -->|creates| E2
    Agent1 -->|creates| E3
    Agent2 -->|reads| E2
    Agent2 -->|reads| E3
    Agent2 -->|creates| E4
```

## 6. Engram Knowledge Graph Structure

```mermaid
graph LR
    E1[Engram: Sky is blue]
    E2[Engram: Rayleigh scattering]
    E3[Engram: Sunlight contains all colors]
    E4[Engram: Blue light scatters more]

    E1 -->|explained_by| E2
    E2 -->|depends_on| E3
    E2 -->|causes| E4
    E4 -->|causes| E1

    C1[Context: Physics Facts]
    C1 -.->|contains| E1
    C1 -.->|contains| E2
    C1 -.->|contains| E3
    C1 -.->|contains| E4

    A1[Agent: Physics Expert]
    A1 -.->|has_access_to| C1
```

## 7. Integration Implementation Components

```mermaid
classDiagram
    class EngramClient {
        +connection: gRPC channel
        +store_memory()
        +retrieve_related()
        +search_by_vector()
        +create_context()
        +add_to_context()
    }

    class EngramModel {
        +id: str
        +content: str
        +source: str
        +confidence: float
        +metadata: Dict
    }

    class ConnectionModel {
        +source_id: str
        +target_id: str
        +relationship_type: str
        +weight: float
    }

    class Agent {
        +name: str
        +tools: List[Tool]
        +dependencies: List[Dependency]
    }

    class Tool {
        +name: str
        +description: str
        +function: Callable
    }

    EngramClient --> EngramModel : manages
    EngramClient --> ConnectionModel : manages
    Agent --> Tool : uses
    Agent --> EngramClient : depends on
```

## 8. Multi-Agent Workflow Process

```mermaid
sequenceDiagram
    participant Client
    participant WorkflowEngine
    participant ResearchAgent
    participant SynthesisAgent
    participant EngramLite

    Client->>WorkflowEngine: run_research_workflow("quantum computing")
    WorkflowEngine->>EngramLite: create_context("Research: quantum computing")
    EngramLite-->>WorkflowEngine: return context_id

    WorkflowEngine->>ResearchAgent: run_agent(query="quantum computing")
    ResearchAgent->>EngramLite: store_memory(finding1)
    EngramLite-->>ResearchAgent: return engram_id1
    ResearchAgent->>EngramLite: store_memory(finding2)
    EngramLite-->>ResearchAgent: return engram_id2
    ResearchAgent->>EngramLite: add_engram_to_context(engram_id1, context_id)
    ResearchAgent->>EngramLite: add_engram_to_context(engram_id2, context_id)
    ResearchAgent-->>WorkflowEngine: return research_results

    WorkflowEngine->>SynthesisAgent: run_agent(context_id=context_id)
    SynthesisAgent->>EngramLite: get_context_engrams(context_id)
    EngramLite-->>SynthesisAgent: return [finding1, finding2]
    SynthesisAgent->>EngramLite: store_memory(synthesis)
    EngramLite-->>SynthesisAgent: return synthesis_id
    SynthesisAgent->>EngramLite: create_connection(engram_id1, synthesis_id)
    SynthesisAgent->>EngramLite: create_connection(engram_id2, synthesis_id)
    SynthesisAgent-->>WorkflowEngine: return synthesis

    WorkflowEngine-->>Client: return final_result
```