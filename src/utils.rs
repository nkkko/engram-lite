use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Load environment variables from a .env file
pub fn load_env_from_file(file_path: &str) -> std::io::Result<()> {
    let path = Path::new(file_path);
    
    // Skip if file doesn't exist
    if !path.exists() {
        return Ok(());
    }
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // Parse KEY=VALUE pairs
        if let Some(equal_sign_pos) = line.find('=') {
            let key = line[..equal_sign_pos].trim();
            let value = line[equal_sign_pos + 1..].trim();
            
            // Only set the env var if it's not already set
            if env::var(key).is_err() {
                env::set_var(key, value);
            }
        }
    }
    
    Ok(())
}

/// Get the Anthropic API key from environment
pub fn get_anthropic_api_key() -> Option<String> {
    // Check for ANTHROPIC_API_KEY first
    if let Ok(key) = env::var("ANTHROPIC_API_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }
    
    // Check for ANTHROPIC_KEY as fallback
    if let Ok(key) = env::var("ANTHROPIC_KEY") {
        if !key.is_empty() {
            return Some(key);
        }
    }
    
    None
}

/// Check if Claude capabilities are available
pub fn has_claude_capabilities() -> bool {
    get_anthropic_api_key().is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    
    #[test]
    fn test_get_anthropic_api_key() {
        // Save the original value to restore later
        let original = env::var("ANTHROPIC_API_KEY").ok();
        
        // Clear the env var for testing
        env::remove_var("ANTHROPIC_API_KEY");
        
        // Test getting the API key when not set
        assert!(get_anthropic_api_key().is_none());
        
        // Set the API key and test again
        env::set_var("ANTHROPIC_API_KEY", "test_key");
        assert_eq!(get_anthropic_api_key(), Some("test_key".to_string()));
        
        // Restore the original value
        if let Some(val) = original {
            env::set_var("ANTHROPIC_API_KEY", val);
        } else {
            env::remove_var("ANTHROPIC_API_KEY");
        }
    }
}