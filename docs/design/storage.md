# Storage Design

EngramAI Lite uses RocksDB as its storage engine, providing a fast, persistent, and ACID-compliant foundation for the memory graph.

## Storage Architecture

The storage layer is built with a multi-layered approach:

1. **RocksDB Layer**: Low-level persistence with column families
2. **Storage API Layer**: High-level operations for data manipulation
3. **Transaction Layer**: ACID transactions for data integrity

## RocksDB Column Families

RocksDB organizes data into column families, which EngramAI Lite uses to separate different entity types:

| Column Family | Purpose |
|---------------|---------|
| `engrams` | Stores engram data |
| `connections` | Stores connection data |
| `collections` | Stores collection data |
| `agents` | Stores agent data |
| `contexts` | Stores context data |
| `metadata` | Stores system-wide metadata |

## Key Design

Each entity is stored with a composite key consisting of a type prefix and the entity ID:

```
<type_prefix>:<entity_id>
```

For example:
- Engram keys: `engram:3a7c9f8e-1234-5678-90ab-cdef01234567`
- Connection keys: `connection:7b2d1e9c-1234-5678-90ab-cdef01234567`

## Value Storage

Values are stored as JSON-serialized data using `serde_json`. This provides:

1. **Schema flexibility**: Easy to evolve the data model
2. **Human-readable format**: Easier debugging and data inspection
3. **Compatibility**: Works well with import/export functionality

## Storage Implementation

The core `Storage` struct encapsulates RocksDB operations:

```rust
pub struct Storage {
    pub db: DB,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        // Initialize RocksDB with column families
    }
    
    // Engram operations
    pub fn put_engram(&self, engram: &Engram) -> Result<()> { ... }
    pub fn get_engram(&self, id: &EngramId) -> Result<Option<Engram>> { ... }
    pub fn delete_engram(&self, id: &EngramId) -> Result<()> { ... }
    pub fn list_engrams(&self) -> Result<Vec<String>> { ... }
    
    // Similar methods for other entity types
    
    // Transaction support
    pub fn begin_transaction(&self) -> Transaction { ... }
}
```

## Transaction Support

EngramAI Lite provides ACID transaction support through RocksDB's `WriteBatch` mechanism:

```rust
pub struct Transaction<'a> {
    batch: WriteBatch,
    db: &'a DB,
}

impl<'a> Transaction<'a> {
    // Operation methods
    pub fn put_engram(&mut self, engram: &Engram) -> Result<()> { ... }
    pub fn delete_engram(&mut self, id: &EngramId) -> Result<()> { ... }
    
    // Similar methods for other entity types
    
    // Commit or abort the transaction
    pub fn commit(self) -> Result<()> { ... }
    pub fn abort(self) { ... }
}
```

Usage example:

```rust
// Begin a transaction
let mut txn = storage.begin_transaction();

// Perform operations
txn.put_engram(&engram1)?;
txn.put_engram(&engram2)?;
txn.put_connection(&connection)?;

// Commit the transaction (or call abort() to discard changes)
txn.commit()?;
```

## Serialization and Deserialization

The storage layer converts between Rust structs and binary data:

```rust
// Serialize a struct to bytes
fn serialize<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    serde_json::to_vec(value).map_err(|e| EngramError::SerializationError(e.to_string()))
}

// Deserialize bytes to a struct
fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T> {
    serde_json::from_slice(bytes).map_err(|e| EngramError::SerializationError(e.to_string()))
}
```

## Performance Considerations

The storage layer includes several optimizations:

1. **Column Families**: Separate column families for different entity types improve scan performance
2. **Prefix Encoding**: Keys use type prefixes for logical grouping
3. **Batch Operations**: Bulk operations are performed in batches for efficiency
4. **Compaction**: Database compaction is available to reclaim space and improve read performance

## Integration with In-Memory Graph

While RocksDB provides persistence, EngramAI Lite maintains an in-memory graph representation using `petgraph` for fast traversal and query operations. The storage layer serves as the system of record, while the in-memory graph enables high-performance graph algorithms.

## Future Storage Enhancements

Future enhancements to the storage layer may include:

1. **Tiered Storage**: Hot/cold data separation for performance
2. **Bloom Filters**: Faster negative lookups
3. **Custom Comparators**: Optimized key ordering
4. **Compression Tuning**: Better space efficiency
5. **Sharding**: Distributing data across multiple database instances