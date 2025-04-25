use engram_lite::error::Result;
use engram_lite::graph::MemoryGraph;
use engram_lite::schema::{Agent, Collection, Connection, Engram};
use engram_lite::storage::Storage;
use std::collections::HashSet;
use std::io::{self, Write};

struct EngramCli {
    storage: Storage,
    memory_graph: MemoryGraph,
}

impl EngramCli {
    fn new(db_path: &str) -> Result<Self> {
        let storage = Storage::new(db_path)?;
        let memory_graph = MemoryGraph::new();
        
        Ok(Self {
            storage,
            memory_graph,
        })
    }
    
    fn run(&mut self) -> Result<()> {
        println!("EngramAI CLI - Type 'help' for available commands");
        
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            
            let input = input.trim();
            if input.is_empty() {
                continue;
            }
            
            let parts: Vec<&str> = input.splitn(2, ' ').collect();
            let command = parts[0];
            let args = parts.get(1).unwrap_or(&"");
            
            match command {
                "help" => self.show_help(),
                "add-engram" => self.add_engram(args)?,
                "get-engram" => self.get_engram(args)?,
                "add-connection" => self.add_connection(args)?,
                "create-collection" => self.create_collection(args)?,
                "add-to-collection" => self.add_to_collection(args)?,
                "create-agent" => self.create_agent(args)?,
                "grant-access" => self.grant_access(args)?,
                "query" => self.query(args)?,
                "list-engrams" => self.list_engrams()?,
                "list-collections" => self.list_collections()?,
                "list-agents" => self.list_agents()?,
                "list-contexts" => self.list_contexts()?,
                "delete-engram" => self.delete_engram(args)?,
                "delete-collection" => self.delete_collection(args)?,
                "delete-connection" => self.delete_connection(args)?,
                "delete-agent" => self.delete_agent(args)?,
                "delete-context" => self.delete_context(args)?,
                "filter-by-source" => self.filter_by_source(args)?,
                "filter-by-confidence" => self.filter_by_confidence(args)?,
                "stats" => self.show_stats()?,
                "compact" => self.compact_database(args)?,
                "refresh" => self.refresh_memory_graph()?,
                "export" => self.export(args)?,
                "import" => self.import(args)?,
                "exit" | "quit" => break,
                _ => println!("Unknown command. Type 'help' for available commands."),
            }
        }
        
        Ok(())
    }
    
    fn show_help(&self) {
        println!("Available commands:");
        println!("Creation Commands:");
        println!("  add-engram <content>;<source>;<confidence>    - Add a new engram");
        println!("  add-connection <source-id>;<target-id>;<type>;<weight> - Add a connection between engrams");
        println!("  create-collection <n>;<description> - Create a new collection");
        println!("  create-agent <n>;<description>      - Create a new agent");
        
        println!("\nRetrieval Commands:");
        println!("  get-engram <id>                        - Get engram by ID");
        println!("  query <source>                         - Query engrams by source");
        println!("  filter-by-source <source>              - Filter engrams by source");
        println!("  filter-by-confidence <value>           - Filter engrams by minimum confidence");
        
        println!("\nList Commands:");
        println!("  list-engrams                           - List all engrams");
        println!("  list-collections                       - List all collections");
        println!("  list-agents                            - List all agents");
        println!("  list-contexts                          - List all contexts");
        
        println!("\nRelationship Commands:");
        println!("  add-to-collection <engram-id>;<collection-id> - Add engram to collection");
        println!("  grant-access <agent-id>;<collection-id> - Grant collection access to agent");
        
        println!("\nData Maintenance Commands:");
        println!("  delete-engram <id>                     - Delete an engram and all its connections");
        println!("  delete-connection <id>                 - Delete a connection");
        println!("  delete-collection <id>                 - Delete a collection");
        println!("  delete-agent <id>                      - Delete an agent");
        println!("  delete-context <id>                    - Delete a context");
        println!("  stats                                  - Show system statistics");
        println!("  compact                                - Compact the database to reclaim space");
        println!("  refresh                                - Reload memory graph from storage");
        
        println!("\nImport/Export Commands:");
        println!("  export <file-path>                     - Export all data to JSON file");
        println!("  export <file-path>;<collection-id>     - Export a specific collection to JSON file");
        println!("  import <file-path>                     - Import data from JSON file");
        
        println!("\nSystem Commands:");
        println!("  help                                   - Show this help message");
        println!("  exit, quit                             - Exit the program");
    }
    
    fn add_engram(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 3 {
            println!("Usage: add-engram <content>;<source>;<confidence>");
            return Ok(());
        }
        
        let content = parts[0].trim();
        let source = parts[1].trim();
        let confidence = parts[2].trim().parse::<f64>().unwrap_or(1.0);
        
        let engram = Engram::new(content.to_string(), source.to_string(), confidence, None);
        
        self.storage.put_engram(&engram)?;
        let id = self.memory_graph.add_engram(engram.clone())?;
        
        println!("Engram added with ID: {}", id);
        
        Ok(())
    }
    
    fn get_engram(&self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: get-engram <id>");
            return Ok(());
        }
        
        if let Some(engram) = self.storage.get_engram(&id.to_string())? {
            println!("ID: {}", engram.id);
            println!("Content: {}", engram.content);
            println!("Source: {}", engram.source);
            println!("Confidence: {}", engram.confidence);
            println!("Timestamp: {}", engram.timestamp);
        } else {
            println!("Engram not found with ID: {}", id);
        }
        
        Ok(())
    }
    
    fn add_connection(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 4 {
            println!("Usage: add-connection <source-id>;<target-id>;<type>;<weight>");
            return Ok(());
        }
        
        let source_id = parts[0].trim();
        let target_id = parts[1].trim();
        let relationship_type = parts[2].trim();
        let weight = parts[3].trim().parse::<f64>().unwrap_or(1.0);
        
        let connection = Connection::new(
            source_id.to_string(),
            target_id.to_string(),
            relationship_type.to_string(),
            weight,
            None,
        );
        
        self.storage.put_connection(&connection)?;
        let id = self.memory_graph.add_connection(connection.clone())?;
        
        println!("Connection added with ID: {}", id);
        
        Ok(())
    }
    
    fn create_collection(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 2 {
            println!("Usage: create-collection <n>;<description>");
            return Ok(());
        }
        
        let name = parts[0].trim();
        let description = parts[1].trim();
        
        let collection = Collection::new(name.to_string(), description.to_string(), None);
        
        self.storage.put_collection(&collection)?;
        let id = self.memory_graph.add_collection(collection.clone())?;
        
        println!("Collection created with ID: {}", id);
        
        Ok(())
    }
    
    fn add_to_collection(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 2 {
            println!("Usage: add-to-collection <engram-id>;<collection-id>");
            return Ok(());
        }
        
        let engram_id = parts[0].trim();
        let collection_id = parts[1].trim();
        
        // Verify engram exists
        if self.storage.get_engram(&engram_id.to_string())?.is_none() {
            println!("Engram not found with ID: {}", engram_id);
            return Ok(());
        }
        
        // Get collection, modify it, and store it back
        if let Some(mut collection) = self.storage.get_collection(&collection_id.to_string())? {
            collection.add_engram(engram_id.to_string());
            self.storage.put_collection(&collection)?;
            
            // Update memory graph
            self.memory_graph.add_engram_to_collection(&engram_id.to_string(), &collection_id.to_string())?;
            
            println!("Engram added to collection");
        } else {
            println!("Collection not found with ID: {}", collection_id);
        }
        
        Ok(())
    }
    
    fn create_agent(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 2 {
            println!("Usage: create-agent <n>;<description>");
            return Ok(());
        }
        
        let name = parts[0].trim();
        let description = parts[1].trim();
        
        let capabilities = HashSet::new();
        let agent = Agent::new(name.to_string(), description.to_string(), Some(capabilities), None);
        
        self.storage.put_agent(&agent)?;
        let id = self.memory_graph.add_agent(agent.clone())?;
        
        println!("Agent created with ID: {}", id);
        
        Ok(())
    }
    
    fn grant_access(&mut self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        if parts.len() < 2 {
            println!("Usage: grant-access <agent-id>;<collection-id>");
            return Ok(());
        }
        
        let agent_id = parts[0].trim();
        let collection_id = parts[1].trim();
        
        // Get agent, modify it, and store it back
        if let Some(mut agent) = self.storage.get_agent(&agent_id.to_string())? {
            agent.grant_access(collection_id.to_string());
            self.storage.put_agent(&agent)?;
            
            println!("Access granted");
        } else {
            println!("Agent not found with ID: {}", agent_id);
        }
        
        Ok(())
    }
    
    fn query(&self, args: &str) -> Result<()> {
        let source = args.trim();
        if source.is_empty() {
            println!("Usage: query <source>");
            return Ok(());
        }
        
        let engrams = self.memory_graph.get_engrams_by_source(source)?;
        
        if engrams.is_empty() {
            println!("No engrams found from source: {}", source);
        } else {
            println!("Engrams from source '{}':", source);
            for engram in engrams {
                println!("  [{}] {} (confidence: {})", engram.id, engram.content, engram.confidence);
            }
        }
        
        Ok(())
    }
    
    fn list_engrams(&self) -> Result<()> {
        let engrams = self.memory_graph.get_recent_engrams(100)?;
        
        if engrams.is_empty() {
            println!("No engrams found");
        } else {
            println!("Engrams:");
            for engram in engrams {
                println!("  [{}] {} (source: {}, confidence: {})", 
                    engram.id, engram.content, engram.source, engram.confidence);
            }
        }
        
        Ok(())
    }
    
    fn list_collections(&self) -> Result<()> {
        let collection_ids = self.storage.list_collections()?;
        
        if collection_ids.is_empty() {
            println!("No collections found");
        } else {
            println!("Collections:");
            for id in collection_ids {
                if let Some(collection) = self.storage.get_collection(&id)? {
                    println!("  [{}] {} - {} ({} engrams)", 
                        collection.id, collection.name, collection.description, collection.engram_ids.len());
                }
            }
        }
        
        Ok(())
    }
    
    fn list_agents(&self) -> Result<()> {
        let agent_ids = self.storage.list_agents()?;
        
        if agent_ids.is_empty() {
            println!("No agents found");
        } else {
            println!("Agents:");
            for id in agent_ids {
                if let Some(agent) = self.storage.get_agent(&id)? {
                    println!("  [{}] {} - {} (access to {} collections)", 
                        agent.id, agent.name, agent.description, agent.accessible_collections.len());
                }
            }
        }
        
        Ok(())
    }
    
    fn list_contexts(&self) -> Result<()> {
        let context_ids = self.storage.list_contexts()?;
        
        if context_ids.is_empty() {
            println!("No contexts found");
        } else {
            println!("Contexts:");
            for id in context_ids {
                if let Some(context) = self.storage.get_context(&id)? {
                    println!("  [{}] {} - {} ({} engrams, {} agents)", 
                        context.id, context.name, context.description, 
                        context.engram_ids.len(), context.agent_ids.len());
                }
            }
        }
        
        Ok(())
    }
    
    fn delete_engram(&mut self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: delete-engram <engram-id>");
            return Ok(());
        }
        
        // Check if engram exists
        if self.storage.get_engram(&id.to_string())?.is_none() {
            println!("Engram not found with ID: {}", id);
            return Ok(());
        }
        
        // Find all connections related to this engram
        let connection_ids = self.storage.find_connections_for_engram(&id.to_string())?;
        
        // Begin a transaction for atomic delete
        let mut txn = self.storage.begin_transaction();
        
        // Delete all related connections first
        for conn_id in &connection_ids {
            // First get the connection for the index information
            let conn = self.storage.get_connection(conn_id)?;
            txn.delete_connection(conn_id, conn.as_ref())?;
        }
        
        // Delete the engram
        txn.delete_engram(&id.to_string())?;
        
        // Commit the transaction
        txn.commit()?;
        
        println!("Deleted engram with ID: {}", id);
        println!("Also deleted {} related connections", connection_ids.len());
        
        // Refresh the memory graph to maintain consistency
        self.refresh_memory_graph()?;
        
        Ok(())
    }
    
    fn delete_connection(&mut self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: delete-connection <connection-id>");
            return Ok(());
        }
        
        // Check if connection exists
        if self.storage.get_connection(&id.to_string())?.is_none() {
            println!("Connection not found with ID: {}", id);
            return Ok(());
        }
        
        // Delete the connection
        self.storage.delete_connection(&id.to_string())?;
        
        println!("Deleted connection with ID: {}", id);
        
        // Refresh the memory graph to maintain consistency
        self.refresh_memory_graph()?;
        
        Ok(())
    }
    
    fn delete_collection(&mut self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: delete-collection <collection-id>");
            return Ok(());
        }
        
        // Check if collection exists
        if self.storage.get_collection(&id.to_string())?.is_none() {
            println!("Collection not found with ID: {}", id);
            return Ok(());
        }
        
        // Delete the collection
        self.storage.delete_collection(&id.to_string())?;
        
        println!("Deleted collection with ID: {}", id);
        
        // Refresh the memory graph to maintain consistency
        self.refresh_memory_graph()?;
        
        Ok(())
    }
    
    fn delete_agent(&mut self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: delete-agent <agent-id>");
            return Ok(());
        }
        
        // Check if agent exists
        if self.storage.get_agent(&id.to_string())?.is_none() {
            println!("Agent not found with ID: {}", id);
            return Ok(());
        }
        
        // Delete the agent
        self.storage.delete_agent(&id.to_string())?;
        
        println!("Deleted agent with ID: {}", id);
        
        // Refresh the memory graph to maintain consistency
        self.refresh_memory_graph()?;
        
        Ok(())
    }
    
    fn delete_context(&mut self, args: &str) -> Result<()> {
        let id = args.trim();
        if id.is_empty() {
            println!("Usage: delete-context <context-id>");
            return Ok(());
        }
        
        // Check if context exists
        if self.storage.get_context(&id.to_string())?.is_none() {
            println!("Context not found with ID: {}", id);
            return Ok(());
        }
        
        // Delete the context
        self.storage.delete_context(&id.to_string())?;
        
        println!("Deleted context with ID: {}", id);
        
        // Refresh the memory graph to maintain consistency
        self.refresh_memory_graph()?;
        
        Ok(())
    }
    
    fn filter_by_source(&self, args: &str) -> Result<()> {
        let source = args.trim();
        if source.is_empty() {
            println!("Usage: filter-by-source <source>");
            return Ok(());
        }
        
        let engrams = self.memory_graph.get_engrams_by_source(source)?;
        
        if engrams.is_empty() {
            println!("No engrams found from source: {}", source);
        } else {
            println!("Engrams from source '{}':", source);
            for engram in engrams {
                println!("  [{}] {} (confidence: {})", engram.id, engram.content, engram.confidence);
            }
        }
        
        Ok(())
    }
    
    fn filter_by_confidence(&self, args: &str) -> Result<()> {
        let min_confidence = match args.trim().parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Usage: filter-by-confidence <minimum-confidence-value>");
                println!("Example: filter-by-confidence 0.7");
                return Ok(());
            }
        };
        
        let engrams = self.memory_graph.get_engrams_by_confidence(min_confidence)?;
        
        if engrams.is_empty() {
            println!("No engrams found with confidence ≥ {}", min_confidence);
        } else {
            println!("Engrams with confidence ≥ {}:", min_confidence);
            for engram in engrams {
                println!("  [{}] {} (confidence: {}, source: {})", 
                    engram.id, engram.content, engram.confidence, engram.source);
            }
        }
        
        Ok(())
    }
    
    fn show_stats(&self) -> Result<()> {
        let engram_count = self.storage.list_engrams()?.len();
        let connection_count = self.storage.list_connections()?.len();
        let collection_count = self.storage.list_collections()?.len();
        let agent_count = self.storage.list_agents()?.len();
        let context_count = self.storage.list_contexts()?.len();
        
        println!("EngramAI System Statistics:");
        println!("  Engrams:     {}", engram_count);
        println!("  Connections: {}", connection_count);
        println!("  Collections: {}", collection_count);
        println!("  Agents:      {}", agent_count);
        println!("  Contexts:    {}", context_count);
        
        Ok(())
    }
    
    fn compact_database(&self, _args: &str) -> Result<()> {
        println!("Compacting database...");
        
        // Implementing basic compaction for now
        // In a more advanced implementation, we could add options for selective compaction
        
        let cf_names = vec![
            "engrams",
            "connections",
            "collections",
            "agents",
            "contexts",
            "metadata",
        ];
        
        // Compact column families
        for cf_name in cf_names {
            if let Some(cf) = self.storage.db.cf_handle(cf_name) {
                self.storage.db.compact_range_cf(cf, None::<&[u8]>, None::<&[u8]>);
                println!("  Compacted column family: {}", cf_name);
            }
        }
        
        println!("Database compaction completed");
        Ok(())
    }
    
    fn export(&self, args: &str) -> Result<()> {
        let parts: Vec<&str> = args.split(';').collect();
        let file_path = parts[0].trim();
        
        if file_path.is_empty() {
            println!("Usage: export <file-path> or export <file-path>;<collection-id>");
            return Ok(());
        }
        
        let path = std::path::Path::new(file_path);
        
        if parts.len() > 1 && !parts[1].trim().is_empty() {
            // Export a specific collection
            let collection_id = parts[1].trim();
            println!("Exporting collection {} to {}", collection_id, file_path);
            match engram_lite::export_collection_to_file(&self.storage, collection_id, path) {
                Ok(_) => println!("Collection exported successfully"),
                Err(e) => println!("Export failed: {}", e),
            }
        } else {
            // Export all data
            println!("Exporting all data to {}", file_path);
            match engram_lite::export_to_file(&self.storage, path) {
                Ok(_) => println!("Data exported successfully"),
                Err(e) => println!("Export failed: {}", e),
            }
        }
        
        Ok(())
    }
    
    fn import(&mut self, args: &str) -> Result<()> {
        let file_path = args.trim();
        if file_path.is_empty() {
            println!("Usage: import <file-path>");
            return Ok(());
        }
        
        let path = std::path::Path::new(file_path);
        
        println!("Importing data from {}", file_path);
        match engram_lite::import_from_file(&self.storage, path) {
            Ok(_) => {
                println!("Data imported successfully");
                
                // Refresh memory graph from storage
                println!("Refreshing memory graph...");
                self.refresh_memory_graph()?;
            }
            Err(e) => println!("Import failed: {}", e),
        }
        
        Ok(())
    }
    
    // Helper method to reload the memory graph from storage
    fn refresh_memory_graph(&mut self) -> Result<()> {
        // Create a new memory graph
        self.memory_graph = engram_lite::MemoryGraph::new();
        
        // Load engrams
        println!("Loading engrams...");
        let engram_ids = self.storage.list_engrams()?;
        for id in &engram_ids {
            if let Some(engram) = self.storage.get_engram(id)? {
                self.memory_graph.add_engram(engram)?;
            }
        }
        
        // Load connections
        println!("Loading connections...");
        let connection_ids = self.storage.list_connections()?;
        for id in &connection_ids {
            if let Some(connection) = self.storage.get_connection(id)? {
                // Only add connections if both source and target exist
                if engram_ids.contains(&connection.source_id) && engram_ids.contains(&connection.target_id) {
                    self.memory_graph.add_connection(connection)?;
                }
            }
        }
        
        // Load collections
        println!("Loading collections...");
        let collection_ids = self.storage.list_collections()?;
        for id in &collection_ids {
            if let Some(collection) = self.storage.get_collection(id)? {
                self.memory_graph.add_collection(collection)?;
            }
        }
        
        // Load agents
        println!("Loading agents...");
        let agent_ids = self.storage.list_agents()?;
        for id in &agent_ids {
            if let Some(agent) = self.storage.get_agent(id)? {
                self.memory_graph.add_agent(agent)?;
            }
        }
        
        // Load contexts
        println!("Loading contexts...");
        let context_ids = self.storage.list_contexts()?;
        for id in &context_ids {
            if let Some(context) = self.storage.get_context(id)? {
                self.memory_graph.add_context(context)?;
            }
        }
        
        println!("Memory graph refreshed successfully");
        Ok(())
    }
}

// This module is intended to be used by the engramlt binary
// But we add a main function for direct compilation

fn main() -> Result<()> {
    // This is a library component, not meant to be run directly
    println!("This is a library component. Use engramlt instead.");
    Ok(())
}

pub fn run(db_path: &str) -> Result<()> {
    match EngramCli::new(db_path) {
        Ok(mut cli) => {
            if let Err(e) = cli.run() {
                eprintln!("Error: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to initialize CLI: {}", e);
        }
    }
    
    Ok(())
}