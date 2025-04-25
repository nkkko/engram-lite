use engram_lite::embedding::{Embedding, EmbeddingModel, EmbeddingService};
use engram_lite::schema::Engram;
use engram_lite::vector_search::VectorIndex;
use engram_lite::utils;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::Arc;

// Helper to load environment variables
fn load_env() {
    let env_path = ".env";
    if Path::new(env_path).exists() {
        let file = File::open(env_path).expect("Failed to open .env file");
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(line) = line {
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }

                if let Some(equal_sign_pos) = line.find('=') {
                    let key = line[..equal_sign_pos].trim();
                    let value = line[equal_sign_pos + 1..].trim();
                    
                    // Only set if not already set
                    if env::var(key).is_err() {
                        env::set_var(key, value);
                    }
                }
            }
        }
    }
}

#[test]
fn test_vector_index_creation() {
    // Create a vector index with default settings
    let vector_index = VectorIndex::new();
    
    // Basic sanity check
    assert!(vector_index.dimensions() > 0);
    assert_eq!(vector_index.len(), 0);
}

#[test]
fn test_vector_index_with_embedding_service() {
    // Load environment variables
    load_env();
    
    // Create embedding service
    let embedding_service = Arc::new(EmbeddingService::with_model_type(EmbeddingModel::E5MultilingualLargeInstruct));
    
    // Create vector index with the embedding service
    let vector_index = VectorIndex::with_embedding_service(embedding_service);
    
    // Verify dimensions match the model
    assert_eq!(vector_index.dimensions(), 1024);
    assert_eq!(vector_index.len(), 0);
}

#[test]
fn test_vector_index_add_and_search() {
    // Load environment variables
    load_env();
    
    // Skip test if no HuggingFace API key available
    if utils::get_huggingface_api_key().is_none() {
        println!("Skipping test_vector_index_add_and_search - no HuggingFace API key found");
        return;
    }
    
    // Create embedding service
    let embedding_service = Arc::new(EmbeddingService::new());
    
    // Create vector index
    let vector_index = VectorIndex::with_embedding_service(embedding_service);
    
    // Create test engrams with related content
    let engram1 = Engram::new(
        "The quick brown fox jumps over the lazy dog".to_string(),
        "test".to_string(),
        0.9,
        None,
    );
    
    let engram2 = Engram::new(
        "A swift canine leaps above a sleepy hound".to_string(), // Similar to engram1
        "test".to_string(),
        0.9,
        None,
    );
    
    let engram3 = Engram::new(
        "Artificial intelligence is transforming technology".to_string(), // Different topic
        "test".to_string(),
        0.9,
        None,
    );
    
    // Add engrams to the index
    vector_index.add_engram(&engram1).expect("Failed to add engram1");
    vector_index.add_engram(&engram2).expect("Failed to add engram2");
    vector_index.add_engram(&engram3).expect("Failed to add engram3");
    
    // Verify engrams were added
    assert_eq!(vector_index.len(), 3);
    
    // Search for similar engrams
    let results = vector_index.search("fox jumping over dog", 2).expect("Search failed");
    
    // Check that we got 2 results
    assert_eq!(results.len(), 2);
    
    // The first two engrams should be returned as they're semantically similar
    // They should be returned in order of similarity
    let result_ids: Vec<String> = results.iter().map(|r| r.id.clone()).collect();
    assert!(result_ids.contains(&engram1.id));
    assert!(result_ids.contains(&engram2.id));
    
    // The third engram should not be returned as it's on a different topic
    assert!(!result_ids.contains(&engram3.id));
}

#[test]
fn test_vector_index_delete() {
    // Load environment variables
    load_env();
    
    // Skip test if no HuggingFace API key available
    if utils::get_huggingface_api_key().is_none() {
        println!("Skipping test_vector_index_delete - no HuggingFace API key found");
        return;
    }
    
    // Create vector index
    let vector_index = VectorIndex::new();
    
    // Create test engram
    let engram = Engram::new(
        "Test content for deletion".to_string(),
        "test".to_string(),
        0.9,
        None,
    );
    let engram_id = engram.id.clone();
    
    // Add engram to the index
    vector_index.add_engram(&engram).expect("Failed to add engram");
    
    // Verify engram was added
    assert_eq!(vector_index.len(), 1);
    
    // Delete the engram
    vector_index.delete_engram(&engram_id).expect("Failed to delete engram");
    
    // Verify engram was removed
    assert_eq!(vector_index.len(), 0);
    
    // Search should return empty results
    let results = vector_index.search("test content", 10).expect("Search failed");
    assert_eq!(results.len(), 0);
}

#[test]
fn test_hybrid_search() {
    // Load environment variables
    load_env();
    
    // Skip test if no HuggingFace API key available
    if utils::get_huggingface_api_key().is_none() {
        println!("Skipping test_hybrid_search - no HuggingFace API key found");
        return;
    }
    
    // Create vector index
    let vector_index = VectorIndex::new();
    
    // Create test engrams with specific keywords
    let engram1 = Engram::new(
        "The quick brown fox jumps over the lazy dog".to_string(),
        "animals".to_string(),
        0.9,
        None,
    );
    
    let engram2 = Engram::new(
        "A swift canine leaps above a sleepy hound".to_string(),
        "animals".to_string(),
        0.9,
        None,
    );
    
    let engram3 = Engram::new(
        "Artificial intelligence and machine learning are transforming technology".to_string(),
        "technology".to_string(),
        0.9,
        None,
    );
    
    // Add engrams to the index
    vector_index.add_engram(&engram1).expect("Failed to add engram1");
    vector_index.add_engram(&engram2).expect("Failed to add engram2");
    vector_index.add_engram(&engram3).expect("Failed to add engram3");
    
    // Perform hybrid search combining vector similarity and keyword matching
    let results = vector_index.hybrid_search(
        "machine learning",  // Semantic query
        &["technology"],     // Required keywords
        None,               // No excluded keywords
        None,               // No minimum confidence
        None,               // No source filter
        10                  // Limit
    ).expect("Hybrid search failed");
    
    // Check that we got appropriate results
    assert!(results.len() > 0);
    
    // The technology-related engram should be returned
    assert!(results.iter().any(|r| r.id == engram3.id));
    
    // The animal-related engrams should not be returned even if semantically similar
    assert!(!results.iter().any(|r| r.id == engram1.id));
    assert!(!results.iter().any(|r| r.id == engram2.id));
}