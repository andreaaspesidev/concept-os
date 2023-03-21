// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{
    CbfHeaderBase, CbfHeaderDependency, CbfHeaderInterrupt, CbfHeaderMain, CbfHeaderRegion,
    CbfHeaderRelocation,
};

#[repr(packed, C)]
pub struct CbfHeaderBaseGen {
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

impl<'a> CbfHeaderBase<'a> for CbfHeaderBaseGen {
    fn cbf_version(&self) -> super::CbfVersion {
        let p = core::ptr::addr_of!(self.version);
        unsafe { p.read_unaligned() }.into()
    }
    fn total_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.total_size);
        unsafe { p.read_unaligned() }
    }

    fn component_id(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_id);
        unsafe { p.read_unaligned() }
    }
    fn component_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_version);
        unsafe { p.read_unaligned() }
    }

    fn padding_bytes(&self) -> u16 {
        let p = core::ptr::addr_of!(self.padding_bytes);
        unsafe { p.read_unaligned() }
    }

    fn num_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_count);
        unsafe { p.read_unaligned() }
    }
    fn offset_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_offset);
        unsafe { p.read_unaligned() }
    }

    fn num_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_count);
        unsafe { p.read_unaligned() }
    }
    fn offset_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_offset);
        unsafe { p.read_unaligned() }
    }

    fn num_relocations(&self) -> u32 {
        let p = core::ptr::addr_of!(self.relocation_count);
        unsafe { p.read_unaligned() }
    }
    fn offset_relocation(&self) -> u16 {
        let p = core::ptr::addr_of!(self.relocation_offset);
        unsafe { p.read_unaligned() }
    }

    fn num_dependencies(&self) -> u16 {
        let p = core::ptr::addr_of!(self.dependencies_count);
        unsafe { p.read_unaligned() }
    }
    fn offset_dependencies(&self) -> u16 {
        let p = core::ptr::addr_of!(self.dependencies_offset);
        unsafe { p.read_unaligned() }
    }

    fn offset_trailer(&self) -> u32 {
        let p = core::ptr::addr_of!(self.trailer_offset);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfHeaderBaseGen>(),
            )
        }
    }
}

#[repr(packed, C)]
pub struct CbfHeaderMainGen {
    component_priority: u16,  // 0
    component_flags: u16,     // 2
    component_min_ram: u32,   // 4
    entry_point_offset: u32,  // 8
    data_section_offset: u32, // 12
    data_section_size: u32,   // 16
}

impl<'a> CbfHeaderMain<'a> for CbfHeaderMainGen {
    fn component_priority(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_priority);
        unsafe { p.read_unaligned() }
    }

    fn component_flags(&self) -> super::ComponentFlags {
        let p = core::ptr::addr_of!(self.component_flags);
        unsafe { super::ComponentFlags::from_bits_truncate(p.read_unaligned()) }
    }

    fn component_min_ram(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_min_ram);
        unsafe { p.read_unaligned() }
    }

    fn entry_point_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.entry_point_offset);
        unsafe { p.read_unaligned() }
    }

    fn data_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_offset);
        unsafe { p.read_unaligned() }
    }

    fn databss_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_size);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfHeaderMainGen>(),
            )
        }
    }
}

#[repr(packed, C)]
pub struct CbfHeaderRegionGen {
    region_base_address: u32, // 0
    region_size: u32,         // 4
    region_attributes: u32,   // 8
}

impl<'a> CbfHeaderRegion<'a> for CbfHeaderRegionGen {
    fn base_address(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_base_address);
        unsafe { p.read_unaligned() }
    }

    fn size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_size);
        unsafe { p.read_unaligned() }
    }

    fn attributes(&self) -> super::RegionAttributes {
        let p = core::ptr::addr_of!(self.region_attributes);
        unsafe { super::RegionAttributes::from_bits_truncate(p.read_unaligned()) }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfHeaderRegionGen>(),
            )
        }
    }
}

#[repr(packed, C)]
pub struct CbfHeaderInterruptGen {
    irq_number: u32,        //0
    notification_mask: u32, //4
}

impl<'a> CbfHeaderInterrupt<'a> for CbfHeaderInterruptGen {
    fn irq_number(&self) -> u32 {
        let p = core::ptr::addr_of!(self.irq_number);
        unsafe { p.read_unaligned() }
    }

    fn notification_mask(&self) -> u32 {
        let p = core::ptr::addr_of!(self.notification_mask);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfHeaderInterruptGen>(),
            )
        }
    }
}

#[repr(packed, C)]
pub struct CbfHeaderRelocationGen {
    address_offset: u32, //0
}

impl<'a> CbfHeaderRelocation<'a> for CbfHeaderRelocationGen {
    fn value(&self) -> u32 {
        let p = core::ptr::addr_of!(self.address_offset);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                self as *const Self as *const u8,
                core::mem::size_of::<CbfHeaderRelocationGen>(),
            )
        }
    }
}

#[repr(packed, C)]
pub struct CbfHeaderDependencyGen {
    component_id: u32, //0
    min_version: u32,  //4
    max_version: u32,  //8
}

impl<'a> CbfHeaderDependency<'a> for CbfHeaderDependencyGen {
    fn component_id(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_id);
        (unsafe { p.read_unaligned() }) as u16
    }

    fn min_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.min_version);
        unsafe { p.read_unaligned() }
    }

    fn max_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.max_version);
        unsafe { p.read_unaligned() }
    }

    fn get_raw(&self) -> &'a [u8] {
        unsafe {
            core::slice::from_raw_parts(
                (self as *const Self) as *const u8,
                core::mem::size_of::<Self>(),
            )
        }
    }
}
