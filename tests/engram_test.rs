use engram_lite::schema::Engram;
use chrono::Utc;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_engram_ttl_functionality() {
    // Create engram with TTL
    let mut engram = Engram::new(
        "Test content".to_string(),
        "test".to_string(),
        0.9,
        None,
    );
    
    // Test set_ttl
    engram.set_ttl(60);
    assert_eq!(engram.ttl, Some(60));
    
    // Test clear_ttl
    engram.clear_ttl();
    assert_eq!(engram.ttl, None);
    
    // Test is_expired
    engram.set_ttl(1);
    assert!(!engram.is_expired()); // Should not be expired immediately
    
    // Wait for expiration
    sleep(Duration::from_secs(2));
    assert!(engram.is_expired());
    
    // Test time_remaining
    let mut engram2 = Engram::new(
        "Another test".to_string(),
        "test".to_string(),
        0.8,
        None,
    );
    assert_eq!(engram2.time_remaining(), None); // No TTL set
    
    engram2.set_ttl(3600); // 1 hour
    let remaining = engram2.time_remaining().unwrap();
    assert!(remaining > 0 && remaining <= 3600);
}

#[test]
fn test_engram_importance_and_access() {
    // Create engram
    let mut engram = Engram::new(
        "Test content".to_string(),
        "test".to_string(),
        0.9,
        None,
    );
    
    // Test set_importance and bounds
    engram.set_importance(0.75);
    assert_eq!(engram.importance, 0.75);
    
    engram.set_importance(1.5); // Out of bounds high
    assert_eq!(engram.importance, 1.0);
    
    engram.set_importance(-0.5); // Out of bounds low
    assert_eq!(engram.importance, 0.0);
    
    // Test record_access
    let before_count = engram.access_count;
    let before_access = engram.last_accessed;
    
    sleep(Duration::from_millis(10)); // Ensure time difference
    engram.record_access();
    
    assert_eq!(engram.access_count, before_count + 1);
    assert!(engram.last_accessed > before_access);
}