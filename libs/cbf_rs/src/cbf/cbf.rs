// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::{Debug, Error, Formatter};
use core::slice::from_raw_parts;

use crate::header::{CbfHeaderDependencyGen, CbfHeaderDependencyIter, CbfHeaderDependencyWrapper};
use crate::trailer::{CbfTrailerGen, CbfTrailerWrapper, CBF_CHECKSUM_OFFSET};
use crate::{
    header::{
        CbfHeaderBaseGen, CbfHeaderBaseWrapper, CbfHeaderInterruptGen, CbfHeaderInterruptIter,
        CbfHeaderInterruptWrapper, CbfHeaderMainGen, CbfHeaderMainWrapper, CbfHeaderRegionGen,
        CbfHeaderRegionIter, CbfHeaderRegionWrapper, CbfHeaderRelocationGen,
        CbfHeaderRelocationIter, CbfHeaderRelocationWrapper, CbfVersion, CBF_HEADER_MIN_SIZE,
        CBF_MAGIC,
    },
    section::{CbfPayloadSectionGen, CbfPayloadSectionWrapper},
    CbfFile,
};

/// Memorizes a reference to the cbf buffer (start address)
pub struct CbfGen<'a>(&'a [u8]);

impl<'a> CbfGen<'a> {
    pub fn from_bytes(buffer: &'a [u8]) -> Result<Self, crate::Error> {
        if buffer.len() < CBF_HEADER_MIN_SIZE {
            return Err(crate::Error::BufferTooShort);
        }
        if !buffer.starts_with(&CBF_MAGIC) {
            return Err(crate::Error::InvalidMagic);
        }
        let cbf = Self(buffer);
        if cbf.header_base().cbf_version() != CbfVersion::V1 {
            return Err(crate::Error::UnsupportedVersion);
        }
        Ok(cbf)
    }

    /*
        Raw data mapping
    */

    fn content(&self) -> &[u8] {
        self.0
    }
    fn cbf_header_base_raw(&self) -> &CbfHeaderBaseGen {
        unsafe { &*(self.content().as_ptr() as *const CbfHeaderBaseGen) }
    }
    fn cbf_header_main_raw(&self) -> &CbfHeaderMainGen {
        let offset = core::mem::size_of::<CbfHeaderBaseGen>();
        unsafe {
            let main_ptr = self.content().as_ptr().add(offset);
            &*(main_ptr as *const CbfHeaderMainGen)
        }
    }
    fn cbf_trailer_raw(&'a self) -> &CbfTrailerGen {
        let offset = self.header_base().offset_trailer() as usize;
        unsafe {
            let main_ptr = self.content().as_ptr().add(offset);
            &*(main_ptr as *const CbfTrailerGen)
        }
    }
    fn cbf_header_region_raw(&'a self) -> &'a [CbfHeaderRegionGen] {
        let offset = self.header_base().offset_regions() as usize;
        let num = self.header_base().num_regions() as usize;
        unsafe {
            let region_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(region_ptr as *const CbfHeaderRegionGen, num)
        }
    }
    fn cbf_header_interrupt_raw(&'a self) -> &'a [CbfHeaderInterruptGen] {
        let offset = self.header_base().offset_interrupts() as usize;
        let num = self.header_base().num_interrupts() as usize;
        unsafe {
            let int_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(int_ptr as *const CbfHeaderInterruptGen, num)
        }
    }
    fn cbf_header_relocation_raw(&'a self) -> &'a [CbfHeaderRelocationGen] {
        let offset = self.header_base().offset_relocation() as usize;
        let num = self.header_base().num_relocations() as usize;
        unsafe {
            let rel_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(rel_ptr as *const CbfHeaderRelocationGen, num)
        }
    }
    fn cbf_header_dependency_raw(&'a self) -> &'a [CbfHeaderDependencyGen] {
        let offset = self.header_base().offset_dependencies() as usize;
        let num = self.header_base().num_dependencies() as usize;
        unsafe {
            let rel_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(rel_ptr as *const CbfHeaderDependencyGen, num)
        }
    }
    fn cbf_payload_offset(&self) -> usize {
        core::mem::size_of::<CbfHeaderBaseGen>()
            + core::mem::size_of::<CbfHeaderMainGen>()
            + core::mem::size_of::<CbfHeaderRegionGen>()
                * self.header_base().num_regions() as usize
            + core::mem::size_of::<CbfHeaderInterruptGen>()
                * self.header_base().num_interrupts() as usize
            + core::mem::size_of::<CbfHeaderRelocationGen>()
                * self.header_base().num_relocations() as usize
            + core::mem::size_of::<CbfHeaderDependencyGen>()
                * self.header_base().num_dependencies() as usize
            + self.header_base().padding_bytes() as usize
    }
    fn cbf_payload_read_only_gen(&self) -> CbfPayloadSectionGen {
        let offset = self.cbf_payload_offset();
        let size = match self.header_main().data_offset() {
            0 => self.header_base().total_size() - core::mem::size_of::<CbfTrailerGen>() as u32 - offset as u32,
            data_offset => data_offset - offset as u32,
        };
        return CbfPayloadSectionGen {
            offset: offset as u32,
            size: size,
        };
    }

    fn cbf_payload_data_gen(&self) -> Option<CbfPayloadSectionGen> {
        let offset = self.header_main().data_offset();
        let size = self.data_size();
        return match offset {
            0 => None,
            _ => Some(CbfPayloadSectionGen {
                offset: offset,
                size: size,
            }),
        };
    }

    /*
        Size calcs
    */
    fn data_size(&self) -> u32 {
        let offset = self.header_main().data_offset();
        if offset == 0 {
            return 0;
        } else {
            return self.header_base().total_size() - core::mem::size_of::<CbfTrailerGen>() as u32 - offset;
        }
    }

    fn bss_size(&self) -> u32 {
        let cum_size = self.header_main().databss_size();
        return cum_size - self.data_size();
    }

    fn payload_size(&self) -> u32 {
        let offset = self.cbf_payload_offset();
        self.header_base().total_size() - core::mem::size_of::<CbfTrailerGen>() as u32 - offset as u32
    }

    /*
        Direct access
    */
    pub fn region_nth(&self, index: usize) -> Option<CbfHeaderRegionWrapper> {
        self.cbf_header_region_raw()
            .get(index)
            .map(|r| CbfHeaderRegionWrapper::new(self, r))
    }
    pub fn interrupt_nth(&self, index: usize) -> Option<CbfHeaderInterruptWrapper> {
        self.cbf_header_interrupt_raw()
            .get(index)
            .map(|i| CbfHeaderInterruptWrapper::new(self, i))
    }
    pub fn relocation_nth(&self, index: usize) -> Option<CbfHeaderRelocationWrapper> {
        self.cbf_header_relocation_raw()
            .get(index)
            .map(|r| CbfHeaderRelocationWrapper::new(self, r))
    }
    pub fn dependency_nth(&self, index: usize) -> Option<CbfHeaderDependencyWrapper> {
        self.cbf_header_dependency_raw()
            .get(index)
            .map(|r| CbfHeaderDependencyWrapper::new(self, r))
    }

    /*
        Iterator support
    */
    pub fn region_iter(&self) -> CbfHeaderRegionIter {
        CbfHeaderRegionIter::new(self)
    }
    pub fn interrupt_iter(&self) -> CbfHeaderInterruptIter {
        CbfHeaderInterruptIter::new(self)
    }
    pub fn relocation_iter(&self) -> CbfHeaderRelocationIter {
        CbfHeaderRelocationIter::new(self)
    }
    pub fn dependency_iter(&self) -> CbfHeaderDependencyIter {
        CbfHeaderDependencyIter::new(self)
    }

    /*
        Validation
    */
    pub fn validate(&self) -> bool {
        let bytes = self.content();
        let mut index: usize = 0;
        let mut checksum: u32 = 0;
        let chechsum_offset: usize =
            self.header_base().offset_trailer() as usize + CBF_CHECKSUM_OFFSET;
        loop {
            let mut word: u32 = 0;
            let mut available: usize = 4;
            // Check if enough bytes are available
            if bytes.len() <= index + 4 {
                available = bytes.len() - index;
                if available == 0 {
                    break;
                }
            }
            if index == chechsum_offset {
                // Consider the checksum field as zeros
                word = 0;
            } else {
                // Convert the 4 bytes into a word
                let mut i = 0;
                for c in &bytes[index..index + available] {
                    word |= u32::from(*c) << (8 * i);
                    i += 1;
                }
            }
            checksum ^= word;
            index += available;
        }
        return self.trailer().checksum() == checksum;
    }
}

impl<'a> CbfFile for CbfGen<'a> {
    fn content(&self) -> &[u8] {
        self.content()
    }

    fn header_base(&self) -> CbfHeaderBaseWrapper {
        CbfHeaderBaseWrapper::new(self, self.cbf_header_base_raw())
    }

    fn header_main(&self) -> crate::header::CbfHeaderMainWrapper {
        CbfHeaderMainWrapper::new(self, self.cbf_header_main_raw())
    }

    fn trailer(&self) -> crate::trailer::CbfTrailerWrapper {
        CbfTrailerWrapper::new(self, self.cbf_trailer_raw())
    }

    fn checksum_offset(&self) -> u32 {
        self.header_base().offset_trailer() + CBF_CHECKSUM_OFFSET as u32
    }

    fn region_nth(&self, index: usize) -> Option<CbfHeaderRegionWrapper> {
        self.region_nth(index)
    }
    fn region_iter(&self) -> CbfHeaderRegionIter {
        self.region_iter()
    }

    fn interrupt_nth(&self, index: usize) -> Option<CbfHeaderInterruptWrapper> {
        self.interrupt_nth(index)
    }
    fn interrupt_iter(&self) -> CbfHeaderInterruptIter {
        self.interrupt_iter()
    }

    fn relocation_nth(&self, index: usize) -> Option<CbfHeaderRelocationWrapper> {
        self.relocation_nth(index)
    }
    fn relocation_iter(&self) -> CbfHeaderRelocationIter {
        self.relocation_iter()
    }

    fn dependency_nth(&self, index: usize) -> Option<CbfHeaderDependencyWrapper> {
        self.dependency_nth(index)
    }
    fn dependency_iter(&self) -> CbfHeaderDependencyIter {
        self.dependency_iter()
    }

    fn read_only_section(&self) -> CbfPayloadSectionWrapper {
        CbfPayloadSectionWrapper::new(self, self.cbf_payload_read_only_gen())
    }

    fn data_section(&self) -> Option<CbfPayloadSectionWrapper> {
        let section = self.cbf_payload_data_gen();
        if section.is_some() {
            return Some(CbfPayloadSectionWrapper::new(self, section.unwrap()));
        } else {
            return None;
        }
    }

    fn bss_size(&self) -> u32 {
        self.bss_size()
    }

    fn validate(&self) -> bool {
        self.validate()
    }

    fn payload_size(&self) -> u32 {
        self.payload_size()
    }
}

impl<'a> Debug for CbfGen<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf File")
            .field("Memory Location", &self.content().as_ptr())
            .finish()
    }
}
