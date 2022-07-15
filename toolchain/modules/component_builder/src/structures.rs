use serde::Deserialize;

#[derive(Deserialize, PartialEq, Debug)]
pub struct BoardConfig {
    pub board: Board,
    pub linker: Linker
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Board {
    pub target: String
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Linker {
    pub flash_origin: String,
    pub flash_size: String,
    pub ram_origin: String,
    pub ram_size: String
}