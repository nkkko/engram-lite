# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
EngramAI is a knowledge memory graph system for AI agents, designed to store and retrieve information in a structured way that mimics human cognitive processes. The system organizes knowledge as "engrams" (knowledge units) connected in a semantic network with typed relationships, supporting context-aware memory retrieval and multi-agent collaboration.

## IMPORTANT
- Never do mocks, never hardcode stuff just to get everything works.
- Always use tests and benchmarks to learn what needs to be improved.
- Always fix things.

## Build/Run Commands
- Run prototype: `python prototype.py`
- Test specific functionality: `python -c "from prototype import <class>; <test_code>"`
- Run agent collaboration demo: `python agent_collaboration.py`
- Run memory analysis: `python memory_analysis.py --input memory_graph.json --analyze-forgetting --health-report`
- Generate visualizations: `python memory_analysis.py --input memory_graph.json --visualize --detect-communities`

## Core Components
- **Engram**: Atomic unit of knowledge with metadata (confidence, timestamp, source)
- **Connection**: Typed relationship between engrams with strength/weight
- **Collection**: Named grouping of engrams for organization
- **Agent**: Entity with capabilities and access controls
- **Context**: Shareable environment with relevant engrams for agent collaboration
- **MemoryGraph**: Graph database implementation using NetworkX

## Code Style Guidelines
- Use typed annotations (Dict, List, Optional, etc.) from the typing module
- Maintain class separation between core components
- Follow NetworkX conventions for graph operations
- Keep docstrings descriptive and up-to-date
- Include type hints and return value documentation

## Memory Operations
- **Storage**: Add engrams, connections, collections, agents, and contexts to the graph
- **Retrieval**: Get engrams by various criteria (ID, source, confidence, recency)
- **Relationships**: Track and query connections between knowledge units
- **Access Control**: Manage agent permissions for collections and contexts
- **Analysis**: Identify forgetting candidates, abstractions, and community structure

## Naming Conventions
- Classes: CamelCase (e.g., MemoryGraph, DataGenerator)
- Methods/functions: snake_case (e.g., add_engram, get_connections_between)
- Variables: descriptive snake_case (e.g., engram_id, connection_type)
- Constants: UPPER_CASE with underscores

## Error Handling
- Validate parameters before graph operations
- Use try/except blocks for external API calls and file operations
- Return structured error responses from agent methods
- Check for existence of engrams before creating connections

## Import Organization
- Standard library imports first (uuid, datetime, json)
- Third-party dependencies second (networkx, matplotlib, anthropic)
- Module-specific imports last

## Future Development Areas
- Vector embeddings for semantic search capabilities
- Database integration for persistence (replacing in-memory NetworkX)
- Forgetting mechanisms and abstraction policies
- Temporal reasoning and time-based queries
- Performance optimizations for large graphs
- Improved LLM integration with dedicated tools

## Commit Messages
- Ensure that commit messages follow a consistent format and provide clear descriptions of changes made to the code.
- Never attribute commits to Claude Code