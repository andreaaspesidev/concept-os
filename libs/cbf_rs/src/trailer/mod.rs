// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Error;
use core::fmt::Formatter;
use core::ops::Deref;
use core::fmt::Debug;


use crate::cbf::CbfFile;

pub use self::cbf_trailer::CbfTrailerGen;

mod cbf_trailer;

pub const CBF_CHECKSUM_OFFSET: usize = 0x00;

pub trait CbfTrailer<'a> {
    fn checksum(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}
/*
    Wrappers
*/
pub struct CbfTrailerWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfTrailer<'a>,
}
impl<'a> CbfTrailerWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfTrailer<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfTrailerWrapper<'a> {
    type Target = dyn CbfTrailer<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfTrailerWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Trailer")
            .field("Checksum", &format_args!("{:#010x}",&self.checksum()))
            .finish()
    }
}