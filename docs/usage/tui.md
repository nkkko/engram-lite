# User Interfaces

EngramAI Lite includes both a Terminal User Interface (TUI) mode and a Web User Interface for visualizing and interacting with the memory graph. The TUI provides a terminal-based interactive experience, while the Web UI offers a browser-based interface for more visual exploration.

## Features

- Dashboard overview with system statistics
- Tabbed interface to navigate between different entity types:
  - Engrams - Knowledge units stored in the system
  - Connections - Relationships between engrams
  - Collections - Named groupings of engrams
  - Agents - Entities with capabilities and access controls
  - Contexts - Shareable environments with relevant engrams
- Detailed entity inspection
- Command input with history
- Keyboard navigation

## Technology

The TUI is built using [ratatui](https://github.com/ratatui-org/ratatui) v0.29.0, a Rust library for building rich terminal user interfaces, and [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation.

## Installation

The TUI features are optional and require the `tui` feature flag to be enabled:

```bash
# Install EngramAI Lite with TUI support
cargo install --path . --features=tui
```

If you're building from source, you'll need to include the `tui` feature flag:

```bash
cargo build --features=tui
```

## Usage

```bash
# Launch the TUI
engramlt tui

# Specify a database path
engramlt tui --db-path /path/to/database
```

### Terminal Requirements

The TUI requires a compatible terminal with support for:
- ANSI escape sequences
- Direct terminal access (not redirected I/O)
- Raw mode support

For best results, run the TUI in a native terminal application rather than through an IDE's integrated terminal or other intermediary.

### Fallback Mode

If the TUI cannot initialize the terminal (e.g., due to incompatible terminal, insufficient permissions, or running in a non-interactive environment), it will automatically fall back to a simplified CLI mode that displays:

- System statistics
- Recent engrams
- Basic information

The fallback mode does not support interactive navigation or commands but ensures that the application can still provide useful information when full TUI functionality is not available.

## Navigation

| Key       | Action                       |
|-----------|------------------------------|
| 1-8       | Switch between tabs          |
| Tab       | Cycle to next tab            |
| Up/Down   | Navigate lists               |
| Esc       | Reset selection              |
| e         | Enter command mode           |
| c         | Go to command tab and enter command mode |
| Enter     | Execute command              |
| q         | Quit application             |

## Available Commands

| Command           | Description                   |
|-------------------|-------------------------------|
| help              | Show help message             |
| stats             | Show system statistics        |
| list-engrams      | List all engrams              |
| list-connections  | List all connections          |
| list-collections  | List all collections          |
| list-agents       | List all agents               |
| list-contexts     | List all contexts             |
| refresh           | Refresh data from storage     |
| exit, quit        | Exit the application          |

## Screenshots

```
┌─Navigation──────────────────────────────────────────────────────────────────┐
│Dashboard  Engrams  Connections  Collections  Agents  Contexts  Commands  Help│
└─────────────────────────────────────────────────────────────────────────────┘
┌─Dashboard──────────────────────────────────────────────────────────────────┐
│EngramAI System Statistics                                                   │
│Engrams: 3 | Connections: 2 | Collections: 1 | Agents: 1 | Contexts: 1      │
└─────────────────────────────────────────────────────────────────────────────┘
┌─Recent Engrams──────────────────────┐┌─Recent Connections────────────────────┐
│[e-001] Climate change is            ││[c-001] e-001 → e-003 (causes)        │
│accelerating faster than predicted.  ││[c-002] e-002 → e-003 (supports)      │
│[e-002] Solar panels are becoming    ││                                       │
│more affordable and efficient.       ││                                       │
│[e-003] Renewable energy can replace ││                                       │
│fossil fuels for most applications.  ││                                       │
└───────────────────────────────────┘└───────────────────────────────────────┘
┌─Command Input──────────────────────────────────────────────────────────────┐
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

# Web User Interface

The Web UI provides a browser-based interface for exploring and interacting with the memory graph.

## Features

- Dashboard with system statistics
- Navigation to different entity types:
  - Engrams - Knowledge units with content and metadata
  - Connections - Typed relationships between engrams
  - Collections - Named groupings of engrams with details view
  - Agents - Entities with capabilities and metadata
- API documentation page
- Demo data integration
- Responsive design

## Usage

```bash
# Start the web server
engramlt web

# Specify a database path
engramlt web --db-path /path/to/database

# Specify a custom port (default is 3000)
engramlt web --port 8080 --db-path /path/to/database
```

## Demo Data

To populate the database with realistic demo data showcasing a multi-agent collaboration scenario:

```bash
# Generate demo data
engramlt demo --db-path /path/to/database

# Start the web server to view the demo data
engramlt web --db-path /path/to/database
```

The demo data includes:
- Four agent types (developer, tester, documenter, project manager)
- Multiple engram types (code snippets, documentation, tests, discussions)
- Typed relationships between engrams
- Collections for organizing the project data
- A shared context for collaboration

## Accessing the Web UI

After starting the web server, you can access the Web UI at:

```
http://localhost:3000
```

## Future Enhancements

The following enhancements are planned for future releases:

- Interactive graph visualization
- Advanced search and filtering
- Command auto-completion
- Visual query builder
- Export/import functionality
- Keyboard shortcuts customization
- Data manipulation forms
- Interactive query interface