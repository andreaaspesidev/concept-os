// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Formatter;
use core::fmt::Debug;

mod cbf_section;
use crate::cbf::CbfFile;
use crate::utils;

pub use self::cbf_section::CbfPayloadSectionGen;

pub trait CbfPayloadSection {
    fn size(&self) -> u32;
    fn offset(&self) -> u32;
}

pub struct CbfPayloadSectionWrapper<'a> {
    elf_file: &'a dyn CbfFile,
    inner: CbfPayloadSectionGen,
}

impl<'a> CbfPayloadSectionWrapper<'a> {
    pub fn new(elf_file: &'a dyn CbfFile, inner: CbfPayloadSectionGen) -> Self {
        Self { elf_file, inner }
    }

    pub fn content(&self) -> &'a [u8] {
        let offset = self.inner.offset() as usize;
        let size = self.inner.size() as usize;
        &self.elf_file.content()[offset..offset + size]
    }

    pub fn offset(&self) -> u32 {
        self.inner.offset()
    }

    pub fn size(&self) -> u32 {
        self.inner.size()
    }
}

impl<'a> Debug for CbfPayloadSectionWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("Payload Section")
            .field("offset", &self.inner.offset())
            .field("size", &self.inner.size())
            .finish()?;
        f.write_str("\n------------------------\n")?;
        utils::dump_section(self.content(), f)?;
        f.write_str("\n------------------------\n")?;
        Ok(())
    }
}