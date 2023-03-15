// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]

mod hbf;
mod header;
mod trailer;
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