#![no_std]

mod hbf;
mod header;
mod section;
pub mod utils;

pub use hbf::HbfFile;
pub use header::{HbfHeaderBase, HbfHeaderMain, HbfHeaderRegion, HbfHeaderInterrupt, HbfHeaderRelocation};
pub use header::ComponentFlags;
pub use header::RegionAttributes;
pub use header::{HBF_HEADER_MIN_SIZE, FIXED_HEADER_SIZE};
use hbf::hbf::HbfGen;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferTooShort,
    InvalidMagic,
    UnsupportedVersion
}

pub fn parse_hbf<'a>(buffer: &'a [u8]) -> Result<impl HbfFile + 'a, Error> {
    HbfGen::from_bytes(buffer)
}