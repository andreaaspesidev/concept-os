// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "fmt")]
use core::fmt::{Debug, Error, Formatter};

/**
 * Defines
 */
pub const HBF_CHECKSUM_OFFSET: u32 = 0x00;

pub const HBF_TRAILER_SIZE: usize = core::mem::size_of::<HbfTrailer>();

/**
 * Structures
 */

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct HbfTrailer {
    checksum: u32,            // 0
}

impl<'a> HbfTrailer {
    pub fn checksum(&self) -> u32 {
        let p = core::ptr::addr_of!(self.checksum);
        unsafe { p.read_unaligned() }
    }

    pub fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}

#[cfg(feature = "fmt")]
impl Debug for HbfTrailer {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Trailer")
            .field("Checksum", &format_args!("{:#010x}", &self.checksum()))
            .finish()
    }
}