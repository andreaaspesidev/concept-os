use serde::{Serialize, Deserialize};
use serde_hex::{SerHex, StrictPfx, CompactPfx};

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
    pub interrupts: Option<Vec<Interrupt>>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Component {
    pub id: u16,
    pub version: u16,
    pub priority: u8,
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
    pub irq: u16,
    #[serde(with = "SerHex::<StrictPfx>")]
    pub notification_mask: u32
}