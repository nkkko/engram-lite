#[cfg(feature = "python")]
use pyo3::prelude::*;
use crate::schema::{Engram, Connection, EngramId};
use crate::storage::Storage;
use crate::embedding::{Embedding, EmbeddingModel, EmbeddingService};
use crate::vector_search::{VectorIndex, VectorQuery, HybridQuery, HybridSearchEngine, CombinationMethod};
use crate::error::EngramError;
use std::collections::HashMap;
use std::sync::Arc;

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyEngram {
    #[pyo3(get)]
    pub id: String,
    
    #[pyo3(get, set)]
    pub content: String,
    
    #[pyo3(get)]
    pub timestamp: String,
    
    #[pyo3(get, set)]
    pub source: String,
    
    #[pyo3(get, set)]
    pub confidence: f64,
    
    #[pyo3(get, set)]
    pub metadata: HashMap<String, String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEngram {
    #[new]
    fn new(content: String, source: String, confidence: f64) -> Self {
        let engram = Engram::new(content, source, confidence, None);
        
        Self {
            id: engram.id,
            content: engram.content,
            timestamp: engram.timestamp.to_string(),
            source: engram.source,
            confidence: engram.confidence,
            metadata: engram.metadata.into_iter()
                .filter_map(|(k, v)| {
                    if let serde_json::Value::String(s) = v {
                        Some((k, s))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
    
    pub fn to_dict(&self) -> HashMap<String, PyObject> {
        let mut dict = HashMap::new();
        Python::with_gil(|py| {
            dict.insert("id".to_string(), self.id.clone().into_py(py));
            dict.insert("content".to_string(), self.content.clone().into_py(py));
            dict.insert("timestamp".to_string(), self.timestamp.clone().into_py(py));
            dict.insert("source".to_string(), self.source.clone().into_py(py));
            dict.insert("confidence".to_string(), self.confidence.into_py(py));
            dict.insert("metadata".to_string(), self.metadata.clone().into_py(py));
        });
        
        dict
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyConnection {
    #[pyo3(get)]
    pub id: String,
    
    #[pyo3(get)]
    pub source_id: String,
    
    #[pyo3(get)]
    pub target_id: String,
    
    #[pyo3(get, set)]
    pub relationship_type: String,
    
    #[pyo3(get, set)]
    pub weight: f64,
    
    #[pyo3(get, set)]
    pub metadata: HashMap<String, String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyConnection {
    #[new]
    fn new(source_id: String, target_id: String, relationship_type: String, weight: f64) -> Self {
        let connection = Connection::new(source_id, target_id, relationship_type, weight, None);
        
        Self {
            id: connection.id,
            source_id: connection.source_id,
            target_id: connection.target_id,
            relationship_type: connection.relationship_type,
            weight: connection.weight,
            metadata: connection.metadata.into_iter()
                .filter_map(|(k, v)| {
                    if let serde_json::Value::String(s) = v {
                        Some((k, s))
                    } else {
                        None
                    }
                })
                .collect(),
        }
    }
    
    pub fn to_dict(&self) -> HashMap<String, PyObject> {
        let mut dict = HashMap::new();
        Python::with_gil(|py| {
            dict.insert("id".to_string(), self.id.clone().into_py(py));
            dict.insert("source_id".to_string(), self.source_id.clone().into_py(py));
            dict.insert("target_id".to_string(), self.target_id.clone().into_py(py));
            dict.insert("relationship_type".to_string(), self.relationship_type.clone().into_py(py));
            dict.insert("weight".to_string(), self.weight.into_py(py));
            dict.insert("metadata".to_string(), self.metadata.clone().into_py(py));
        });
        
        dict
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub struct PyEmbedding {
    #[pyo3(get)]
    pub vector: Vec<f32>,
    
    #[pyo3(get)]
    pub model: String,
    
    #[pyo3(get)]
    pub dimensions: usize,
    
    #[pyo3(get, set)]
    pub metadata: HashMap<String, String>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEmbedding {
    #[new]
    fn new(vector: Vec<f32>, model: String) -> Self {
        let embedding = Embedding::new(vector, model);
        
        Self {
            vector: embedding.vector,
            model: embedding.model,
            dimensions: embedding.dimensions,
            metadata: embedding.metadata,
        }
    }
    
    pub fn cosine_similarity(&self, other: &PyEmbedding) -> PyResult<f32> {
        let self_embedding = Embedding::new(self.vector.clone(), self.model.clone());
        let other_embedding = Embedding::new(other.vector.clone(), other.model.clone());
        
        self_embedding.cosine_similarity(&other_embedding)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))
    }
    
    pub fn normalize(&mut self) {
        let mut embedding = Embedding::new(self.vector.clone(), self.model.clone());
        embedding.normalize();
        self.vector = embedding.vector;
    }
}

#[cfg(feature = "python")]
#[pyclass]
#[derive(Clone)]
pub enum PyEmbeddingModelType {
    E5 = 0,
    GTE = 1, 
    JINA = 2,
    Custom = 3,
}

#[cfg(feature = "python")]
#[pyclass]
pub struct PyEmbeddingService {
    service: Arc<EmbeddingService>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEmbeddingService {
    #[new]
    fn new() -> Self {
        Self {
            service: Arc::new(EmbeddingService::new()),
        }
    }
    
    #[staticmethod]
    fn with_model_type(model_type: PyEmbeddingModelType) -> Self {
        let model = match model_type {
            PyEmbeddingModelType::E5 => EmbeddingModel::E5MultilingualLargeInstruct,
            PyEmbeddingModelType::GTE => EmbeddingModel::GteModernBertBase,
            PyEmbeddingModelType::JINA => EmbeddingModel::JinaEmbeddingsV3,
            PyEmbeddingModelType::Custom => EmbeddingModel::Custom,
        };
        
        Self {
            service: Arc::new(EmbeddingService::with_model_type(model)),
        }
    }
    
    #[staticmethod]
    fn with_model(model_name: Option<&str>) -> Self {
        Self {
            service: Arc::new(EmbeddingService::new_with_model(model_name)),
        }
    }
    
    pub fn embed_text(&self, text: &str) -> PyResult<PyEmbedding> {
        let embedding = self.service.embed_text(text)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(PyEmbedding {
            vector: embedding.vector,
            model: embedding.model,
            dimensions: embedding.dimensions,
            metadata: embedding.metadata,
        })
    }
    
    pub fn embed_batch(&self, texts: Vec<&str>) -> PyResult<Vec<PyEmbedding>> {
        let embeddings = self.service.embed_batch(&texts)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(embeddings.into_iter().map(|e| PyEmbedding {
            vector: e.vector,
            model: e.model,
            dimensions: e.dimensions,
            metadata: e.metadata,
        }).collect())
    }
    
    pub fn get_model_name(&self) -> String {
        self.service.get_model_name()
    }
    
    pub fn get_dimensions(&self) -> usize {
        self.service.get_dimensions()
    }
}

#[cfg(feature = "python")]
#[pyclass]
pub struct PyEngramDB {
    storage: Storage,
    vector_index: Option<VectorIndex>,
    embedding_service: Option<Arc<EmbeddingService>>,
}

#[cfg(feature = "python")]
#[pymethods]
impl PyEngramDB {
    #[new]
    fn new(db_path: &str) -> PyResult<Self> {
        let storage = Storage::new(db_path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(Self {
            storage,
            vector_index: None,
            embedding_service: None,
        })
    }
    
    pub fn init_vector_search(&mut self, model_type: Option<PyEmbeddingModelType>) -> PyResult<()> {
        let embedding_service = match model_type {
            Some(model) => {
                let rust_model = match model {
                    PyEmbeddingModelType::E5 => EmbeddingModel::E5MultilingualLargeInstruct,
                    PyEmbeddingModelType::GTE => EmbeddingModel::GteModernBertBase,
                    PyEmbeddingModelType::JINA => EmbeddingModel::JinaEmbeddingsV3,
                    PyEmbeddingModelType::Custom => {
                        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                            "Custom model type requires a model name. Use init_vector_search_with_model instead."
                        ));
                    }
                };
                Arc::new(EmbeddingService::with_model_type(rust_model))
            }
            None => Arc::new(EmbeddingService::new()),
        };
        
        self.vector_index = Some(VectorIndex::with_embedding_service(embedding_service.clone()));
        self.embedding_service = Some(embedding_service);
        
        Ok(())
    }
    
    pub fn init_vector_search_with_model(&mut self, model_name: Option<&str>) -> PyResult<()> {
        let embedding_service = Arc::new(EmbeddingService::new_with_model(model_name));
        self.vector_index = Some(VectorIndex::with_embedding_service(embedding_service.clone()));
        self.embedding_service = Some(embedding_service);
        
        Ok(())
    }
    
    pub fn add_engram(&self, engram: PyEngram) -> PyResult<String> {
        // Convert PyEngram to Engram
        let mut metadata = HashMap::new();
        for (k, v) in engram.metadata {
            metadata.insert(k, serde_json::Value::String(v));
        }
        
        let rust_engram = Engram::new(
            engram.content,
            engram.source,
            engram.confidence,
            Some(metadata),
        );
        
        // Store the engram
        self.storage.put_engram(&rust_engram)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        // If vector search is enabled, add to vector index
        if let Some(ref vector_index) = self.vector_index {
            vector_index.add_engram(&rust_engram)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        }
        
        Ok(rust_engram.id)
    }
    
    pub fn get_engram(&self, engram_id: &str) -> PyResult<Option<PyEngram>> {
        let result = self.storage.get_engram(&engram_id.to_string())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        match result {
            Some(engram) => {
                let metadata = engram.metadata.into_iter()
                    .filter_map(|(k, v)| {
                        if let serde_json::Value::String(s) = v {
                            Some((k, s))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                Ok(Some(PyEngram {
                    id: engram.id,
                    content: engram.content,
                    timestamp: engram.timestamp.to_string(),
                    source: engram.source,
                    confidence: engram.confidence,
                    metadata,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub fn add_connection(&self, connection: PyConnection) -> PyResult<String> {
        // Convert PyConnection to Connection
        let mut metadata = HashMap::new();
        for (k, v) in connection.metadata {
            metadata.insert(k, serde_json::Value::String(v));
        }
        
        let rust_connection = Connection::new(
            connection.source_id,
            connection.target_id,
            connection.relationship_type,
            connection.weight,
            Some(metadata),
        );
        
        // Store the connection
        self.storage.put_connection(&rust_connection)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(rust_connection.id)
    }
    
    pub fn get_connection(&self, connection_id: &str) -> PyResult<Option<PyConnection>> {
        let result = self.storage.get_connection(&connection_id.to_string())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        match result {
            Some(connection) => {
                let metadata = connection.metadata.into_iter()
                    .filter_map(|(k, v)| {
                        if let serde_json::Value::String(s) = v {
                            Some((k, s))
                        } else {
                            None
                        }
                    })
                    .collect();
                
                Ok(Some(PyConnection {
                    id: connection.id,
                    source_id: connection.source_id,
                    target_id: connection.target_id,
                    relationship_type: connection.relationship_type,
                    weight: connection.weight,
                    metadata,
                }))
            }
            None => Ok(None),
        }
    }
    
    pub fn delete_engram(&self, engram_id: &str) -> PyResult<bool> {
        // Remove from vector index if enabled
        if let Some(ref vector_index) = self.vector_index {
            vector_index.remove_engram(&engram_id.to_string())
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        }
        
        // Remove from storage
        self.storage.delete_engram(&engram_id.to_string())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(true)
    }
    
    pub fn delete_connection(&self, connection_id: &str) -> PyResult<bool> {
        self.storage.delete_connection(&connection_id.to_string())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        
        Ok(true)
    }
    
    pub fn query_by_text(&self, query: &str, limit: Option<usize>) -> PyResult<Vec<PyEngram>> {
        // Check if vector search is enabled
        if let Some(ref vector_index) = self.vector_index {
            // Use vector search
            let results = vector_index.search(query, limit.unwrap_or(10))
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
            
            let mut engrams = Vec::new();
            for (engram_id, _) in results {
                if let Ok(Some(engram)) = self.storage.get_engram(&engram_id) {
                    let metadata = engram.metadata.into_iter()
                        .filter_map(|(k, v)| {
                            if let serde_json::Value::String(s) = v {
                                Some((k, s))
                            } else {
                                None
                            }
                        })
                        .collect();
                    
                    engrams.push(PyEngram {
                        id: engram.id,
                        content: engram.content,
                        timestamp: engram.timestamp.to_string(),
                        source: engram.source,
                        confidence: engram.confidence,
                        metadata,
                    });
                }
            }
            
            Ok(engrams)
        } else {
            // Return error since vector search is not enabled
            Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Vector search is not enabled. Call init_vector_search first."
            ))
        }
    }
    
    pub fn get_embedding_service(&self) -> PyResult<PyEmbeddingService> {
        match &self.embedding_service {
            Some(service) => Ok(PyEmbeddingService {
                service: service.clone(),
            }),
            None => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Embedding service is not initialized. Call init_vector_search first."
            )),
        }
    }
}

// Define the Python module
#[cfg(feature = "python")]
#[pymodule]
fn engram_lite(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyEngram>()?;
    m.add_class::<PyConnection>()?;
    m.add_class::<PyEmbedding>()?;
    m.add_class::<PyEmbeddingModelType>()?;
    m.add_class::<PyEmbeddingService>()?;
    m.add_class::<PyEngramDB>()?;
    
    Ok(())
}