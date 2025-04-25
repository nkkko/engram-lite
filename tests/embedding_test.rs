use engram_lite::embedding::{EmbeddingModel, EmbeddingService};
use engram_lite::utils;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load environment variables from .env file for testing
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
fn test_e5_embedding_generation() {
    // Load environment variables
    load_env();

    // Check if HuggingFace API key is available
    if let Some(api_key) = utils::get_huggingface_api_key() {
        println!("Found Hugging Face API key: {}", &api_key[..5]); // Print first 5 chars for verification
        
        // Create embedding service with E5 model
        let service = EmbeddingService::with_model_type(EmbeddingModel::E5MultilingualLargeInstruct);
        
        // Test text for embedding
        let text = "This is a test sentence to verify that the E5 embedding model is working correctly.";
        
        // Generate embedding
        let result = service.embed_text(text);
        match result {
            Ok(embedding) => {
                // Verify the dimensions are correct for E5
                assert_eq!(embedding.dimensions, 1024);
                assert_eq!(embedding.vector.len(), 1024);
                
                // Verify model name
                assert_eq!(embedding.model, "intfloat/multilingual-e5-large-instruct");
                
                // Check that vector has non-zero values
                let sum: f32 = embedding.vector.iter().sum();
                assert!(sum != 0.0, "Vector should not be all zeros");
                
                println!("Successfully generated E5 embedding with {} dimensions", embedding.dimensions);
                
                // Test normalization
                let norm = embedding.normalized();
                let norm_length: f32 = norm.vector.iter().map(|x| x * x).sum::<f32>().sqrt();
                assert!((norm_length - 1.0).abs() < 0.001, "Normalized vector should have length 1.0");
            },
            Err(e) => {
                panic!("Failed to generate embedding: {:?}", e);
            }
        }
    } else {
        // Skip test if no API key found
        println!("Skipping test_e5_embedding_generation - no HuggingFace API key found in .env");
    }
}

#[test]
fn test_gte_embedding_generation() {
    // Load environment variables
    load_env();

    // Check if HuggingFace API key is available
    if let Some(_) = utils::get_huggingface_api_key() {
        // Create embedding service with GTE model
        let service = EmbeddingService::with_model_type(EmbeddingModel::GteModernBertBase);
        
        // Test text for embedding
        let text = "This is a test sentence to verify that the GTE embedding model is working correctly.";
        
        // Generate embedding
        let result = service.embed_text(text);
        match result {
            Ok(embedding) => {
                // Verify the dimensions are correct for GTE
                assert_eq!(embedding.dimensions, 768);
                assert_eq!(embedding.vector.len(), 768);
                
                // Verify model name
                assert_eq!(embedding.model, "Alibaba-NLP/gte-modernbert-base");
                
                println!("Successfully generated GTE embedding with {} dimensions", embedding.dimensions);
            },
            Err(e) => {
                panic!("Failed to generate GTE embedding: {:?}", e);
            }
        }
    } else {
        // Skip test if no API key found
        println!("Skipping test_gte_embedding_generation - no HuggingFace API key found in .env");
    }
}

#[test]
fn test_batch_embedding_generation() {
    // Load environment variables
    load_env();

    // Check if HuggingFace API key is available
    if let Some(_) = utils::get_huggingface_api_key() {
        // Create embedding service
        let service = EmbeddingService::with_model_type(EmbeddingModel::E5MultilingualLargeInstruct);
        
        // Test multiple texts
        let texts = [
            "This is the first test sentence.", 
            "This is the second test sentence.",
            "This is the third test sentence with different content."
        ];
        
        // Generate batch embeddings
        let result = service.embed_batch(&texts);
        match result {
            Ok(embeddings) => {
                // Verify we got the right number of embeddings
                assert_eq!(embeddings.len(), texts.len());
                
                // Verify each embedding
                for (i, embedding) in embeddings.iter().enumerate() {
                    assert_eq!(embedding.dimensions, 1024);
                    assert_eq!(embedding.vector.len(), 1024);
                    println!("Successfully generated embedding {} of {}", i+1, texts.len());
                }
                
                // Test similarity between embeddings
                // Similar sentences should have higher similarity
                let sim_1_2 = match embeddings[0].cosine_similarity(&embeddings[1]) {
                    Ok(value) => value,
                    Err(e) => panic!("Failed to compute similarity: {:?}", e)
                };
                
                let sim_1_3 = match embeddings[0].cosine_similarity(&embeddings[2]) {
                    Ok(value) => value,
                    Err(e) => panic!("Failed to compute similarity: {:?}", e)
                };
                
                println!("Similarity between similar sentences: {:.4}", sim_1_2);
                println!("Similarity between different sentences: {:.4}", sim_1_3);
                
                // The first two sentences are more similar than the first and third
                assert!(sim_1_2 > sim_1_3, 
                    "Similar sentences should have higher similarity (got {:.4} vs {:.4})", 
                    sim_1_2, sim_1_3);
            },
            Err(e) => {
                panic!("Failed to generate batch embeddings: {:?}", e);
            }
        }
    } else {
        // Skip test if no API key found
        println!("Skipping test_batch_embedding_generation - no HuggingFace API key found in .env");
    }
}