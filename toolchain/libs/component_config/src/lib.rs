pub mod structures;

use crate::structures::ComponentConfig;
use std::fs;
use std::error::Error;

/*
    Methods
*/

pub fn read_component_config(file_name: &str) -> Result<ComponentConfig, Box<dyn Error>> {
    // Read file
    let file_content = fs::read_to_string(file_name)?;
    // Parse config from file
    let config = toml::from_str(&file_content)?;
    return Ok(config);
}

pub fn write_component_config(file_name: &str, config: &ComponentConfig) -> Result<(), Box<dyn Error>> {
    // Generate config
    let file_content = toml::to_string_pretty(config)?;
    // Write to the file
    fs::write(file_name, file_content)?;
    return Ok(());
}



/*
    Tests
*/
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::structures::{Component, ComponentFlag, Region, RegionAttribute, Interrupt};
    use tempfile::NamedTempFile;

    use super::*;
    
    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }
    
    #[test]
    fn config_generation1() {
        // Create temp config
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path().to_str().unwrap();
        let test_file_path = get_test_file_path("test1.toml");

        let config = ComponentConfig {
            component: Component {
                id: 1,
                version: 1,
                priority: 1,
                flags: vec![ComponentFlag::START_AT_BOOT],
                min_ram: 1024
            },
            regions: None,
            interrupts: None
        };
        // Write to file
        let result = write_component_config(&file_path, &config);
        assert!(result.is_ok());
        // Check file content
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, fs::read_to_string(test_file_path).unwrap());
        // Recover structure
        let recovered = read_component_config(&file_path).unwrap();
        assert_eq!(recovered, config);
    }

    #[test]
    fn config_generation2() {
        // Create temp config
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path().to_str().unwrap();
        let test_file_path = get_test_file_path("test2.toml");

        let config = ComponentConfig {
            component: Component {
                id: 2,
                version: 3,
                priority: 4,
                flags: vec![],
                min_ram: 2048
            },
            regions: Some(vec![
                Region{
                    base_address: 0x0800_0000,
                    size: 0x1000,
                    attributes: vec![RegionAttribute::READ, RegionAttribute::WRITE]
                },
                Region{
                    base_address: 0x0800_1000,
                    size: 0x2000,
                    attributes: vec![RegionAttribute::DMA]
                }
            ]),
            interrupts: None
        };
        // Write to file
        let result = write_component_config(&file_path, &config);
        assert!(result.is_ok());
        // Check file content
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, fs::read_to_string(test_file_path).unwrap());
        // Recover structure
        let recovered = read_component_config(&file_path).unwrap();
        assert_eq!(recovered, config);
    }

    #[test]
    fn config_generation3() {
        // Create temp config
        let file = NamedTempFile::new().unwrap();
        let file_path = file.path().to_str().unwrap();
        let test_file_path = get_test_file_path("test3.toml");

        let config = ComponentConfig {
            component: Component {
                id: 2,
                version: 3,
                priority: 4,
                flags: vec![],
                min_ram: 2048
            },
            regions: Some(vec![
                Region{
                    base_address: 0x0800_0000,
                    size: 0x1000,
                    attributes: vec![RegionAttribute::READ, RegionAttribute::WRITE]
                },
                Region{
                    base_address: 0x0800_1000,
                    size: 0x2000,
                    attributes: vec![RegionAttribute::DMA]
                }
            ]),
            interrupts: Some(vec![
                Interrupt{
                    irq: 1,
                    notification_mask: 0b00000000_00000000_00000000_00000001
                },
                Interrupt{
                    irq: 2,
                    notification_mask: 0b00000000_00000000_00000000_00000010
                }
            ])
        };
        // Write to file
        let result = write_component_config(&file_path, &config);
        assert!(result.is_ok());
        // Check file content
        let file_content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(file_content, fs::read_to_string(test_file_path).unwrap());
        // Recover structure
        let recovered = read_component_config(&file_path).unwrap();
        assert_eq!(recovered, config);
    }
}