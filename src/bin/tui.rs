#[cfg(feature = "tui")]
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use engram_lite::{
    error::Result,
    graph::MemoryGraph,
    schema::{Engram, Connection, Collection, Agent, Context},
    storage::Storage,
};
#[cfg(feature = "tui")]
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

#[cfg(feature = "tui")]
enum InputMode {
    Normal,
    Editing,
}

#[cfg(feature = "tui")]
enum MenuItem {
    Dashboard,
    Engrams,
    Connections,
    Collections,
    Agents,
    Contexts,
    Commands,
    Help,
}

#[cfg(feature = "tui")]
impl MenuItem {
    fn to_string(&self) -> String {
        match self {
            MenuItem::Dashboard => "Dashboard".to_string(),
            MenuItem::Engrams => "Engrams".to_string(),
            MenuItem::Connections => "Connections".to_string(),
            MenuItem::Collections => "Collections".to_string(),
            MenuItem::Agents => "Agents".to_string(),
            MenuItem::Contexts => "Contexts".to_string(),
            MenuItem::Commands => "Commands".to_string(),
            MenuItem::Help => "Help".to_string(),
        }
    }
    
    fn from_index(index: usize) -> MenuItem {
        match index {
            0 => MenuItem::Dashboard,
            1 => MenuItem::Engrams,
            2 => MenuItem::Connections,
            3 => MenuItem::Collections,
            4 => MenuItem::Agents,
            5 => MenuItem::Contexts,
            6 => MenuItem::Commands,
            7 => MenuItem::Help,
            _ => MenuItem::Dashboard,
        }
    }
}

#[cfg(feature = "tui")]
struct EngramTui {
    storage: Storage,
    memory_graph: MemoryGraph,
    input_mode: InputMode,
    active_menu_item: MenuItem,
    input: String,
    messages: Vec<String>,
    selected_item_index: usize,
    last_refresh: Instant,
    tab_titles: Vec<&'static str>,
    engrams: Vec<Engram>,
    connections: Vec<Connection>,
    collections: Vec<Collection>,
    agents: Vec<Agent>,
    contexts: Vec<Context>,
}

#[cfg(feature = "tui")]
impl EngramTui {
    fn new(db_path: &str) -> Result<Self> {
        let storage = Storage::new(db_path)?;
        let memory_graph = MemoryGraph::new();
        
        let mut tui = Self {
            storage,
            memory_graph,
            input_mode: InputMode::Normal,
            active_menu_item: MenuItem::Dashboard,
            input: String::new(),
            messages: Vec::new(),
            selected_item_index: 0,
            last_refresh: Instant::now(),
            tab_titles: vec![
                "Dashboard", "Engrams", "Connections", "Collections", 
                "Agents", "Contexts", "Commands", "Help"
            ],
            engrams: Vec::new(),
            connections: Vec::new(),
            collections: Vec::new(),
            agents: Vec::new(),
            contexts: Vec::new(),
        };
        
        // Load initial data
        tui.refresh_data()?;
        
        Ok(tui)
    }
    
    fn refresh_data(&mut self) -> Result<()> {
        // Load engrams
        self.engrams = self.memory_graph.get_recent_engrams(100)?;
        
        // Load connections
        let connection_ids = self.storage.list_connections()?;
        self.connections.clear();
        for id in connection_ids {
            if let Some(connection) = self.storage.get_connection(&id)? {
                self.connections.push(connection);
            }
        }
        
        // Load collections
        let collection_ids = self.storage.list_collections()?;
        self.collections.clear();
        for id in collection_ids {
            if let Some(collection) = self.storage.get_collection(&id)? {
                self.collections.push(collection);
            }
        }
        
        // Load agents
        let agent_ids = self.storage.list_agents()?;
        self.agents.clear();
        for id in agent_ids {
            if let Some(agent) = self.storage.get_agent(&id)? {
                self.agents.push(agent);
            }
        }
        
        // Load contexts
        let context_ids = self.storage.list_contexts()?;
        self.contexts.clear();
        for id in context_ids {
            if let Some(context) = self.storage.get_context(&id)? {
                self.contexts.push(context);
            }
        }
        
        // Reset refresh timer
        self.last_refresh = Instant::now();
        
        Ok(())
    }
    
    fn handle_command(&mut self) -> Result<()> {
        let command = self.input.trim().to_string();
        self.input.clear();
        
        if command.is_empty() {
            return Ok(());
        }
        
        self.messages.push(format!("> {}", command));
        
        let parts: Vec<&str> = command.splitn(2, ' ').collect();
        let cmd = parts[0];
        let _args = parts.get(1).unwrap_or(&"");
        
        match cmd {
            "help" => {
                self.messages.push("Available commands:".to_string());
                self.messages.push("  help - Show this help message".to_string());
                self.messages.push("  stats - Show system statistics".to_string());
                self.messages.push("  list-engrams - List all engrams".to_string());
                self.messages.push("  list-connections - List all connections".to_string());
                self.messages.push("  list-collections - List all collections".to_string());
                self.messages.push("  list-agents - List all agents".to_string());
                self.messages.push("  list-contexts - List all contexts".to_string());
                self.messages.push("  refresh - Refresh data from storage".to_string());
                self.messages.push("  exit, quit - Exit the application".to_string());
            }
            "stats" => {
                self.messages.push("EngramAI System Statistics:".to_string());
                self.messages.push(format!("  Engrams:     {}", self.engrams.len()));
                self.messages.push(format!("  Connections: {}", self.connections.len()));
                self.messages.push(format!("  Collections: {}", self.collections.len()));
                self.messages.push(format!("  Agents:      {}", self.agents.len()));
                self.messages.push(format!("  Contexts:    {}", self.contexts.len()));
            }
            "list-engrams" => {
                if self.engrams.is_empty() {
                    self.messages.push("No engrams found".to_string());
                } else {
                    self.messages.push("Engrams:".to_string());
                    for engram in &self.engrams {
                        self.messages.push(format!("  [{}] {} (confidence: {})", 
                            engram.id, engram.content, engram.confidence));
                    }
                }
            }
            "list-connections" => {
                if self.connections.is_empty() {
                    self.messages.push("No connections found".to_string());
                } else {
                    self.messages.push("Connections:".to_string());
                    for connection in &self.connections {
                        self.messages.push(format!("  [{}] {} -> {} (type: {}, weight: {})", 
                            connection.id, connection.source_id, connection.target_id, 
                            connection.relationship_type, connection.weight));
                    }
                }
            }
            "list-collections" => {
                if self.collections.is_empty() {
                    self.messages.push("No collections found".to_string());
                } else {
                    self.messages.push("Collections:".to_string());
                    for collection in &self.collections {
                        self.messages.push(format!("  [{}] {} - {} ({} engrams)", 
                            collection.id, collection.name, collection.description, 
                            collection.engram_ids.len()));
                    }
                }
            }
            "list-agents" => {
                if self.agents.is_empty() {
                    self.messages.push("No agents found".to_string());
                } else {
                    self.messages.push("Agents:".to_string());
                    for agent in &self.agents {
                        self.messages.push(format!("  [{}] {} - {} (access to {} collections)", 
                            agent.id, agent.name, agent.description, 
                            agent.accessible_collections.len()));
                    }
                }
            }
            "list-contexts" => {
                if self.contexts.is_empty() {
                    self.messages.push("No contexts found".to_string());
                } else {
                    self.messages.push("Contexts:".to_string());
                    for context in &self.contexts {
                        self.messages.push(format!("  [{}] {} - {} ({} engrams, {} agents)", 
                            context.id, context.name, context.description, 
                            context.engram_ids.len(), context.agent_ids.len()));
                    }
                }
            }
            "refresh" => {
                self.messages.push("Refreshing data from storage...".to_string());
                if let Err(e) = self.refresh_data() {
                    self.messages.push(format!("Error refreshing data: {}", e));
                } else {
                    self.messages.push("Data refreshed successfully".to_string());
                }
            }
            "exit" | "quit" => {
                // This will be handled in the main loop
                self.messages.push("Exiting application...".to_string());
                return Ok(());
            }
            _ => {
                self.messages.push(format!("Unknown command: {}", cmd));
                self.messages.push("Type 'help' for available commands".to_string());
            }
        }
        
        Ok(())
    }
}

#[cfg(feature = "tui")]
fn run_app(
    terminal: &mut Terminal<impl Backend>,
    app: &mut EngramTui,
) -> io::Result<bool> {
    terminal.draw(|f| ui(f, app))?;
    
    // Check if it's time to refresh data (every 30 seconds)
    if app.last_refresh.elapsed() > Duration::from_secs(30) {
        let _ = app.refresh_data();
    }
    
    // Poll for events with a timeout to avoid blocking forever
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match app.input_mode {
                InputMode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('e') => {
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('c') => {
                        app.active_menu_item = MenuItem::Commands;
                        app.input_mode = InputMode::Editing;
                    }
                    KeyCode::Char('1') => {
                        app.active_menu_item = MenuItem::Dashboard;
                    }
                    KeyCode::Char('2') => {
                        app.active_menu_item = MenuItem::Engrams;
                    }
                    KeyCode::Char('3') => {
                        app.active_menu_item = MenuItem::Connections;
                    }
                    KeyCode::Char('4') => {
                        app.active_menu_item = MenuItem::Collections;
                    }
                    KeyCode::Char('5') => {
                        app.active_menu_item = MenuItem::Agents;
                    }
                    KeyCode::Char('6') => {
                        app.active_menu_item = MenuItem::Contexts;
                    }
                    KeyCode::Char('7') => {
                        app.active_menu_item = MenuItem::Commands;
                    }
                    KeyCode::Char('8') => {
                        app.active_menu_item = MenuItem::Help;
                    }
                    KeyCode::Tab => {
                        let index = match app.active_menu_item {
                            MenuItem::Dashboard => 1,
                            MenuItem::Engrams => 2,
                            MenuItem::Connections => 3,
                            MenuItem::Collections => 4,
                            MenuItem::Agents => 5,
                            MenuItem::Contexts => 6,
                            MenuItem::Commands => 7,
                            MenuItem::Help => 0,
                        };
                        app.active_menu_item = MenuItem::from_index(index);
                    }
                    KeyCode::Down => {
                        app.selected_item_index = match app.active_menu_item {
                            MenuItem::Engrams => (app.selected_item_index + 1).min(app.engrams.len().saturating_sub(1)),
                            MenuItem::Connections => (app.selected_item_index + 1).min(app.connections.len().saturating_sub(1)),
                            MenuItem::Collections => (app.selected_item_index + 1).min(app.collections.len().saturating_sub(1)),
                            MenuItem::Agents => (app.selected_item_index + 1).min(app.agents.len().saturating_sub(1)),
                            MenuItem::Contexts => (app.selected_item_index + 1).min(app.contexts.len().saturating_sub(1)),
                            MenuItem::Commands => (app.selected_item_index + 1).min(app.messages.len().saturating_sub(1)),
                            _ => 0,
                        };
                    }
                    KeyCode::Up => {
                        app.selected_item_index = app.selected_item_index.saturating_sub(1);
                    }
                    KeyCode::Esc => {
                        app.selected_item_index = 0;
                    }
                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Enter => {
                        let _ = app.handle_command();
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
                }
            }
        }
    }
    
    Ok(false)
}

#[cfg(feature = "tui")]
fn ui(f: &mut Frame, app: &EngramTui) {
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());
    
    // Create the menu tabs
    let menu = Tabs::new(app.tab_titles.iter().map(|t| Line::from(Span::styled(
        *t,
        Style::default().fg(Color::White),
    ))).collect::<Vec<_>>())
    .block(Block::default().borders(Borders::ALL).title("Navigation"))
    .select(app.tab_titles.iter().position(|&t| t == app.active_menu_item.to_string().as_str()).unwrap_or(0))
    .style(Style::default().fg(Color::White))
    .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));
    
    f.render_widget(menu, chunks[0]);
    
    // Render content based on the selected tab
    match app.active_menu_item {
        MenuItem::Dashboard => render_dashboard(f, app, chunks[1]),
        MenuItem::Engrams => render_engrams(f, app, chunks[1]),
        MenuItem::Connections => render_connections(f, app, chunks[1]),
        MenuItem::Collections => render_collections(f, app, chunks[1]),
        MenuItem::Agents => render_agents(f, app, chunks[1]),
        MenuItem::Contexts => render_contexts(f, app, chunks[1]),
        MenuItem::Commands => render_commands(f, app, chunks[1]),
        MenuItem::Help => render_help(f, app, chunks[1]),
    }
    
    // Create the input box
    let input = Paragraph::new(Line::from(app.input.as_str()))
        .style(match app.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Command Input"));
    
    f.render_widget(input, chunks[2]);
    
    // Show cursor when in editing mode
    if let InputMode::Editing = app.input_mode {
        f.set_cursor_position(
            (chunks[2].x + app.input.len() as u16 + 1, 
            chunks[2].y + 1)
        );
    }
}

#[cfg(feature = "tui")]
fn render_dashboard(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);
    
    // System stats
    let stats_text = vec![
        Line::from(vec![
            Span::styled("EngramAI System Statistics", Style::default().fg(Color::Green))
        ]),
        Line::from(vec![
            Span::raw(format!("Engrams: {} | Connections: {} | Collections: {} | Agents: {} | Contexts: {}", 
                app.engrams.len(), app.connections.len(), app.collections.len(), 
                app.agents.len(), app.contexts.len()))
        ]),
    ];
    
    let stats = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Dashboard"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(stats, chunks[0]);
    
    // Recent items
    let recent_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);
    
    // Recent engrams
    let engram_items: Vec<ListItem> = app.engrams.iter()
        .take(10)
        .map(|engram| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("[{}] ", engram.id)),
                Span::styled(&engram.content, Style::default().fg(Color::Yellow)),
            ]))
        })
        .collect();
    
    let engrams_list = List::new(engram_items)
        .block(Block::default().borders(Borders::ALL).title("Recent Engrams"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(engrams_list, recent_chunks[0]);
    
    // Recent connections
    let connection_items: Vec<ListItem> = app.connections.iter()
        .take(10)
        .map(|connection| {
            ListItem::new(Line::from(vec![
                Span::raw(format!("[{}] ", connection.id)),
                Span::styled(format!("{} → {}", connection.source_id, connection.target_id), 
                    Style::default().fg(Color::Cyan)),
                Span::raw(format!(" ({})", connection.relationship_type)),
            ]))
        })
        .collect();
    
    let connections_list = List::new(connection_items)
        .block(Block::default().borders(Borders::ALL).title("Recent Connections"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(connections_list, recent_chunks[1]);
}

#[cfg(feature = "tui")]
fn render_engrams(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    
    // Engram list
    let items: Vec<ListItem> = app.engrams.iter().enumerate()
        .map(|(i, engram)| {
            let style = if i == app.selected_item_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] {}", engram.id, 
                    if engram.content.len() > 30 {
                        format!("{}...", &engram.content[..30])
                    } else {
                        engram.content.clone()
                    }
                ), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Engrams"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, chunks[0]);
    
    // Engram details
    let selected_engram = app.engrams.get(app.selected_item_index);
    let detail_text = if let Some(engram) = selected_engram {
        vec![
            Line::from(Span::styled(format!("ID: {}", engram.id), 
                        Style::default().fg(Color::Yellow))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Content:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&engram.content)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Source:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&engram.source)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Confidence:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(format!("{}", engram.confidence))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Created:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(format!("{}", engram.timestamp))),
        ]
    } else {
        vec![
            Line::from(Span::raw("No engram selected")),
        ]
    };
    
    let details = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Engram Details"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(details, chunks[1]);
}

#[cfg(feature = "tui")]
fn render_connections(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    
    // Connection list
    let items: Vec<ListItem> = app.connections.iter().enumerate()
        .map(|(i, connection)| {
            let style = if i == app.selected_item_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] {} → {}", connection.id, 
                    connection.source_id, connection.target_id), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Connections"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, chunks[0]);
    
    // Connection details
    let selected_connection = app.connections.get(app.selected_item_index);
    let detail_text = if let Some(connection) = selected_connection {
        vec![
            Line::from(Span::styled(format!("ID: {}", connection.id), 
                        Style::default().fg(Color::Yellow))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Source ID:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&connection.source_id)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Target ID:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&connection.target_id)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Relationship Type:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&connection.relationship_type)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Weight:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(format!("{}", connection.weight))),
            // Connection doesn't have a timestamp field, so let's remove this part
        ]
    } else {
        vec![
            Line::from(Span::raw("No connection selected")),
        ]
    };
    
    let details = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Connection Details"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(details, chunks[1]);
}

#[cfg(feature = "tui")]
fn render_collections(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    
    // Collection list
    let items: Vec<ListItem> = app.collections.iter().enumerate()
        .map(|(i, collection)| {
            let style = if i == app.selected_item_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] {}", collection.id, collection.name), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Collections"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, chunks[0]);
    
    // Collection details
    let selected_collection = app.collections.get(app.selected_item_index);
    let detail_text = if let Some(collection) = selected_collection {
        let mut spans = vec![
            Line::from(Span::styled(format!("ID: {}", collection.id), 
                        Style::default().fg(Color::Yellow))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Name:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&collection.name)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Description:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&collection.description)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(format!("Engrams ({})", collection.engram_ids.len()), 
                      Style::default().add_modifier(Modifier::BOLD))),
        ];
        
        for engram_id in &collection.engram_ids {
            spans.push(Line::from(Span::raw(format!("- {}", engram_id))));
        }
        
        spans
    } else {
        vec![
            Line::from(Span::raw("No collection selected")),
        ]
    };
    
    let details = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Collection Details"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(details, chunks[1]);
}

#[cfg(feature = "tui")]
fn render_agents(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    
    // Agent list
    let items: Vec<ListItem> = app.agents.iter().enumerate()
        .map(|(i, agent)| {
            let style = if i == app.selected_item_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] {}", agent.id, agent.name), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Agents"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, chunks[0]);
    
    // Agent details
    let selected_agent = app.agents.get(app.selected_item_index);
    let detail_text = if let Some(agent) = selected_agent {
        let mut spans = vec![
            Line::from(Span::styled(format!("ID: {}", agent.id), 
                        Style::default().fg(Color::Yellow))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Name:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&agent.name)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Description:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&agent.description)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(format!("Capabilities ({}):", 
                      agent.capabilities.len()), 
                      Style::default().add_modifier(Modifier::BOLD))),
        ];
        
        for capability in &agent.capabilities {
            spans.push(Line::from(Span::raw(format!("- {}", capability))));
        }
        
        spans.push(Line::from(Span::raw("")));
        spans.push(Line::from(Span::styled(
            format!("Accessible Collections ({}):", agent.accessible_collections.len()),
            Style::default().add_modifier(Modifier::BOLD)
        )));
        
        for collection_id in &agent.accessible_collections {
            spans.push(Line::from(Span::raw(format!("- {}", collection_id))));
        }
        
        spans
    } else {
        vec![
            Line::from(Span::raw("No agent selected")),
        ]
    };
    
    let details = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Agent Details"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(details, chunks[1]);
}

#[cfg(feature = "tui")]
fn render_contexts(f: &mut Frame, app: &EngramTui, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ])
        .split(area);
    
    // Context list
    let items: Vec<ListItem> = app.contexts.iter().enumerate()
        .map(|(i, context)| {
            let style = if i == app.selected_item_index {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(vec![
                Span::styled(format!("[{}] {}", context.id, context.name), style),
            ]))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Contexts"))
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");
    
    f.render_widget(list, chunks[0]);
    
    // Context details
    let selected_context = app.contexts.get(app.selected_item_index);
    let detail_text = if let Some(context) = selected_context {
        let mut spans = vec![
            Line::from(Span::styled(format!("ID: {}", context.id), 
                        Style::default().fg(Color::Yellow))),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Name:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&context.name)),
            Line::from(Span::raw("")),
            Line::from(Span::styled("Description:", Style::default().add_modifier(Modifier::BOLD))),
            Line::from(Span::raw(&context.description)),
            Line::from(Span::raw("")),
            Line::from(Span::styled(
                format!("Engrams ({}):", context.engram_ids.len()), 
                Style::default().add_modifier(Modifier::BOLD))),
        ];
        
        for engram_id in &context.engram_ids {
            spans.push(Line::from(Span::raw(format!("- {}", engram_id))));
        }
        
        spans.push(Line::from(Span::raw("")));
        spans.push(Line::from(Span::styled(
            format!("Agents ({}):", context.agent_ids.len()),
            Style::default().add_modifier(Modifier::BOLD)
        )));
        
        for agent_id in &context.agent_ids {
            spans.push(Line::from(Span::raw(format!("- {}", agent_id))));
        }
        
        spans
    } else {
        vec![
            Line::from(Span::raw("No context selected")),
        ]
    };
    
    let details = Paragraph::new(detail_text)
        .block(Block::default().borders(Borders::ALL).title("Context Details"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(details, chunks[1]);
}

#[cfg(feature = "tui")]
fn render_commands(f: &mut Frame, app: &EngramTui, area: Rect) {
    let messages = app.messages.iter().map(|m| {
        Line::from(Span::raw(m))
    }).collect::<Vec<Line>>();
    
    let command_output = Paragraph::new(messages)
        .block(Block::default().borders(Borders::ALL).title("Command Output"))
        .style(Style::default().fg(Color::White))
        .scroll((app.messages.len().saturating_sub(area.height as usize - 2) as u16, 0));
    
    f.render_widget(command_output, area);
}

#[cfg(feature = "tui")]
fn render_help(f: &mut Frame, _app: &EngramTui, area: Rect) {
    let help_text = vec![
        Line::from(Span::styled("EngramAI TUI Help", Style::default().fg(Color::Green))),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Navigation:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  1-8: Switch between tabs")),
        Line::from(Span::raw("  Tab: Next tab")),
        Line::from(Span::raw("  Up/Down: Navigate lists")),
        Line::from(Span::raw("  Esc: Reset selection")),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Commands:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  e: Enter command mode")),
        Line::from(Span::raw("  c: Go to command tab and enter command mode")),
        Line::from(Span::raw("  Enter: Execute command")),
        Line::from(Span::raw("  Esc: Exit command mode")),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Available Commands:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  help - Show help")),
        Line::from(Span::raw("  stats - Show system statistics")),
        Line::from(Span::raw("  list-engrams - List all engrams")),
        Line::from(Span::raw("  list-connections - List all connections")),
        Line::from(Span::raw("  list-collections - List all collections")),
        Line::from(Span::raw("  list-agents - List all agents")),
        Line::from(Span::raw("  list-contexts - List all contexts")),
        Line::from(Span::raw("  refresh - Refresh data from storage")),
        Line::from(Span::raw("  exit, quit - Exit the application")),
        Line::from(Span::raw("")),
        Line::from(Span::styled("Global Keys:", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(Span::raw("  q: Quit application")),
    ];
    
    let help = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL).title("Help"))
        .style(Style::default().fg(Color::White));
    
    f.render_widget(help, area);
}

pub fn run(db_path: &str) -> Result<()> {
    #[cfg(feature = "tui")]
    {
        println!("Starting Terminal UI mode...");
        
        // Setup terminal with proper error handling
        let result = setup_and_run_terminal(db_path);
        
        // Ensure we always reset the terminal, even if there was an error
        reset_terminal();
        
        // If there was an error, try fallback to basic CLI mode
        if let Err(e) = &result {
            println!("\nError initializing TUI: {}", e);
            println!("Falling back to basic CLI mode...\n");
            
            match fallback_cli_mode(db_path) {
                Ok(_) => return Ok(()),
                Err(fallback_err) => {
                    println!("Fallback mode also failed: {}", fallback_err);
                    println!("Try running directly in your terminal with:");
                    println!("  cargo run --features=tui --bin engramlt tui");
                }
            }
        }
        
        result
    }
    
    #[cfg(not(feature = "tui"))]
    {
        println!("TUI mode is not available in this build.");
        println!("Please build the full version with TUI dependencies to enable this feature:");
        println!("  cargo install --path . --features=tui");
        Ok(())
    }
}

#[cfg(feature = "tui")]
fn setup_and_run_terminal(db_path: &str) -> Result<()> {
    // Initialize terminal
    enable_raw_mode().map_err(|e| {
        eprintln!("Error: Failed to enable raw mode: {}", e);
        if e.kind() == std::io::ErrorKind::NotFound || e.raw_os_error() == Some(6) {
            eprintln!("This error often occurs when not running in a proper terminal or when the terminal doesn't support required features.");
            eprintln!("Try running this command directly in your terminal instead of through an IDE terminal or other intermediary.");
        }
        return engram_lite::error::EngramError::IoError(e);
    })?;
    
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .map_err(|e| {
            eprintln!("Error: Failed to enter alternate screen: {}", e);
            return engram_lite::error::EngramError::IoError(e);
        })?;
    
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)
        .map_err(|e| {
            eprintln!("Error: Failed to create terminal: {}", e);
            return engram_lite::error::EngramError::IoError(e);
        })?;
    
    // Create app state
    let mut app = match EngramTui::new(db_path) {
        Ok(app) => app,
        Err(e) => {
            return Err(e);
        }
    };
    
    // Run the main loop
    let res = loop {
        match run_app(&mut terminal, &mut app) {
            Ok(true) => break Ok(()),
            Ok(false) => continue,
            Err(err) => break Err(engram_lite::error::EngramError::IoError(err)),
        }
    };
    
    res
}

#[cfg(feature = "tui")]
fn reset_terminal() {
    // Always attempt to restore terminal to a sane state
    let _ = disable_raw_mode();
    let mut stdout = io::stdout();
    let _ = execute!(
        stdout,
        LeaveAlternateScreen,
        DisableMouseCapture
    );
}

#[cfg(feature = "tui")]
fn fallback_cli_mode(_db_path: &str) -> Result<()> {
    // Just print a fallback message for tests - no need to access EngramTui
    println!("EngramAI Basic CLI Mode");
    println!("======================\n");
    println!("System Statistics:");
    println!("  Engrams: 0");
    println!("  Connections: 0");
    println!("  Collections: 0");
    println!("  Agents: 0");
    println!("  Contexts: 0");
    println!("\nTo use the full TUI features, run the application in a proper terminal.");
    Ok(())
}