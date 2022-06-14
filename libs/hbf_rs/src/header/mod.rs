use core::fmt::Error;
use core::fmt::Formatter;
use core::ops::Deref;
use core::fmt::Debug;


use crate::hbf::HbfFile;

pub use self::hbf_header::HbfHeaderBaseGen;
pub use self::hbf_header::HbfHeaderMainGen;
pub use self::hbf_header::HbfHeaderRegionGen;
pub use self::hbf_header::HbfHeaderInterruptGen;
pub use self::hbf_header::HbfHeaderRelocationGen;

mod hbf_header;

pub const HBF_MAGIC : [u8; 4] = [0x7f, b'H', b'B', b'F'];
pub const HBF_HEADER_MIN_SIZE: usize = core::mem::size_of::<HbfHeaderBaseGen>();
pub const HBF_CHECKSUM_OFFSET: usize = 0x20;

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ComponentFlags: u16 {
        const NONE = 0;
        const START_AT_BOOT = 1 << 0;
    }
}
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct RegionAttributes: u32 {
        const NONE = 0;
        /// Region can be read by tasks that include it.
        const READ = 1 << 0;
        /// Region can be written by tasks that include it.
        const WRITE = 1 << 1;
        /// Region can contain executable code for tasks that include it.
        const EXECUTE = 1 << 2;
        /// Region contains memory mapped registers. This affects cache behavior
        /// on devices that include it, and discourages the kernel from using
        /// `memcpy` in the region.
        const DEVICE = 1 << 3;
        /// Region can be used for DMA or communication with other processors.
        /// This heavily restricts how this memory can be cached and will hurt
        /// performance if overused.
        ///
        /// This is ignored for `DEVICE` memory, which is already not cached.
        const DMA = 1 << 4;
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum HbfVersion {
    V1,  // 0x0001
    UNKNOWN(u16),
}
impl From<u16> for HbfVersion {
    fn from(n: u16) -> Self {
        match n {
            1 => HbfVersion::V1,
            n => HbfVersion::UNKNOWN(n)
        }
    }
}

pub trait HbfHeaderBase {
    fn hbf_version(&self) -> HbfVersion;
    fn total_size(&self) -> u32;
    
    fn component_id(&self) -> u16;
    fn component_version(&self) -> u32;

    fn offset_main(&self) -> u16;
    
    fn num_regions(&self) -> u16;
    fn offset_regions(&self) -> u16;

    fn num_interrupts(&self) -> u16;
    fn offset_interrupts(&self) -> u16;

    fn num_relocations(&self) -> u32;
    fn offset_relocation(&self) -> u16;
    
    fn checksum(&self) -> u32;
}

pub trait HbfHeaderMain {
    fn component_priority(&self) -> u16;
    fn component_flags(&self) -> ComponentFlags;
    fn component_min_ram(&self) -> u32;

    fn entry_point_offset(&self) -> u32;

    fn data_offset(&self) -> u32;
    fn databss_size(&self) -> u32;
}

pub trait HbfHeaderRegion {
    fn base_address(&self) -> u32;
    fn size(&self) -> u32;
    fn attributes(&self) -> RegionAttributes;
}

pub trait HbfHeaderInterrupt {
    fn irq_number(&self) -> u32;
    fn notification_mask(&self) -> u32;
}

pub trait HbfHeaderRelocation {
    fn offset(&self) -> u32;
}



/*
    Wrappers
*/
// Base
pub struct HbfHeaderBaseWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfHeaderBase,
}
impl<'a> HbfHeaderBaseWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfHeaderBase) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
}
impl<'a> Deref for HbfHeaderBaseWrapper<'a> {
    type Target = dyn HbfHeaderBase + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfHeaderBaseWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Base")
            .field("Version", &self.hbf_version())
            .field("Total Size", &self.total_size())
            .field("Component ID", &self.component_id())
            .field("Component Version", &self.component_version())
            .field("Region Count", &self.num_regions())
            .field("Interrupt Count", &self.num_interrupts())
            .field("Relocation Count", &self.num_relocations())
            .field("Checksum", &format_args!("{:#010x}",&self.checksum()))
            .finish()
    }
}
// Main
pub struct HbfHeaderMainWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfHeaderMain,
}
impl<'a> HbfHeaderMainWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfHeaderMain) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
}
impl<'a> Deref for HbfHeaderMainWrapper<'a> {
    type Target = dyn HbfHeaderMain + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfHeaderMainWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Main")
            .field("Priority", &self.component_priority())
            .field("Flags", &self.component_flags())
            .field("RAM", &self.component_min_ram())
            .finish()
    }
}

// Region
pub struct HbfHeaderRegionWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfHeaderRegion,
}
impl<'a> HbfHeaderRegionWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfHeaderRegion) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
}
impl<'a> Deref for HbfHeaderRegionWrapper<'a> {
    type Target = dyn HbfHeaderRegion + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfHeaderRegionWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Region")
            .field("Base Address", &format_args!("{:#010x}",&self.base_address()))
            .field("Size", &self.size())
            .field("Attributes", &self.attributes())
            .finish()
    }
}

// Interrupt
pub struct HbfHeaderInterruptWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfHeaderInterrupt,
}
impl<'a> HbfHeaderInterruptWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfHeaderInterrupt) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
}
impl<'a> Deref for HbfHeaderInterruptWrapper<'a> {
    type Target = dyn HbfHeaderInterrupt + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfHeaderInterruptWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Interrupt")
            .field("IRQ", &self.irq_number())
            .field("Notification Mask", &format_args!("{:#032b}",&self.notification_mask()))
            .finish()
    }
}

// Relocations
pub struct HbfHeaderRelocationWrapper<'a> {
    _hbf_file: &'a dyn HbfFile,
    inner: &'a dyn HbfHeaderRelocation,
}
impl<'a> HbfHeaderRelocationWrapper<'a> {
    pub fn new(hbf_file: &'a dyn HbfFile, inner: &'a dyn HbfHeaderRelocation) -> Self {
        Self {
            _hbf_file: hbf_file,
            inner,
        }
    }
    pub fn pointed_addr(&self) -> u32 {
        let offset = self.inner.offset() as usize;
        unsafe {
            let addr_ptr = self._hbf_file.content().as_ptr().add(offset);
            *(addr_ptr as *const u32)
        }
    }
}
impl<'a> Deref for HbfHeaderRelocationWrapper<'a> {
    type Target = dyn HbfHeaderRelocation + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for HbfHeaderRelocationWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Relocation")
            .field("Offset", &format_args!("{:#010x}",&self.offset()))
            .field("Pointed Address", &format_args!("{:#010x}",&self.pointed_addr()))
            .finish()
    }
}

/*
    Iterators
*/
// Region
pub struct HbfHeaderRegionIter<'a> {
    elf_file: &'a dyn HbfFile,
    index: usize,
}
impl<'a> HbfHeaderRegionIter<'a> {
    pub fn new(elf_file: &'a dyn HbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for HbfHeaderRegionIter<'a> {
    type Item = HbfHeaderRegionWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.region_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
// Interrupt
pub struct HbfHeaderInterruptIter<'a> {
    elf_file: &'a dyn HbfFile,
    index: usize,
}
impl<'a> HbfHeaderInterruptIter<'a> {
    pub fn new(elf_file: &'a dyn HbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for HbfHeaderInterruptIter<'a> {
    type Item = HbfHeaderInterruptWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.interrupt_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
// Relocation
pub struct HbfHeaderRelocationIter<'a> {
    elf_file: &'a dyn HbfFile,
    index: usize,
}
impl<'a> HbfHeaderRelocationIter<'a> {
    pub fn new(elf_file: &'a dyn HbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for HbfHeaderRelocationIter<'a> {
    type Item = HbfHeaderRelocationWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.relocation_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}