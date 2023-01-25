use std::error::Error;
mod structures;
pub use structures::AppConfig;

pub fn read_configuration(path: &str) -> Result<AppConfig, Box<dyn Error>> {
    // Read file
    let content = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&content)?)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::read_configuration;

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn test1() {
        let test_file_path = get_test_file_path("example1.toml");
        let config = read_configuration(&test_file_path).unwrap();
        assert_eq!(config.name, "stm32f303re_demo");
        println!("{:?}", config);
    }
}