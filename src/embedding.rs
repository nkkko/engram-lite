use crate::error::{EngramError, Result};
use crate::dimension_reduction::{DimensionReducer, ReductionMethod};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// Represents a vector embedding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embedding {
    /// The actual vector data
    pub vector: Vec<f32>,
    
    /// The model used to generate this embedding
    pub model: String,
    
    /// The dimensionality of the vector
    pub dimensions: usize,
    
    /// Optional metadata about the embedding
    pub metadata: HashMap<String, String>,
}

// Add conversion methods for storage::Embedding
impl From<crate::storage::Embedding> for Embedding {
    fn from(e: crate::storage::Embedding) -> Self {
        Embedding {
            vector: e.vector,
            model: e.model,
            dimensions: e.dimensions,
            metadata: e.metadata,
        }
    }
}

impl From<&crate::storage::Embedding> for Embedding {
    fn from(e: &crate::storage::Embedding) -> Self {
        Embedding {
            vector: e.vector.clone(),
            model: e.model.clone(),
            dimensions: e.dimensions,
            metadata: e.metadata.clone(),
        }
    }
}

impl Embedding {
    /// Create a new embedding from a vector and model
    pub fn new(vector: Vec<f32>, model: String) -> Self {
        let dimensions = vector.len();
        Self {
            vector,
            model,
            dimensions,
            metadata: HashMap::new(),
        }
    }

    /// Create a new embedding with metadata
    pub fn with_metadata(vector: Vec<f32>, model: String, metadata: HashMap<String, String>) -> Self {
        let dimensions = vector.len();
        Self {
            vector,
            model,
            dimensions,
            metadata,
        }
    }

    /// Calculate cosine similarity between this embedding and another
    pub fn cosine_similarity(&self, other: &Embedding) -> Result<f32> {
        if self.dimensions != other.dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Cannot compare embeddings with different dimensions: {} vs {}",
                self.dimensions, other.dimensions
            )));
        }

        let mut dot_product = 0.0;
        let mut magnitude_a = 0.0;
        let mut magnitude_b = 0.0;

        for i in 0..self.dimensions {
            dot_product += self.vector[i] * other.vector[i];
            magnitude_a += self.vector[i] * self.vector[i];
            magnitude_b += other.vector[i] * other.vector[i];
        }

        magnitude_a = magnitude_a.sqrt();
        magnitude_b = magnitude_b.sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return Ok(0.0);
        }

        Ok(dot_product / (magnitude_a * magnitude_b))
    }

    /// Calculate Euclidean distance between this embedding and another
    pub fn euclidean_distance(&self, other: &Embedding) -> Result<f32> {
        if self.dimensions != other.dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Cannot compare embeddings with different dimensions: {} vs {}",
                self.dimensions, other.dimensions
            )));
        }

        let mut sum_squared_diff = 0.0;
        for i in 0..self.dimensions {
            let diff = self.vector[i] - other.vector[i];
            sum_squared_diff += diff * diff;
        }

        Ok(sum_squared_diff.sqrt())
    }

    /// Normalize the embedding vector in-place (L2 normalization)
    pub fn normalize(&mut self) {
        let mut magnitude = 0.0;
        for val in &self.vector {
            magnitude += val * val;
        }
        magnitude = magnitude.sqrt();

        if magnitude > 0.0 {
            for val in &mut self.vector {
                *val /= magnitude;
            }
        }
    }

    /// Create a normalized copy of this embedding
    pub fn normalized(&self) -> Self {
        let mut normalized = self.clone();
        normalized.normalize();
        normalized
    }
}

/// Available embedding models
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum EmbeddingModel {
    /// E5 Multilingual Large Instruct (Default) - 1024 dimensions
    E5MultilingualLargeInstruct,
    
    /// GTE Modern BERT Base - 768 dimensions
    GteModernBertBase,
    
    /// Jina Embeddings V3 - 768 dimensions
    JinaEmbeddingsV3,
    
    /// Custom model specified by name
    Custom,
}

impl EmbeddingModel {
    /// Get the name of the model
    pub fn model_name(&self) -> String {
        match self {
            Self::E5MultilingualLargeInstruct => "intfloat/multilingual-e5-large-instruct".to_string(),
            Self::GteModernBertBase => "Alibaba-NLP/gte-modernbert-base".to_string(),
            Self::JinaEmbeddingsV3 => "jinaai/jina-embeddings-v3".to_string(),
            Self::Custom => "custom".to_string(),
        }
    }
    
    /// Get the dimensions of the model
    pub fn dimensions(&self) -> usize {
        match self {
            Self::E5MultilingualLargeInstruct => 1024,
            Self::GteModernBertBase => 768,
            Self::JinaEmbeddingsV3 => 768,
            Self::Custom => 0, // Custom models require explicit dimension setting
        }
    }
}

/// Simple LRU cache for embeddings
pub struct EmbeddingCache {
    /// The maximum number of entries to keep in the cache
    capacity: usize,
    
    /// The cache entries (text key -> embedding)
    entries: HashMap<String, (Embedding, u64)>,
    
    /// The access count for timing out entries
    access_count: u64,
}

impl EmbeddingCache {
    /// Create a new cache with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: HashMap::with_capacity(capacity),
            access_count: 0,
        }
    }
    
    /// Add an embedding to the cache
    pub fn insert(&mut self, key: String, embedding: Embedding) {
        // If we're at capacity, remove the least recently used entry
        if self.entries.len() >= self.capacity {
            let to_remove = self.entries
                .iter()
                .min_by_key(|(_, (_, count))| *count)
                .map(|(k, _)| k.clone());
            
            if let Some(key) = to_remove {
                self.entries.remove(&key);
            }
        }
        
        // Insert the new entry
        self.access_count += 1;
        self.entries.insert(key, (embedding, self.access_count));
    }
    
    /// Get an embedding from the cache
    pub fn get(&mut self, key: &str) -> Option<Embedding> {
        if let Some((embedding, count)) = self.entries.get_mut(key) {
            // Update the access count
            self.access_count += 1;
            *count = self.access_count;
            
            // Return a clone of the embedding
            Some(embedding.clone())
        } else {
            None
        }
    }
    
    /// Clear the cache
    pub fn clear(&mut self) {
        self.entries.clear();
        self.access_count = 0;
    }
    
    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// Service for generating embeddings
pub struct EmbeddingService {
    /// The model to use for generating embeddings
    model: EmbeddingModel,
    
    /// The name of the custom model (if using a custom model)
    custom_model_name: Option<String>,
    
    /// The dimensions of the custom model (if using a custom model)
    custom_dimensions: Option<usize>,
    
    /// Cache for previously generated embeddings
    cache: Arc<Mutex<EmbeddingCache>>,
    
    /// Whether to use an instruction prefix for e5 models
    use_instruction_prefix: bool,
    
    /// Whether to normalize embeddings after generation
    normalize_embeddings: bool,
    
    /// Dimensionality reducer for creating smaller embeddings
    pub dimension_reducer: Option<Arc<Mutex<DimensionReducer>>>,
    
    /// Whether to use reduced embeddings instead of full embeddings
    use_reduced_embeddings: bool,
}

/// Builder for EmbeddingService
pub struct EmbeddingServiceBuilder {
    model: EmbeddingModel,
    custom_model_name: Option<String>,
    custom_dimensions: Option<usize>,
    use_instruction_prefix: bool,
    normalize_embeddings: bool,
    use_reduced_embeddings: bool,
}

impl EmbeddingServiceBuilder {
    /// Create a new builder with default values
    pub fn new() -> Self {
        Self {
            model: EmbeddingModel::E5MultilingualLargeInstruct,
            custom_model_name: None,
            custom_dimensions: None,
            use_instruction_prefix: true,
            normalize_embeddings: true,
            use_reduced_embeddings: false,
        }
    }
    
    /// Set the model type
    pub fn model_type(mut self, model: EmbeddingModel) -> Self {
        self.model = model;
        if model == EmbeddingModel::E5MultilingualLargeInstruct {
            self.use_instruction_prefix = true;
        }
        self
    }
    
    /// Set the normalize flag
    pub fn normalize(mut self, normalize: bool) -> Self {
        self.normalize_embeddings = normalize;
        self
    }
    
    /// Set the custom model name
    pub fn custom_model(mut self, name: String) -> Self {
        self.model = EmbeddingModel::Custom;
        self.custom_model_name = Some(name);
        self
    }
    
    /// Set the instruction prefix flag
    pub fn use_instruction_prefix(mut self, use_prefix: bool) -> Self {
        self.use_instruction_prefix = use_prefix;
        self
    }
    
    /// Set the dimensions
    pub fn dimensions(mut self, dimensions: usize) -> Self {
        self.custom_dimensions = Some(dimensions);
        self
    }
    
    /// Set the use reduced embeddings flag
    pub fn use_reduced_embeddings(mut self, use_reduced: bool) -> Self {
        self.use_reduced_embeddings = use_reduced;
        self
    }
    
    /// Build the EmbeddingService
    pub fn build(self) -> EmbeddingService {
        EmbeddingService {
            model: self.model,
            custom_model_name: self.custom_model_name,
            custom_dimensions: self.custom_dimensions,
            cache: Arc::new(Mutex::new(EmbeddingCache::new(1000))),
            use_instruction_prefix: self.use_instruction_prefix,
            normalize_embeddings: self.normalize_embeddings,
            dimension_reducer: None,
            use_reduced_embeddings: self.use_reduced_embeddings,
        }
    }
}

impl EmbeddingService {
    /// Create a builder for EmbeddingService
    pub fn builder() -> EmbeddingServiceBuilder {
        EmbeddingServiceBuilder::new()
    }
    
    /// Get the current model type
    pub fn get_model_type(&self) -> EmbeddingModel {
        self.model
    }
    
    /// Create a new embedding service with the default model (E5 Multilingual Large Instruct)
    pub fn new() -> Self {
        Self {
            model: EmbeddingModel::E5MultilingualLargeInstruct,
            custom_model_name: None,
            custom_dimensions: None,
            cache: Arc::new(Mutex::new(EmbeddingCache::new(1000))),
            use_instruction_prefix: true,
            normalize_embeddings: true,
            dimension_reducer: None,
            use_reduced_embeddings: false,
        }
    }
    
    /// Create a new embedding service with a specific model type
    pub fn with_model_type(model: EmbeddingModel) -> Self {
        Self {
            model,
            custom_model_name: None,
            custom_dimensions: None,
            cache: Arc::new(Mutex::new(EmbeddingCache::new(1000))),
            use_instruction_prefix: model == EmbeddingModel::E5MultilingualLargeInstruct,
            normalize_embeddings: true,
            dimension_reducer: None,
            use_reduced_embeddings: false,
        }
    }
    
    /// Create a new embedding service with a custom model
    pub fn new_with_model(model_name: Option<&str>) -> Self {
        match model_name {
            Some("intfloat/multilingual-e5-large-instruct") => Self::with_model_type(EmbeddingModel::E5MultilingualLargeInstruct),
            Some("Alibaba-NLP/gte-modernbert-base") => Self::with_model_type(EmbeddingModel::GteModernBertBase),
            Some("jinaai/jina-embeddings-v3") => Self::with_model_type(EmbeddingModel::JinaEmbeddingsV3),
            Some(name) => Self {
                model: EmbeddingModel::Custom,
                custom_model_name: Some(name.to_string()),
                custom_dimensions: None,
                cache: Arc::new(Mutex::new(EmbeddingCache::new(1000))),
                use_instruction_prefix: name.contains("e5") || name.contains("instruct"),
                normalize_embeddings: true,
                dimension_reducer: None,
                use_reduced_embeddings: false,
            },
            None => Self::with_model_type(EmbeddingModel::E5MultilingualLargeInstruct),
        }
    }
    
    /// Set the custom dimensions for a custom model
    pub fn with_dimensions(mut self, dimensions: usize) -> Self {
        self.custom_dimensions = Some(dimensions);
        self
    }
    
    /// Set the cache size
    pub fn with_cache_size(mut self, cache_size: usize) -> Self {
        self.cache = Arc::new(Mutex::new(EmbeddingCache::new(cache_size)));
        self
    }
    
    /// Set whether to use instruction prefix for e5 models
    pub fn with_instruction_prefix(mut self, use_prefix: bool) -> Self {
        self.use_instruction_prefix = use_prefix;
        self
    }
    
    /// Set whether to normalize embeddings after generation
    pub fn with_normalization(mut self, normalize: bool) -> Self {
        self.normalize_embeddings = normalize;
        self
    }
    
    /// Configure dimensionality reduction
    pub fn with_dimension_reduction(mut self, method: ReductionMethod, target_dimensions: usize) -> Self {
        let reducer = DimensionReducer::new(method, target_dimensions);
        self.dimension_reducer = Some(Arc::new(Mutex::new(reducer)));
        self
    }
    
    /// Set whether to use reduced embeddings by default
    pub fn use_reduced_embeddings(&mut self, use_reduced: bool) -> &mut Self {
        self.use_reduced_embeddings = use_reduced;
        self
    }
    
    /// Train the dimensionality reducer with a set of embeddings
    pub fn train_reducer(&self, embeddings: &[Embedding]) -> Result<()> {
        if let Some(reducer_arc) = &self.dimension_reducer {
            let mut reducer = reducer_arc.lock().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire lock on dimension reducer".to_string())
            })?;
            
            reducer.train(embeddings)?;
            Ok(())
        } else {
            Err(EngramError::InvalidOperation("No dimension reducer configured".to_string()))
        }
    }
    
    /// Reduce the dimensionality of an embedding
    pub fn reduce_embedding(&self, embedding: &Embedding) -> Result<Embedding> {
        if let Some(reducer_arc) = &self.dimension_reducer {
            let reducer = reducer_arc.lock().map_err(|_| {
                EngramError::ConcurrencyError("Failed to acquire lock on dimension reducer".to_string())
            })?;
            
            if !reducer.is_trained() {
                return Err(EngramError::InvalidState("Dimension reducer is not trained".to_string()));
            }
            
            reducer.reduce(embedding)
        } else {
            Err(EngramError::InvalidOperation("No dimension reducer configured".to_string()))
        }
    }
    
    /// Reduce the dimensionality of a storage embedding
    pub fn reduce_storage_embedding(&self, embedding: &crate::storage::Embedding) -> Result<Embedding> {
        // Convert storage::Embedding to embedding::Embedding
        let embedding_converted = Embedding::from(embedding);
        self.reduce_embedding(&embedding_converted)
    }
    
    /// Get the model name
    pub fn get_model_name(&self) -> String {
        match self.model {
            EmbeddingModel::Custom => {
                self.custom_model_name.clone().unwrap_or_else(|| "custom".to_string())
            }
            _ => self.model.model_name(),
        }
    }
    
    /// Get the model dimensions
    pub fn get_dimensions(&self) -> usize {
        match self.model {
            EmbeddingModel::Custom => {
                self.custom_dimensions.unwrap_or(768) // Default to 768 for custom models
            }
            _ => self.model.dimensions(),
        }
    }
    
    /// Create an embedding from text
    pub fn embed_text(&self, text: &str) -> Result<Embedding> {
        // Generate a cache key that includes whether this is a reduced embedding
        let cache_key = if self.use_reduced_embeddings {
            format!("reduced:{}", text)
        } else {
            text.to_string()
        };
        
        // Check if we have this in the cache
        if let Ok(mut cache) = self.cache.lock() {
            if let Some(embedding) = cache.get(&cache_key) {
                return Ok(embedding);
            }
        }
        
        // This is a placeholder for real embedding generation
        // In a full implementation, we would call an embedding model API here
        // For now, we'll generate a random vector with the correct dimensions
        let dimensions = self.get_dimensions();
        let model_name = self.get_model_name();
        
        // Generate a placeholder vector (would be replaced with actual embedding calculation)
        // In practice, this would use a model-specific library or API call
        let mut vector = Vec::with_capacity(dimensions);
        
        // Generate a deterministic embedding based on the hash of the text
        // This is just for demo purposes - real embeddings would come from a model
        let text_hash = text.bytes().fold(0u64, |acc, b| acc.wrapping_add(b as u64));
        let mut value = text_hash;
        
        for _ in 0..dimensions {
            // Generate pseudorandom but deterministic values from the text hash
            value = value.wrapping_mul(6364136223846793005).wrapping_add(1);
            let float_val = (value % 1000) as f32 / 500.0 - 1.0;
            vector.push(float_val);
        }
        
        // Create the embedding
        let mut embedding = Embedding::new(vector, model_name);
        
        // Add metadata about the generation
        let mut metadata = HashMap::new();
        metadata.insert("text_length".to_string(), text.len().to_string());
        metadata.insert("model_type".to_string(), format!("{:?}", self.model));
        embedding.metadata = metadata;
        
        // Normalize if requested
        if self.normalize_embeddings {
            embedding.normalize();
        }
        
        // Apply dimensionality reduction if configured and requested
        let final_embedding = if self.use_reduced_embeddings {
            if let Some(reducer_arc) = &self.dimension_reducer {
                let reducer = reducer_arc.lock().map_err(|_| {
                    EngramError::ConcurrencyError("Failed to acquire lock on dimension reducer".to_string())
                })?;
                
                if reducer.is_trained() {
                    // Reduce the dimensionality
                    reducer.reduce(&embedding)?
                } else {
                    // If reducer exists but isn't trained, return the original embedding
                    embedding
                }
            } else {
                // If no reducer is configured, return the original embedding
                embedding
            }
        } else {
            // If not using reduced embeddings, return the original
            embedding
        };
        
        // Add to cache
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(cache_key, final_embedding.clone());
        }
        
        Ok(final_embedding)
    }
    
    /// Embed multiple texts in a batch
    pub fn embed_batch(&self, texts: &[&str]) -> Result<Vec<Embedding>> {
        // In a real implementation, we would batch the requests to the embedding model
        // For now, just embed each text individually
        let mut embeddings = Vec::with_capacity(texts.len());
        
        for text in texts {
            embeddings.push(self.embed_text(text)?);
        }
        
        Ok(embeddings)
    }
    
    /// Create a batch processing utility for dimensionality reduction
    pub fn batch_reduce_embeddings(&self, storage: &crate::storage::Storage) -> Result<usize> {
        // 1. Get a list of all engram IDs
        let engram_ids = storage.list_engrams()?;
        
        if engram_ids.is_empty() {
            return Ok(0);
        }
        
        // 2. Get all original embeddings
        let mut storage_embeddings = Vec::new();
        let mut id_map = Vec::new();
        
        for id in &engram_ids {
            if let Some(embedding) = storage.get_embedding(id)? {
                storage_embeddings.push(embedding);
                id_map.push(id.clone());
            }
        }
        
        if storage_embeddings.is_empty() {
            return Ok(0);
        }
        
        // 3. Make sure we have a dimension reducer and it's trained
        if self.dimension_reducer.is_none() {
            return Err(EngramError::InvalidOperation("No dimension reducer configured".to_string()));
        }
        
        // Convert storage embeddings to embedding::Embedding for training
        let mut embeddings = Vec::new();
        for storage_embedding in &storage_embeddings {
            embeddings.push(Embedding::from(storage_embedding));
        }
        
        // Train the reducer with these embeddings if needed
        self.train_reducer(&embeddings)?;
        
        // 4. Reduce all embeddings
        let mut reduced_count = 0;
        
        for (i, storage_embedding) in storage_embeddings.iter().enumerate() {
            // Convert to embedding::Embedding and reduce
            let embedding = Embedding::from(storage_embedding);
            
            // Reduce dimensionality
            if let Ok(reduced) = self.reduce_embedding(&embedding) {
                // Convert back to storage::Embedding and store
                let storage_reduced = crate::storage::Embedding::from(&reduced);
                storage.put_reduced_embedding(&id_map[i], &storage_reduced)?;
                reduced_count += 1;
            }
        }
        
        Ok(reduced_count)
    }
    
    /// Load a model from a file path (for local models)
    pub fn load_model_from_path(&self, _path: &Path) -> Result<()> {
        // This would load a local model from a file
        // For now, just a placeholder
        Err(EngramError::NotImplemented("Loading local models not yet implemented".to_string()))
    }
}

impl Default for EmbeddingService {
    fn default() -> Self {
        Self::new()
    }
}

/// HNSW Vector index for efficient similarity search
/// Based on the paper "Efficient and robust approximate nearest neighbor search using Hierarchical Navigable Small World graphs"
pub struct HnswIndex {
    /// The maximum number of connections per node
    #[allow(dead_code)]
    m: usize,
    
    /// The number of connections for newly added nodes
    #[allow(dead_code)]
    ef_construction: usize,
    
    /// The number of nearest neighbors to return per query
    #[allow(dead_code)]
    ef_search: usize,
    
    /// The dimensionality of vectors in the index
    dimensions: usize,
    
    /// The actual index data structure - this would typically be a graph structure
    /// For this implementation, we're using a placeholder structure
    data: Vec<(String, Embedding)>,
}

impl HnswIndex {
    /// Create a new HNSW index with default parameters
    pub fn new(dimensions: usize) -> Self {
        Self {
            m: 16,                // Default M parameter (max connections per node)
            ef_construction: 200, // Default ef construction (search width during construction)
            ef_search: 50,        // Default ef search (search width during query)
            dimensions,
            data: Vec::new(),
        }
    }
    
    /// Create a new HNSW index with custom parameters
    pub fn with_params(dimensions: usize, m: usize, ef_construction: usize, ef_search: usize) -> Self {
        Self {
            m,
            ef_construction,
            ef_search,
            dimensions,
            data: Vec::new(),
        }
    }
    
    /// Add a vector to the index
    pub fn add(&mut self, id: &str, embedding: Embedding) -> Result<()> {
        // Validate dimensions
        if embedding.dimensions != self.dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Embedding dimensions ({}) don't match index dimensions ({})",
                embedding.dimensions, self.dimensions
            )));
        }
        
        // In a real implementation, this would add to the HNSW graph structure
        // For now, just add to our placeholder data structure
        self.data.push((id.to_string(), embedding));
        
        Ok(())
    }
    
    /// Search for the nearest neighbors to a query vector
    pub fn search(&self, query: &Embedding, k: usize) -> Result<Vec<(String, f32)>> {
        // Validate dimensions
        if query.dimensions != self.dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Query dimensions ({}) don't match index dimensions ({})",
                query.dimensions, self.dimensions
            )));
        }
        
        // In a real implementation, this would use the HNSW graph for efficient search
        // For now, just do a linear search through our placeholder data
        let mut results = Vec::new();
        
        for (id, embedding) in &self.data {
            let similarity = query.cosine_similarity(embedding)?;
            results.push((id.clone(), similarity));
        }
        
        // Sort by similarity (highest first)
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        // Truncate to k results
        if results.len() > k {
            results.truncate(k);
        }
        
        Ok(results)
    }
    
    /// Remove a vector from the index
    pub fn remove(&mut self, id: &str) -> Result<bool> {
        let len_before = self.data.len();
        self.data.retain(|(vec_id, _)| vec_id != id);
        
        Ok(self.data.len() < len_before)
    }
    
    /// Get the number of vectors in the index
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if the index is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// Clear the index
    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_embedding_creation() {
        let vector = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let model = "test-model".to_string();
        
        let embedding = Embedding::new(vector.clone(), model.clone());
        
        assert_eq!(embedding.vector, vector);
        assert_eq!(embedding.model, model);
        assert_eq!(embedding.dimensions, 5);
    }
    
    #[test]
    fn test_embedding_similarity() {
        let vec1 = vec![1.0, 0.0, 0.0, 0.0];
        let vec2 = vec![0.0, 1.0, 0.0, 0.0];
        let vec3 = vec![1.0, 1.0, 0.0, 0.0];
        
        let emb1 = Embedding::new(vec1, "test".to_string());
        let emb2 = Embedding::new(vec2, "test".to_string());
        let emb3 = Embedding::new(vec3, "test".to_string());
        
        // Orthogonal vectors should have similarity 0
        assert_eq!(emb1.cosine_similarity(&emb2).unwrap(), 0.0);
        
        // Same vector should have similarity 1
        assert_eq!(emb1.cosine_similarity(&emb1).unwrap(), 1.0);
        
        // For vec1 and vec3, similarity should be 1/sqrt(2)
        let sim = emb1.cosine_similarity(&emb3).unwrap();
        assert!((sim - 1.0/2.0_f32.sqrt()).abs() < 1e-6);
    }
    
    #[test]
    fn test_embedding_normalization() {
        let vec = vec![3.0, 4.0];
        let mut emb = Embedding::new(vec, "test".to_string());
        
        // Before normalization, magnitude should be 5
        let magnitude: f32 = emb.vector.iter().map(|v| v*v).sum::<f32>().sqrt();
        assert!((magnitude - 5.0).abs() < 1e-6);
        
        // After normalization, magnitude should be 1
        emb.normalize();
        let magnitude: f32 = emb.vector.iter().map(|v| v*v).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 1e-6);
    }
    
    #[test]
    fn test_embedding_service() {
        let service = EmbeddingService::new();
        
        let text1 = "This is a test";
        let text2 = "This is another test";
        
        let emb1 = service.embed_text(text1).unwrap();
        let emb2 = service.embed_text(text2).unwrap();
        
        // Check dimensions
        assert_eq!(emb1.dimensions, service.get_dimensions());
        
        // Check embeddings are different
        assert!(emb1.cosine_similarity(&emb2).unwrap() < 1.0);
        
        // Check normalization
        let magnitude: f32 = emb1.vector.iter().map(|v| v*v).sum::<f32>().sqrt();
        assert!((magnitude - 1.0).abs() < 1e-5);
    }
    
    #[test]
    fn test_embedding_cache() {
        let mut cache = EmbeddingCache::new(2);
        
        let emb1 = Embedding::new(vec![1.0, 0.0], "test".to_string());
        let emb2 = Embedding::new(vec![0.0, 1.0], "test".to_string());
        let emb3 = Embedding::new(vec![1.0, 1.0], "test".to_string());
        
        // Add two embeddings
        cache.insert("one".to_string(), emb1.clone());
        cache.insert("two".to_string(), emb2.clone());
        
        // Check cache size
        assert_eq!(cache.len(), 2);
        
        // Retrieve an embedding
        let retrieved = cache.get("one").unwrap();
        assert_eq!(retrieved.vector, emb1.vector);
        
        // Add a third embedding, which should evict the least recently used
        cache.insert("three".to_string(), emb3);
        
        // "one" was retrieved more recently than "two", so "two" should be evicted
        assert!(cache.get("one").is_some());
        assert!(cache.get("two").is_none());
        assert!(cache.get("three").is_some());
    }
    
    #[test]
    fn test_hnsw_index() {
        let mut index = HnswIndex::new(3);
        
        let emb1 = Embedding::new(vec![1.0, 0.0, 0.0], "test".to_string());
        let emb2 = Embedding::new(vec![0.0, 1.0, 0.0], "test".to_string());
        let emb3 = Embedding::new(vec![0.0, 0.0, 1.0], "test".to_string());
        
        // Add embeddings to index
        index.add("one", emb1.clone()).unwrap();
        index.add("two", emb2.clone()).unwrap();
        index.add("three", emb3.clone()).unwrap();
        
        // Search for nearest neighbors
        let results = index.search(&emb1, 2).unwrap();
        
        // First result should be "one" itself with similarity 1.0
        assert_eq!(results[0].0, "one");
        assert!((results[0].1 - 1.0).abs() < 1e-6);
        
        // Remove an item
        index.remove("two").unwrap();
        
        // Check it's been removed
        let results = index.search(&emb2, 3).unwrap();
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|(id, _)| id != "two"));
    }
}