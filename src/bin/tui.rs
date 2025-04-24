use engram_lite::error::Result;

// This is a placeholder for the Terminal User Interface implementation
// In the future, this will be implemented using the ratatui crate

pub fn run(db_path: &str) -> Result<()> {
    println!("TUI mode is not implemented yet");
    println!("Using database at: {}", db_path);
    println!("This will be implemented using the ratatui crate in the future");
    println!("Press Ctrl+C to exit");
    
    // Keep the program running until user presses Ctrl+C
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}