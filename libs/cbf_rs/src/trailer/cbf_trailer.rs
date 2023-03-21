// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::CbfTrailer;

#[repr(packed, C)]
pub struct CbfTrailerGen {
    checksum: u32,            // 0
}

impl<'a> CbfTrailer<'a> for CbfTrailerGen {
    fn checksum(&self) -> u32 {
        let p = core::ptr::addr_of!(self.checksum);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfTrailerGen>(),
            )
        }
    }
}