// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "fmt")]
use core::fmt::{Debug, Error, Formatter};

/**
 * Defines
 */
pub const CBF_MAGIC: [u8; 4] = [0x7f, b'C', b'B', b'F'];

pub const CBF_HEADER_MIN_SIZE: usize = core::mem::size_of::<CbfHeaderBase>();
pub const FIXED_HEADER_SIZE: usize =
    core::mem::size_of::<CbfHeaderBase>() + core::mem::size_of::<CbfHeaderMain>();
pub const REGION_SIZE: usize = core::mem::size_of::<CbfHeaderRegion>();
pub const INTERRUPT_SIZE: usize = core::mem::size_of::<CbfHeaderInterrupt>();
pub const RELOC_SIZE: usize = core::mem::size_of::<CbfHeaderRelocation>();
pub const DEPENDENCY_SIZE: usize = core::mem::size_of::<CbfHeaderDependency>();


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

/**
 * Structures
 */

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderBase {
    magic_number: u32,        // 0
    version: u16,             // 4
    total_size: u32,          // 6
    component_id: u16,        // 10
    component_version: u32,   // 12
    padding_bytes: u16,       // 16
    region_offset: u16,       // 18
    region_count: u16,        // 20
    interrupt_offset: u16,    // 22
    interrupt_count: u16,     // 24
    relocation_offset: u16,   // 26
    relocation_count: u32,    // 28
    dependencies_offset: u16, // 32
    dependencies_count: u16,  // 34
    trailer_offset: u32,      // 36
}

impl<'a> CbfHeaderBase {
    pub fn cbf_version(&self) -> CbfVersion {
        let p = core::ptr::addr_of!(self.version);
        unsafe { p.read_unaligned() }.into()
    }
    pub fn total_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.total_size);
        unsafe { p.read_unaligned() }
    }

    pub fn component_id(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_id);
        unsafe { p.read_unaligned() }
    }
    pub fn component_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_version);
        unsafe { p.read_unaligned() }
    }

    pub fn padding_bytes(&self) -> u16 {
        let p = core::ptr::addr_of!(self.padding_bytes);
        unsafe { p.read_unaligned() }
    }

    pub fn num_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_count);
        unsafe { p.read_unaligned() }
    }
    pub fn offset_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn num_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_count);
        unsafe { p.read_unaligned() }
    }
    pub fn offset_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn num_relocations(&self) -> u32 {
        let p = core::ptr::addr_of!(self.relocation_count);
        unsafe { p.read_unaligned() }
    }
    pub fn offset_relocation(&self) -> u16 {
        let p = core::ptr::addr_of!(self.relocation_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn num_dependencies(&self) -> u16 {
        let p = core::ptr::addr_of!(self.dependencies_count);
        unsafe { p.read_unaligned() }
    }
    pub fn offset_dependencies(&self) -> u16 {
        let p = core::ptr::addr_of!(self.dependencies_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn trailer_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.trailer_offset);
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
impl Debug for CbfHeaderBase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Base")
            .field("Version", &self.cbf_version())
            .field("Total Size", &self.total_size())
            .field("Component ID", &self.component_id())
            .field("Component Version", &self.component_version())
            .field("Region Count", &self.num_regions())
            .field("Interrupt Count", &self.num_interrupts())
            .field("Relocation Count", &self.num_relocations())
            .field("Dependencies Count", &self.num_dependencies())
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderMain {
    component_priority: u16,  // 0
    component_flags: u16,     // 2
    component_min_ram: u32,   // 4
    entry_point_offset: u32,  // 8
    data_section_offset: u32, // 12
    data_section_size: u32,   // 16
}

impl<'a> CbfHeaderMain {
    pub fn component_priority(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_priority);
        unsafe { p.read_unaligned() }
    }

    pub fn component_flags(&self) -> ComponentFlags {
        let p = core::ptr::addr_of!(self.component_flags);
        unsafe { ComponentFlags::from_bits_truncate(p.read_unaligned()) }
    }

    pub fn component_min_ram(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_min_ram);
        unsafe { p.read_unaligned() }
    }

    pub fn entry_point_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.entry_point_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn data_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_offset);
        unsafe { p.read_unaligned() }
    }

    pub fn databss_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_size);
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
impl Debug for CbfHeaderMain {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Main")
            .field("Priority", &self.component_priority())
            .field("Flags", &self.component_flags())
            .field("RAM", &self.component_min_ram())
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderRegion {
    region_base_address: u32, // 0
    region_size: u32,         // 4
    region_attributes: u32,   // 8
}

impl<'a> CbfHeaderRegion {
    pub fn base_address(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_base_address);
        unsafe { p.read_unaligned() }
    }

    pub fn size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_size);
        unsafe { p.read_unaligned() }
    }

    pub fn attributes(&self) -> RegionAttributes {
        let p = core::ptr::addr_of!(self.region_attributes);
        unsafe { RegionAttributes::from_bits_truncate(p.read_unaligned()) }
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
impl Debug for CbfHeaderRegion {
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

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderInterrupt {
    irq_number: u32,        //0
    notification_mask: u32, //4
}

impl<'a> CbfHeaderInterrupt {
    pub fn irq_number(&self) -> u32 {
        let p = core::ptr::addr_of!(self.irq_number);
        unsafe { p.read_unaligned() }
    }

    pub fn notification_mask(&self) -> u32 {
        let p = core::ptr::addr_of!(self.notification_mask);
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
impl Debug for CbfHeaderInterrupt {
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

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderRelocation {
    address_offset: u32, //0
}

impl<'a> CbfHeaderRelocation {
    pub fn value(&self) -> u32 {
        let p = core::ptr::addr_of!(self.address_offset);
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
impl Debug for CbfHeaderRelocation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Relocation")
            .field("Value", &format_args!("{:#010x}", &self.value()))
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct CbfHeaderDependency {
    component_id: u32, //0
    min_version: u32,  //4
    max_version: u32,  //8
}

impl<'a> CbfHeaderDependency {
    pub fn component_id(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_id);
        (unsafe { p.read_unaligned() }) as u16
    }

    pub fn min_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.min_version);
        unsafe { p.read_unaligned() }
    }

    pub fn max_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.max_version);
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
impl Debug for CbfHeaderDependency {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Cbf Dependency")
            .field("Component ID", &self.component_id())
            .field("Min Version", &self.min_version())
            .field("MaxVersion", &self.max_version())
            .finish()
    }
}
