# Report: Utilizing Engram-Lite as a Memory and Runtime for Mastra.ai Multi-Agent Workflows

## 1. Overview of Components

### Engram-Lite

Engram-Lite is a memory graph storage system designed for AI agents that provides:
- Structured knowledge representation via engrams, connections, collections, and contexts
- Persistent RocksDB-based storage with ACID transactions
- Graph-based relations using petgraph for fast traversals
- Efficient indexing for relationship queries
- gRPC API for remote access
- Vector embedding capabilities for semantic search

### Mastra.ai

Mastra.ai is an open-source TypeScript agent framework that offers:
- Agent development with memory and tool-calling capabilities
- Graph-based workflow engine for deterministic LLM call execution
- Retrieval-Augmented Generation (RAG) capabilities
- Support for multiple LLM providers (OpenAI, Anthropic, Google Gemini)
- Evaluation and observability features
- Integration with React, Next.js, and Node.js applications

## 2. Integration Architecture

### 2.1 Engram-Lite as an Enhanced Memory System

Engram-Lite can serve as an advanced memory layer for Mastra.ai agents through:

1. TypeScript Client Wrapper: Creating a TypeScript client that interacts with Engram-Lite's gRPC API:

```typescript
// Engram-Lite TypeScript client
class EngramLiteClient {
  private client: GrpcClient;

  constructor(host: string = "localhost", port: number = 50051) {
    this.client = new GrpcClient(`${host}:${port}`);
  }

  async storeMemory(content: string, source: string, confidence: number, metadata: Record<string, any> = {}): Promise<string> {
    const response = await this.client.createEngram({
      content,
      source,
      confidence,
      metadata
    });
    return response.engram.id;
  }

  async retrieveMemory(id: string): Promise<any> {
    const response = await this.client.getEngram({ id });
    return response.engram;
  }

  async searchByText(query: string, limit: number = 5): Promise<any[]> {
    const response = await this.client.searchByText({
      query,
      limit
    });
    return response.results;
  }

  // Additional methods for connections, collections, contexts...
}
```

2. Integration with Mastra.ai's Workflow Engine:

```typescript
import { workflow } from 'mastra';
import { EngramLiteClient } from './engram-lite-client';

// Initialize the memory client
const memoryClient = new EngramLiteClient();

// Create a workflow that uses Engram-Lite for memory
const researchWorkflow = workflow()
  .step('research', async (input, context) => {
    // Execute research task
    const researchResult = await context.model.generate({
      prompt: `Research information about: ${input.topic}`
    });

    // Store research results in Engram-Lite
    const engramId = await memoryClient.storeMemory(
      researchResult.content,
      'research_workflow',
      0.9,
      { topic: input.topic }
    );

    return {
      ...researchResult,
      engramId
    };
  })
  .step('summarize', async (input, context) => {
    // Retrieve relevant memories from Engram-Lite
    const relatedMemories = await memoryClient.searchByText(input.topic, 10);

    // Combine related memories as context
    const memories = relatedMemories.map(m => m.engram.content).join('\n\n');

    // Generate summary with context from memory
    const summary = await context.model.generate({
      prompt: `Summarize this information about ${input.topic}:\n\n${memories}`
    });

    // Store the summary in Engram-Lite
    const summaryId = await memoryClient.storeMemory(
      summary.content,
      'summary_workflow',
      0.95,
      { topic: input.topic, type: 'summary' }
    );

    // Create connection between research and summary
    await memoryClient.createConnection({
      sourceId: input.engramId,
      targetId: summaryId,
      relationshipType: 'summarized_as',
      weight: 1.0
    });

    return {
      summary: summary.content,
      summaryId
    };
  });
```

### 2.2 Engram-Lite as a Collaborative Agent Runtime

Engram-Lite's context mechanism can be used to create shared environments for Mastra.ai agents:

```typescript
import { agent } from 'mastra';
import { EngramLiteClient } from './engram-lite-client';

// Initialize memory client
const memoryClient = new EngramLiteClient();

// Create a collaborative context
async function createCollaborativeContext(name: string, description: string) {
  // Create context in Engram-Lite
  const contextResponse = await memoryClient.createContext({
    name,
    description
  });

  return contextResponse.context.id;
}

// Create researcher agent with memory access
const researcherAgent = agent('researcher')
  .description('Researches information and stores findings in memory')
  .tool('storeMemory', async (content: string, confidence: number, metadata: Record<string, any> = {}) => {
    const engramId = await memoryClient.storeMemory(
      content,
      'researcher_agent',
      confidence,
      metadata
    );
    return `Stored memory with ID: ${engramId}`;
  })
  .tool('addToContext', async (engramId: string, contextId: string) => {
    await memoryClient.addEngramToContext(engramId, contextId);
    return `Added engram ${engramId} to context ${contextId}`;
  });

// Create analyst agent with memory access
const analystAgent = agent('analyst')
  .description('Analyzes information from shared context and draws conclusions')
  .tool('getContextMemories', async (contextId: string) => {
    const memories = await memoryClient.getContextEngrams(contextId);
    return memories.map(m => m.content);
  })
  .tool('createConnection', async (sourceId: string, targetId: string, relationshipType: string) => {
    await memoryClient.createConnection({
      sourceId,
      targetId,
      relationshipType,
      weight: 0.9
    });
    return `Created connection from ${sourceId} to ${targetId}`;
  });
```

### 2.3 Graph-Based Workflow Orchestration

Leveraging Engram-Lite's graph structure to orchestrate complex Mastra.ai workflows:

```typescript
import { workflow, orchestrator } from 'mastra';
import { EngramLiteClient } from './engram-lite-client';

// Initialize memory client
const memoryClient = new EngramLiteClient();

// Define workflow steps as functions
async function researchStep(topic: string) {
  // Implementation...
}

async function analysisStep(researchId: string) {
  // Implementation...
}

async function summaryStep(analysisIds: string[]) {
  // Implementation...
}

// Create workflow orchestrator
const projectWorkflow = orchestrator()
  .registerStep('research', researchStep)
  .registerStep('analysis', analysisStep)
  .registerStep('summary', summaryStep);

// Store workflow structure in Engram-Lite
async function initializeWorkflow(name: string, description: string) {
  // Create a collection for the workflow
  const collectionResponse = await memoryClient.createCollection({
    name,
    description
  });

  const collectionId = collectionResponse.collection.id;

  // Store workflow steps as engrams
  const researchId = await memoryClient.storeMemory(
    'Research step: gather information on the topic',
    'workflow_definition',
    1.0,
    { step_type: 'research', position: 0 }
  );

  const analysisId = await memoryClient.storeMemory(
    'Analysis step: analyze research findings',
    'workflow_definition',
    1.0,
    { step_type: 'analysis', position: 1 }
  );

  const summaryId = await memoryClient.storeMemory(
    'Summary step: summarize analysis results',
    'workflow_definition',
    1.0,
    { step_type: 'summary', position: 2 }
  );

  // Add steps to collection
  await memoryClient.addEngramToCollection(researchId, collectionId);
  await memoryClient.addEngramToCollection(analysisId, collectionId);
  await memoryClient.addEngramToCollection(summaryId, collectionId);

  // Create connections between steps (workflow graph)
  await memoryClient.createConnection({
    sourceId: researchId,
    targetId: analysisId,
    relationshipType: 'next_step',
    weight: 1.0
  });

  await memoryClient.createConnection({
    sourceId: analysisId,
    targetId: summaryId,
    relationshipType: 'next_step',
    weight: 1.0
  });

  return {
    workflowId: collectionId,
    steps: {
      research: researchId,
      analysis: analysisId,
      summary: summaryId
    }
  };
}

// Execute workflow with Engram-Lite tracking
async function executeWorkflow(workflowId: string, input: any) {
  // Get workflow definition from Engram-Lite
  const workflow = await memoryClient.getCollection(workflowId);

  // Create context for this workflow execution
  const contextResponse = await memoryClient.createContext({
    name: `Execution of ${workflow.name}`,
    description: `Runtime context for workflow ${workflowId}`
  });

  const contextId = contextResponse.context.id;

  // Execute steps according to graph structure
  // For each step, store inputs and outputs in the context

  // Return final result and context ID for future reference
  return {
    result: finalOutput,
    contextId
  };
}
```

## 3. Key Integration Features

### 3.1 Enhanced Memory for Agents and Workflows

Mastra.ai agents can benefit from Engram-Lite's structured memory capabilities:

```typescript
// Define a Mastra agent with Engram-Lite memory
const assistantAgent = agent('assistant')
  .memory({
    store: async (key: string, value: any) => {
      await memoryClient.storeMemory(
        JSON.stringify(value),
        'assistant_memory',
        1.0,
        { memory_key: key }
      );
    },
    retrieve: async (key: string) => {
      const memories = await memoryClient.searchByMetadata('memory_key', key);
      if (memories.length > 0) {
        return JSON.parse(memories[0].engram.content);
      }
      return null;
    }
  })
  .conversation();
```

### 3.2 Shared State Across Workflow Stages

Using Engram-Lite contexts to maintain shared state across workflow steps:

```typescript
// Create a workflow with shared context
const analysisWorkflow = workflow()
  .initialize(async () => {
    // Create a shared context for this workflow run
    const contextResponse = await memoryClient.createContext({
      name: `Analysis Run ${Date.now()}`,
      description: 'Shared context for analysis workflow'
    });

    return { contextId: contextResponse.context.id };
  })
  .step('dataCollection', async (input, context, shared) => {
    // Collect data
    const data = await collectData(input.source);

    // Store in shared context
    const dataId = await memoryClient.storeMemory(
      JSON.stringify(data),
      'data_collection',
      1.0,
      { type: 'raw_data' }
    );
    await memoryClient.addEngramToContext(dataId, shared.contextId);

    return { dataId };
  })
  .step('dataProcessing', async (input, context, shared) => {
    // Get data from context
    const contextMemories = await memoryClient.getContextEngrams(shared.contextId);
    const rawData = contextMemories
      .filter(m => m.metadata.type === 'raw_data')
      .map(m => JSON.parse(m.content));

    // Process data
    const processedData = processData(rawData);

    // Store processed results
    const processedId = await memoryClient.storeMemory(
      JSON.stringify(processedData),
      'data_processing',
      1.0,
      { type: 'processed_data' }
    );
    await memoryClient.addEngramToContext(processedId, shared.contextId);

    return { processedId };
  });
```

### 3.3 Graph-Based Navigation for Complex Workflows

Using Engram-Lite's graph capabilities to enable dynamic workflow paths:

```typescript
// Create a workflow with dynamic branching based on memory graph
const decisionWorkflow = workflow()
  .step('analyze', async (input, context) => {
    // Analyze input
    const analysis = await context.model.generate({
      prompt: `Analyze the following situation: ${input.situation}`
    });

    // Store in Engram-Lite
    const analysisId = await memoryClient.storeMemory(
      analysis.content,
      'decision_analysis',
      0.9
    );

    // Determine next steps based on graph traversal
    const connectedOptions = await memoryClient.findConnections({
      engramId: analysisId,
      direction: 'OUTGOING',
      relationshipTypes: ['suggests_action'],
      maxDepth: 1
    });

    // Return analysis and possible next steps
    return {
      analysis: analysis.content,
      options: connectedOptions.paths.map(p => ({
        action: p.connections[0].target_id,
        weight: p.connections[0].weight
      }))
    };
  })
  .branch('decide', async (input, context) => {
    // Choose highest-weighted option
    const bestOption = input.options.sort((a, b) => b.weight - a.weight)[0];

    // Return decision for branching
    return {
      path: bestOption.action
    };
  }, {
    // Dynamic branches based on graph connections
    paths: async (input) => {
      // Get all potential actions from Engram-Lite
      const actions = await memoryClient.searchByMetadata('type', 'action');

      // Create dynamic paths
      return actions.map(action => ({
        id: action.id,
        handler: async (input, context) => {
          // Execute the selected action
          const actionEngram = await memoryClient.getEngram(action.id);
          const actionResult = await executeAction(actionEngram.content, input);

          return { result: actionResult };
        }
      }));
    }
  });
```

### 3.4 Semantic Search for Knowledge Retrieval

Integrating Engram-Lite's vector search with Mastra.ai's RAG capabilities:

```typescript
// Create a RAG workflow using Engram-Lite for semantic search
const ragWorkflow = workflow()
  .step('query', async (input, context) => {
    // Generate embedding for query using Mastra's embedding functionality
    const embedding = await context.embeddings.embed(input.query);

    // Use Engram-Lite's vector search
    const searchResults = await memoryClient.searchByVector({
      embedding: { vector: embedding, model: 'mastra_default', dimensions: embedding.length },
      limit: 5
    });

    // Format results for the model
    const context = searchResults.map(r => r.engram.content).join('\n\n');

    return { query: input.query, context };
  })
  .step('generate', async (input, context) => {
    // Generate response with retrieved context
    const response = await context.model.generate({
      prompt: `Context information:\n${input.context}\n\nBased on this context, answer the following question: ${input.query}`
    });

    return { response: response.content };
  });
```

## 4. Implementation Guide

### 4.1 Setting Up Engram-Lite as a Service for Mastra.ai

1. Build and run Engram-Lite as a gRPC server:
```bash
cargo build --release --features="grpc"
./target/release/engram_server --db-path /path/to/database --host 0.0.0.0 --port 50051
```

2. Create a TypeScript client for Mastra.ai:
```typescript
// engram-lite-client.ts
import * as grpc from '@grpc/grpc-js';
import * as protoLoader from '@grpc/proto-loader';
import * as path from 'path';

// Load protobuf definition
const PROTO_PATH = path.resolve(__dirname, './proto/engram.proto');
const packageDefinition = protoLoader.loadSync(PROTO_PATH);
const protoDescriptor = grpc.loadPackageDefinition(packageDefinition);
const engramService = protoDescriptor.engram.v1.EngramService;

export class EngramLiteClient {
  private client: any;

  constructor(host: string = 'localhost', port: number = 50051) {
    this.client = new engramService(`${host}:${port}`, grpc.credentials.createInsecure());
  }

  // Core CRUD operations
  async createEngram(data: any): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.CreateEngram(data, (err: any, response: any) => {
        if (err) reject(err);
        else resolve(response);
      });
    });
  }

  async getEngram(id: string): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.GetEngram({ id }, (err: any, response: any) => {
        if (err) reject(err);
        else resolve(response);
      });
    });
  }

  // Search operations
  async searchByText(query: string, limit: number = 10): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.SearchByText({ query, limit }, (err: any, response: any) => {
        if (err) reject(err);
        else resolve(response);
      });
    });
  }

  async searchByVector(request: any): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.SearchByVector(request, (err: any, response: any) => {
        if (err) reject(err);
        else resolve(response);
      });
    });
  }

  // Context operations
  async createContext(data: any): Promise<any> {
    return new Promise((resolve, reject) => {
      this.client.CreateContext(data, (err: any, response: any) => {
        if (err) reject(err);
        else resolve(response);
      });
    });
  }

  // ... additional methods for other operations
}
```

### 4.2 Integrating with Mastra.ai Agents and Workflows

1. Create a memory provider for Mastra.ai agents:
```typescript
// engram-memory-provider.ts
import { EngramLiteClient } from './engram-lite-client';

export class EngramMemoryProvider {
  private client: EngramLiteClient;
  private source: string;

  constructor(source: string, host: string = 'localhost', port: number = 50051) {
    this.client = new EngramLiteClient(host, port);
    this.source = source;
  }

  async store(key: string, value: any): Promise<string> {
    const response = await this.client.createEngram({
      content: typeof value === 'string' ? value : JSON.stringify(value),
      source: this.source,
      confidence: 1.0,
      metadata: { key }
    });

    return response.engram.id;
  }

  async retrieve(key: string): Promise<any> {
    const response = await this.client.searchByText({
      query: key,
      limit: 1,
      metadata_filters: { key }
    });

    if (response.results && response.results.length > 0) {
      const content = response.results[0].engram.content;
      try {
        return JSON.parse(content);
      } catch {
        return content;
      }
    }

    return null;
  }
}

// In your Mastra agent setup
import { agent } from 'mastra';
import { EngramMemoryProvider } from './engram-memory-provider';

const memoryProvider = new EngramMemoryProvider('assistant_agent');

const assistant = agent('assistant')
  .memory(memoryProvider)
  .conversation();
```

2. Create a workflow context manager:
```typescript
// engram-workflow-context.ts
import { EngramLiteClient } from './engram-lite-client';

export class EngramWorkflowContext {
  private client: EngramLiteClient;
  private contextId: string | null = null;

  constructor(host: string = 'localhost', port: number = 50051) {
    this.client = new EngramLiteClient(host, port);
  }

  async initialize(name: string, description: string): Promise<string> {
    const response = await this.client.createContext({
      name,
      description
    });

    this.contextId = response.context.id;
    return this.contextId;
  }

  async storeData(content: any, metadata: Record<string, any> = {}): Promise<string> {
    if (!this.contextId) {
      throw new Error('Context not initialized');
    }

    const contentStr = typeof content === 'string' ? content : JSON.stringify(content);

    const engramResponse = await this.client.createEngram({
      content: contentStr,
      source: 'workflow_execution',
      confidence: 1.0,
      metadata
    });

    const engramId = engramResponse.engram.id;

    await this.client.addEngramToContext(engramId, this.contextId);

    return engramId;
  }

  async getData(filter: Record<string, any> = {}): Promise<any[]> {
    if (!this.contextId) {
      throw new Error('Context not initialized');
    }

    const contextEngrams = await this.client.getContextEngrams(this.contextId);

    // Filter engrams based on metadata
    const filtered = contextEngrams.filter(engram => {
      for (const [key, value] of Object.entries(filter)) {
        if (engram.metadata[key] !== value) {
          return false;
        }
      }
      return true;
    });

    // Parse content if JSON
    return filtered.map(engram => {
      try {
        return {
          id: engram.id,
          content: JSON.parse(engram.content),
          metadata: engram.metadata
        };
      } catch {
        return {
          id: engram.id,
          content: engram.content,
          metadata: engram.metadata
        };
      }
    });
  }
}

// In your Mastra workflow
import { workflow } from 'mastra';
import { EngramWorkflowContext } from './engram-workflow-context';

const workflowContext = new EngramWorkflowContext();

const dataProcessingWorkflow = workflow()
  .initialize(async () => {
    const contextId = await workflowContext.initialize(
      'Data Processing Flow',
      'Workflow for processing and analyzing data'
    );
    return { contextId };
  })
  .step('ingest', async (input, context) => {
    // Process input data
    const processedData = transformData(input.data);

    // Store in context
    const dataId = await workflowContext.storeData(processedData, {
      type: 'processed_data',
      step: 'ingest'
    });

    return { dataId };
  });
  // Additional steps...
```

### 4.3 Building an Agent Runtime with Engram-Lite

1. Create an agent manager that uses Engram-Lite for coordination:
```typescript
// engram-agent-manager.ts
import { agent } from 'mastra';
import { EngramLiteClient } from './engram-lite-client';

export class EngramAgentManager {
  private client: EngramLiteClient;
  private agents: Map<string, any> = new Map();

  constructor(host: string = 'localhost', port: number = 50051) {
    this.client = new EngramLiteClient(host, port);
  }

  async registerAgent(agentId: string, agentDefinition: any) {
    // Create agent in Engram-Lite
    const agentResponse = await this.client.createAgent({
      name: agentDefinition.name,
      description: agentDefinition.description,
      capabilities: agentDefinition.capabilities || []
    });

    // Store agent reference
    this.agents.set(agentId, agentDefinition);

    return agentResponse.agent.id;
  }

  async createCollaborationContext(name: string, description: string, agentIds: string[]) {
    // Create a context for collaboration
    const contextResponse = await this.client.createContext({
      name,
      description
    });

    const contextId = contextResponse.context.id;

    // Add agents to context
    for (const agentId of agentIds) {
      await this.client.addAgentToContext(agentId, contextId);
    }

    return contextId;
  }

  async executeAgentTask(agentId: string, contextId: string, task: any) {
    // Get agent
    const agentDef = this.agents.get(agentId);
    if (!agentDef) {
      throw new Error(`Agent ${agentId} not found`);
    }

    // Get context memories
    const contextEngrams = await this.client.getContextEngrams(contextId);
    const contextData = contextEngrams.map(e => e.content).join('\n\n');

    // Execute agent task
    const result = await agentDef.execute(task, { contextData });

    // Store result in context
    const resultId = await this.client.createEngram({
      content: typeof result === 'string' ? result : JSON.stringify(result),
      source: agentId,
      confidence: 0.9,
      metadata: { task_id: task.id, type: 'task_result' }
    });

    await this.client.addEngramToContext(resultId, contextId);

    return {
      result,
      resultId
    };
  }
}
```

## 5. Advantages of the Integration

1. **Structured Memory**: Engram-Lite provides a more sophisticated memory model than typical key-value stores, enabling complex knowledge relationships.
2. **Graph-Based Workflows**: Workflow dependencies and relationships can be explicitly modeled and traversed in the graph.
3. **Context-Aware Collaboration**: Agents can share knowledge through contexts rather than direct message passing.
4. **Persistent Memory**: Information persists across workflow executions, enabling long-term learning.
5. **Metadata-Rich Knowledge**: Engram-Lite's metadata system enables filtering and organizing knowledge beyond basic content.
6. **Typed Relationships**: The connection types in Engram-Lite enable modeling different types of relationships between knowledge units.
7. **Vector Search**: Semantic search capabilities complement Mastra.ai's RAG functionality.

## 6. Conclusion

Engram-Lite provides an ideal memory and runtime infrastructure for Mastra.ai-based multi-agent systems by offering:

1. A persistent, structured knowledge graph beyond simple key-value pairs
2. Context-based collaborative environments for multi-agent interaction
3. Sophisticated query capabilities for retrieving relevant knowledge
4. A graph-based model that naturally complements workflow structures
5. Vector embedding support for semantic search

By integrating Engram-Lite with Mastra.ai, developers can create more sophisticated agent systems with:
- Persistent memory across sessions
- Complex knowledge relationships
- Collaborative agent environments
- Structured workflows with shared context
- Semantic search capabilities

The integration leverages the strengths of both systems: Mastra.ai's TypeScript-based agent and workflow engine combined with Engram-Lite's sophisticated memory graph system, creating a powerful platform for building advanced multi-agent applications.