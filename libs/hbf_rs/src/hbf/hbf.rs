use core::{slice::from_raw_parts};
use core::fmt::{Debug, Formatter, Error};

use crate::header::HBF_CHECKSUM_OFFSET;
use crate::{header::{HBF_HEADER_MIN_SIZE, HbfHeaderBaseGen, HbfHeaderMainGen, HbfHeaderBaseWrapper, HbfHeaderRegionGen, HbfHeaderInterruptGen, HbfHeaderRelocationGen, HbfHeaderRegionWrapper, HbfHeaderInterruptWrapper, HbfHeaderRelocationWrapper, HbfHeaderRegionIter, HbfHeaderInterruptIter, HbfHeaderRelocationIter, HbfHeaderMainWrapper, HBF_MAGIC, HbfVersion}, section::{HbfPayloadSectionWrapper, HbfPayloadSectionGen}, HbfFile};


/// Memorizes a reference to the hbf buffer (start address)
pub struct HbfGen<'a>(&'a [u8]);

impl <'a> HbfGen<'a> {
    pub fn from_bytes(buffer: &'a [u8]) -> Result<Self, crate::Error> {
        if buffer.len() < HBF_HEADER_MIN_SIZE {
            return Err(crate::Error::BufferTooShort);
        }
        if !buffer.starts_with(&HBF_MAGIC) {
            return Err(crate::Error::InvalidMagic);
        }
        let hbf = Self(buffer);
        if hbf.header_base().hbf_version() != HbfVersion::V1 {
            return Err(crate::Error::UnsupportedVersion);
        }
        Ok(hbf)
    }

    /*
        Raw data mapping
    */

    fn content(&self) -> &[u8] {
        self.0
    }
    fn hbf_header_base_raw(&self) -> &HbfHeaderBaseGen {
        unsafe { &*(self.content().as_ptr() as *const HbfHeaderBaseGen) }
    }
    fn hbf_header_main_raw(&self) -> &HbfHeaderMainGen {
        let offset = self.header_base().offset_main() as usize;
        unsafe {
            let main_ptr = self.content().as_ptr().add(offset);
            &*(main_ptr as *const HbfHeaderMainGen)
        }
    }
    fn hbf_header_region_raw(&'a self) -> &'a [HbfHeaderRegionGen] {
        let offset = self.header_base().offset_regions() as usize;
        let num = self.header_base().num_regions() as usize;
        unsafe {
            let region_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(region_ptr as *const HbfHeaderRegionGen, num)
        }
    }
    fn hbf_header_interrupt_raw(&'a self) -> &'a [HbfHeaderInterruptGen] {
        let offset = self.header_base().offset_interrupts() as usize;
        let num = self.header_base().num_interrupts() as usize;
        unsafe {
            let int_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(int_ptr as *const HbfHeaderInterruptGen, num)
        }
    }
    fn hbf_header_relocation_raw(&'a self) -> &'a [HbfHeaderRelocationGen] {
        let offset = self.header_base().offset_relocation() as usize;
        let num = self.header_base().num_relocations() as usize;
        unsafe {
            let rel_ptr = self.content().as_ptr().add(offset);
            from_raw_parts(rel_ptr as *const HbfHeaderRelocationGen, num)
        }
    }
    fn hbf_payload_read_only_gen(&self) -> HbfPayloadSectionGen {
        let offset = core::mem::size_of::<HbfHeaderBaseGen>()
            + core::mem::size_of::<HbfHeaderMainGen>()
            + core::mem::size_of::<HbfHeaderRegionGen>() * self.header_base().num_regions() as usize
            + core::mem::size_of::<HbfHeaderInterruptGen>() * self.header_base().num_interrupts() as usize
            + core::mem::size_of::<HbfHeaderRelocationGen>() * self.header_base().num_relocations() as usize;
        let size = match self.header_main().data_offset() {
            0 => self.header_base().total_size() - offset as u32,
            data_offset =>  data_offset - offset as u32
        };
        return HbfPayloadSectionGen {
            offset: offset as u32,
            size: size
        };
    }

    fn hbf_payload_data_gen(&self) -> Option<HbfPayloadSectionGen> {
        let offset = self.header_main().data_offset();
        let size = self.data_size();
        return match offset {
            0 => None,
            _ => Some(HbfPayloadSectionGen {
                offset: offset,
                size: size
            })
        };
    }

    fn hbf_payload_bss_gen(&self) -> Option<HbfPayloadSectionGen> {
        let size = self.bss_size();
        return match size {
            0 => None,
            _ => Some(HbfPayloadSectionGen {
                offset: 0,
                size: size
            })
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
            return self.header_base().total_size() - offset;
        }
    }

    fn bss_size(&self) -> u32 {
        let cum_size = self.header_main().databss_size();
        return cum_size - self.data_size();
    }
    
    /*
        Direct access
    */
    pub fn region_nth(&self, index: usize) -> Option<HbfHeaderRegionWrapper> {
        self.hbf_header_region_raw()
            .get(index)
            .map(|r| HbfHeaderRegionWrapper::new(self, r))
    }
    pub fn interrupt_nth(&self, index: usize) -> Option<HbfHeaderInterruptWrapper> {
        self.hbf_header_interrupt_raw()
            .get(index)
            .map(|i| HbfHeaderInterruptWrapper::new(self, i))
    }
    pub fn relocation_nth(&self, index: usize) -> Option<HbfHeaderRelocationWrapper> {
        self.hbf_header_relocation_raw()
            .get(index)
            .map(|r| HbfHeaderRelocationWrapper::new(self, r))
    }

    /*
        Iterator support
    */
    pub fn region_iter(&self) -> HbfHeaderRegionIter {
        HbfHeaderRegionIter::new(self)
    }
    pub fn interrupt_iter(&self) -> HbfHeaderInterruptIter {
        HbfHeaderInterruptIter::new(self)
    }
    pub fn relocation_iter(&self) -> HbfHeaderRelocationIter {
        HbfHeaderRelocationIter::new(self)
    }

    /*
        Validation
    */
    pub fn validate(&self) -> bool {
        let bytes = self.content();
        let mut index : usize = 0;
        let mut checksum: u32 = 0;
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
            if index == HBF_CHECKSUM_OFFSET {
                // Consider the checksum field as zeros
                word = 0;
            } else {
                // Convert the 4 bytes into a word
                let mut i = 0;
                for c in &bytes[index..index+available] {
                    word |= u32::from(*c) << (8 * i);
                    i += 1;
                }
            }
            checksum ^= word;
            index += available;
        }
        return self.header_base().checksum() == checksum;
    }

}

impl <'a> HbfFile for HbfGen<'a> {
    fn content(&self) -> &[u8] {
        self.content()
    }

    fn header_base(&self) -> HbfHeaderBaseWrapper {
        HbfHeaderBaseWrapper::new(self, self.hbf_header_base_raw())
    }

    fn header_main(&self) -> crate::header::HbfHeaderMainWrapper {
        HbfHeaderMainWrapper::new(self, self.hbf_header_main_raw())
    }

    fn region_nth(&self, index: usize) -> Option<HbfHeaderRegionWrapper> {
        self.region_nth(index)
    }
    fn region_iter(&self) -> HbfHeaderRegionIter {
        self.region_iter()
    }

    fn interrupt_nth(&self, index: usize) -> Option<HbfHeaderInterruptWrapper> {
        self.interrupt_nth(index)
    }
    fn interrupt_iter(&self) -> HbfHeaderInterruptIter {
        self.interrupt_iter()
    }

    fn relocation_nth(&self, index: usize) -> Option<HbfHeaderRelocationWrapper> {
        self.relocation_nth(index)
    }
    fn relocation_iter(&self) -> HbfHeaderRelocationIter {
        self.relocation_iter()
    }

    fn read_only_section(&self) -> HbfPayloadSectionWrapper {
        HbfPayloadSectionWrapper::new(self, self.hbf_payload_read_only_gen())
    }

    fn data_section(&self) -> Option<HbfPayloadSectionWrapper> {
        let section = self.hbf_payload_data_gen();
        if section.is_some() {
            return Some(HbfPayloadSectionWrapper::new(self, section.unwrap()));
        } else {
            return None;
        }
    }

    fn bss_section(&self) -> Option<HbfPayloadSectionWrapper> {
        let section = self.hbf_payload_bss_gen();
        if section.is_some() {
            return Some(HbfPayloadSectionWrapper::new(self, section.unwrap()));
        } else {
            return None;
        }
    }

    fn validate(&self) -> bool {
        self.validate()
    }

}

impl <'a> Debug for HbfGen<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf File")
            .field("Memory Location", &self.content().as_ptr())
            .finish()
    }
}