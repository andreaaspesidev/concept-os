// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Error;
use core::fmt::Formatter;
use core::ops::Deref;
use core::fmt::Debug;


use crate::hbf::HbfFile;

pub use self::hbf_trailer::HbfTrailerGen;

mod hbf_trailer;

pub const HBF_CHECKSUM_OFFSET: usize = 0x00;

pub trait HbfTrailer<'a> {
    fn checksum(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}
/*
    Wrappers
*/
pub struct HbfTrailerWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfTrailer<'a>,
}
impl<'a> HbfTrailerWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfTrailer<'a>) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
}
impl<'a> Deref for HbfTrailerWrapper<'a> {
    type Target = dyn HbfTrailer<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfTrailerWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Trailer")
            .field("Checksum", &format_args!("{:#010x}",&self.checksum()))
            .finish()
    }
}