use core::fmt::{Debug, Error, Formatter};

/**
 * Defines
 */
pub const HBF_MAGIC: [u8; 4] = [0x7f, b'H', b'B', b'F'];

pub const HBF_CHECKSUM_OFFSET: u32 = 0x20;

pub const HBF_HEADER_MIN_SIZE: usize = core::mem::size_of::<HbfHeaderBase>();
pub const FIXED_HEADER_SIZE: usize = core::mem::size_of::<HbfHeaderBase>()
    + core::mem::size_of::<HbfHeaderMain>();
pub const REGION_SIZE: usize = core::mem::size_of::<HbfHeaderRegion>();
pub const INTERRUPT_SIZE: usize = core::mem::size_of::<HbfHeaderInterrupt>();
pub const RELOC_SIZE: usize = core::mem::size_of::<HbfHeaderRelocation>();

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
    V1, // 0x0001
    UNKNOWN(u16),
}
impl From<u16> for HbfVersion {
    fn from(n: u16) -> Self {
        match n {
            1 => HbfVersion::V1,
            n => HbfVersion::UNKNOWN(n),
        }
    }
}

/**
 * Structures
 */

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct HbfHeaderBase {
    magic_number: u32,      // 0
    version: u16,           // 4
    total_size: u32,        // 6
    component_id: u16,      // 10
    component_version: u32, // 12
    main_offset: u16,       // 16
    region_offset: u16,     // 18
    region_count: u16,      // 20
    interrupt_offset: u16,  // 22
    interrupt_count: u16,   // 24
    relocation_offset: u16, // 26
    relocation_count: u32,  // 28
    checksum: u32,          // 32
}

impl<'a> HbfHeaderBase {
    pub fn hbf_version(&self) -> HbfVersion {
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

    pub fn offset_main(&self) -> u16 {
        let p = core::ptr::addr_of!(self.main_offset);
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

impl Debug for HbfHeaderBase {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Base")
            .field("Version", &self.hbf_version())
            .field("Total Size", &self.total_size())
            .field("Component ID", &self.component_id())
            .field("Component Version", &self.component_version())
            .field("Region Count", &self.num_regions())
            .field("Interrupt Count", &self.num_interrupts())
            .field("Relocation Count", &self.num_relocations())
            .field("Checksum", &format_args!("{:#010x}", &self.checksum()))
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct HbfHeaderMain {
    component_priority: u16,  // 0
    component_flags: u16,     // 2
    component_min_ram: u32,   // 4
    entry_point_offset: u32,  // 8
    data_section_offset: u32, // 12
    data_section_size: u32,   // 16
}

impl<'a> HbfHeaderMain {
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

impl Debug for HbfHeaderMain {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Main")
            .field("Priority", &self.component_priority())
            .field("Flags", &self.component_flags())
            .field("RAM", &self.component_min_ram())
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(packed, C)]
pub struct HbfHeaderRegion {
    region_base_address: u32, // 0
    region_size: u32,         // 4
    region_attributes: u32,   // 8
}

impl<'a> HbfHeaderRegion {
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

impl Debug for HbfHeaderRegion {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Region")
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
pub struct HbfHeaderInterrupt {
    irq_number: u32,        //0
    notification_mask: u32, //4
}

impl<'a> HbfHeaderInterrupt {
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

impl Debug for HbfHeaderInterrupt {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Interrupt")
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
pub struct HbfHeaderRelocation {
    address_offset: u32, //0
}

impl<'a> HbfHeaderRelocation {
    pub fn offset(&self) -> u32 {
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

impl Debug for HbfHeaderRelocation {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf Relocation")
            .field("Offset", &format_args!("{:#010x}", &self.offset()))
            .finish()
    }
}