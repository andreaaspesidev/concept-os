use super::{HbfHeaderBase, HbfHeaderMain, HbfHeaderRegion, HbfHeaderInterrupt, HbfHeaderRelocation};


#[repr(packed, C)]
pub struct HbfHeaderBaseGen {
    magic_number: u32,          // 0
    version: u16,               // 4
    total_size: u32,            // 6
    component_id: u16,          // 10
    component_version: u32,     // 12
    main_offset: u16,           // 16
    region_offset: u16,         // 18
    region_count: u16,          // 20
    interrupt_offset: u16,      // 22
    interrupt_count: u16,       // 24
    relocation_offset: u16,     // 26
    relocation_count: u32,      // 28
    checksum: u32               // 32
}

impl HbfHeaderBase for HbfHeaderBaseGen {
    fn hbf_version(&self) -> super::HbfVersion {
        let p = core::ptr::addr_of!(self.version);
        unsafe {p.read_unaligned()}.into()
    }
    fn total_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.total_size);
        unsafe {p.read_unaligned()}
    }

    fn component_id(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_id);
        unsafe {p.read_unaligned()}
    }
    fn component_version(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_version);
        unsafe {p.read_unaligned()}
    }

    fn offset_main(&self) -> u16 {
        let p = core::ptr::addr_of!(self.main_offset);
        unsafe {p.read_unaligned()}
    }

    fn num_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_count);
        unsafe {p.read_unaligned()}
    }
    fn offset_regions(&self) -> u16 {
        let p = core::ptr::addr_of!(self.region_offset);
        unsafe {p.read_unaligned()}
    }

    fn num_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_count);
        unsafe {p.read_unaligned()}
    }
    fn offset_interrupts(&self) -> u16 {
        let p = core::ptr::addr_of!(self.interrupt_offset);
        unsafe {p.read_unaligned()}
    }

    fn num_relocations(&self) -> u32 {
        let p = core::ptr::addr_of!(self.relocation_count);
        unsafe {p.read_unaligned()}
    }
    fn offset_relocation(&self) -> u16 {
        let p = core::ptr::addr_of!(self.relocation_offset);
        unsafe {p.read_unaligned()}
    }

    fn checksum(&self) -> u32 {
        let p = core::ptr::addr_of!(self.checksum);
        unsafe {p.read_unaligned()}
    }
}

#[repr(packed, C)]
pub struct HbfHeaderMainGen {
    component_priority: u16,            // 0
    component_flags: u16,               // 2
    component_min_ram: u32,             // 4
    entry_point_offset: u32,            // 8
    data_section_offset: u32,           // 12
    data_section_size: u32              // 16
}

impl HbfHeaderMain for HbfHeaderMainGen {
    fn component_priority(&self) -> u16 {
        let p = core::ptr::addr_of!(self.component_priority);
        unsafe {p.read_unaligned()}
    }

    fn component_flags(&self) -> super::ComponentFlags {
        let p = core::ptr::addr_of!(self.component_flags);
        unsafe {super::ComponentFlags::from_bits_truncate(p.read_unaligned())}
    }

    fn component_min_ram(&self) -> u32 {
        let p = core::ptr::addr_of!(self.component_min_ram);
        unsafe {p.read_unaligned()}
    }

    fn entry_point_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.entry_point_offset);
        unsafe {p.read_unaligned()}
    }


    fn data_offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_offset);
        unsafe {p.read_unaligned()}
    }

    fn databss_size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.data_section_size);
        unsafe {p.read_unaligned()}
    }
}

#[repr(packed, C)]
pub struct HbfHeaderRegionGen {
    region_base_address: u32,               // 0
    region_size: u32,                       // 4
    region_attributes: u32                  // 8
}

impl HbfHeaderRegion for HbfHeaderRegionGen {
    fn base_address(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_base_address);
        unsafe {p.read_unaligned()}
    }

    fn size(&self) -> u32 {
        let p = core::ptr::addr_of!(self.region_size);
        unsafe {p.read_unaligned()}
    }

    fn attributes(&self) -> super::RegionAttributes {
        let p = core::ptr::addr_of!(self.region_attributes);
        unsafe {super::RegionAttributes::from_bits_truncate(p.read_unaligned())}
    }
}

#[repr(packed, C)]
pub struct HbfHeaderInterruptGen {
    irq_number: u32,         //0
    notification_mask: u32   //4
}

impl HbfHeaderInterrupt for HbfHeaderInterruptGen {
    fn irq_number(&self) -> u32 {
        let p = core::ptr::addr_of!(self.irq_number);
        unsafe {p.read_unaligned()}
    }

    fn notification_mask(&self) -> u32 {
        let p = core::ptr::addr_of!(self.notification_mask);
        unsafe {p.read_unaligned()}
    }
}

#[repr(packed, C)]
pub struct HbfHeaderRelocationGen {
    address_offset: u32     //0
}

impl HbfHeaderRelocation for HbfHeaderRelocationGen {
    fn offset(&self) -> u32 {
        let p = core::ptr::addr_of!(self.address_offset);
        unsafe {p.read_unaligned()}
    }
}

