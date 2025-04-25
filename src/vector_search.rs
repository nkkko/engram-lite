use crate::embedding::{Embedding, EmbeddingService, HnswIndex};
use crate::error::{EngramError, Result};
use crate::schema::{Engram, EngramId};
use crate::storage::Storage;
use crate::index::SearchIndex;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Vector search index for efficient similarity search
pub struct VectorIndex {
    /// HNSW index for fast approximate nearest neighbor search
    index: RwLock<HnswIndex>,
    
    /// Mapping from engram IDs to their position in the index
    id_to_index: RwLock<HashMap<EngramId, usize>>,
    
    /// The embedding service used to generate embeddings
    embedding_service: Arc<EmbeddingService>,
    
    /// The dimensionality of the vectors
    #[allow(dead_code)]
    dimensions: usize,
    
    /// Whether to use reduced embeddings for search
    use_reduced_embeddings: bool,
}

impl VectorIndex {
    /// Create a new vector index with the default embedding service
    pub fn new() -> Self {
        let embedding_service = Arc::new(EmbeddingService::new());
        let dimensions = embedding_service.get_dimensions();
        
        Self {
            index: RwLock::new(HnswIndex::new(dimensions)),
            id_to_index: RwLock::new(HashMap::new()),
            embedding_service,
            dimensions,
            use_reduced_embeddings: false,
        }
    }
    
    /// Create a new vector index with a specific embedding service
    pub fn with_embedding_service(embedding_service: Arc<EmbeddingService>) -> Self {
        let dimensions = embedding_service.get_dimensions();
        
        Self {
            index: RwLock::new(HnswIndex::new(dimensions)),
            id_to_index: RwLock::new(HashMap::new()),
            embedding_service,
            dimensions,
            use_reduced_embeddings: false,
        }
    }
    
    /// Configure the vector index to use reduced embeddings
    pub fn with_reduced_embeddings(mut self, use_reduced: bool) -> Self {
        self.use_reduced_embeddings = use_reduced;
        self
    }
    
    /// Add an engram to the index
    pub fn add_engram(&self, engram: &Engram) -> Result<()> {
        // Check if we already have an embedding in the metadata
        if let Some(embedding) = self.get_embedding_from_metadata(engram) {
            // Add to index
            self.index.write().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire write lock on vector index".to_string())
            })?.add(&engram.id, embedding)?;
            
            // Update mapping
            self.id_to_index.write().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire write lock on id mapping".to_string())
            })?.insert(engram.id.clone(), self.index.read().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
            })?.len() - 1);
            
            return Ok(());
        }
        
        // Get embedding
        let embedding = self.embedding_service.embed_text(&engram.content)?;
        
        // Use reduced embedding if needed
        let embedding_to_use = if self.use_reduced_embeddings {
            // Try to reduce the embedding if a dimension reducer is available
            match self.embedding_service.reduce_embedding(&embedding) {
                Ok(reduced) => reduced,
                Err(_) => embedding
            }
        } else {
            embedding
        };
        
        // Add to index
        self.index.write().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire write lock on vector index".to_string())
        })?.add(&engram.id, embedding_to_use)?;
        
        // Update mapping
        self.id_to_index.write().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire write lock on id mapping".to_string())
        })?.insert(engram.id.clone(), self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?.len() - 1);
        
        Ok(())
    }
    
    /// Add an engram to the index using stored embeddings from storage
    pub fn add_engram_with_storage(&self, engram: &Engram, storage: &Storage) -> Result<()> {
        // First try to get the appropriate type of embedding from storage
        let embedding = if self.use_reduced_embeddings {
            // Try to get a reduced embedding
            if let Some(reduced) = storage.get_reduced_embedding(&engram.id)? {
                // Convert storage::Embedding to embedding::Embedding
                // Manual conversion from storage::Embedding to embedding::Embedding
                Some(crate::embedding::Embedding {
                    vector: reduced.vector.clone(),
                    model: reduced.model.clone(),
                    dimensions: reduced.dimensions,
                    metadata: reduced.metadata.clone(),
                })
            } else if let Some(original) = storage.get_embedding(&engram.id)? {
                // If we have the original but not reduced, try to reduce it
                if let Some(reducer_arc) = &self.embedding_service.dimension_reducer {
                    let reducer = reducer_arc.lock().map_err(|_| {
                        EngramError::ConcurrencyError("Failed to acquire lock on dimension reducer".to_string())
                    })?;
                    
                    if reducer.is_trained() {
                        // Reduce and store for future use
                        // Manual conversion
                        let original_embedding = crate::embedding::Embedding {
                            vector: original.vector.clone(),
                            model: original.model.clone(),
                            dimensions: original.dimensions,
                            metadata: original.metadata.clone(),
                        };
                        let reduced = reducer.reduce(&original_embedding)?;
                        // Manual conversion back to storage::Embedding
                        let storage_reduced = crate::storage::Embedding {
                            vector: reduced.vector.clone(),
                            model: reduced.model.clone(),
                            dimensions: reduced.dimensions,
                            metadata: reduced.metadata.clone(),
                        };
                        storage.put_reduced_embedding(&engram.id, &storage_reduced)?;
                        Some(reduced)
                    } else {
                        // Fall back to original if reducer isn't trained
                        // Convert to embedding::Embedding
                        // Manual conversion
                        Some(crate::embedding::Embedding {
                            vector: original.vector.clone(),
                            model: original.model.clone(),
                            dimensions: original.dimensions,
                            metadata: original.metadata.clone(),
                        })
                    }
                } else {
                    // No reducer available, convert and use original
                    // Manual conversion
                    Some(crate::embedding::Embedding {
                        vector: original.vector.clone(),
                        model: original.model.clone(),
                        dimensions: original.dimensions,
                        metadata: original.metadata.clone(),
                    })
                }
            } else {
                None
            }
        } else {
            // Just try to get the original embedding and convert it
            match storage.get_embedding(&engram.id)? {
                Some(original) => {
                    // Manual conversion
                    Some(crate::embedding::Embedding {
                        vector: original.vector.clone(),
                        model: original.model.clone(),
                        dimensions: original.dimensions,
                        metadata: original.metadata.clone(),
                    })
                },
                None => None
            }
        };
        
        if let Some(embedding) = embedding {
            // Add to index
            self.index.write().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire write lock on vector index".to_string())
            })?.add(&engram.id, embedding)?;
            
            // Update mapping
            self.id_to_index.write().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire write lock on id mapping".to_string())
            })?.insert(engram.id.clone(), self.index.read().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
            })?.len() - 1);
            
            return Ok(());
        }
        
        // If we don't have a stored embedding, generate a new one
        self.add_engram(engram)
    }
    
    /// Remove an engram from the index
    pub fn remove_engram(&self, engram_id: &EngramId) -> Result<bool> {
        // Remove from index
        let removed = self.index.write().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire write lock on vector index".to_string())
        })?.remove(engram_id)?;
        
        // Remove from mapping
        if removed {
            self.id_to_index.write().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire write lock on id mapping".to_string())
            })?.remove(engram_id);
        }
        
        Ok(removed)
    }
    
    /// Search for similar engrams
    pub fn search(&self, query: &str, k: usize) -> Result<Vec<(EngramId, f32)>> {
        // Generate embedding from query
        let embedding = self.embedding_service.embed_text(query)?;
        
        // Use reduced embedding if needed
        let query_embedding = if self.use_reduced_embeddings {
            // Try to reduce the embedding if a dimension reducer is available
            match self.embedding_service.reduce_embedding(&embedding) {
                Ok(reduced) => reduced,
                Err(_) => embedding
            }
        } else {
            embedding
        };
        
        // Search index
        self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?.search(&query_embedding, k)
    }
    
    /// Search using an existing embedding
    pub fn search_by_embedding(&self, embedding: &Embedding, k: usize) -> Result<Vec<(EngramId, f32)>> {
        // Search index
        self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?.search(embedding, k)
    }
    
    /// Search for similar engrams to an existing engram
    pub fn search_similar_to(&self, engram_id: &EngramId, k: usize) -> Result<Vec<(EngramId, f32)>> {
        // Get the existing embedding from the index
        let embedding = self.get_embedding_for_engram(engram_id)?;
        
        // Search index
        self.search_by_embedding(&embedding, k)
    }
    
    /// Get the embedding service
    pub fn get_embedding_service(&self) -> Arc<EmbeddingService> {
        self.embedding_service.clone()
    }
    
    /// Get the number of vectors in the index
    pub fn len(&self) -> Result<usize> {
        Ok(self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?.len())
    }
    
    /// Check if the index is empty
    pub fn is_empty(&self) -> Result<bool> {
        Ok(self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?.is_empty())
    }
    
    /// Clear the index
    pub fn clear(&self) -> Result<()> {
        self.index.write().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire write lock on vector index".to_string())
        })?.clear();
        
        self.id_to_index.write().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire write lock on id mapping".to_string())
        })?.clear();
        
        Ok(())
    }
    
    /// Helper: Try to extract an embedding from engram metadata
    fn get_embedding_from_metadata(&self, engram: &Engram) -> Option<Embedding> {
        // Check if we have an embedding vector in the metadata
        if let Some(serde_json::Value::Array(vector)) = engram.metadata.get("embedding_vector") {
            // Convert the array to a Vec<f32>
            let mut embedding_vector = Vec::with_capacity(vector.len());
            for value in vector {
                if let serde_json::Value::Number(num) = value {
                    if let Some(float) = num.as_f64() {
                        embedding_vector.push(float as f32);
                    } else {
                        return None;
                    }
                } else {
                    return None;
                }
            }
            
            // Get the model name
            let model = engram.metadata.get("embedding_model")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown")
                .to_string();
            
            // Create the embedding
            return Some(Embedding::new(embedding_vector, model));
        }
        
        None
    }
    
    /// Helper: Get the embedding for an engram by ID
    pub fn get_embedding_for_engram(&self, engram_id: &EngramId) -> Result<Embedding> {
        // Acquire read lock on the index
        let index = self.index.read().map_err(|_| {
            EngramError::ConcurrencyError("Failed to acquire read lock on vector index".to_string())
        })?;
        
        // Use the new get_embedding method from HnswIndex
        if let Some(embedding) = index.get_embedding(engram_id) {
            return Ok(embedding);
        }
        
        // If we get here, the engram wasn't found in the index
        Err(EngramError::NotFound(format!("Embedding for engram {} not found in index", engram_id)))
    }
}

/// Query for vector similarity search
pub struct VectorQuery {
    /// The text query to embed
    pub text: Option<String>,
    
    /// The pre-computed embedding to search with
    pub embedding: Option<Embedding>,
    
    /// The engram ID to find similar engrams to
    pub similar_to_id: Option<EngramId>,
    
    /// The number of results to return
    pub limit: usize,
    
    /// The minimum similarity score (0.0 to 1.0)
    pub min_score: Option<f32>,
    
    /// Whether to exclude the query engram from results
    pub exclude_self: bool,
}

impl VectorQuery {
    /// Create a new vector query with the specified text
    pub fn new(text: &str) -> Self {
        Self {
            text: Some(text.to_string()),
            embedding: None,
            similar_to_id: None,
            limit: 10,
            min_score: None,
            exclude_self: true,
        }
    }
    
    /// Create a new query with a pre-computed embedding
    pub fn with_embedding(embedding: Embedding) -> Self {
        Self {
            text: None,
            embedding: Some(embedding),
            similar_to_id: None,
            limit: 10,
            min_score: None,
            exclude_self: true,
        }
    }
    
    /// Create a query to find similar engrams to an existing engram
    pub fn similar_to(engram_id: EngramId) -> Self {
        Self {
            text: None,
            embedding: None,
            similar_to_id: Some(engram_id),
            limit: 10,
            min_score: None,
            exclude_self: true,
        }
    }
    
    /// Set the limit for the number of results
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    /// Set the minimum similarity score
    pub fn with_min_score(mut self, min_score: f32) -> Self {
        self.min_score = Some(min_score);
        self
    }
    
    /// Set whether to exclude the query engram from results
    pub fn exclude_self(mut self, exclude: bool) -> Self {
        self.exclude_self = exclude;
        self
    }
}

/// Combined search parameters for hybrid retrieval
pub struct HybridQuery {
    /// Optional text for keyword search
    pub text: Option<String>,
    
    /// Optional vector query for semantic search
    pub vector_query: Option<VectorQuery>,
    
    /// Optional metadata filters
    pub metadata_filters: HashMap<String, String>,
    
    /// Optional source filter
    pub source: Option<String>,
    
    /// Optional minimum confidence
    pub min_confidence: Option<f64>,
    
    /// The maximum number of results to return
    pub limit: usize,
    
    /// How to combine results (sum, max, weighted)
    pub combination_method: CombinationMethod,
    
    /// Weights for different score components
    pub weights: HashMap<String, f32>,
}

/// Method for combining scores from different search components
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CombinationMethod {
    /// Sum the scores
    Sum,
    
    /// Take the maximum score
    Max,
    
    /// Use weighted combination
    Weighted,
}

impl HybridQuery {
    /// Create a new hybrid query
    pub fn new() -> Self {
        let mut weights = HashMap::new();
        weights.insert("keyword".to_string(), 1.0);
        weights.insert("vector".to_string(), 1.0);
        weights.insert("metadata".to_string(), 0.5);
        
        Self {
            text: None,
            vector_query: None,
            metadata_filters: HashMap::new(),
            source: None,
            min_confidence: None,
            limit: 10,
            combination_method: CombinationMethod::Weighted,
            weights,
        }
    }
    
    /// Add text for both keyword and vector search
    pub fn with_text(mut self, text: &str) -> Self {
        self.text = Some(text.to_string());
        
        // Also use for vector search if no vector query exists
        if self.vector_query.is_none() {
            self.vector_query = Some(VectorQuery::new(text));
        }
        
        self
    }
    
    /// Add a vector query
    pub fn with_vector_query(mut self, query: VectorQuery) -> Self {
        self.vector_query = Some(query);
        self
    }
    
    /// Add a metadata filter
    pub fn with_metadata_filter(mut self, key: &str, value: &str) -> Self {
        self.metadata_filters.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Add a source filter
    pub fn with_source(mut self, source: &str) -> Self {
        self.source = Some(source.to_string());
        self
    }
    
    /// Add a minimum confidence filter
    pub fn with_min_confidence(mut self, confidence: f64) -> Self {
        self.min_confidence = Some(confidence);
        self
    }
    
    /// Set the limit for the number of results
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    /// Set the combination method
    pub fn with_combination_method(mut self, method: CombinationMethod) -> Self {
        self.combination_method = method;
        self
    }
    
    /// Set a weight for a specific component
    pub fn with_weight(mut self, component: &str, weight: f32) -> Self {
        self.weights.insert(component.to_string(), weight);
        self
    }
}

/// A result from a hybrid search query
#[derive(Debug)]
pub struct HybridSearchResult {
    /// The matching engram
    pub engram: Engram,
    
    /// The overall score
    pub score: f32,
    
    /// Component scores
    pub component_scores: HashMap<String, f32>,
}

/// Engine for hybrid search (combining keyword, vector, and metadata search)
pub struct HybridSearchEngine<'a> {
    /// The storage backend
    storage: &'a Storage,
    
    /// The text search index
    text_index: &'a SearchIndex,
    
    /// The vector search index
    vector_index: &'a VectorIndex,
}

impl<'a> HybridSearchEngine<'a> {
    /// Create a new hybrid search engine
    pub fn new(
        storage: &'a Storage,
        text_index: &'a SearchIndex,
        vector_index: &'a VectorIndex,
    ) -> Self {
        Self {
            storage,
            text_index,
            vector_index,
        }
    }
    
    /// Execute a hybrid search query
    pub fn search(&self, query: &HybridQuery) -> Result<Vec<HybridSearchResult>> {
        // Track relevance scores for each engram
        let mut scores: HashMap<EngramId, HashMap<String, f32>> = HashMap::new();
        
        // 1. Text/keyword search if applicable
        if let Some(text) = &query.text {
            let keyword_results = self.text_index.text_index.search(text);
            
            for engram_id in keyword_results {
                let entry = scores.entry(engram_id).or_insert_with(HashMap::new);
                entry.insert("keyword".to_string(), 1.0); // Score of 1.0 for exact matches
            }
        }
        
        // 2. Vector search if applicable
        if let Some(vector_query) = &query.vector_query {
            let vector_results = self.execute_vector_query(vector_query)?;
            
            for (engram_id, similarity) in vector_results {
                let entry = scores.entry(engram_id).or_insert_with(HashMap::new);
                entry.insert("vector".to_string(), similarity);
            }
        }
        
        // 3. Metadata filters if applicable
        if !query.metadata_filters.is_empty() {
            for (key, value) in &query.metadata_filters {
                let metadata_results = self.text_index.metadata_index.find_by_key_value(key, value);
                
                for engram_id in metadata_results {
                    if scores.contains_key(&engram_id) {
                        let entry = scores.get_mut(&engram_id).unwrap();
                        entry.insert("metadata".to_string(), 1.0); // Score of 1.0 for exact metadata matches
                    }
                }
            }
            
            // Remove engrams that don't match metadata filters
            scores.retain(|_, component_scores| component_scores.contains_key("metadata"));
        }
        
        // 4. Source filter if applicable
        if let Some(source) = &query.source {
            let source_results = self.text_index.find_by_source(source);
            
            // Remove engrams that don't match the source
            scores.retain(|engram_id, _| source_results.contains(engram_id));
        }
        
        // 5. Confidence filter if applicable
        if let Some(min_confidence) = query.min_confidence {
            let confidence_results = self.text_index.find_by_min_confidence(min_confidence);
            
            // Remove engrams that don't meet the confidence threshold
            scores.retain(|engram_id, _| confidence_results.contains(engram_id));
        }
        
        // 6. Combine scores
        let mut final_results = Vec::new();
        
        for (engram_id, component_scores) in scores {
            // Get the engram
            if let Some(engram) = self.storage.get_engram(&engram_id)? {
                // Calculate combined score
                let score = self.calculate_combined_score(&component_scores, &query.weights, query.combination_method);
                
                final_results.push(HybridSearchResult {
                    engram,
                    score,
                    component_scores,
                });
            }
        }
        
        // Sort by score (highest first)
        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        
        // Apply limit
        if final_results.len() > query.limit {
            final_results.truncate(query.limit);
        }
        
        Ok(final_results)
    }
    
    /// Execute a vector query
    fn execute_vector_query(&self, query: &VectorQuery) -> Result<Vec<(EngramId, f32)>> {
        if let Some(text) = &query.text {
            // Search by text
            let results = self.vector_index.search(text, query.limit)?;
            
            // Apply minimum score filter if needed
            if let Some(min_score) = query.min_score {
                Ok(results.into_iter().filter(|(_, score)| *score >= min_score).collect())
            } else {
                Ok(results)
            }
        } else if let Some(embedding) = &query.embedding {
            // Search by embedding
            let results = self.vector_index.search_by_embedding(embedding, query.limit)?;
            
            // Apply minimum score filter if needed
            if let Some(min_score) = query.min_score {
                Ok(results.into_iter().filter(|(_, score)| *score >= min_score).collect())
            } else {
                Ok(results)
            }
        } else if let Some(engram_id) = &query.similar_to_id {
            // Search for similar engrams
            let results = self.vector_index.search_similar_to(engram_id, 
                if query.exclude_self { query.limit + 1 } else { query.limit }
            )?;
            
            // Filter out the query engram if needed
            let filtered: Vec<(EngramId, f32)> = if query.exclude_self {
                results.into_iter()
                    .filter(|(id, _)| id != engram_id)
                    .take(query.limit)
                    .collect()
            } else {
                results
            };
            
            // Apply minimum score filter if needed
            if let Some(min_score) = query.min_score {
                Ok(filtered.into_iter().filter(|(_, score)| *score >= min_score).collect())
            } else {
                Ok(filtered)
            }
        } else {
            Err(EngramError::InvalidOperation("Vector query must include text, embedding, or similar_to_id".to_string()))
        }
    }
    
    /// Calculate a combined score from component scores
    fn calculate_combined_score(
        &self,
        component_scores: &HashMap<String, f32>,
        weights: &HashMap<String, f32>,
        method: CombinationMethod,
    ) -> f32 {
        match method {
            CombinationMethod::Sum => {
                // Simple sum of all scores
                component_scores.values().sum()
            }
            CombinationMethod::Max => {
                // Maximum score across all components
                component_scores.values().cloned().fold(0.0, f32::max)
            }
            CombinationMethod::Weighted => {
                // Weighted sum of scores
                let mut weighted_sum = 0.0;
                let mut weight_sum = 0.0;
                
                for (component, score) in component_scores {
                    if let Some(weight) = weights.get(component) {
                        weighted_sum += score * weight;
                        weight_sum += weight;
                    }
                }
                
                if weight_sum > 0.0 {
                    weighted_sum / weight_sum
                } else {
                    0.0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::embedding::EmbeddingService;
    use std::sync::Arc;
    
    #[test]
    fn test_vector_index() {
        let embedding_service = Arc::new(EmbeddingService::new());
        let vector_index = VectorIndex::with_embedding_service(embedding_service);
        
        // Create test engrams with explicit content related to climate
        let engram1 = Engram::new(
            "Climate change is accelerating faster than predicted.".to_string(),
            "research".to_string(),
            0.9,
            None,
        );
        
        // Create test engram with explicitly different content
        let engram2 = Engram::new(
            "Solar panels are becoming more affordable and efficient.".to_string(),
            "observation".to_string(),
            0.8,
            None,
        );
        
        // Add to index
        vector_index.add_engram(&engram1).unwrap();
        vector_index.add_engram(&engram2).unwrap();
        
        // Search for similar engrams with very specific query
        let results = vector_index.search("climate warming global", 2).unwrap();
        
        // Check we got the expected number of results
        assert_eq!(results.len(), 2);
        
        // Results should be sorted by similarity
        assert!(results[0].1 >= results[1].1);
        
        // At least one of the engrams should be about climate
        let has_climate_engram = results.iter().any(|(id, _)| *id == engram1.id);
        assert!(has_climate_engram, "Search results should include the climate change engram");
        
        // Test get_embedding_for_engram
        let embedding = vector_index.get_embedding_for_engram(&engram1.id).unwrap();
        assert_eq!(embedding.dimensions, vector_index.dimensions);
    }
    
    #[test]
    fn test_hybrid_query_builder() {
        let query = HybridQuery::new()
            .with_text("climate change")
            .with_metadata_filter("category", "environment")
            .with_source("research")
            .with_min_confidence(0.7)
            .with_limit(5)
            .with_combination_method(CombinationMethod::Weighted)
            .with_weight("vector", 2.0);
        
        assert_eq!(query.text, Some("climate change".to_string()));
        assert_eq!(query.metadata_filters.get("category"), Some(&"environment".to_string()));
        assert_eq!(query.source, Some("research".to_string()));
        assert_eq!(query.min_confidence, Some(0.7));
        assert_eq!(query.limit, 5);
        assert_eq!(query.combination_method, CombinationMethod::Weighted);
        assert_eq!(query.weights.get("vector"), Some(&2.0));
    }
}