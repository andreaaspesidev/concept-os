use core::fmt::Formatter;
use core::fmt::Debug;

mod hbf_section;
use crate::hbf::HbfFile;
use crate::utils;

pub use self::hbf_section::HbfPayloadSectionGen;

pub trait HbfPayloadSection {
    fn size(&self) -> u32;
    fn offset(&self) -> u32;
}

pub struct HbfPayloadSectionWrapper<'a> {
    elf_file: &'a dyn HbfFile,
    inner: HbfPayloadSectionGen,
}

impl<'a> HbfPayloadSectionWrapper<'a> {
    pub fn new(elf_file: &'a dyn HbfFile, inner: HbfPayloadSectionGen) -> Self {
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

impl<'a> Debug for HbfPayloadSectionWrapper<'a> {
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