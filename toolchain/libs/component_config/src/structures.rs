// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Serialize, Deserialize};
use serde_hex::{SerHex, StrictPfx, CompactPfx};
use std::collections::BTreeMap;

/*
    Enums
*/
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ComponentFlag {
    START_AT_BOOT
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum RegionAttribute {
    READ,
    WRITE,
    EXECUTE,
    DEVICE,
    DMA
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ComponentConfig {
    pub component: Component,
    pub regions: Option<Vec<Region>>,
    pub interrupts: Option<Vec<Interrupt>>,
    pub dependencies: Option<Vec<Dependency>>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Component {
    pub id: u16,
    pub version: u32,
    pub priority: u16,
    pub flags: Vec<ComponentFlag>,
    pub min_ram: u32
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Region {
    #[serde(with = "SerHex::<StrictPfx>")]
    pub base_address: u32,
    #[serde(with = "SerHex::<CompactPfx>")]
    pub size: u32,
    pub attributes: Vec<RegionAttribute>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Interrupt {
    pub irq: u32,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub notification_mask: u32
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Dependency {
    pub component_id: u16,
    pub min_version: u32,
    pub max_version: u32
}


#[derive(Deserialize, PartialEq, Debug)]
pub struct ComponentExtendedConfig {
    pub component: ComponentExtended,
    pub dependencies: Option<Vec<Dependency>>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct ComponentExtended {
    pub id: u16,
    pub version: u32,
    pub priority: u16,
    pub flags: Vec<ComponentFlag>,
    pub min_ram: u32,
    pub peripherals: Option<Vec<String>>,
    pub interrupts: Option<BTreeMap<String, u32>>,
}