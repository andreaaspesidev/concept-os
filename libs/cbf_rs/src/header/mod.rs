// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Debug;
use core::fmt::Error;
use core::fmt::Formatter;
use core::ops::Deref;

use crate::cbf::CbfFile;

pub use self::cbf_header::CbfHeaderBaseGen;
pub use self::cbf_header::CbfHeaderDependencyGen;
pub use self::cbf_header::CbfHeaderInterruptGen;
pub use self::cbf_header::CbfHeaderMainGen;
pub use self::cbf_header::CbfHeaderRegionGen;
pub use self::cbf_header::CbfHeaderRelocationGen;

mod cbf_header;

pub const CBF_MAGIC: [u8; 4] = [0x7f, b'C', b'B', b'F'];
pub const CBF_HEADER_MIN_SIZE: usize = core::mem::size_of::<CbfHeaderBaseGen>();
pub const FIXED_HEADER_SIZE: usize =
    core::mem::size_of::<CbfHeaderBaseGen>() + core::mem::size_of::<CbfHeaderMainGen>();

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
pub enum CbfVersion {
    V1, // 0x0001
    UNKNOWN(u16),
}
impl From<u16> for CbfVersion {
    fn from(n: u16) -> Self {
        match n {
            1 => CbfVersion::V1,
            n => CbfVersion::UNKNOWN(n),
        }
    }
}

pub trait CbfHeaderBase<'a> {
    fn cbf_version(&self) -> CbfVersion;
    fn total_size(&self) -> u32;

    fn component_id(&self) -> u16;
    fn component_version(&self) -> u32;

    fn padding_bytes(&self) -> u16;

    fn num_regions(&self) -> u16;
    fn offset_regions(&self) -> u16;

    fn num_interrupts(&self) -> u16;
    fn offset_interrupts(&self) -> u16;

    fn num_relocations(&self) -> u32;
    fn offset_relocation(&self) -> u16;

    fn num_dependencies(&self) -> u16;
    fn offset_dependencies(&self) -> u16;

    fn offset_trailer(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}

pub trait CbfHeaderMain<'a> {
    fn component_priority(&self) -> u16;
    fn component_flags(&self) -> ComponentFlags;
    fn component_min_ram(&self) -> u32;

    fn entry_point_offset(&self) -> u32;

    fn data_offset(&self) -> u32;
    fn databss_size(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}

pub trait CbfHeaderRegion<'a> {
    fn base_address(&self) -> u32;
    fn size(&self) -> u32;
    fn attributes(&self) -> RegionAttributes;

    fn get_raw(&self) -> &'a [u8];
}

pub trait CbfHeaderInterrupt<'a> {
    fn irq_number(&self) -> u32;
    fn notification_mask(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}

pub trait CbfHeaderRelocation<'a> {
    fn value(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}

pub trait CbfHeaderDependency<'a> {
    fn component_id(&self) -> u16;
    fn min_version(&self) -> u32;
    fn max_version(&self) -> u32;

    fn get_raw(&self) -> &'a [u8];
}

/*
    Wrappers
*/
// Base
pub struct CbfHeaderBaseWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderBase<'a>,
}
impl<'a> CbfHeaderBaseWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderBase<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderBaseWrapper<'a> {
    type Target = dyn CbfHeaderBase<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderBaseWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Base")
            .field("Version", &self.cbf_version())
            .field("Total Size", &self.total_size())
            .field("Component ID", &self.component_id())
            .field("Component Version", &self.component_version())
            .field("Region Count", &self.num_regions())
            .field("Interrupt Count", &self.num_interrupts())
            .field("Relocation Count", &self.num_relocations())
            .finish()
    }
}
// Main
pub struct CbfHeaderMainWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderMain<'a>,
}
impl<'a> CbfHeaderMainWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderMain<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderMainWrapper<'a> {
    type Target = dyn CbfHeaderMain<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderMainWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Main")
            .field("Priority", &self.component_priority())
            .field("Flags", &self.component_flags())
            .field("RAM", &self.component_min_ram())
            .finish()
    }
}

// Region
pub struct CbfHeaderRegionWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderRegion<'a>,
}
impl<'a> CbfHeaderRegionWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderRegion<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderRegionWrapper<'a> {
    type Target = dyn CbfHeaderRegion<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderRegionWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Region")
            .field(
                "Base Address",
                &format_args!("{:#010x}", &self.base_address()),
            )
            .field("Size", &self.size())
            .field("Attributes", &self.attributes())
            .finish()
    }
}

// Interrupt
pub struct CbfHeaderInterruptWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderInterrupt<'a>,
}
impl<'a> CbfHeaderInterruptWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderInterrupt<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderInterruptWrapper<'a> {
    type Target = dyn CbfHeaderInterrupt<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderInterruptWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Interrupt")
            .field("IRQ", &self.irq_number())
            .field(
                "Notification Mask",
                &format_args!("{:#032b}", &self.notification_mask()),
            )
            .finish()
    }
}

// Relocations
pub struct CbfHeaderRelocationWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderRelocation<'a>,
}
impl<'a> CbfHeaderRelocationWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderRelocation<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderRelocationWrapper<'a> {
    type Target = dyn CbfHeaderRelocation<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderRelocationWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Relocation")
            .field("Value", &format_args!("{:#010x}", &self.value()))
            .finish()
    }
}

// Dependency
pub struct CbfHeaderDependencyWrapper<'a> {
    _cbf_file: &'a dyn CbfFile,
    inner: &'a dyn CbfHeaderDependency<'a>,
}
impl<'a> CbfHeaderDependencyWrapper<'a> {
    pub fn new(cbf_file: &'a dyn CbfFile, inner: &'a dyn CbfHeaderDependency<'a>) -> Self {
        Self {
            _cbf_file: cbf_file,
            inner,
        }
    }
}
impl<'a> Deref for CbfHeaderDependencyWrapper<'a> {
    type Target = dyn CbfHeaderDependency<'a> + 'a;
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}
impl<'a> Debug for CbfHeaderDependencyWrapper<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Dependency")
            .field("Component ID", &self.component_id())
            .field("Min Version", &self.min_version())
            .field("Max Version", &self.max_version())
            .finish()
    }
}

/*
    Iterators
*/
// Region
pub struct CbfHeaderRegionIter<'a> {
    elf_file: &'a dyn CbfFile,
    index: usize,
}
impl<'a> CbfHeaderRegionIter<'a> {
    pub fn new(elf_file: &'a dyn CbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for CbfHeaderRegionIter<'a> {
    type Item = CbfHeaderRegionWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.region_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
// Interrupt
pub struct CbfHeaderInterruptIter<'a> {
    elf_file: &'a dyn CbfFile,
    index: usize,
}
impl<'a> CbfHeaderInterruptIter<'a> {
    pub fn new(elf_file: &'a dyn CbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for CbfHeaderInterruptIter<'a> {
    type Item = CbfHeaderInterruptWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.interrupt_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
// Relocation
pub struct CbfHeaderRelocationIter<'a> {
    elf_file: &'a dyn CbfFile,
    index: usize,
}
impl<'a> CbfHeaderRelocationIter<'a> {
    pub fn new(elf_file: &'a dyn CbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for CbfHeaderRelocationIter<'a> {
    type Item = CbfHeaderRelocationWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.relocation_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
// Dependency
pub struct CbfHeaderDependencyIter<'a> {
    elf_file: &'a dyn CbfFile,
    index: usize,
}
impl<'a> CbfHeaderDependencyIter<'a> {
    pub fn new(elf_file: &'a dyn CbfFile) -> Self {
        Self { elf_file, index: 0 }
    }
}
impl<'a> Iterator for CbfHeaderDependencyIter<'a> {
    type Item = CbfHeaderDependencyWrapper<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.elf_file.dependency_nth(self.index).map(|e| {
            self.index += 1;
            e
        })
    }
}
