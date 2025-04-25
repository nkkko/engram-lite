use crate::embedding::Embedding;
use crate::error::{EngramError, Result};
use linfa::prelude::*;
use linfa_reduction::Pca;
use linfa_reduction::random_projection::GaussianRandomProjection;
use ndarray::{Array2, s};
use std::fmt;
use serde::{Deserialize, Serialize};

/// Dimensionality reduction strategies for embedding vectors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReductionMethod {
    /// Principal Component Analysis - linear reduction preserving maximum variance
    PCA,
    /// Random projection - faster approximation using random matrices
    RandomProjection,
    /// Truncation - simply keep the first N dimensions (simplest method)
    Truncation,
}

impl fmt::Display for ReductionMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReductionMethod::PCA => write!(f, "PCA"),
            ReductionMethod::RandomProjection => write!(f, "RandomProjection"),
            ReductionMethod::Truncation => write!(f, "Truncation"),
        }
    }
}

/// Manages dimensionality reduction for embeddings
pub struct DimensionReducer {
    /// The method used for dimensionality reduction
    method: ReductionMethod,
    /// The target dimensionality for reduced embeddings
    target_dimensions: usize,
    /// Whether the reducer has been trained on data
    trained: bool,
    /// Original dimensions of the input embeddings
    original_dimensions: usize,
    /// PCA state if using PCA method
    pca_model: Option<Pca<f64>>,
    /// Random projection model if using RandomProjection method
    rp_model: Option<GaussianRandomProjection<f64>>,
}

impl DimensionReducer {
    /// Create a new dimensionality reducer with the specified method and target dimensions
    pub fn new(method: ReductionMethod, target_dimensions: usize) -> Self {
        Self {
            method,
            target_dimensions,
            trained: false,
            original_dimensions: 0,
            pca_model: None,
            rp_model: None,
        }
    }
    
    /// Train the reducer on a set of embeddings
    pub fn train(&mut self, embeddings: &[Embedding]) -> Result<()> {
        if embeddings.is_empty() {
            return Err(EngramError::InvalidOperation("Cannot train on empty embedding set".to_string()));
        }
        
        // Get dimensions from the first embedding
        let dims = embeddings[0].dimensions;
        
        // Ensure all embeddings have the same dimensions
        for emb in embeddings.iter().skip(1) {
            if emb.dimensions != dims {
                return Err(EngramError::InvalidOperation(format!(
                    "All embeddings must have same dimensions: found {} vs {}", 
                    emb.dimensions, dims
                )));
            }
        }
        
        // Check if target dimensions make sense
        if self.target_dimensions >= dims {
            return Err(EngramError::InvalidOperation(format!(
                "Target dimensions ({}) must be less than original dimensions ({})",
                self.target_dimensions, dims
            )));
        }
        
        self.original_dimensions = dims;
        
        // Create a matrix from the embeddings (convert f32 to f64 for linfa)
        let mut data = Array2::zeros((embeddings.len(), dims));
        for (i, emb) in embeddings.iter().enumerate() {
            for (j, &val) in emb.vector.iter().enumerate() {
                data[[i, j]] = val as f64;
            }
        }
        
        // Apply the selected reduction method
        match self.method {
            ReductionMethod::PCA => {
                self.train_pca(data)?;
            },
            ReductionMethod::RandomProjection => {
                self.train_random_projection(data)?;
            },
            ReductionMethod::Truncation => {
                // No training needed for truncation
                self.trained = true;
            }
        }
        
        Ok(())
    }
    
    /// Train PCA model
    fn train_pca(&mut self, data: Array2<f64>) -> Result<()> {
        // Create dataset with records as samples 
        let dataset = DatasetBase::from(data);
        
        // Initialize PCA with correct parameters for version 0.7.1
        let pca = Pca::params(self.target_dimensions)
            .fit(&dataset)
            .map_err(|e| EngramError::ComputationError(format!("PCA fitting error: {}", e)))?;
        
        self.pca_model = Some(pca);
        self.trained = true;
        Ok(())
    }
    
    /// Train Random Projection model
    fn train_random_projection(&mut self, data: Array2<f64>) -> Result<()> {
        // Create dataset with records as samples
        let dataset = DatasetBase::from(data);
        
        // Initialize RandomProjection with target_dim
        // Note: We don't need a specific RNG for GaussianRandomProjection
        let rp = GaussianRandomProjection::<f64>::params()
            .target_dim(self.target_dimensions)
            .fit(&dataset)
            .map_err(|e| EngramError::ComputationError(format!("Random projection fitting error: {}", e)))?;
        
        self.rp_model = Some(rp);
        self.trained = true;
        Ok(())
    }
    
    /// Reduce a single embedding to lower dimensions
    pub fn reduce(&self, embedding: &Embedding) -> Result<Embedding> {
        if !self.trained {
            return Err(EngramError::InvalidOperation("Reducer has not been trained".to_string()));
        }
        
        if embedding.dimensions != self.original_dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Embedding dimensions ({}) don't match training dimensions ({})",
                embedding.dimensions, self.original_dimensions
            )));
        }
        
        // Convert the embedding to a 2D ndarray for processing (convert f32 to f64)
        // Create array and convert to f64 for linfa
        let mut data = Array2::zeros((1, self.original_dimensions));
        for (j, &val) in embedding.vector.iter().enumerate() {
            data[[0, j]] = val as f64;
        }
        
        // Create a dataset
        let dataset = DatasetBase::from(data);
        
        // Apply the dimensionality reduction based on the method
        let reduced_vector = match self.method {
            ReductionMethod::PCA => {
                if let Some(model) = &self.pca_model {
                    let transformed = model.transform(dataset);
                    // Convert f64 back to f32 for storage
                    let records = transformed.records().to_owned();
                    records.iter().map(|&x| x as f32).collect()
                } else {
                    return Err(EngramError::InvalidState("PCA model not trained".to_string()));
                }
            },
            ReductionMethod::RandomProjection => {
                if let Some(model) = &self.rp_model {
                    let transformed = model.transform(dataset);
                    // Convert f64 back to f32 for storage
                    let records = transformed.records().to_owned();
                    records.iter().map(|&x| x as f32).collect()
                } else {
                    return Err(EngramError::InvalidState("Random projection model not trained".to_string()));
                }
            },
            ReductionMethod::Truncation => {
                // Simply keep the first N dimensions
                embedding.vector.iter().take(self.target_dimensions).cloned().collect()
            }
        };
        
        // Create a new embedding with the reduced dimensions
        let mut reduced_embedding = Embedding::new(
            reduced_vector, 
            format!("{}_reduced", &embedding.model)
        );
        
        // Copy metadata and add reduction information
        let mut metadata = embedding.metadata.clone();
        metadata.insert("original_dimensions".to_string(), embedding.dimensions.to_string());
        metadata.insert("reduction_method".to_string(), self.method.to_string());
        metadata.insert("target_dimensions".to_string(), self.target_dimensions.to_string());
        reduced_embedding.metadata = metadata;
        
        Ok(reduced_embedding)
    }
    
    /// Reduce multiple embeddings at once
    pub fn reduce_batch(&self, embeddings: &[Embedding]) -> Result<Vec<Embedding>> {
        if embeddings.is_empty() {
            return Ok(Vec::new());
        }
        
        if !self.trained {
            return Err(EngramError::InvalidOperation("Reducer has not been trained".to_string()));
        }
        
        // For truncation method, just process each embedding individually
        if self.method == ReductionMethod::Truncation {
            let mut results = Vec::with_capacity(embeddings.len());
            for embedding in embeddings {
                results.push(self.reduce(embedding)?);
            }
            return Ok(results);
        }
        
        // For PCA and RandomProjection, we can batch process for efficiency
        // First, check all embeddings have same dimensions
        let dims = embeddings[0].dimensions;
        for (i, emb) in embeddings.iter().enumerate().skip(1) {
            if emb.dimensions != dims {
                return Err(EngramError::InvalidOperation(format!(
                    "Embedding at index {} has different dimensions ({}), expected {}",
                    i, emb.dimensions, dims
                )));
            }
        }
        
        if dims != self.original_dimensions {
            return Err(EngramError::InvalidOperation(format!(
                "Embeddings dimensions ({}) don't match training dimensions ({})",
                dims, self.original_dimensions
            )));
        }
        
        // Create a matrix from all embeddings (convert f32 to f64)
        let mut data = Array2::zeros((embeddings.len(), dims));
        for (i, emb) in embeddings.iter().enumerate() {
            for (j, &val) in emb.vector.iter().enumerate() {
                data[[i, j]] = val as f64;
            }
        }
        
        // Create a dataset
        let dataset = DatasetBase::from(data);
        
        // Apply batch transformation
        let transformed = match self.method {
            ReductionMethod::PCA => {
                if let Some(model) = &self.pca_model {
                    model.transform(dataset)
                } else {
                    return Err(EngramError::InvalidState("PCA model not trained".to_string()));
                }
            },
            ReductionMethod::RandomProjection => {
                if let Some(model) = &self.rp_model {
                    model.transform(dataset)
                } else {
                    return Err(EngramError::InvalidState("Random projection model not trained".to_string()));
                }
            },
            ReductionMethod::Truncation => {
                // This case is handled earlier, but include to be complete
                return Err(EngramError::InvalidOperation("Should not reach here for truncation method".to_string()));
            }
        };
        
        // Convert each row of transformed data back to an Embedding
        let mut results = Vec::with_capacity(embeddings.len());
        
        for (i, emb) in embeddings.iter().enumerate() {
            // Convert f64 back to f32 for the reduced vector
            let reduced_vec: Vec<f32> = transformed.records()
                .slice(s![i, ..])
                .iter()
                .map(|&x| x as f32)
                .collect();
            
            // Create a new embedding with reduced dimensions
            let mut reduced_embedding = Embedding::new(
                reduced_vec,
                format!("{}_reduced", &emb.model),
            );
            
            // Copy metadata and add reduction information
            let mut metadata = emb.metadata.clone();
            metadata.insert("original_dimensions".to_string(), emb.dimensions.to_string());
            metadata.insert("reduction_method".to_string(), self.method.to_string());
            metadata.insert("target_dimensions".to_string(), self.target_dimensions.to_string());
            reduced_embedding.metadata = metadata;
            
            results.push(reduced_embedding);
        }
        
        Ok(results)
    }
    
    /// Check if the reducer has been trained
    pub fn is_trained(&self) -> bool {
        self.trained
    }
    
    /// Get the target dimensions
    pub fn target_dimensions(&self) -> usize {
        self.target_dimensions
    }
    
    /// Get the original dimensions
    pub fn original_dimensions(&self) -> usize {
        self.original_dimensions
    }
    
    /// Get the reduction method
    pub fn method(&self) -> &ReductionMethod {
        &self.method
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn create_test_embeddings(count: usize, dimensions: usize) -> Vec<Embedding> {
        let mut embeddings = Vec::with_capacity(count);
        for i in 0..count {
            let mut vector = Vec::with_capacity(dimensions);
            for j in 0..dimensions {
                // Create deterministic but varying values
                vector.push(((i + j) % 10) as f32 / 10.0);
            }
            embeddings.push(Embedding::new(vector, "test-model".to_string()));
        }
        embeddings
    }
    
    #[test]
    fn test_pca_reduction() {
        let embeddings = create_test_embeddings(10, 20);
        let target_dims = 5;
        
        let mut reducer = DimensionReducer::new(ReductionMethod::PCA, target_dims);
        reducer.train(&embeddings).unwrap();
        
        assert!(reducer.is_trained());
        assert_eq!(reducer.original_dimensions(), 20);
        assert_eq!(reducer.target_dimensions(), target_dims);
        
        // Reduce a single embedding
        let reduced = reducer.reduce(&embeddings[0]).unwrap();
        
        // Check dimensions
        assert_eq!(reduced.dimensions, target_dims);
        
        // Check metadata
        assert_eq!(reduced.metadata.get("original_dimensions").unwrap(), "20");
        assert_eq!(reduced.metadata.get("reduction_method").unwrap(), "PCA");
    }
    
    #[test]
    fn test_random_projection() {
        let embeddings = create_test_embeddings(10, 20);
        let target_dims = 5;
        
        let mut reducer = DimensionReducer::new(ReductionMethod::RandomProjection, target_dims);
        reducer.train(&embeddings).unwrap();
        
        // Reduce a batch
        let reduced_batch = reducer.reduce_batch(&embeddings).unwrap();
        
        // Check all were reduced properly
        assert_eq!(reduced_batch.len(), embeddings.len());
        for reduced in &reduced_batch {
            assert_eq!(reduced.dimensions, target_dims);
        }
    }
    
    #[test]
    fn test_truncation() {
        let embeddings = create_test_embeddings(10, 20);
        let target_dims = 5;
        
        let mut reducer = DimensionReducer::new(ReductionMethod::Truncation, target_dims);
        reducer.train(&embeddings).unwrap();
        
        // Reduce a single embedding
        let reduced = reducer.reduce(&embeddings[0]).unwrap();
        
        // Check dimensions
        assert_eq!(reduced.dimensions, target_dims);
        
        // Check that the truncation method keeps the first N values
        for i in 0..target_dims {
            assert_eq!(reduced.vector[i], embeddings[0].vector[i]);
        }
    }
    
    #[test]
    fn test_invalid_reduction() {
        let embeddings = create_test_embeddings(5, 10);
        
        // Target dimensions too large
        let mut reducer = DimensionReducer::new(ReductionMethod::PCA, 15);
        assert!(reducer.train(&embeddings).is_err());
        
        // Empty embeddings
        let mut reducer = DimensionReducer::new(ReductionMethod::PCA, 5);
        assert!(reducer.train(&[]).is_err());
        
        // Inconsistent dimensions
        let mut mixed_embeddings = create_test_embeddings(3, 10);
        mixed_embeddings.push(Embedding::new(vec![0.1, 0.2], "test-model".to_string()));
        let mut reducer = DimensionReducer::new(ReductionMethod::PCA, 5);
        assert!(reducer.train(&mixed_embeddings).is_err());
    }
}