use std::{error::Error, fs};

use serde::{Serialize, Deserialize};

/*
    Structures (needed to read the config file)
*/
#[derive(Serialize, Deserialize)]
struct RelocationConfig {
    general: General,
    relocations: Relocations
}
#[derive(Serialize, Deserialize)]
struct General {
    flash_linked_address: u32,
    sram_linked_address: u32,
    total_relocations: u32
}
#[derive(Serialize, Deserialize)]
struct Relocations {
    points: Vec<u32>
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ParseResult {
    pub points: Vec<u32>
}

pub fn parse_relocations(relocation_config_file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Read file
    let file_content = fs::read_to_string(relocation_config_file)?;
    // Parse config from file
    let config: RelocationConfig = toml::from_str(&file_content)?;
    // Generate response
    return Ok(ParseResult{
        points: config.relocations.points
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
        let file_path = get_test_file_path("example1/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            points: vec![1090520430, 2667578738, 1090520442, 2667578750, 1090520576, 2667578884, 1090520588, 2667578896, 1090520712, 2667579020, 1090520724, 2667579032, 1090520852, 2667579160, 1090520864, 2667579172, 1090520998, 2667579308, 1090521040, 2667579350, 1090521084, 2667579394, 1090521122, 2667579432, 1107298358, 1107298362, 2650802240, 2650802244, 1090521174, 2667579484, 1107298410, 1107298414, 2650802292, 2650802296, 1090521226, 2667579536, 1107298460, 1107298464, 2650802342, 2650802346, 1090521276, 2667579586, 1107298510, 1107298514, 2650802392, 2650802396, 1090521326, 2667579636, 1090521410, 2667579718, 1090521434, 2667579742, 1090521474, 2667579784, 1090521756, 2667580064, 1090521888, 2667580204, 1090522110, 2667580426, 1090522154, 2667580470, 1090522366, 2667580678, 1090522538, 2667580846, 1090522582, 2667580890, 5772, 5780, 5816, 5824, 5872, 5884, 5888, 5892]
        };
        assert_eq!(config, reference_config);
    }
}