#[cfg(test)]
mod tests {
    use engram_lite::index::{ImportanceIndex, TemporalIndex};
    use engram_lite::schema::Engram;
    use chrono::{DateTime, Datelike, Duration, Timelike, Utc};
    use std::thread::sleep;
    use std::time::Duration as StdDuration;

    fn create_test_engram(id: &str, content: &str, source: &str, confidence: f64) -> Engram {
        let mut engram = Engram::new(
            content.to_string(),
            source.to_string(),
            confidence,
            None,
        );
        // Override the UUID with a fixed ID for testing
        engram.id = id.to_string();
        engram
    }

    fn create_test_engram_with_timestamp(
        id: &str, 
        content: &str, 
        source: &str, 
        confidence: f64,
        timestamp: DateTime<Utc>
    ) -> Engram {
        let mut engram = create_test_engram(id, content, source, confidence);
        engram.timestamp = timestamp;
        engram
    }

    #[test]
    fn test_temporal_index_basic() {
        let mut index = TemporalIndex::new();
        
        // Create test engrams with different timestamps
        let now = Utc::now();
        let yesterday = now - Duration::days(1);
        let last_week = now - Duration::days(7);
        
        let engram1 = create_test_engram_with_timestamp("e1", "Recent engram", "test", 0.9, now);
        let engram2 = create_test_engram_with_timestamp("e2", "Yesterday's engram", "test", 0.8, yesterday);
        let engram3 = create_test_engram_with_timestamp("e3", "Last week's engram", "test", 0.7, last_week);
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Test get_most_recent
        let recent = index.get_most_recent(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0], "e1");
        assert_eq!(recent[1], "e2");
        
        // Test find_before
        let before_cutoff = now - Duration::hours(12);
        let old_engrams = index.find_before(&before_cutoff);
        assert_eq!(old_engrams.len(), 2);
        assert!(old_engrams.contains("e2"));
        assert!(old_engrams.contains("e3"));
        
        // Test find_after
        let after_cutoff = now - Duration::days(2);
        let new_engrams = index.find_after(&after_cutoff);
        assert_eq!(new_engrams.len(), 2);
        assert!(new_engrams.contains("e1"));
        assert!(new_engrams.contains("e2"));
        
        // Test find_between
        let start = now - Duration::days(2);
        let end = now - Duration::hours(12);
        let between_engrams = index.find_between(&start, &end);
        assert_eq!(between_engrams.len(), 1);
        assert!(between_engrams.contains("e2"));
    }

    #[test]
    fn test_temporal_index_granularity() {
        let mut index = TemporalIndex::new();
        
        // Create test engrams with timestamps in different granularity
        let now = Utc::now();
        
        // Extract time components to create consistent test timestamps
        let year = now.year();
        let month = now.month();
        let day = now.day();
        
        // Create test engrams at different time granularities
        let engram1 = create_test_engram_with_timestamp(
            "e1", 
            "Morning engram", 
            "test", 
            0.9,
            Utc::now().with_hour(8).unwrap()
        );
        
        let engram2 = create_test_engram_with_timestamp(
            "e2", 
            "Afternoon engram", 
            "test", 
            0.8,
            Utc::now().with_hour(14).unwrap()
        );
        
        let engram3 = create_test_engram_with_timestamp(
            "e3", 
            "Previous day engram", 
            "test", 
            0.7,
            Utc::now().with_day(if day > 1 { day - 1 } else { day }).unwrap()
        );
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Test find_by_year
        let year_engrams = index.find_by_year(year);
        assert_eq!(year_engrams.len(), 3);
        
        // Test find_by_month
        let month_engrams = index.find_by_month(year, month);
        assert_eq!(month_engrams.len(), 3);
        
        // Test find_by_day (only today's engrams)
        let today_engrams = index.find_by_day(year, month, day);
        assert!(today_engrams.contains("e1"));
        assert!(today_engrams.contains("e2"));
        
        // Test find_by_hour
        let morning_engrams = index.find_by_hour(8);
        assert_eq!(morning_engrams.len(), 1);
        assert!(morning_engrams.contains("e1"));
        
        let afternoon_engrams = index.find_by_hour(14);
        assert_eq!(afternoon_engrams.len(), 1);
        assert!(afternoon_engrams.contains("e2"));
    }

    #[test]
    fn test_importance_tracking() {
        let mut index = ImportanceIndex::new();
        
        // Create test engrams with different importance scores
        let mut engram1 = create_test_engram("e1", "High importance", "test", 0.9);
        engram1.importance = 0.9;
        
        let mut engram2 = create_test_engram("e2", "Medium importance", "test", 0.8);
        engram2.importance = 0.5;
        
        let mut engram3 = create_test_engram("e3", "Low importance", "test", 0.7);
        engram3.importance = 0.2;
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Test find_by_min_importance
        let important_engrams = index.find_by_min_importance(0.5);
        assert_eq!(important_engrams.len(), 2);
        assert!(important_engrams.contains("e1"));
        assert!(important_engrams.contains("e2"));
        
        // Test get_most_important
        let most_important = index.get_most_important(2);
        assert_eq!(most_important.len(), 2);
        assert_eq!(most_important[0], "e1");
        assert_eq!(most_important[1], "e2");
        
        // Test updating importance
        index.update_importance(&"e3".to_string(), 0.8).unwrap();
        
        // After update, e3 should now be considered important
        let important_after_update = index.find_by_min_importance(0.7);
        assert_eq!(important_after_update.len(), 2);
        assert!(important_after_update.contains("e1"));
        assert!(important_after_update.contains("e3"));
        
        // Most important should now be different
        let most_important_after = index.get_most_important(2);
        assert_eq!(most_important_after.len(), 2);
        assert_eq!(most_important_after[0], "e1");
        assert_eq!(most_important_after[1], "e3");
    }

    #[test]
    fn test_access_frequency_tracking() {
        let mut index = ImportanceIndex::new();
        
        // Create test engrams
        let engram1 = create_test_engram("e1", "Frequently accessed", "test", 0.9);
        let engram2 = create_test_engram("e2", "Moderately accessed", "test", 0.8);
        let engram3 = create_test_engram("e3", "Rarely accessed", "test", 0.7);
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Record different access patterns
        for _ in 0..10 {
            index.record_access(&"e1".to_string()).unwrap();
        }
        
        for _ in 0..5 {
            index.record_access(&"e2".to_string()).unwrap();
        }
        
        index.record_access(&"e3".to_string()).unwrap();
        
        // Test find_by_min_access_count
        let frequent_engrams = index.find_by_min_access_count(5);
        assert_eq!(frequent_engrams.len(), 2);
        assert!(frequent_engrams.contains("e1"));
        assert!(frequent_engrams.contains("e2"));
        
        let very_frequent_engrams = index.find_by_min_access_count(8);
        assert_eq!(very_frequent_engrams.len(), 1);
        assert!(very_frequent_engrams.contains("e1"));
        
        // Test access recency
        // First, make e3 the most recently accessed
        sleep(StdDuration::from_millis(10)); // Ensure a time difference
        index.record_access(&"e3".to_string()).unwrap();
        
        // Now test recency sorting (should be e3, e2, e1)
        // Since recency_sorted is internal, we need to test through find_by_last_accessed_after
        // using a time just before updating e3
        let now = Utc::now();
        let recent_engrams = index.find_by_last_accessed_after(&(now - Duration::milliseconds(5)));
        assert!(recent_engrams.contains("e3"));
        assert!(!recent_engrams.contains("e1"));
        assert!(!recent_engrams.contains("e2"));
    }

    #[test]
    fn test_ttl_functionality() {
        let mut index = ImportanceIndex::new();
        
        // Create test engrams
        let mut engram1 = create_test_engram("e1", "Short TTL", "test", 0.9);
        engram1.set_ttl(1); // 1 second TTL
        
        let mut engram2 = create_test_engram("e2", "Medium TTL", "test", 0.8);
        engram2.set_ttl(60); // 60 seconds TTL
        
        let engram3 = create_test_engram("e3", "No TTL", "test", 0.7);
        // No TTL set for engram3
        
        // Add engrams to index
        index.add_engram(&engram1).unwrap();
        index.add_engram(&engram2).unwrap();
        index.add_engram(&engram3).unwrap();
        
        // Set TTL via the index
        index.set_ttl(&"e3".to_string(), Some(120)).unwrap(); // 120 seconds TTL
        
        // Wait for the short TTL to expire
        sleep(StdDuration::from_secs(2));
        
        // Check for expired engrams
        let expired = index.get_expired_engrams();
        assert!(expired.contains("e1"));
        assert!(!expired.contains("e2"));
        assert!(!expired.contains("e3"));
    }

    #[test]
    fn test_engram_schema_methods() {
        // Test TTL methods
        let mut engram = create_test_engram("e1", "Test engram", "test", 0.9);
        
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
        sleep(StdDuration::from_secs(2));
        assert!(engram.is_expired());
        
        // Test time_remaining
        let mut engram2 = create_test_engram("e2", "Another test engram", "test", 0.8);
        assert_eq!(engram2.time_remaining(), None); // No TTL set
        
        engram2.set_ttl(3600); // 1 hour
        let remaining = engram2.time_remaining().unwrap();
        assert!(remaining > 0 && remaining <= 3600);
        
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
        
        sleep(StdDuration::from_millis(10)); // Ensure time difference
        engram.record_access();
        
        assert_eq!(engram.access_count, before_count + 1);
        assert!(engram.last_accessed > before_access);
    }
}