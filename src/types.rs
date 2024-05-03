/// Generic Result type for error handling
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
