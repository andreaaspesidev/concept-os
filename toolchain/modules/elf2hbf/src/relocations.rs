use std::{error::Error, fs};

use serde::{Serialize, Deserialize};

/*
    Structures (needed to read the config file)
*/
#[derive(Serialize, Deserialize)]
struct RelocationConfig {
    relocations: Relocations
}
#[derive(Serialize, Deserialize)]
struct Relocations {
    rodata: Vec<u32>,
    data: Vec<u32>
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ParseResult {
    pub rodata: Option<Vec<u32>>,
    pub data: Option<Vec<u32>>
}

pub fn parse_relocations(relocation_config_file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Read file
    let file_content = fs::read_to_string(relocation_config_file)?;
    // Parse config from file
    let config: RelocationConfig = toml::from_str(&file_content)?;
    // Generate response
    return Ok(ParseResult{
        rodata: match config.relocations.rodata.len() {
            0 => None,
            _ => Some(config.relocations.rodata)
        },
        data: match config.relocations.data.len() {
            0 => None,
            _ => Some(config.relocations.data)
        }
    });
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{parse_relocations, ParseResult};
    
    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn read_test1() {
        let file_path = get_test_file_path("component1/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            rodata: None,
            data: None
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test2() {
        let file_path = get_test_file_path("component2/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            rodata: Some(vec![ 12, 20, 68, 76, 136, 148, 152, 156,]),
            data: None
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test3() {
        let file_path = get_test_file_path("component3/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            rodata: None,
            data: Some(vec![ 0, 4, 8, 16, 24, 32])
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test4() {
        let file_path = get_test_file_path("component4/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            rodata: None,
            data: None
        };
        assert_eq!(config, reference_config);
    }
    
}