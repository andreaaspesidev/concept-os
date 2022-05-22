#![no_std]

mod hbf;
mod header;
mod section;
pub mod utils;

pub use hbf::HbfFile;
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