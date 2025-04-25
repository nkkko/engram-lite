mod schema;
mod storage;
mod graph;
mod error;
mod benchmark;
mod index;
mod query;

use crate::error::Result;
use crate::graph::MemoryGraph;
use crate::schema::{Agent, Collection, Connection, Context, Engram};
use crate::storage::Storage;
use std::collections::HashSet;
use std::path::Path;

fn run_demo() -> Result<()> {
    println!("Creating storage...");
    let storage = Storage::new("./engram_db")?;
    
    println!("Creating memory graph...");
    let mut memory_graph = MemoryGraph::new();
    
    // Create some engrams
    println!("Creating engrams...");
    let engram1 = Engram::new(
        "Climate change is accelerating faster than predicted.".to_string(),
        "research".to_string(),
        0.9,
        None,
    );
    
    let engram2 = Engram::new(
        "Solar panels are becoming more affordable and efficient.".to_string(),
        "observation".to_string(),
        0.8,
        None,
    );
    
    let engram3 = Engram::new(
        "Renewable energy can replace fossil fuels for most applications.".to_string(),
        "inference".to_string(),
        0.7,
        None,
    );
    
    // Store engrams
    storage.put_engram(&engram1)?;
    storage.put_engram(&engram2)?;
    storage.put_engram(&engram3)?;
    
    // Add to graph
    let engram1_id = memory_graph.add_engram(engram1.clone())?;
    let engram2_id = memory_graph.add_engram(engram2.clone())?;
    let engram3_id = memory_graph.add_engram(engram3.clone())?;
    
    println!("Created engrams with IDs:");
    println!("  - {}", engram1_id);
    println!("  - {}", engram2_id);
    println!("  - {}", engram3_id);
    
    // Create connections
    println!("Creating connections...");
    let connection1 = Connection::new(
        engram1_id.clone(),
        engram3_id.clone(),
        "causes".to_string(),
        0.8,
        None,
    );
    
    let connection2 = Connection::new(
        engram2_id.clone(),
        engram3_id.clone(),
        "supports".to_string(),
        0.9,
        None,
    );
    
    // Store connections
    storage.put_connection(&connection1)?;
    storage.put_connection(&connection2)?;
    
    // Add to graph
    memory_graph.add_connection(connection1.clone())?;
    memory_graph.add_connection(connection2.clone())?;
    
    // Create a collection
    println!("Creating collection...");
    let mut collection = Collection::new(
        "Climate Knowledge".to_string(),
        "Collection of climate-related knowledge".to_string(),
        None,
    );
    
    collection.add_engram(engram1_id.clone());
    collection.add_engram(engram2_id.clone());
    collection.add_engram(engram3_id.clone());
    
    // Store collection
    storage.put_collection(&collection)?;
    
    // Add to graph
    memory_graph.add_collection(collection.clone())?;
    
    // Create an agent
    println!("Creating agent...");
    let mut capabilities = HashSet::new();
    capabilities.insert("query".to_string());
    capabilities.insert("analyze".to_string());
    
    let mut agent = Agent::new(
        "Climate Researcher".to_string(),
        "Analyzes climate data and trends".to_string(),
        Some(capabilities),
        None,
    );
    
    agent.grant_access(collection.id.clone());
    
    // Store agent
    storage.put_agent(&agent)?;
    
    // Add to graph
    memory_graph.add_agent(agent.clone())?;
    
    // Create a context
    println!("Creating context...");
    let mut context = Context::new(
        "Climate Discussion".to_string(),
        "Context for discussing climate change and solutions".to_string(),
        None,
    );
    
    context.add_engram(engram1_id.clone());
    context.add_engram(engram3_id.clone());
    context.add_agent(agent.id.clone());
    
    // Store context
    storage.put_context(&context)?;
    
    // Add to graph
    memory_graph.add_context(context.clone())?;
    
    // Demonstrate retrieval
    println!("\nDemonstrating retrieval...");
    
    // Get an engram by ID
    println!("Getting engram by ID:");
    if let Some(engram) = storage.get_engram(&engram1_id)? {
        println!("  Content: {}", engram.content);
        println!("  Confidence: {}", engram.confidence);
    }
    
    // Get connections between engrams
    println!("Getting connections between engrams:");
    let connections = memory_graph.get_connections_between(&engram1_id, &engram3_id)?;
    for connection in connections {
        println!("  Type: {}, Weight: {}", connection.relationship_type, connection.weight);
    }
    
    // Get agent accessible engrams
    println!("Getting agent accessible engrams:");
    let accessible_engrams = memory_graph.get_agent_accessible_engrams(&agent.id)?;
    for engram in accessible_engrams {
        println!("  {}", engram.content);
    }
    
    // Get engrams by confidence
    println!("Getting high-confidence engrams (>= 0.8):");
    let high_confidence = memory_graph.get_engrams_by_confidence(0.8)?;
    for engram in high_confidence {
        println!("  {} (confidence: {})", engram.content, engram.confidence);
    }
    
    // Get context engrams
    println!("Getting engrams in context:");
    let context_engrams = memory_graph.get_context_engrams(&context.id)?;
    for engram in context_engrams {
        println!("  {}", engram.content);
    }
    
    // Get agents in context
    println!("Getting agents in context:");
    let context_agents = memory_graph.get_agents_in_context(&context.id)?;
    for agent in context_agents {
        println!("  {} - {}", agent.name, agent.description);
    }
    
    println!("\nDemo completed successfully!");
    Ok(())
}

/// Run benchmarks for performance testing
fn run_benchmarks() -> Result<()> {
    use crate::benchmark::run_all_benchmarks;
    use crate::index::SearchIndex;
    use tempfile::tempdir;

    println!("Running benchmarks...");

    // Create a temporary directory for the database
    let dir = tempdir().map_err(|e| error::EngramError::IoError(e))?;
    let db_path = dir.path().to_str().unwrap();
    
    // Initialize storage and index
    let storage = Storage::new(db_path)?;
    let index = SearchIndex::new();
    
    // Run benchmarks
    let results = run_all_benchmarks(&storage, &index)?;
    
    // Print results
    println!("\nBenchmark Results:");
    println!("=================");
    
    for result in results {
        println!("\n{}", result.format());
    }
    
    // Clean up temp directory
    dir.close().map_err(|e| error::EngramError::IoError(e))?;
    
    println!("\nBenchmarks completed successfully!");
    Ok(())
}

fn main() {
    use std::env;
    
    println!("EngramAI - Memory Graph System");
    
    // Check command line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--benchmark" {
        // Run benchmarks
        if let Err(e) = run_benchmarks() {
            eprintln!("Error during benchmarks: {}", e);
        }
        return;
    }
    
    // Load API key from .env
    match engram_lite::get_anthropic_api_key() {
        Some(_) => {
            println!("Anthropic API key found.");
            // This would be used for LLM integration later
        }
        None => {
            println!("Warning: Anthropic API key not found. LLM features will be disabled.");
        }
    }
    
    if let Err(e) = run_demo() {
        eprintln!("Error during demo: {}", e);
    }
}