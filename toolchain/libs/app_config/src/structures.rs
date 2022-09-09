use serde::Deserialize;

/**
 * Structures
 */

#[derive(Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub name: String,
    pub board: String,
    pub kernel_ram: u32,
    pub components: Vec<String>
}