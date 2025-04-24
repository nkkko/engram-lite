# Terminal User Interface

EngramAI Lite includes a Terminal User Interface (TUI) mode that provides an interactive terminal-based interface for visualizing and interacting with the memory graph.

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

## Usage

```bash
# Launch the TUI
engramlt tui

# Specify a database path
engramlt tui --db-path /path/to/database
```

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

## Future Enhancements

The following enhancements are planned for future releases:

- Interactive graph visualization
- Advanced search and filtering
- Command auto-completion
- Visual query builder
- Export/import functionality
- Keyboard shortcuts customization