// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]

mod cbf;
mod header;
mod trailer;
mod section;
pub mod utils;

pub use cbf::CbfFile;
pub use header::{CbfHeaderBase, CbfHeaderMain, CbfHeaderRegion, CbfHeaderInterrupt, CbfHeaderRelocation};
pub use header::ComponentFlags;
pub use header::RegionAttributes;
pub use header::{CBF_HEADER_MIN_SIZE, FIXED_HEADER_SIZE};
use cbf::cbf::CbfGen;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    BufferTooShort,
    InvalidMagic,
    UnsupportedVersion
}

pub fn parse_cbf<'a>(buffer: &'a [u8]) -> Result<impl CbfFile + 'a, Error> {
    CbfGen::from_bytes(buffer)
}