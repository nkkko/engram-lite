use crate::error::Result;
use crate::schema::{Engram, Connection};
use crate::storage::Storage;
use crate::index::SearchIndex;
use crate::query::{EngramQuery, QueryService};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Data for a single benchmark run
pub struct BenchmarkResult {
    /// Name of the operation being benchmarked
    pub operation: String,
    
    /// Number of iterations performed
    pub iterations: usize,
    
    /// Total time taken for all iterations
    pub total_time: Duration,
    
    /// Average time per operation
    pub avg_time: Duration,
    
    /// Additional metrics specific to this benchmark
    pub metrics: HashMap<String, f64>,
}

impl BenchmarkResult {
    /// Create a new benchmark result
    pub fn new(operation: &str, iterations: usize, total_time: Duration) -> Self {
        let avg_time = if iterations > 0 {
            Duration::from_nanos((total_time.as_nanos() / iterations as u128) as u64)
        } else {
            Duration::from_secs(0)
        };
        
        Self {
            operation: operation.to_string(),
            iterations,
            total_time,
            avg_time,
            metrics: HashMap::new(),
        }
    }
    
    /// Add a metric to the result
    pub fn with_metric(mut self, name: &str, value: f64) -> Self {
        self.metrics.insert(name.to_string(), value);
        self
    }
    
    /// Format the result for display
    pub fn format(&self) -> String {
        let mut result = format!(
            "Operation: {}\n  Iterations: {}\n  Total time: {:.4}s\n  Avg time: {:.4}ms\n",
            self.operation,
            self.iterations,
            self.total_time.as_secs_f64(),
            self.avg_time.as_micros() as f64 / 1000.0
        );
        
        if !self.metrics.is_empty() {
            result.push_str("  Metrics:\n");
            for (name, value) in &self.metrics {
                result.push_str(&format!("    {}: {:.4}\n", name, value));
            }
        }
        
        result
    }
}

/// Benchmark for storing engrams
pub fn benchmark_engram_storage(storage: &Storage, count: usize) -> Result<BenchmarkResult> {
    let start = Instant::now();
    let mut rng = StdRng::seed_from_u64(42); // Deterministic for reproducibility
    
    for i in 0..count {
        let confidence = rng.gen_range(0.5..1.0);
        let source = match i % 3 {
            0 => "user_input",
            1 => "system_generated",
            _ => "external_source",
        };
        
        let metadata = if i % 5 == 0 {
            Some(HashMap::from([
                ("category".to_string(), serde_json::json!("benchmark")),
                ("importance".to_string(), serde_json::json!(rng.gen_range(1..10))),
            ]))
        } else {
            None
        };
        
        let engram = Engram::new(
            format!("Benchmark engram content #{} with some text for search", i),
            source.to_string(),
            confidence,
            metadata,
        );
        
        storage.put_engram(&engram)?;
    }
    
    let total_time = start.elapsed();
    
    // Calculate operations per second
    let ops_per_sec = count as f64 / total_time.as_secs_f64();
    
    Ok(BenchmarkResult::new("Engram Storage", count, total_time)
        .with_metric("operations_per_second", ops_per_sec))
}

/// Benchmark for retrieving engrams
pub fn benchmark_engram_retrieval(
    storage: &Storage, 
    engram_ids: &[String], 
    iterations: usize
) -> Result<BenchmarkResult> {
    let start = Instant::now();
    let mut found = 0;
    
    let mut rng = StdRng::seed_from_u64(42);
    
    for _ in 0..iterations {
        // Pick a random ID
        let idx = rng.gen_range(0..engram_ids.len());
        let id = &engram_ids[idx];
        
        // Retrieve the engram
        if let Some(_) = storage.get_engram(id)? {
            found += 1;
        }
    }
    
    let total_time = start.elapsed();
    
    // Calculate operations per second and hit rate
    let ops_per_sec = iterations as f64 / total_time.as_secs_f64();
    let hit_rate = found as f64 / iterations as f64;
    
    Ok(BenchmarkResult::new("Engram Retrieval", iterations, total_time)
        .with_metric("operations_per_second", ops_per_sec)
        .with_metric("hit_rate", hit_rate))
}

/// Benchmark for connection operations
pub fn benchmark_connections(
    storage: &Storage, 
    engram_ids: &[String], 
    count: usize
) -> Result<BenchmarkResult> {
    let start = Instant::now();
    let mut rng = StdRng::seed_from_u64(42);
    let mut connection_ids = Vec::with_capacity(count);
    
    // Create connections
    for i in 0..count {
        // Pick two random engrams
        let source_idx = rng.gen_range(0..engram_ids.len());
        let mut target_idx = rng.gen_range(0..engram_ids.len());
        
        // Ensure source and target are different
        while target_idx == source_idx {
            target_idx = rng.gen_range(0..engram_ids.len());
        }
        
        let source_id = &engram_ids[source_idx];
        let target_id = &engram_ids[target_idx];
        
        // Create a relationship type
        let rel_type = match i % 4 {
            0 => "related_to",
            1 => "causes",
            2 => "part_of",
            _ => "references",
        };
        
        let weight = rng.gen_range(0.1..1.0);
        
        let connection = Connection::new(
            source_id.clone(),
            target_id.clone(),
            rel_type.to_string(),
            weight,
            None,
        );
        
        storage.put_connection(&connection)?;
        connection_ids.push(connection.id);
    }
    
    // Time for creating connections
    let creation_time = start.elapsed();
    
    // Retrieve connections
    let retrieval_start = Instant::now();
    let mut found = 0;
    
    for id in &connection_ids {
        if let Some(_) = storage.get_connection(id)? {
            found += 1;
        }
    }
    
    let retrieval_time = retrieval_start.elapsed();
    
    // Total time for the benchmark
    let total_time = start.elapsed();
    
    // Calculate metrics
    let creation_ops_per_sec = count as f64 / creation_time.as_secs_f64();
    let retrieval_ops_per_sec = connection_ids.len() as f64 / retrieval_time.as_secs_f64();
    let hit_rate = found as f64 / connection_ids.len() as f64;
    
    Ok(BenchmarkResult::new("Connection Operations", count * 2, total_time)
        .with_metric("creation_ops_per_sec", creation_ops_per_sec)
        .with_metric("retrieval_ops_per_sec", retrieval_ops_per_sec)
        .with_metric("hit_rate", hit_rate))
}

/// Benchmark for search operations
pub fn benchmark_search(
    _index: &SearchIndex,
    service: &QueryService,
    iterations: usize
) -> Result<BenchmarkResult> {
    let start = Instant::now();
    let mut rng = StdRng::seed_from_u64(42);
    
    // Search terms to benchmark
    let search_terms = [
        "engram", "benchmark", "content", "text", "search"
    ];
    
    let mut total_results = 0;
    
    for _ in 0..iterations {
        // Pick a random search term
        let term_idx = rng.gen_range(0..search_terms.len());
        let term = search_terms[term_idx];
        
        // Perform the search
        let query = EngramQuery::new().with_text(term);
        let results = service.get_query_engine().query_engrams(&query)?;
        
        total_results += results.len();
    }
    
    let total_time = start.elapsed();
    
    // Calculate operations per second and average results
    let ops_per_sec = iterations as f64 / total_time.as_secs_f64();
    let avg_results = total_results as f64 / iterations as f64;
    
    Ok(BenchmarkResult::new("Search Operations", iterations, total_time)
        .with_metric("operations_per_second", ops_per_sec)
        .with_metric("avg_results_per_query", avg_results))
}

/// Benchmark for graph traversal operations
pub fn benchmark_traversal(
    service: &QueryService,
    engram_ids: &[String],
    iterations: usize
) -> Result<BenchmarkResult> {
    let start = Instant::now();
    let mut rng = StdRng::seed_from_u64(42);
    
    let mut total_engrams = 0;
    let mut total_connections = 0;
    
    for _ in 0..iterations {
        // Pick a random engram
        let idx = rng.gen_range(0..engram_ids.len());
        let id = &engram_ids[idx];
        
        // Traverse its connections
        let result = service.find_connected_engrams(id, 2, None)?;
        
        total_engrams += result.engrams.len();
        total_connections += result.connections.len();
    }
    
    let total_time = start.elapsed();
    
    // Calculate operations per second and average results
    let ops_per_sec = iterations as f64 / total_time.as_secs_f64();
    let avg_engrams = total_engrams as f64 / iterations as f64;
    let avg_connections = total_connections as f64 / iterations as f64;
    
    Ok(BenchmarkResult::new("Graph Traversal", iterations, total_time)
        .with_metric("operations_per_second", ops_per_sec)
        .with_metric("avg_engrams_per_traversal", avg_engrams)
        .with_metric("avg_connections_per_traversal", avg_connections))
}

/// Run all benchmarks and return the results
pub fn run_all_benchmarks(storage: &Storage, index: &SearchIndex) -> Result<Vec<BenchmarkResult>> {
    println!("Starting benchmarks...");
    
    // Number of engrams to create and benchmark with
    let engram_count = 1000;
    
    // Engram storage benchmark
    println!("Benchmarking engram storage...");
    let storage_result = benchmark_engram_storage(storage, engram_count)?;
    
    // Get all engram IDs for other benchmarks
    println!("Retrieving engram IDs...");
    let engram_ids = storage.list_engrams()?;
    
    // Engram retrieval benchmark
    println!("Benchmarking engram retrieval...");
    let retrieval_result = benchmark_engram_retrieval(storage, &engram_ids, 5000)?;
    
    // Connection operations benchmark
    println!("Benchmarking connection operations...");
    let connection_count = 2000;
    let connection_result = benchmark_connections(storage, &engram_ids, connection_count)?;
    
    // Create a query service
    let service = QueryService::new(storage, index);
    
    // Search benchmark
    println!("Benchmarking search operations...");
    let search_result = benchmark_search(index, &service, 1000)?;
    
    // Traversal benchmark
    println!("Benchmarking graph traversal...");
    let traversal_result = benchmark_traversal(&service, &engram_ids, 500)?;
    
    Ok(vec![
        storage_result,
        retrieval_result,
        connection_result,
        search_result,
        traversal_result,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use tempfile::tempdir;
    
    #[test]
    fn test_benchmark_results_format() {
        // Create a simple benchmark result
        let result = BenchmarkResult::new("Test Operation", 1000, Duration::from_millis(1500))
            .with_metric("ops_per_second", 666.6667)
            .with_metric("hit_rate", 0.95);
        
        // Format the result
        let formatted = result.format();
        
        // Verify the format contains the operation name
        assert!(formatted.contains("Test Operation"));
        
        // Verify it contains the iteration count
        assert!(formatted.contains("1000"));
        
        // Verify metrics are included
        assert!(formatted.contains("ops_per_second"));
        assert!(formatted.contains("hit_rate"));
    }
    
    #[test]
    fn test_basic_benchmark_functionality() {
        // Create a temporary directory for the database
        let dir = tempdir().unwrap();
        let db_path = dir.path().to_str().unwrap();
        
        // Initialize storage and index
        let storage = Storage::new(db_path).unwrap();
        let mut index = SearchIndex::new();
        
        // Run a small benchmark
        let engram = Engram::new(
            "Test engram for benchmark".to_string(),
            "test".to_string(),
            0.9,
            None,
        );
        
        // Add to storage and index
        storage.put_engram(&engram).unwrap();
        index.add_engram(&engram).unwrap();
        
        // Simple retrieval benchmark
        let ids = vec![engram.id.clone()];
        let result = benchmark_engram_retrieval(&storage, &ids, 10).unwrap();
        
        // Verify we got sensible results
        assert_eq!(result.operation, "Engram Retrieval");
        assert_eq!(result.iterations, 10);
        assert!(result.total_time > Duration::from_nanos(0));
        
        // Clean up
        dir.close().unwrap();
    }
}