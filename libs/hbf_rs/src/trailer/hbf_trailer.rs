use super::HbfTrailer;

#[repr(packed, C)]
pub struct HbfTrailerGen {
    checksum: u32,            // 0
}

impl<'a> HbfTrailer<'a> for HbfTrailerGen {
    fn checksum(&self) -> u32 {
        let p = core::ptr::addr_of!(self.checksum);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<HbfTrailerGen>(),
            )
        }
    }
}