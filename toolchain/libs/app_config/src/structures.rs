use std::collections::BTreeMap;

use serde::Deserialize;

/**
 * Structures
 */
#[derive(Deserialize, PartialEq, Debug)]
pub struct ComponentConfig {
    pub features: Vec<String>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub name: String,
    pub board: String,
    pub kernel_ram: u32,
    pub components: BTreeMap<String, ComponentConfig>
}