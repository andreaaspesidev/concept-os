// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_hex::{SerHex, StrictPfx, CompactPfx};


#[allow(non_camel_case_types)]
#[derive(Deserialize, PartialEq, Debug)]
pub enum RegionAttribute {
    READ,
    WRITE,
    EXECUTE,
    DEVICE,
    DMA
}

/**
 * Structures
 */

#[derive(Deserialize, PartialEq, Debug)]
pub struct BoardConfig {
    pub board: Board,
    pub linker: Linker,
    pub peripheral: BTreeMap<String, Peripheral>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct Peripheral {
    #[serde(with = "SerHex::<StrictPfx>")]
    pub base_address: u32,
    #[serde(with = "SerHex::<CompactPfx>")]
    pub size: u32,
    pub attributes: Vec<RegionAttribute>,
    pub interrupts: Option<BTreeMap<String, u32>>
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