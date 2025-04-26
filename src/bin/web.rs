use actix_cors::Cors;
use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder, middleware};
use engram_lite::error::Result as EngramResult;
use engram_lite::graph::MemoryGraph;
use engram_lite::storage::Storage;
use engram_lite::schema::{Engram, Connection, Collection, Agent};
use engram_lite::index::SearchIndex;
use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use tera::{Tera, Context as TeraContext};

// Application state
struct AppState {
    db_path: String,
    storage: Arc<Storage>,
    memory_graph: Arc<RwLock<MemoryGraph>>,
    search_index: Arc<RwLock<SearchIndex>>,
    templates: Tera,
}

// Define data transfer objects for API
#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

// Request DTOs
#[derive(Deserialize)]
struct CreateEngramRequest {
    content: String,
    source: String,
    confidence: f64,
    metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct CreateConnectionRequest {
    source_id: String,
    target_id: String,
    connection_type: String,
    weight: f64,
    metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct CreateCollectionRequest {
    name: String,
    description: String,
    metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct CreateAgentRequest {
    name: String,
    description: String,
    capabilities: Option<Vec<String>>,
    metadata: Option<serde_json::Map<String, serde_json::Value>>,
}

#[derive(Deserialize)]
struct QueryRequest {
    text: Option<String>,
    source: Option<String>,
    min_confidence: Option<f64>,
    limit: Option<usize>,
}

// Initialize API response
impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    fn error(msg: &str) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.to_string()),
        }
    }
}

// Main web routes
async fn index(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Get database stats
    let storage = &data.storage;
    let result = storage.get_stats();
    
    if let Ok(stats) = result {
        context.insert("engram_count", &stats.engram_count);
        context.insert("connection_count", &stats.connection_count);
        context.insert("collection_count", &stats.collection_count);
        context.insert("agent_count", &stats.agent_count);
        context.insert("context_count", &stats.context_count);
        context.insert("db_size", &stats.db_size_mb);
    }
    
    let rendered = data.templates.render("index.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// Engrams page
async fn engrams_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Get engrams
    let storage = &data.storage;
    let mut engrams = Vec::new();
    
    if let Ok(ids) = storage.list_engrams() {
        for id in ids {
            if let Ok(Some(engram)) = storage.get_engram(&id) {
                engrams.push(engram);
            }
        }
    }
    
    context.insert("engrams", &engrams);
    context.insert("version", "0.1.0"); // Add version
    
    let rendered = data.templates.render("engrams.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// Connections page
async fn connections_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Get connections
    let storage = &data.storage;
    let mut connections_with_content = Vec::new();
    let mut type_counts = std::collections::HashMap::new();
    
    if let Ok(ids) = storage.list_connections() {
        for id in ids {
            if let Ok(Some(connection)) = storage.get_connection(&id) {
                // Count connection types
                let count = type_counts.entry(connection.relationship_type.clone()).or_insert(0);
                *count += 1;
                
                // Get source and target engram content
                let source_content = match storage.get_engram(&connection.source_id) {
                    Ok(Some(engram)) => engram.content.clone(),
                    _ => format!("Unknown (ID: {})", connection.source_id)
                };
                
                let target_content = match storage.get_engram(&connection.target_id) {
                    Ok(Some(engram)) => engram.content.clone(),
                    _ => format!("Unknown (ID: {})", connection.target_id)
                };
                
                // Create a connection with content for the template
                let conn_with_content = serde_json::json!({
                    "id": connection.id,
                    "source_id": connection.source_id,
                    "target_id": connection.target_id,
                    "source_content": source_content,
                    "target_content": target_content,
                    "relationship_type": connection.relationship_type,
                    "weight": connection.weight,
                    "metadata": connection.metadata
                });
                
                connections_with_content.push(conn_with_content);
            }
        }
    }
    
    context.insert("connections", &connections_with_content);
    context.insert("type_counts", &type_counts);
    context.insert("version", "0.1.0"); // Add version
    
    let rendered = data.templates.render("connections.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// Collections page
async fn collections_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Get collections
    let storage = &data.storage;
    let mut collections_with_counts = Vec::new();
    
    if let Ok(ids) = storage.list_collections() {
        for id in ids {
            if let Ok(Some(collection)) = storage.get_collection(&id) {
                // Use the collection's engram_ids size as count
                let engram_count = collection.engram_ids.len();
                
                // Create collection with count for the template
                let collection_with_count = serde_json::json!({
                    "id": collection.id,
                    "name": collection.name,
                    "description": collection.description,
                    "metadata": collection.metadata,
                    "engram_count": engram_count
                });
                
                collections_with_counts.push(collection_with_count);
            }
        }
    }
    
    context.insert("collections", &collections_with_counts);
    context.insert("version", "0.1.0"); // Add version
    
    let rendered = data.templates.render("collections.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// Collection detail page 
async fn collection_detail_page(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let collection_id = path.into_inner();
    let mut context = TeraContext::new();
    
    let storage = &data.storage;
    
    // Get the collection
    match storage.get_collection(&collection_id) {
        Ok(Some(collection)) => {
            // Get engrams in this collection
            let mut engrams = Vec::new();
            for engram_id in &collection.engram_ids {
                if let Ok(Some(engram)) = storage.get_engram(engram_id) {
                    engrams.push(engram);
                }
            }
            
            context.insert("collection", &collection);
            context.insert("engrams", &engrams);
            context.insert("version", "0.1.0");
            
            // Return collection detail page
            let rendered = data.templates.render("collection-detail.html", &context).unwrap_or_else(|_e| {
                // If the template doesn't exist, return a simple formatted view
                format!(
                    "<html><head><title>Collection: {}</title></head><body>
                     <h1>Collection: {}</h1>
                     <p>{}</p>
                     <h2>Engrams in this collection:</h2>
                     <ul>
                     {}
                     </ul>
                     <p><a href='/collections'>Back to Collections</a></p>
                     </body></html>",
                    collection.name,
                    collection.name,
                    collection.description,
                    engrams.iter()
                        .map(|e| format!("<li>{}</li>", e.content))
                        .collect::<Vec<_>>()
                        .join("\n")
                )
            });
            
            HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered)
        },
        Ok(None) => {
            HttpResponse::NotFound().body(format!("Collection with ID {} not found", collection_id))
        },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Error fetching collection: {}", e))
        }
    }
}

// Agents page
async fn agents_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Get agents
    let storage = &data.storage;
    let mut agents = Vec::new();
    let mut collection_names = std::collections::HashMap::new();
    
    // First get all collection names for reference
    if let Ok(ids) = storage.list_collections() {
        for id in ids {
            if let Ok(Some(collection)) = storage.get_collection(&id) {
                collection_names.insert(collection.id.clone(), collection.name.clone());
            }
        }
    }
    
    // Now get agents
    if let Ok(ids) = storage.list_agents() {
        for id in ids {
            if let Ok(Some(agent)) = storage.get_agent(&id) {
                // Use agent's accessible_collections directly
                let accessible_collections: Vec<String> = agent.accessible_collections
                    .iter()
                    .cloned()
                    .collect();
                
                // Create agent with collections for the template
                let agent_with_collections = serde_json::json!({
                    "id": agent.id,
                    "name": agent.name,
                    "description": agent.description,
                    "capabilities": agent.capabilities,
                    "metadata": agent.metadata,
                    "accessible_collections": accessible_collections
                });
                
                agents.push(agent_with_collections);
            }
        }
    }
    
    context.insert("agents", &agents);
    context.insert("collection_names", &collection_names);
    context.insert("version", "0.1.0"); // Add version
    
    let rendered = data.templates.render("agents.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// API documentation page
async fn api_docs_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    
    // Add version info
    context.insert("version", "0.1.0");
    
    let rendered = data.templates.render("api-docs.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// Graph visualization page
async fn graph_page(data: web::Data<AppState>) -> impl Responder {
    let mut context = TeraContext::new();
    let storage = &data.storage;
    
    // Prepare graph data for Cytoscape.js
    let mut cy_nodes = Vec::new();
    let mut cy_edges = Vec::new();
    
    // Get all engrams
    if let Ok(engram_ids) = storage.list_engrams() {
        for id in &engram_ids {
            if let Ok(Some(engram)) = storage.get_engram(id) {
                // Truncate content for display label if too long
                let display_label = if engram.content.len() > 30 {
                    format!("{}...", &engram.content[0..27])
                } else {
                    engram.content.clone()
                };
                
                // Create Cytoscape node data
                let node_data = serde_json::json!({
                    "group": "nodes", 
                    "data": {
                        "id": engram.id,
                        "label": display_label,
                        "content": engram.content,
                        "source": engram.source,
                        "confidence": engram.confidence,
                        "metadata": engram.metadata
                    }
                });
                
                cy_nodes.push(node_data);
            }
        }
        
        // Get all connections
        if let Ok(connection_ids) = storage.list_connections() {
            for id in &connection_ids {
                if let Ok(Some(connection)) = storage.get_connection(id) {
                    // Only include connections between existing engrams
                    if engram_ids.contains(&connection.source_id) && 
                       engram_ids.contains(&connection.target_id) {
                        // Create Cytoscape edge data
                        let edge_data = serde_json::json!({
                            "group": "edges",
                            "data": {
                                "id": connection.id,
                                "source": connection.source_id,
                                "target": connection.target_id,
                                "relationshipType": connection.relationship_type,
                                "weight": connection.weight
                            }
                        });
                        
                        cy_edges.push(edge_data);
                    }
                }
            }
        }
    }
    
    // Print debug information
    println!("Graph data: {} nodes, {} edges", cy_nodes.len(), cy_edges.len());
    
    // Combine nodes and edges
    let all_elements = [cy_nodes, cy_edges].concat();
    
    // Convert graph data to JSON string for direct embedding as JavaScript object
    let graph_data_json = serde_json::to_string(&all_elements).unwrap_or_else(|_| "[]".to_string());
    
    context.insert("graph_data_json", &graph_data_json);
    context.insert("version", "0.1.0");
    
    let rendered = data.templates.render("graph.html", &context).unwrap_or_else(|e| {
        format!("Template error: {}", e)
    });
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}

// API Routes - Engrams
async fn api_get_engrams(data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    
    match storage.list_engrams() {
        Ok(ids) => {
            let mut engrams = Vec::new();
            for id in ids {
                if let Ok(Some(engram)) = storage.get_engram(&id) {
                    engrams.push(engram);
                }
            }
            HttpResponse::Ok().json(ApiResponse::success(engrams))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(&format!("Failed to list engrams: {}", e))
            )
        }
    }
}

async fn api_get_engram(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let engram_id = path.into_inner();
    let storage = &data.storage;
    
    match storage.get_engram(&engram_id) {
        Ok(Some(engram)) => HttpResponse::Ok().json(ApiResponse::success(engram)),
        Ok(None) => HttpResponse::NotFound().json(
            ApiResponse::<()>::error(&format!("Engram with ID {} not found", engram_id))
        ),
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to get engram: {}", e))
        )
    }
}

async fn api_create_engram(req: web::Json<CreateEngramRequest>, data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    let memory_graph = &data.memory_graph;
    let search_index = &data.search_index;
    
    // Convert metadata if present
    let metadata = req.metadata.clone().map(|map| {
        let mut hm = std::collections::HashMap::new();
        for (k, v) in map {
            hm.insert(k, v);
        }
        hm
    });
    
    // Create the engram
    let engram = Engram::new(
        req.content.clone(),
        req.source.clone(),
        req.confidence,
        metadata,
    );
    
    // Store in storage
    match storage.put_engram(&engram) {
        Ok(_) => {
            // Add to memory graph
            if let Err(e) = memory_graph.write().unwrap().add_engram(engram.clone()) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add engram to memory graph: {}", e))
                );
            }
            
            // Add to search index
            if let Err(e) = search_index.write().unwrap().add_engram(&engram) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add engram to search index: {}", e))
                );
            }
            
            HttpResponse::Created().json(ApiResponse::success(engram))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to store engram: {}", e))
        )
    }
}

async fn api_delete_engram(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let engram_id = path.into_inner();
    let storage = &data.storage;
    // Using _memory_graph prefix to indicate intentionally unused variable
    let _memory_graph = &data.memory_graph;
    let search_index = &data.search_index;
    
    // First get the engram so we can remove it from indexes
    let engram = match storage.get_engram(&engram_id) {
        Ok(Some(e)) => e,
        Ok(None) => return HttpResponse::NotFound().json(
            ApiResponse::<()>::error(&format!("Engram with ID {} not found", engram_id))
        ),
        Err(e) => return HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to get engram: {}", e))
        )
    };
    
    // Remove from storage
    match storage.delete_engram(&engram_id) {
        Ok(()) => {
            // Remove from memory graph if it exists
            // Note: MemoryGraph doesn't have a direct remove_engram method,
            // in a real implementation we'd need to update this
            
            // Remove from search index
            if let Err(e) = search_index.write().unwrap().remove_engram(&engram) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to remove engram from search index: {}", e))
                );
            }
            
            HttpResponse::Ok().json(ApiResponse::<()>::success(()))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to delete engram: {}", e))
        )
    }
}

// API Routes - Connections
async fn api_get_connections(data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    
    match storage.list_connections() {
        Ok(ids) => {
            let mut connections = Vec::new();
            for id in ids {
                if let Ok(Some(connection)) = storage.get_connection(&id) {
                    connections.push(connection);
                }
            }
            HttpResponse::Ok().json(ApiResponse::success(connections))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(&format!("Failed to list connections: {}", e))
            )
        }
    }
}

async fn api_create_connection(req: web::Json<CreateConnectionRequest>, data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    let memory_graph = &data.memory_graph;
    let search_index = &data.search_index;
    
    // Verify engrams exist
    match storage.get_engram(&req.source_id) {
        Ok(None) => return HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(&format!("Source engram with ID {} not found", req.source_id))
        ),
        Err(e) => return HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to verify source engram: {}", e))
        ),
        _ => {}
    }
    
    match storage.get_engram(&req.target_id) {
        Ok(None) => return HttpResponse::BadRequest().json(
            ApiResponse::<()>::error(&format!("Target engram with ID {} not found", req.target_id))
        ),
        Err(e) => return HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to verify target engram: {}", e))
        ),
        _ => {}
    }
    
    // Convert metadata if present
    let metadata = req.metadata.clone().map(|map| {
        let mut hm = std::collections::HashMap::new();
        for (k, v) in map {
            hm.insert(k, v);
        }
        hm
    });
    
    // Create the connection
    let connection = Connection::new(
        req.source_id.clone(),
        req.target_id.clone(),
        req.connection_type.clone(),
        req.weight,
        metadata,
    );
    
    // Store in storage
    match storage.put_connection(&connection) {
        Ok(_) => {
            // Add to memory graph
            if let Err(e) = memory_graph.write().unwrap().add_connection(connection.clone()) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add connection to memory graph: {}", e))
                );
            }
            
            // Add to search index
            if let Err(e) = search_index.write().unwrap().add_connection(&connection) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add connection to search index: {}", e))
                );
            }
            
            HttpResponse::Created().json(ApiResponse::success(connection))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to store connection: {}", e))
        )
    }
}

// API Routes - Collections
async fn api_get_collections(data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    
    match storage.list_collections() {
        Ok(ids) => {
            let mut collections = Vec::new();
            for id in ids {
                if let Ok(Some(collection)) = storage.get_collection(&id) {
                    collections.push(collection);
                }
            }
            HttpResponse::Ok().json(ApiResponse::success(collections))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(&format!("Failed to list collections: {}", e))
            )
        }
    }
}

async fn api_create_collection(req: web::Json<CreateCollectionRequest>, data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    let memory_graph = &data.memory_graph;
    
    // Convert metadata if present
    let metadata = req.metadata.clone().map(|map| {
        let mut hm = std::collections::HashMap::new();
        for (k, v) in map {
            hm.insert(k, v);
        }
        hm
    });
    
    // Create the collection
    let collection = Collection::new(
        req.name.clone(),
        req.description.clone(),
        metadata,
    );
    
    // Store in storage
    match storage.put_collection(&collection) {
        Ok(_) => {
            // Add to memory graph
            if let Err(e) = memory_graph.write().unwrap().add_collection(collection.clone()) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add collection to memory graph: {}", e))
                );
            }
            HttpResponse::Created().json(ApiResponse::success(collection))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to store collection: {}", e))
        )
    }
}

// API Routes - Agents
async fn api_get_agents(data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    
    match storage.list_agents() {
        Ok(ids) => {
            let mut agents = Vec::new();
            for id in ids {
                if let Ok(Some(agent)) = storage.get_agent(&id) {
                    agents.push(agent);
                }
            }
            HttpResponse::Ok().json(ApiResponse::success(agents))
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(
                ApiResponse::<()>::error(&format!("Failed to list agents: {}", e))
            )
        }
    }
}

async fn api_create_agent(req: web::Json<CreateAgentRequest>, data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    let memory_graph = &data.memory_graph;
    
    // Convert capabilities to HashSet
    let capabilities: Option<std::collections::HashSet<String>> = 
        req.capabilities.clone().map(|v| v.into_iter().collect());
    
    // Convert metadata if present
    let metadata = req.metadata.clone().map(|map| {
        let mut hm = std::collections::HashMap::new();
        for (k, v) in map {
            hm.insert(k, v);
        }
        hm
    });
    
    // Create the agent
    let agent = Agent::new(
        req.name.clone(),
        req.description.clone(),
        capabilities,
        metadata,
    );
    
    // Store in storage
    match storage.put_agent(&agent) {
        Ok(_) => {
            // Add to memory graph
            if let Err(e) = memory_graph.write().unwrap().add_agent(agent.clone()) {
                return HttpResponse::InternalServerError().json(
                    ApiResponse::<()>::error(&format!("Failed to add agent to memory graph: {}", e))
                );
            }
            HttpResponse::Created().json(ApiResponse::success(agent))
        },
        Err(e) => HttpResponse::InternalServerError().json(
            ApiResponse::<()>::error(&format!("Failed to store agent: {}", e))
        )
    }
}

// API Routes - Search and Query
async fn api_query(req: web::Json<QueryRequest>, data: web::Data<AppState>) -> impl Responder {
    let storage = &data.storage;
    let search_index = &data.search_index;
    
    // Get the read lock on search index
    let search_index = search_index.read().unwrap();
    
    // Build the query based on request parameters
    let mut engram_ids = HashSet::new();
    
    // Search by text if provided
    if let Some(text) = &req.text {
        let text_results = search_index.text_index.search(text);
        for id in text_results {
            engram_ids.insert(id);
        }
    }
    
    // Filter by source if provided
    if let Some(source) = &req.source {
        let source_results = search_index.find_by_source(source);
        
        // If we already have text search results, intersect them
        if !engram_ids.is_empty() {
            engram_ids.retain(|id| source_results.contains(id));
        } else {
            // Otherwise, use the source results directly
            for id in source_results {
                engram_ids.insert(id);
            }
        }
    }
    
    // Filter by minimum confidence if provided
    if let Some(min_confidence) = req.min_confidence {
        let confidence_results = search_index.find_by_min_confidence(min_confidence);
        
        // If we already have search results, intersect them
        if !engram_ids.is_empty() {
            engram_ids.retain(|id| confidence_results.contains(id));
        } else {
            // Otherwise, use the confidence results directly
            for id in confidence_results {
                engram_ids.insert(id);
            }
        }
    }
    
    // If we don't have any search criteria, get all engrams
    if engram_ids.is_empty() && req.text.is_none() && req.source.is_none() && req.min_confidence.is_none() {
        if let Ok(all_ids) = storage.list_engrams() {
            for id in all_ids {
                engram_ids.insert(id);
            }
        }
    }
    
    // Fetch the full engram objects
    let mut result_engrams = Vec::new();
    for id in engram_ids {
        if let Ok(Some(engram)) = storage.get_engram(&id) {
            result_engrams.push(engram);
        }
    }
    
    // Apply limit if specified
    if let Some(limit) = req.limit {
        if result_engrams.len() > limit {
            result_engrams.truncate(limit);
        }
    }
    
    HttpResponse::Ok().json(ApiResponse::success(result_engrams))
}

// Web Server Implementation
pub fn start_server(db_path: &str, port: u16) -> EngramResult<()> {
    use std::io::Write;
    
    // Create the storage and memory graph
    let storage = Arc::new(Storage::new(db_path)?);
    let memory_graph = Arc::new(RwLock::new(MemoryGraph::new()));
    let search_index = Arc::new(RwLock::new(SearchIndex::new()));
    
    // Load data from storage into memory graph and search index
    println!("Loading data into memory graph and search index...");
    
    // Load engrams
    println!("Loading engrams...");
    let engram_ids = storage.list_engrams()?;
    for id in &engram_ids {
        if let Some(engram) = storage.get_engram(id)? {
            memory_graph.write().unwrap().add_engram(engram.clone())?;
            search_index.write().unwrap().add_engram(&engram)?;
        }
    }
    
    // Load connections
    println!("Loading connections...");
    let connection_ids = storage.list_connections()?;
    for id in &connection_ids {
        if let Some(connection) = storage.get_connection(id)? {
            // Only add connections if both source and target exist
            if engram_ids.contains(&connection.source_id) && engram_ids.contains(&connection.target_id) {
                memory_graph.write().unwrap().add_connection(connection.clone())?;
                search_index.write().unwrap().add_connection(&connection)?;
            }
        }
    }
    
    // Load collections
    println!("Loading collections...");
    let collection_ids = storage.list_collections()?;
    for id in &collection_ids {
        if let Some(collection) = storage.get_collection(id)? {
            memory_graph.write().unwrap().add_collection(collection.clone())?;
        }
    }
    
    // Load agents
    println!("Loading agents...");
    let agent_ids = storage.list_agents()?;
    for id in &agent_ids {
        if let Some(agent) = storage.get_agent(id)? {
            memory_graph.write().unwrap().add_agent(agent.clone())?;
        }
    }
    
    // Load contexts
    println!("Loading contexts...");
    let context_ids = storage.list_contexts()?;
    for id in &context_ids {
        if let Some(context) = storage.get_context(id)? {
            memory_graph.write().unwrap().add_context(context.clone())?;
        }
    }
    
    println!("Memory graph and search index loaded successfully.");
    
    // Set up Tera template engine
    println!("Initializing template engine...");
    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template error: {}", e);
            
            // Create a basic template directory and index.html if none exists
            std::fs::create_dir_all("templates").unwrap_or_default();
            
            // Create a simple index.html template
            let mut file = std::fs::File::create("templates/index.html").unwrap_or_else(|_| {
                panic!("Failed to create templates/index.html")
            });
            
            write!(file, r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>EngramAI Lite</title>
    <style>
        body {{ font-family: system-ui, -apple-system, sans-serif; margin: 0; padding: 20px; line-height: 1.6; }}
        .container {{ max-width: 1000px; margin: 0 auto; }}
        h1 {{ color: #333; }}
        .stats {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: 20px; margin-bottom: 20px; }}
        .stat-card {{ background: #f5f5f5; padding: 15px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}
        .stat-value {{ font-size: 24px; font-weight: bold; margin-bottom: 5px; }}
        .stat-label {{ color: #666; }}
        .api-section {{ margin-top: 30px; }}
        code {{ background: #f1f1f1; padding: 2px 5px; border-radius: 4px; font-family: monospace; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>EngramAI Lite Web Server</h1>
        
        <h2>Database Statistics</h2>
        <div class="stats">
            <div class="stat-card">
                <div class="stat-value">{{ engram_count }}</div>
                <div class="stat-label">Engrams</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{{ connection_count }}</div>
                <div class="stat-label">Connections</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{{ collection_count }}</div>
                <div class="stat-label">Collections</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{{ agent_count }}</div>
                <div class="stat-label">Agents</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{{ context_count }}</div>
                <div class="stat-label">Contexts</div>
            </div>
            <div class="stat-card">
                <div class="stat-value">{{ db_size }} MB</div>
                <div class="stat-label">Database Size</div>
            </div>
        </div>

        <div class="api-section">
            <h2>API Documentation</h2>
            <p>This server provides a RESTful API for interacting with the EngramAI Lite database.</p>
            
            <h3>Engrams</h3>
            <ul>
                <li><code>GET /api/engrams</code> - List all engrams</li>
                <li><code>GET /api/engrams/{{id}}</code> - Get a specific engram</li>
                <li><code>POST /api/engrams</code> - Create a new engram</li>
                <li><code>DELETE /api/engrams/{{id}}</code> - Delete an engram</li>
            </ul>
            
            <h3>Connections</h3>
            <ul>
                <li><code>GET /api/connections</code> - List all connections</li>
                <li><code>POST /api/connections</code> - Create a new connection</li>
            </ul>
            
            <h3>Collections</h3>
            <ul>
                <li><code>GET /api/collections</code> - List all collections</li>
                <li><code>POST /api/collections</code> - Create a new collection</li>
            </ul>
            
            <h3>Agents</h3>
            <ul>
                <li><code>GET /api/agents</code> - List all agents</li>
                <li><code>POST /api/agents</code> - Create a new agent</li>
            </ul>
            
            <h3>Query</h3>
            <ul>
                <li><code>POST /api/query</code> - Search and filter engrams</li>
            </ul>
        </div>
    </div>
</body>
</html>"#).unwrap();
            
            // Recreate Tera with the new template
            Tera::new("templates/**/*.html").unwrap_or_else(|e| {
                panic!("Failed to initialize Tera templates: {}", e);
            })
        }
    };
    
    // Create static directory if it doesn't exist
    std::fs::create_dir_all("static").unwrap_or_default();
    
    // Create application state
    let app_state = web::Data::new(AppState {
        db_path: db_path.to_string(),
        storage,
        memory_graph,
        search_index,
        templates: tera,
    });
    
    // Start HTTP server
    println!("Starting web server on port {}...", port);
    println!("Access the web UI at: http://localhost:{}", port);
    
    // Run the actix web server
    actix_web::rt::System::new().block_on(async {
        HttpServer::new(move || {
            // Configure CORS
            let cors = Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600);
            
            App::new()
                .wrap(middleware::Logger::default())
                .wrap(cors)
                .app_data(app_state.clone())
                // Web UI routes
                .service(web::resource("/").to(index))
                .service(web::resource("/engrams").to(engrams_page))
                .service(web::resource("/connections").to(connections_page))
                .service(web::resource("/collections").to(collections_page))
                .service(web::resource("/collections/{id}").to(collection_detail_page))
                .service(web::resource("/agents").to(agents_page))
                .service(web::resource("/graph").to(graph_page))
                .service(web::resource("/api-docs").to(api_docs_page))
                .service(fs::Files::new("/static", "static").show_files_listing())
                // API routes
                .service(
                    web::scope("/api")
                        // Engrams
                        .service(web::resource("/engrams")
                            .route(web::get().to(api_get_engrams))
                            .route(web::post().to(api_create_engram))
                        )
                        .service(web::resource("/engrams/{id}")
                            .route(web::get().to(api_get_engram))
                            .route(web::delete().to(api_delete_engram))
                        )
                        // Connections
                        .service(web::resource("/connections")
                            .route(web::get().to(api_get_connections))
                            .route(web::post().to(api_create_connection))
                        )
                        // Collections
                        .service(web::resource("/collections")
                            .route(web::get().to(api_get_collections))
                            .route(web::post().to(api_create_collection))
                        )
                        // Agents
                        .service(web::resource("/agents")
                            .route(web::get().to(api_get_agents))
                            .route(web::post().to(api_create_agent))
                        )
                        // Query
                        .service(web::resource("/query")
                            .route(web::post().to(api_query))
                        )
                )
        })
        .bind(("0.0.0.0", port)).expect("Failed to bind to address")
        .run()
        .await
        .unwrap();
    });
    
    Ok(())
}

// Add a main function to satisfy the compiler
fn main() -> EngramResult<()> {
    // This is a library component, not meant to be run directly
    println!("This is a library component. Use engramlt instead.");
    Ok(())
}