use crate::error::Result;
use crate::schema::{Agent, Collection, Connection, Context, Engram};
use crate::storage::Storage;
use std::collections::{HashMap, HashSet};

/// Populates the database with demo data for a multi-agent coding team scenario
pub fn populate_demo_data(db_path: &str) -> Result<()> {
    println!("Populating database with demo data...");
    let storage = Storage::new(db_path)?;
    
    // Clear existing data (optional - depends on whether we want to add to existing data)
    println!("Clearing existing data...");
    let engram_ids = storage.list_engrams()?;
    let connection_ids = storage.list_connections()?;
    let collection_ids = storage.list_collections()?;
    let agent_ids = storage.list_agents()?;
    let context_ids = storage.list_contexts()?;
    
    for id in &connection_ids {
        storage.delete_connection(id)?;
    }
    
    for id in &engram_ids {
        storage.delete_engram(id)?;
    }
    
    for id in &collection_ids {
        storage.delete_collection(id)?;
    }
    
    for id in &agent_ids {
        storage.delete_agent(id)?;
    }
    
    for id in &context_ids {
        storage.delete_context(id)?;
    }
    
    println!("Creating agents...");
    
    // Create agents
    let developer = create_developer_agent(&storage)?;
    let tester = create_tester_agent(&storage)?;
    let documenter = create_documenter_agent(&storage)?;
    let project_manager = create_project_manager_agent(&storage)?;
    
    println!("Creating project collections...");
    
    // Create collections for different aspects of the project
    let requirements = create_collection(&storage, "Requirements", "Project requirements and specifications")?;
    let code_design = create_collection(&storage, "Code Design", "Architecture and design decisions")?;
    let implementation = create_collection(&storage, "Implementation", "Actual code implementation details")?;
    let testing = create_collection(&storage, "Testing", "Test plans, test cases, and results")?;
    let documentation = create_collection(&storage, "Documentation", "Project documentation")?;
    
    println!("Granting collection access to agents...");
    
    // Grant collection access to agents based on their roles
    grant_agent_access(&storage, &developer.id, &[&requirements.id, &code_design.id, &implementation.id])?;
    grant_agent_access(&storage, &tester.id, &[&requirements.id, &implementation.id, &testing.id])?;
    grant_agent_access(&storage, &documenter.id, &[&requirements.id, &code_design.id, &documentation.id])?;
    grant_agent_access(&storage, &project_manager.id, &[&requirements.id, &code_design.id, &implementation.id, &testing.id, &documentation.id])?;
    
    println!("Creating project context...");
    
    // Create a shared context for collaborative work
    let mut project_context = Context::new(
        "Weather App Project".to_string(),
        "A collaborative project to build a weather forecasting application".to_string(),
        None,
    );
    
    // Add agents to the context
    project_context.add_agent(developer.id.clone());
    project_context.add_agent(tester.id.clone());
    project_context.add_agent(documenter.id.clone());
    project_context.add_agent(project_manager.id.clone());
    
    // Store the context
    storage.put_context(&project_context)?;
    
    println!("Adding project requirements...");
    
    // Add project requirements engrams
    let req1 = create_engram(
        &storage,
        "The application should provide weather forecasts for user-specified locations.",
        "project_manager",
        0.95,
        Some(create_metadata(&[("type", "requirement"), ("priority", "high")]))
    )?;
    
    let req2 = create_engram(
        &storage,
        "Users should be able to save favorite locations for quick access.",
        "project_manager",
        0.9,
        Some(create_metadata(&[("type", "requirement"), ("priority", "medium")]))
    )?;
    
    let req3 = create_engram(
        &storage,
        "The application should display temperature, precipitation, wind, and humidity data.",
        "project_manager",
        0.95,
        Some(create_metadata(&[("type", "requirement"), ("priority", "high")]))
    )?;
    
    let req4 = create_engram(
        &storage,
        "The interface should be responsive and work on both desktop and mobile devices.",
        "project_manager",
        0.85,
        Some(create_metadata(&[("type", "requirement"), ("priority", "medium")]))
    )?;
    
    let req5 = create_engram(
        &storage,
        "Weather data should be retrieved from the OpenWeatherMap API.",
        "project_manager",
        0.9,
        Some(create_metadata(&[("type", "requirement"), ("priority", "high")]))
    )?;
    
    // Add requirements to the requirements collection
    add_to_collection(&storage, &requirements.id, &[&req1.id, &req2.id, &req3.id, &req4.id, &req5.id])?;
    
    // Add requirements to the project context
    add_to_context(&storage, &project_context.id, &[&req1.id, &req2.id, &req3.id, &req4.id, &req5.id])?;
    
    println!("Adding design decisions...");
    
    // Add design decision engrams
    let design1 = create_engram(
        &storage,
        "We will use React with TypeScript for the frontend implementation.",
        "developer",
        0.9,
        Some(create_metadata(&[("type", "design_decision"), ("area", "frontend")]))
    )?;
    
    let design2 = create_engram(
        &storage,
        "We will use a Node.js backend with Express to handle API requests.",
        "developer",
        0.85,
        Some(create_metadata(&[("type", "design_decision"), ("area", "backend")]))
    )?;
    
    let design3 = create_engram(
        &storage,
        "We will implement a caching layer to minimize API calls to OpenWeatherMap.",
        "developer",
        0.8,
        Some(create_metadata(&[("type", "design_decision"), ("area", "performance")]))
    )?;
    
    let design4 = create_engram(
        &storage,
        "User preferences will be stored in localStorage for simplicity.",
        "developer",
        0.75,
        Some(create_metadata(&[("type", "design_decision"), ("area", "data_storage")]))
    )?;
    
    // Add design decisions to the code design collection
    add_to_collection(&storage, &code_design.id, &[&design1.id, &design2.id, &design3.id, &design4.id])?;
    
    // Add design decisions to the project context
    add_to_context(&storage, &project_context.id, &[&design1.id, &design2.id, &design3.id, &design4.id])?;
    
    // Connect requirements to design decisions
    create_connection(&storage, &req1.id, &design1.id, "influences", 0.9)?;
    create_connection(&storage, &req1.id, &design2.id, "influences", 0.9)?;
    create_connection(&storage, &req3.id, &design1.id, "influences", 0.85)?;
    create_connection(&storage, &req5.id, &design2.id, "influences", 0.95)?;
    create_connection(&storage, &req5.id, &design3.id, "influences", 0.9)?;
    create_connection(&storage, &req2.id, &design4.id, "influences", 0.9)?;
    
    println!("Adding implementation details...");
    
    // Add implementation engrams
    let impl1 = create_engram(
        &storage,
        "Created WeatherService class to handle API communication with OpenWeatherMap.",
        "developer",
        0.9,
        Some(create_metadata(&[("type", "implementation"), ("component", "api_service")]))
    )?;
    
    let impl2 = create_engram(
        &storage,
        "Implemented responsive UI components using React and styled-components.",
        "developer",
        0.85,
        Some(create_metadata(&[("type", "implementation"), ("component", "ui")]))
    )?;
    
    let impl3 = create_engram(
        &storage,
        "Created LocationSearch component with autocomplete for city lookup.",
        "developer",
        0.8,
        Some(create_metadata(&[("type", "implementation"), ("component", "search")]))
    )?;
    
    let impl4 = create_engram(
        &storage,
        "Implemented favorites feature using localStorage and React context.",
        "developer",
        0.85,
        Some(create_metadata(&[("type", "implementation"), ("component", "favorites")]))
    )?;
    
    let code_snippet = create_engram(
        &storage,
        "```typescript\nclass WeatherService {\n  private apiKey: string;\n  private cache: Map<string, WeatherData>;\n\n  constructor(apiKey: string) {\n    this.apiKey = apiKey;\n    this.cache = new Map();\n  }\n\n  async getWeather(location: string): Promise<WeatherData> {\n    // Check cache first\n    if (this.cache.has(location)) {\n      return this.cache.get(location)!;\n    }\n    \n    // Fetch from API\n    const response = await fetch(\n      `https://api.openweathermap.org/data/2.5/weather?q=${location}&appid=${this.apiKey}&units=metric`\n    );\n    \n    if (!response.ok) {\n      throw new Error(`Weather data fetch failed: ${response.statusText}`);\n    }\n    \n    const data = await response.json();\n    const weatherData = this.transformApiResponse(data);\n    \n    // Update cache\n    this.cache.set(location, weatherData);\n    \n    return weatherData;\n  }\n\n  private transformApiResponse(data: any): WeatherData {\n    return {\n      location: data.name,\n      temperature: data.main.temp,\n      humidity: data.main.humidity,\n      windSpeed: data.wind.speed,\n      precipitation: data.rain ? data.rain['1h'] : 0,\n      description: data.weather[0].description,\n      timestamp: new Date().toISOString()\n    };\n  }\n}\n```",
        "developer",
        0.95,
        Some(create_metadata(&[("type", "code"), ("language", "typescript"), ("component", "api_service")]))
    )?;
    
    // Add implementation details to the implementation collection
    add_to_collection(&storage, &implementation.id, &[&impl1.id, &impl2.id, &impl3.id, &impl4.id, &code_snippet.id])?;
    
    // Add implementation details to the project context
    add_to_context(&storage, &project_context.id, &[&impl1.id, &impl2.id, &impl3.id, &impl4.id, &code_snippet.id])?;
    
    // Connect design decisions to implementation
    create_connection(&storage, &design1.id, &impl2.id, "implements", 0.9)?;
    create_connection(&storage, &design2.id, &impl1.id, "implements", 0.9)?;
    create_connection(&storage, &design3.id, &impl1.id, "implements", 0.85)?;
    create_connection(&storage, &design4.id, &impl4.id, "implements", 0.9)?;
    create_connection(&storage, &impl1.id, &code_snippet.id, "contains", 0.95)?;
    
    println!("Adding test cases...");
    
    // Add test engrams
    let test1 = create_engram(
        &storage,
        "Unit test for WeatherService: should retrieve data from API when not in cache.",
        "tester",
        0.9,
        Some(create_metadata(&[("type", "test_case"), ("test_type", "unit"), ("status", "passed")]))
    )?;
    
    let test2 = create_engram(
        &storage,
        "Unit test for WeatherService: should return cached data when available.",
        "tester",
        0.9,
        Some(create_metadata(&[("type", "test_case"), ("test_type", "unit"), ("status", "passed")]))
    )?;
    
    let test3 = create_engram(
        &storage,
        "Integration test: LocationSearch component should display search results.",
        "tester",
        0.85,
        Some(create_metadata(&[("type", "test_case"), ("test_type", "integration"), ("status", "failed")]))
    )?;
    
    let test4 = create_engram(
        &storage,
        "End-to-end test: User can add a location to favorites.",
        "tester",
        0.8,
        Some(create_metadata(&[("type", "test_case"), ("test_type", "e2e"), ("status", "pending")]))
    )?;
    
    let test_failure = create_engram(
        &storage,
        "LocationSearch component test is failing because the API response is not being mocked correctly.",
        "tester",
        0.95,
        Some(create_metadata(&[("type", "test_issue"), ("priority", "high")]))
    )?;
    
    // Add test cases to the testing collection
    add_to_collection(&storage, &testing.id, &[&test1.id, &test2.id, &test3.id, &test4.id, &test_failure.id])?;
    
    // Add test cases to the project context
    add_to_context(&storage, &project_context.id, &[&test1.id, &test2.id, &test3.id, &test4.id, &test_failure.id])?;
    
    // Connect implementation to tests
    create_connection(&storage, &impl1.id, &test1.id, "verified_by", 0.9)?;
    create_connection(&storage, &impl1.id, &test2.id, "verified_by", 0.9)?;
    create_connection(&storage, &impl3.id, &test3.id, "verified_by", 0.7)?;
    create_connection(&storage, &impl4.id, &test4.id, "verified_by", 0.8)?;
    create_connection(&storage, &test3.id, &test_failure.id, "resulted_in", 0.95)?;
    
    println!("Adding documentation...");
    
    // Add documentation engrams
    let doc1 = create_engram(
        &storage,
        "Weather App API Documentation: Describes the OpenWeatherMap API integration.",
        "documenter",
        0.9,
        Some(create_metadata(&[("type", "documentation"), ("doc_type", "api"), ("status", "draft")]))
    )?;
    
    let doc2 = create_engram(
        &storage,
        "User Guide: How to search for locations and view weather forecasts.",
        "documenter",
        0.85,
        Some(create_metadata(&[("type", "documentation"), ("doc_type", "user_guide"), ("status", "draft")]))
    )?;
    
    let doc3 = create_engram(
        &storage,
        "Developer Guide: How to extend the Weather App with new features.",
        "documenter",
        0.8,
        Some(create_metadata(&[("type", "documentation"), ("doc_type", "developer_guide"), ("status", "todo")]))
    )?;
    
    let api_doc_content = create_engram(
        &storage,
        "# Weather Service API Documentation\n\n## Overview\nThe Weather Service API provides methods to retrieve weather data from OpenWeatherMap.\n\n## Methods\n\n### getWeather(location: string): Promise<WeatherData>\nRetrieves weather data for the specified location.\n\n#### Parameters\n- location: A string representing the city name (e.g., 'London', 'New York')\n\n#### Returns\nA Promise that resolves to a WeatherData object with the following properties:\n- location: string\n- temperature: number (in Celsius)\n- humidity: number (percentage)\n- windSpeed: number (in m/s)\n- precipitation: number (in mm)\n- description: string\n- timestamp: string (ISO format)\n\n#### Error Handling\nThrows an Error if the API request fails.\n\n## Data Caching\nThe Weather Service implements a simple in-memory cache to avoid unnecessary API calls. Weather data is cached by location name and will be returned from cache when available.",
        "documenter",
        0.95,
        Some(create_metadata(&[("type", "documentation_content"), ("doc_type", "api"), ("status", "draft")]))
    )?;
    
    // Add documentation to the documentation collection
    add_to_collection(&storage, &documentation.id, &[&doc1.id, &doc2.id, &doc3.id, &api_doc_content.id])?;
    
    // Add documentation to the project context
    add_to_context(&storage, &project_context.id, &[&doc1.id, &doc2.id, &doc3.id, &api_doc_content.id])?;
    
    // Connect implementation to documentation
    create_connection(&storage, &impl1.id, &doc1.id, "documented_by", 0.9)?;
    create_connection(&storage, &impl2.id, &doc2.id, "documented_by", 0.85)?;
    create_connection(&storage, &impl3.id, &doc2.id, "documented_by", 0.8)?;
    create_connection(&storage, &doc1.id, &api_doc_content.id, "contains", 0.95)?;
    
    println!("Adding team interactions...");
    
    // Add team interactions
    let discussion1 = create_engram(
        &storage,
        "We need to implement mock API responses for the LocationSearch component tests.",
        "developer",
        0.9,
        Some(create_metadata(&[("type", "discussion"), ("meeting", "team_standup"), ("date", "2023-05-10")]))
    )?;
    
    let discussion2 = create_engram(
        &storage,
        "I'll create the API mocking framework for the integration tests today.",
        "developer",
        0.85,
        Some(create_metadata(&[("type", "discussion"), ("meeting", "team_standup"), ("date", "2023-05-10")]))
    )?;
    
    let discussion3 = create_engram(
        &storage,
        "We should also document how to create and use mocks in the developer guide.",
        "tester",
        0.8,
        Some(create_metadata(&[("type", "discussion"), ("meeting", "team_standup"), ("date", "2023-05-10")]))
    )?;
    
    let discussion4 = create_engram(
        &storage,
        "I'll update the developer guide with the testing approach once the mocking framework is ready.",
        "documenter",
        0.85,
        Some(create_metadata(&[("type", "discussion"), ("meeting", "team_standup"), ("date", "2023-05-10")]))
    )?;
    
    // Add discussions to the project context
    add_to_context(&storage, &project_context.id, &[&discussion1.id, &discussion2.id, &discussion3.id, &discussion4.id])?;
    
    // Connect discussions to relevant engrams
    create_connection(&storage, &test_failure.id, &discussion1.id, "prompted", 0.9)?;
    create_connection(&storage, &discussion1.id, &discussion2.id, "followed_by", 0.9)?;
    create_connection(&storage, &discussion2.id, &discussion3.id, "followed_by", 0.9)?;
    create_connection(&storage, &discussion3.id, &discussion4.id, "followed_by", 0.9)?;
    create_connection(&storage, &discussion4.id, &doc3.id, "relates_to", 0.85)?;
    
    println!("Demo data population complete!");
    
    // Print some summary statistics
    let stats = storage.get_stats()?;
    println!("Created {} engrams, {} connections, {} collections, {} agents, and {} contexts",
        stats.engram_count, 
        stats.connection_count,
        stats.collection_count,
        stats.agent_count,
        stats.context_count
    );
    
    println!("\nSuggested exploration commands:");
    println!("1. List agents: engramlt cli --db-path {} (then type 'list-agents')", db_path);
    println!("2. List collections: engramlt cli --db-path {} (then type 'list-collections')", db_path);
    println!("3. Explore project context: Use TUI mode 'engramlt tui --db-path {}'", db_path);
    println!("4. Try web interface: 'engramlt web --db-path {}'", db_path);

    Ok(())
}

// Helper functions

fn create_metadata(pairs: &[(&str, &str)]) -> HashMap<String, serde_json::Value> {
    let mut metadata = HashMap::new();
    for (key, value) in pairs {
        metadata.insert(key.to_string(), serde_json::Value::String(value.to_string()));
    }
    metadata
}

fn create_developer_agent(storage: &Storage) -> Result<Agent> {
    let mut capabilities = HashSet::new();
    capabilities.insert("write_code".to_string());
    capabilities.insert("review_code".to_string());
    capabilities.insert("design_architecture".to_string());
    
    let agent = Agent::new(
        "Software Developer".to_string(),
        "Responsible for designing and implementing code".to_string(),
        Some(capabilities),
        Some(create_metadata(&[
            ("tools", "VS Code, Git"),
            ("languages", "TypeScript, JavaScript, Node.js"),
            ("experience", "Senior")
        ]))
    );
    
    storage.put_agent(&agent)?;
    Ok(agent)
}

fn create_tester_agent(storage: &Storage) -> Result<Agent> {
    let mut capabilities = HashSet::new();
    capabilities.insert("write_tests".to_string());
    capabilities.insert("run_tests".to_string());
    capabilities.insert("report_bugs".to_string());
    
    let agent = Agent::new(
        "Quality Assurance Engineer".to_string(),
        "Responsible for testing and quality assurance".to_string(),
        Some(capabilities),
        Some(create_metadata(&[
            ("tools", "Jest, Cypress, Selenium"),
            ("test_types", "Unit, Integration, E2E"),
            ("experience", "Mid-level")
        ]))
    );
    
    storage.put_agent(&agent)?;
    Ok(agent)
}

fn create_documenter_agent(storage: &Storage) -> Result<Agent> {
    let mut capabilities = HashSet::new();
    capabilities.insert("write_documentation".to_string());
    capabilities.insert("create_diagrams".to_string());
    capabilities.insert("review_documentation".to_string());
    
    let agent = Agent::new(
        "Technical Writer".to_string(),
        "Responsible for creating and maintaining documentation".to_string(),
        Some(capabilities),
        Some(create_metadata(&[
            ("tools", "Markdown, Mermaid, Sphinx"),
            ("doc_types", "API docs, User guides, Architecture diagrams"),
            ("experience", "Senior")
        ]))
    );
    
    storage.put_agent(&agent)?;
    Ok(agent)
}

fn create_project_manager_agent(storage: &Storage) -> Result<Agent> {
    let mut capabilities = HashSet::new();
    capabilities.insert("plan_projects".to_string());
    capabilities.insert("track_progress".to_string());
    capabilities.insert("manage_team".to_string());
    capabilities.insert("gather_requirements".to_string());
    
    let agent = Agent::new(
        "Project Manager".to_string(),
        "Responsible for project planning and coordination".to_string(),
        Some(capabilities),
        Some(create_metadata(&[
            ("tools", "Jira, Confluence, Slack"),
            ("methodologies", "Agile, Scrum"),
            ("experience", "Senior")
        ]))
    );
    
    storage.put_agent(&agent)?;
    Ok(agent)
}

fn create_collection(storage: &Storage, name: &str, description: &str) -> Result<Collection> {
    let collection = Collection::new(
        name.to_string(),
        description.to_string(),
        None
    );
    
    storage.put_collection(&collection)?;
    Ok(collection)
}

fn grant_agent_access(storage: &Storage, agent_id: &str, collection_ids: &[&str]) -> Result<()> {
    if let Some(mut agent) = storage.get_agent(&agent_id.to_string())? {
        for &collection_id in collection_ids {
            agent.grant_access(collection_id.to_string());
        }
        storage.put_agent(&agent)?;
    }
    Ok(())
}

fn create_engram(
    storage: &Storage,
    content: &str,
    source: &str,
    confidence: f64,
    metadata: Option<HashMap<String, serde_json::Value>>
) -> Result<Engram> {
    let engram = Engram::new(
        content.to_string(),
        source.to_string(),
        confidence,
        metadata
    );
    
    storage.put_engram(&engram)?;
    Ok(engram)
}

fn add_to_collection(storage: &Storage, collection_id: &str, engram_ids: &[&str]) -> Result<()> {
    if let Some(mut collection) = storage.get_collection(&collection_id.to_string())? {
        for &engram_id in engram_ids {
            collection.add_engram(engram_id.to_string());
        }
        storage.put_collection(&collection)?;
    }
    Ok(())
}

fn add_to_context(storage: &Storage, context_id: &str, engram_ids: &[&str]) -> Result<()> {
    if let Some(mut context) = storage.get_context(&context_id.to_string())? {
        for &engram_id in engram_ids {
            context.add_engram(engram_id.to_string());
        }
        storage.put_context(&context)?;
    }
    Ok(())
}

fn create_connection(
    storage: &Storage,
    source_id: &str,
    target_id: &str,
    relationship_type: &str,
    weight: f64
) -> Result<Connection> {
    let connection = Connection::new(
        source_id.to_string(),
        target_id.to_string(),
        relationship_type.to_string(),
        weight,
        None
    );
    
    storage.put_connection(&connection)?;
    Ok(connection)
}