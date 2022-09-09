use std::error::Error;
mod structures;
pub use structures::BoardConfig;

pub fn read_configuration(path: &str) -> Result<BoardConfig, Box<dyn Error>> {
    // Read file
    let content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}