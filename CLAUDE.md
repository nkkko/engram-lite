# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
EngramAI is a knowledge memory graph system for AI agents, designed to store and retrieve information in a structured way that mimics human cognitive processes. The system organizes knowledge as "engrams" (knowledge units) connected in a semantic network with typed relationships, supporting context-aware memory retrieval and multi-agent collaboration.

## IMPORTANT
- Never do mocks, never hardcode stuff just to get everything works.
- Always use tests and benchmarks to learn what needs to be improved.
- Always fix things.
- Always commit and push to git, decide on the appropriate semver number and tag the release for minor versions

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

## Versioning and Release Process

### Version Numbering
- Follow semantic versioning (SemVer) with MAJOR.MINOR.PATCH format:
  - MAJOR: Breaking changes that require significant adjustments by users
  - MINOR: New features and functionality in a backward-compatible manner
  - PATCH: Backward-compatible bug fixes and minor improvements
- Example: 0.3.1 indicates development phase, third feature milestone, first patch

### Release Process
1. Update version number in:
   - Cargo.toml (version = "x.y.z")
   - src/main.rs (or other appropriate version declaration file)
   - README.md (if version is mentioned)

2. Run the full test suite:
   ```bash
   cargo test
   ```

3. Create a version tag in git:
   ```bash
   git tag -a v0.3.1 -m "Release version 0.3.1"
   ```

4. Push the tag to remote repository:
   ```bash
   git push origin v0.3.1
   ```

5. Create a GitHub release (if applicable):
   - Draft a new release on GitHub
   - Select the version tag
   - Include release notes with changes, improvements, and fixes
   - Attach any compiled binaries if appropriate

6. On milestone completion (when all tasks in a milestone are done):
   - Increment MINOR version number (e.g., 0.3.0 → 0.4.0)
   - Tag and push as described above

### Version Archive
- Always keep at least 3 previous versions accessible
- Do not delete old version tags
- Store compiled binaries for major releases
- Document breaking changes between major versions

### Reverting to Previous Versions
To check out a specific version for testing or comparison:
```bash
git checkout tags/v0.2.3
```

To return to the current development branch:
```bash
git checkout main
```

### Version Documentation
Include a CHANGELOG.md file that documents:
- Version number and release date
- New features added in each version
- Bug fixes and improvements
- Known issues and limitations
- Breaking changes and migration paths

## Commit Messages
- Ensure that commit messages follow a consistent format and provide clear descriptions of changes made to the code.
- Never attribute commits to Claude Code
- Format: "[type] Brief description of changes"
  - Types: feat, fix, docs, style, refactor, test, chore
  - Example: "[feat] Add vector embedding support for engrams"