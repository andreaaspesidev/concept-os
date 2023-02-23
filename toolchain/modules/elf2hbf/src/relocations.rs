use std::{error::Error, fmt::Display, fs};

use serde::{Deserialize, Serialize};

/*
    Structures (needed to read the config file)
*/
#[derive(Serialize, Deserialize)]
struct RelocationConfig {
    relocations: Relocations,
}
#[derive(Serialize, Deserialize)]
struct Relocations {
    text: Vec<Vec<i32>>,
    rodata: Vec<Vec<i32>>,
    data: Vec<Vec<i32>>,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum RelocationPoint {
    ABS(u32),
    MOVW(i32, u32),
    MOVT(i32, u32),
}

impl RelocationPoint {
    /// Adds an offset to the one contained in the element
    pub fn shift(&self, add_to_section_offset: u32) -> Self {
        match self {
            RelocationPoint::ABS(section_offset) => {
                Self::ABS(*section_offset + add_to_section_offset)
            }
            RelocationPoint::MOVW(paired_offset, section_offset) => {
                Self::MOVW(*paired_offset, *section_offset + add_to_section_offset)
            }
            RelocationPoint::MOVT(paired_offset, section_offset) => {
                Self::MOVT(*paired_offset, *section_offset + add_to_section_offset)
            }
        }
    }

    pub fn encode(&self) -> u32 {
        match self {
            RelocationPoint::ABS(section_offset) => {
                assert!(*section_offset < 16777216);
                // Just encode as it is
                return *section_offset;
            }
            RelocationPoint::MOVW(paired_offset, section_offset) => {
                assert!(*section_offset < 16777216);
                // First, encode the type
                let mut value: u32 = 1 << 30; // MOVW type
                                              // Then encode the offset
                value |= Self::encode_paired(*paired_offset);
                // Lastly encode the offset
                value |= *section_offset;
                // Got it!
                return value;
            }
            RelocationPoint::MOVT(paired_offset, section_offset) => {
                assert!(*section_offset < 16777216);
                // First, encode the type
                let mut value: u32 = 2 << 30; // MOVT type
                                              // Then encode the offset
                value |= Self::encode_paired(*paired_offset);
                // Lastly encode the offset
                value |= *section_offset;
                // Got it!
                return value;
            }
        }
    }
    fn encode_paired(paired_offset: i32) -> u32 {
        assert!(paired_offset > -16 && paired_offset < 16);
        let encoded_paired: u32;
        if paired_offset > 0 {
            encoded_paired = paired_offset as u32;
        } else {
            encoded_paired = twos_complement(paired_offset, 5) as u32;
        }
        return (encoded_paired & 0x1F) << 24;
    }
}

fn twos_complement(value: i32, bits: usize) -> i32 {
    if value < 0 {
        return (1 << bits) as i32 + value;
    } else {
        if value & (1 << (bits - 1)) as i32 != 0 {
            // Sign bit set, compute negative value
            return value - (1 << bits) as i32;
        } else {
            return value;
        }
    }
}

#[derive(PartialEq, PartialOrd, Debug)]
pub struct ParseResult {
    pub text: Option<Vec<RelocationPoint>>,
    pub rodata: Option<Vec<RelocationPoint>>,
    pub data: Option<Vec<RelocationPoint>>,
}

#[derive(Debug)]
pub enum RelocationError {
    WrongRelocationPointFormat,
    InvalidRelocationType,
}

impl Display for RelocationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelocationError::WrongRelocationPointFormat => {
                f.write_str("Wrong relocation point format")
            }
            RelocationError::InvalidRelocationType => f.write_str("Relocation type not supported"),
        }
    }
}

impl Error for RelocationError {}

fn points_to_relocation_points(
    data: &Vec<Vec<i32>>,
) -> Result<Vec<RelocationPoint>, Box<dyn Error>> {
    let mut points = Vec::<RelocationPoint>::new();
    for p in data {
        // Check size
        if p.len() != 3 {
            return Err(Box::new(RelocationError::WrongRelocationPointFormat));
        }
        let rel_type: u8 = p[0].try_into()?;
        let rel_paired_offset = p[1];
        let rel_section_offset: u32 = p[2].try_into()?;
        // Create a relocation point
        points.push(match rel_type {
            0 => Ok(RelocationPoint::ABS(rel_section_offset)),
            1 => Ok(RelocationPoint::MOVW(rel_paired_offset, rel_section_offset)),
            2 => Ok(RelocationPoint::MOVT(rel_paired_offset, rel_section_offset)),
            _ => Err(Box::new(RelocationError::InvalidRelocationType)),
        }?);
    }
    return Ok(points);
}

pub fn parse_relocations(relocation_config_file: &str) -> Result<ParseResult, Box<dyn Error>> {
    // Read file
    let file_content = fs::read_to_string(relocation_config_file)?;
    // Parse config from file
    let config: RelocationConfig = toml::from_str(&file_content)?;
    // Process points to obtain RelocationPoint instances
    let text = points_to_relocation_points(&config.relocations.text)?;
    let rodata = points_to_relocation_points(&config.relocations.rodata)?;
    let data = points_to_relocation_points(&config.relocations.data)?;

    // Generate response
    return Ok(ParseResult {
        text: match text.len() {
            0 => None,
            _ => Some(text),
        },
        rodata: match rodata.len() {
            0 => None,
            _ => Some(rodata),
        },
        data: match data.len() {
            0 => None,
            _ => Some(data),
        },
    });
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use super::{parse_relocations, ParseResult, RelocationPoint};

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
            text: None,
            rodata: None,
            data: None,
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test2() {
        let file_path = get_test_file_path("component2/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            text: None,
            rodata: Some(vec![
                RelocationPoint::ABS(12),
                RelocationPoint::ABS(20),
                RelocationPoint::ABS(68),
                RelocationPoint::ABS(76),
                RelocationPoint::ABS(136),
                RelocationPoint::ABS(148),
                RelocationPoint::ABS(152),
                RelocationPoint::ABS(156),
            ]),
            data: None,
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test3() {
        let file_path = get_test_file_path("component3/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            text: Some(vec![
                RelocationPoint::MOVW(1, 18),
                RelocationPoint::MOVT(-1, 24),
                RelocationPoint::MOVW(1, 34),
                RelocationPoint::MOVT(-1, 40),
                RelocationPoint::MOVW(1, 58),
                RelocationPoint::MOVT(-1, 62),
                RelocationPoint::MOVW(1, 104),
                RelocationPoint::MOVT(-1, 108),
                RelocationPoint::MOVW(1, 158),
                RelocationPoint::MOVT(-1, 164),
                RelocationPoint::MOVW(1, 212),
                RelocationPoint::MOVT(-1, 218),
                RelocationPoint::MOVW(1, 228),
                RelocationPoint::MOVT(-1, 234),
            ]),
            rodata: None,
            data: Some(vec![
                RelocationPoint::ABS(0),
                RelocationPoint::ABS(4),
                RelocationPoint::ABS(8),
                RelocationPoint::ABS(16),
                RelocationPoint::ABS(24),
                RelocationPoint::ABS(32),
            ]),
        };
        assert_eq!(config, reference_config);
    }
    #[test]
    fn read_test4() {
        let file_path = get_test_file_path("component4/output/relocations.toml");
        let config = parse_relocations(&file_path).unwrap();
        let reference_config = ParseResult {
            text: Some(vec![
                RelocationPoint::MOVW(1, 20),
                RelocationPoint::MOVT(-1, 26),
                RelocationPoint::MOVW(2, 76),
                RelocationPoint::MOVW(2, 80),
                RelocationPoint::MOVT(-2, 84),
                RelocationPoint::MOVT(-2, 90),
                RelocationPoint::MOVW(1, 104),
                RelocationPoint::MOVT(-1, 110),
                RelocationPoint::MOVW(1, 126),
                RelocationPoint::MOVT(-1, 130),
            ]),
            rodata: None,
            data: None,
        };
        assert_eq!(config, reference_config);
    }
}
