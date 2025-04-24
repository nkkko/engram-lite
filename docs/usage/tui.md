# Terminal User Interface

> **Note**: The Terminal User Interface (TUI) is currently under development and not yet available in the main release.

The EngramAI Lite TUI will provide an interactive terminal-based interface for visualizing and interacting with the memory graph.

## Planned Features

- Interactive graph visualization
- Multiple panels for viewing different entity types
- Command input with auto-completion
- Search and filter capabilities
- Context-sensitive help
- Keyboard shortcuts for efficient navigation

## Technology

The TUI will be built using [ratatui](https://github.com/ratatui-org/ratatui), a Rust library for building rich terminal user interfaces.

## Preview

```
┌─EngramAI Lite TUI────────────────────────────────────────────────────────┐
│                                                                           │
│ ┌─Graph View─────────────────────────┐ ┌─Engram Details─────────────────┐ │
│ │                                     │ │ ID: 3a7c9f8e-1234-5678-90ab-  │ │
│ │            ┌───────┐               │ │    cdef01234567                │ │
│ │            │ Paris │               │ │ Content: The capital of France │ │
│ │            └───┬───┘               │ │          is Paris              │ │
│ │                │                   │ │ Source: geography              │ │
│ │                │ related           │ │ Confidence: 0.95               │ │
│ │                │                   │ │ Timestamp: 2023-06-15T14:30:22Z│ │
│ │        ┌───────┴──────┐           │ │                                 │ │
│ │        │ Eiffel Tower │           │ │ ┌─Connections───────────────────┐ │
│ │        └──────────────┘           │ │ │ → Eiffel Tower (related, 0.8) │ │
│ │                                     │ │                                 │ │
│ └─────────────────────────────────────┘ └─────────────────────────────────┘ │
│                                                                           │
│ ┌─Command Input──────────────────────────────────────────────────────────┐ │
│ │ > get-engram 3a7c9f8e-1234-5678-90ab-cdef01234567                     │ │
│ └───────────────────────────────────────────────────────────────────────┘ │
│                                                                           │
│ Status: Ready | DB: engram_db | Engrams: 24 | Connections: 37            │
└───────────────────────────────────────────────────────────────────────────┘
```

## Usage (Future)

```bash
# Launch the TUI
engramlt tui

# Specify a database path
engramlt tui --db-path /path/to/database
```

## Stay Tuned

The TUI is under active development as part of Milestone 1. Check back for updates in future releases!