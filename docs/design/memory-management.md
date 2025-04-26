# Memory Management

EngramAI Lite implements intelligent memory management features inspired by human cognitive processes. This document explains the memory management system and its components.

## Overview

The memory management system provides several key capabilities:

1. **Importance Tracking**: Scoring and tracking importance of engrams
2. **Temporal Organization**: Time-based indexing and retrieval
3. **Access Frequency**: Tracking how often engrams are accessed
4. **Forgetting Mechanisms**: Policies for pruning less important or outdated memories
5. **TTL Support**: Time-to-live for ephemeral information

## Importance Scoring

Engrams have an importance score between 0.0 and 1.0, calculated based on:

- **Centrality**: How connected the engram is in the memory graph
- **Access Frequency**: How often the engram is accessed/retrieved
- **Recency**: How recently the engram was accessed or created
- **Explicit Importance**: Directly assigned importance

```rust
// Example of updating importance based on centrality
pub fn calculate_importance_by_centrality(&mut self, id: &EngramId) -> Result<f64> {
    // Get incoming and outgoing connections
    let incoming = self.get_incoming_connections_count(id);
    let outgoing = self.get_outgoing_connections_count(id);
    
    // Simple centrality score - more connections = more important
    // Normalize to 0.0-1.0 range using a logarithmic scale
    let connection_count = incoming + outgoing;
    let importance = if connection_count > 0.0 {
        (1.0 + connection_count.ln().max(0.0) / 5.0).min(1.0)
    } else {
        0.2 // Base importance for isolated engrams
    };
    
    // Update the importance
    self.update_importance(id, importance)?;
    
    Ok(importance)
}
```

## Temporal Organization

The `TemporalIndex` provides efficient time-based organization and querying:

- **Year/Month/Day/Hour Indexing**: Multi-granular time buckets
- **Recency List**: Maintained sorted list of engrams by recency
- **Time Range Queries**: Before, after, and between operators

```rust
// Example of temporal query
let today_engrams = temporal_index.find_by_day(2023, 4, 15);
let recent_engrams = temporal_index.get_most_recent(10);
let last_week = temporal_index.find_between(&(now - Duration::days(7)), &now);
```

## Access Tracking

Access patterns are recorded to help determine importance and optimize retrieval:

- **Access Counter**: Tracks number of times each engram is accessed
- **Last Accessed**: Records the timestamp of last access
- **Access Frequency Buckets**: Groups engrams by access frequency

```rust
// Example of recording an access
engram.record_access(); // Updates access_count and last_accessed
index.record_access(&engram.id)?; // Updates index tracking
```

## Forgetting Mechanisms

Multiple forgetting policies are available for memory pruning:

- **Age-Based**: Forget old engrams regardless of other factors
- **Importance Threshold**: Forget engrams below an importance threshold
- **Access Frequency**: Forget rarely accessed engrams
- **Hybrid**: Combined approach using all factors
- **TTL Expiration**: Forget engrams that have exceeded their time-to-live

```rust
// Example of applying a forgetting policy
let policy = ForgettingPolicy::Hybrid {
    max_importance: 0.3,
    max_access_count: 2,
    min_idle_seconds: 7 * 24 * 60 * 60, // 7 days
    max_items: 100,
};

let candidates = policy.get_forgetting_candidates(&search_index);
for id in candidates {
    storage.delete_engram(&id)?;
}
```

## TTL Support

Time-to-live (TTL) provides expiration for ephemeral information:

- **Configurable TTL**: Set in seconds when creating or updating an engram
- **Automatic Expiration**: TTL-based forgetting policy automatically removes expired engrams
- **TTL Checking**: Methods to check if an engram has expired or time remaining

```rust
// Example of setting and checking TTL
engram.set_ttl(3600); // 1 hour TTL
if engram.is_expired() {
    // Handle expiration
}
let remaining = engram.time_remaining().unwrap_or(0);
```

## Integration with Query System

Memory management features are integrated with the query system:

- **Importance Filtering**: Query by minimum importance
- **Recency Sorting**: Sort results by recency
- **Access Frequency Filtering**: Query by minimum access count

```rust
// Example of querying with memory management factors
let query = EngramQuery::new()
    .with_text("climate")
    .with_min_importance(0.5)
    .with_sort_by_recency(true);

let results = query_engine.query_engrams(&query)?;
```

## Benefits and Use Cases

The memory management system provides several benefits:

1. **Natural Forgetting**: Mimics human memory by forgetting less important information
2. **Performance Optimization**: Keeps the memory graph at a manageable size
3. **Emphasis on Important Information**: Prioritizes more important engrams in queries
4. **Temporal Context**: Makes time-based queries more efficient
5. **Ephemeral Storage**: Supports automatic cleanup of temporary information