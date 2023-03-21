// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::error::Error;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use component_config::structures::ComponentConfig;
use component_config::structures::ComponentFlag as CF;
use component_config::structures::RegionAttribute as RF;

use crate::parse_elf::ElfSection;
use crate::relocations::RelocationPoint;

const ALIGN_TO: usize = 8;

/*
  ----------
    Traits
  ----------
*/
trait Validable {
    /// Checks if this instance contains a valid structure
    fn is_valid(&self) -> bool;
}

trait Sizeable {
    fn size(&self) -> u32;
}

trait Serializable {
    fn as_bytes(&self) -> Vec<u8>;
}

/*
  ----------
    Flags
  ----------
*/
bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ComponentFlags: u16 {
        const NONE = 0;
        const START_AT_BOOT = 1 << 0;
        const RESERVED = !1;
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

        const RESERVED = !((1 << 5) - 1);
    }
}

/*
  ----------
    Enums
  ----------
*/
#[repr(u16)]
#[derive(Clone, Debug, Copy)]
#[allow(dead_code)]
enum CbfVersionType {
    V1 = 1,
}

/*
  --------------
    Structures
  --------------
*/
pub struct CbfFile {
    header: CbfHeader,
    payload: CbfPayload,
    trailer: CbfTrailer
}

struct CbfHeader {
    base: CbfHeaderBase,
    main: CbfHeaderMain,
    regions: Option<Vec<CbfHeaderRegion>>,
    interrupts: Option<Vec<CbfHeaderInterrupt>>,
    relocations: Option<Vec<CbfHeaderRelocation>>,
    dependencies: Option<Vec<CbfHeaderDependency>>,
}

struct CbfHeaderBase {
    magic_number: u32,       // 0
    version: CbfVersionType, // 4
    total_size: u32,         // 6
    component_id: u16,       // 10
    component_version: u32,  // 12
    padding_bytes: u16,      // 16
    region_offset: u16,      // 18
    region_count: u16,       // 20
    interrupt_offset: u16,   // 22
    interrupt_count: u16,    // 24
    relocation_offset: u16,  // 26
    relocation_count: u32,   // 28
    dependency_offset: u16,  // 32
    dependency_count: u16,   // 34
    trailer_offset: u32,     // 36
}

struct CbfHeaderMain {
    component_priority: u16,         // 0
    component_flags: ComponentFlags, // 2
    component_min_ram: u32,          // 4
    entry_point_offset: u32,         // 8
    data_section_offset: u32,        // 12
    data_section_size: u32,          // 16
}

struct CbfHeaderRegion {
    region_base_address: u32,            // 0
    region_size: u32,                    // 4
    region_attributes: RegionAttributes, // 8
}

struct CbfHeaderInterrupt {
    irq_number: u32,        //0
    notification_mask: u32, //4
}

struct CbfHeaderRelocation {
    point: RelocationPoint, //0
}

struct CbfHeaderDependency {
    component_id: u32, //0
    min_version: u32,  //4
    max_version: u32,  //8
}

struct CbfPayload {
    text_section: Vec<u8>,
    rodata_section: Option<Vec<u8>>,
    data_section: Option<Vec<u8>>,
}


struct CbfTrailer {
    checksum: u32,         // 0
}

/*
  -------------------
    Implementations
  -------------------
*/
impl Validable for CbfHeaderBase {
    fn is_valid(&self) -> bool {
        if self.total_size == 0 {
            return false;
        }
        if self.component_id == 0 {
            return false;
        }
        if self.component_version == 0 {
            return false;
        }
        if self.region_count > 0 && self.region_offset == 0 {
            return false;
        }
        if self.region_count == 0 && self.region_offset > 0 {
            return false;
        }
        if self.interrupt_count > 0 && self.interrupt_offset == 0 {
            return false;
        }
        if self.interrupt_count == 0 && self.interrupt_offset > 0 {
            return false;
        }
        if self.relocation_count > 0 && self.relocation_offset == 0 {
            return false;
        }
        if self.relocation_count == 0 && self.relocation_offset > 0 {
            return false;
        }
        if self.dependency_count > 0 && self.dependency_offset == 0 {
            return false;
        }
        if self.dependency_count == 0 && self.dependency_offset > 0 {
            return false;
        }
        if self.trailer_offset == 0 {
            return false;
        }
        return true;
    }
}
impl Validable for CbfHeaderMain {
    fn is_valid(&self) -> bool {
        if self.component_flags == ComponentFlags::RESERVED {
            return false;
        }
        if self.component_min_ram == 0 {
            return false;
        }
        if self.entry_point_offset == 0 {
            return false;
        }
        if self.entry_point_offset % 4 != 0 {
            // Alignment constraint
            return false;
        }
        if self.data_section_size > 0 && self.data_section_offset == 0 {
            return false;
        }
        if self.data_section_size == 0 && self.data_section_offset > 0 {
            return false;
        }
        return true;
    }
}
impl Validable for CbfHeaderRegion {
    fn is_valid(&self) -> bool {
        if self.region_size == 0 {
            return false;
        }
        //TODO: further check on address space?
        if self.region_base_address == 0 {
            return false;
        }
        if self.region_attributes == RegionAttributes::RESERVED {
            return false;
        }
        return true;
    }
}
impl Validable for CbfHeaderInterrupt {
    fn is_valid(&self) -> bool {
        if self.notification_mask.count_ones() != 1 {
            return false;
        }
        return true;
    }
}
impl Validable for CbfHeaderDependency {
    fn is_valid(&self) -> bool {
        if self.component_id == 0 {
            return false;
        }
        return true;
    }
}
impl Validable for CbfHeader {
    fn is_valid(&self) -> bool {
        // Cascaded checks
        if !self.base.is_valid() {
            return false;
        }
        if !self.main.is_valid() {
            return false;
        }
        if self.regions.is_some() {
            let regions = (&self.regions).as_ref().unwrap();
            for r in regions.iter() {
                if !r.is_valid() {
                    return false;
                }
            }
        }
        if self.interrupts.is_some() {
            let interrupts = (&self.interrupts).as_ref().unwrap();
            for i in interrupts.iter() {
                if !i.is_valid() {
                    return false;
                }
            }
        }
        if self.dependencies.is_some() {
            let dependencies = (&self.dependencies).as_ref().unwrap();
            for r in dependencies.iter() {
                if !r.is_valid() {
                    return false;
                }
            }
        }
        // check size
        if self.base.total_size <= core::mem::size_of::<CbfHeader> as u32 {
            return false;
        }
        // check offset jumps
        if self.base.region_offset != 0
            && self.base.region_offset <= core::mem::size_of::<CbfHeaderBase> as u16
        {
            return false;
        }
        if self.base.interrupt_offset != 0
            && self.base.interrupt_offset <= core::mem::size_of::<CbfHeaderBase> as u16
        {
            return false;
        }
        if self.base.relocation_offset != 0
            && self.base.relocation_offset <= core::mem::size_of::<CbfHeaderBase> as u16
        {
            return false;
        }
        if self.base.dependency_offset != 0
            && self.base.dependency_offset <= core::mem::size_of::<CbfHeaderBase> as u16
        {
            return false;
        }
        if self.main.entry_point_offset <= core::mem::size_of::<CbfHeader> as u32 {
            return false;
        }
        if self.main.data_section_offset != 0
            && self.main.data_section_offset <= core::mem::size_of::<CbfHeader> as u32
        {
            return false;
        }
        return true;
    }
}
impl Validable for CbfPayload {
    fn is_valid(&self) -> bool {
        //TODO: alignment checks
        if (&self.text_section).is_empty() {
            return false;
        }
        return true;
    }
}
impl Validable for CbfTrailer {
    fn is_valid(&self) -> bool {
        if self.checksum == 0 {
            return false;
        }
        return true;
    }
}
impl Validable for CbfFile {
    fn is_valid(&self) -> bool {
        if !self.header.is_valid() {
            return false;
        }
        if !self.payload.is_valid() {
            return false;
        }
        if !self.trailer.is_valid() {
            return false;
        }
        //TODO: checks on pointers of relocations, Entry Point Offset, Data Section Offset, Data Section Size
        return true;
    }
}

impl Sizeable for CbfHeaderBase {
    fn size(&self) -> u32 {
        return 40;
    }
}
impl Sizeable for CbfHeaderMain {
    fn size(&self) -> u32 {
        return 20;
    }
}
impl Sizeable for CbfHeaderRegion {
    fn size(&self) -> u32 {
        return 12;
    }
}
impl Sizeable for CbfHeaderInterrupt {
    fn size(&self) -> u32 {
        return 8;
    }
}
impl Sizeable for CbfHeaderRelocation {
    fn size(&self) -> u32 {
        return 4;
    }
}
impl Sizeable for CbfHeaderDependency {
    fn size(&self) -> u32 {
        return 12;
    }
}
impl Sizeable for CbfHeader {
    fn size(&self) -> u32 {
        let mut size: u32 = 0;
        // base
        size += self.base.size();
        // main
        size += self.main.size();
        // regions
        if self.regions.is_some() {
            let regs = (&self.regions).as_ref().unwrap();
            if regs.len() == 0 {
                panic!("Empty region array");
            }
            size += regs[0].size() * (regs.len() as u32);
        }
        // interrupts
        if self.interrupts.is_some() {
            let ints = (&self.interrupts).as_ref().unwrap();
            if ints.len() == 0 {
                panic!("Empty interrupt array");
            }
            size += ints[0].size() * (ints.len() as u32);
        }
        // relocs
        if self.relocations.is_some() {
            let rels = (&self.relocations).as_ref().unwrap();
            if rels.len() == 0 {
                panic!("Empty relocation array");
            }
            size += rels[0].size() * (rels.len() as u32);
        }
        // dependencies
        if self.dependencies.is_some() {
            let deps = (&self.dependencies).as_ref().unwrap();
            if deps.len() == 0 {
                panic!("Empty dependency array");
            }
            size += deps[0].size() * (deps.len() as u32);
        }
        // Padding bytes
        return size + self.base.padding_bytes as u32;
    }
}
impl Sizeable for CbfPayload {
    fn size(&self) -> u32 {
        let mut size: u32 = 0;
        size += self.text_section.len() as u32;
        if self.rodata_section.is_some() {
            let rodata = (&self.rodata_section).as_ref().unwrap();
            size += rodata.len() as u32;
        }
        if self.data_section.is_some() {
            let data = (&self.data_section).as_ref().unwrap();
            size += data.len() as u32;
        }
        return size;
    }
}
impl Sizeable for CbfTrailer {
    fn size(&self) -> u32 {
        return 4;
    }
}
impl Sizeable for CbfFile {
    fn size(&self) -> u32 {
        return self.header.size() + self.payload.size() + self.trailer.size();
    }
}

impl Serializable for CbfHeaderBase {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            magic_number: u32,          // 0
            version: CbfVersionType,    // 4
            total_size: u32,            // 6
            component_id: u16,          // 10
            component_version: u32,     // 12
            padding_bytes: u16,         // 16
            region_offset: u16,         // 18
            region_count: u16,          // 20
            interrupt_offset: u16,      // 22
            interrupt_count: u16,       // 24
            relocation_offset: u16,     // 26
            relocation_count: u32,      // 28
            dependency_offset: u16,     // 32
            dependency_count: u16,      // 34
            trailer_offset: u32         // 36
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.magic_number.to_le_bytes());
        buffer.extend_from_slice(&(self.version as u16).to_le_bytes());
        buffer.extend_from_slice(&self.total_size.to_le_bytes());
        buffer.extend_from_slice(&self.component_id.to_le_bytes());
        buffer.extend_from_slice(&self.component_version.to_le_bytes());
        buffer.extend_from_slice(&self.padding_bytes.to_le_bytes());
        buffer.extend_from_slice(&self.region_offset.to_le_bytes());
        buffer.extend_from_slice(&self.region_count.to_le_bytes());
        buffer.extend_from_slice(&self.interrupt_offset.to_le_bytes());
        buffer.extend_from_slice(&self.interrupt_count.to_le_bytes());
        buffer.extend_from_slice(&self.relocation_offset.to_le_bytes());
        buffer.extend_from_slice(&self.relocation_count.to_le_bytes());
        buffer.extend_from_slice(&self.dependency_offset.to_le_bytes());
        buffer.extend_from_slice(&self.dependency_count.to_le_bytes());
        buffer.extend_from_slice(&self.trailer_offset.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeaderMain {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            component_priority: u16,            // 0
            component_flags: u16,               // 2
            component_min_ram: u32,             // 4
            entry_point_offset: u32,            // 8
            data_section_offset: u32,           // 12
            data_section_size: u32              // 16
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.component_priority.to_le_bytes());
        buffer.extend_from_slice(&self.component_flags.bits.to_le_bytes());
        buffer.extend_from_slice(&self.component_min_ram.to_le_bytes());
        buffer.extend_from_slice(&self.entry_point_offset.to_le_bytes());
        buffer.extend_from_slice(&self.data_section_offset.to_le_bytes());
        buffer.extend_from_slice(&self.data_section_size.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeaderRegion {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            region_base_address: u32,               // 0
            region_size: u32,                       // 4
            region_attributes: RegionAttributes     // 8
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.region_base_address.to_le_bytes());
        buffer.extend_from_slice(&self.region_size.to_le_bytes());
        buffer.extend_from_slice(&self.region_attributes.bits.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeaderInterrupt {
    /*
        irq_number: u32,         //0
        notification_mask: u32   //4
    */
    fn as_bytes(&self) -> Vec<u8> {
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.irq_number.to_le_bytes());
        buffer.extend_from_slice(&self.notification_mask.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeaderRelocation {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            address_offset: u32     //0
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.point.encode().to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeaderDependency {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            component_id: u32,     //0
            min_version: u32,      //4
            max_version: u32       //8
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.component_id.to_le_bytes());
        buffer.extend_from_slice(&self.min_version.to_le_bytes());
        buffer.extend_from_slice(&self.max_version.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfTrailer {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            checksum: u32,          // 0
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(&self.checksum.to_le_bytes());
        return buffer;
    }
}
impl Serializable for CbfHeader {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            base: CbfHeaderBase,
            main: CbfHeaderMain,
            regions: Option<Vec<CbfHeaderRegion>>,
            interrupts: Option<Vec<CbfHeaderInterrupt>>,
            relocations: Option<Vec<CbfHeaderRelocation>>,
            dependencies: Option<Vec<CbfHeaderDependency>>
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend(self.base.as_bytes());
        buffer.extend(self.main.as_bytes());
        if self.regions.is_some() {
            let regs = (&self.regions).as_ref().unwrap();
            for r in regs {
                buffer.extend(r.as_bytes());
            }
        }
        if self.interrupts.is_some() {
            let ints = (&self.interrupts).as_ref().unwrap();
            for int in ints {
                buffer.extend(int.as_bytes());
            }
        }
        if self.relocations.is_some() {
            let rels = (&self.relocations).as_ref().unwrap();
            for rel in rels {
                buffer.extend(rel.as_bytes());
            }
        }
        if self.dependencies.is_some() {
            let deps = (&self.dependencies).as_ref().unwrap();
            for dep in deps {
                buffer.extend(dep.as_bytes());
            }
        }
        // Add padding
        for _ in 0..self.base.padding_bytes {
            buffer.extend([0xFF]);
        }
        return buffer;
    }
}
impl Serializable for CbfPayload {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            text_section: <Vec<u8>,
            rodata_section: Option<Vec<u8>>,
            data_section: Option<Vec<u8>>
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend_from_slice(self.text_section.as_slice());
        if self.rodata_section.is_some() {
            let ro_data = (&self.rodata_section).as_ref().unwrap();
            buffer.extend_from_slice(ro_data.as_slice());
        }
        if self.data_section.is_some() {
            let data = (&self.data_section).as_ref().unwrap();
            buffer.extend_from_slice(data.as_slice());
        }
        return buffer;
    }
}
impl Serializable for CbfFile {
    fn as_bytes(&self) -> Vec<u8> {
        /*
            header: CbfHeader,
            payload: CbfPayload,
            trailer: CbfTrailer
        */
        let mut buffer = Vec::<u8>::new();
        buffer.extend(self.header.as_bytes());
        buffer.extend(self.payload.as_bytes());
        buffer.extend(self.trailer.as_bytes());
        return buffer;
    }
}

const CBF_MAGIC: u32 = 0x464243_7F;
impl CbfFile {
    /// Create a new structure with default initialization
    pub fn new() -> Self {
        Self {
            header: CbfHeader {
                base: CbfHeaderBase {
                    magic_number: CBF_MAGIC,
                    version: CbfVersionType::V1,
                    total_size: 60, // Header only
                    component_id: 0,
                    component_version: 0,
                    padding_bytes: 0,
                    region_offset: 0,
                    region_count: 0,
                    interrupt_offset: 0,
                    interrupt_count: 0,
                    relocation_offset: 0,
                    relocation_count: 0,
                    dependency_offset: 0,
                    dependency_count: 0,
                    trailer_offset: 0,
                },
                main: CbfHeaderMain {
                    component_priority: 0,
                    component_flags: ComponentFlags::NONE,
                    component_min_ram: 0,
                    entry_point_offset: 0,
                    data_section_offset: 0,
                    data_section_size: 0,
                },
                regions: None,
                interrupts: None,
                relocations: None,
                dependencies: None,
            },
            payload: CbfPayload {
                text_section: Vec::new(),
                rodata_section: None,
                data_section: None,
            },
            trailer: CbfTrailer { checksum: 0 }
        }
    }

    fn adjust_padding(&mut self) {
        // Clear current padding
        self.header.base.padding_bytes = 0;
        // Compute new padding
        self.header.base.padding_bytes = (self.header.size() % ALIGN_TO as u32) as u16;
    }

    pub fn initialize_header(&mut self, config: &ComponentConfig) {
        // Copy component data
        self.header.base.component_id = config.component.id;
        self.header.base.component_version = config.component.version;
        self.header.main.component_priority = config.component.priority;
        self.header.main.component_min_ram = config.component.min_ram;
        // Convert component flags
        for flag in &config.component.flags {
            match *flag {
                CF::START_AT_BOOT => {
                    self.header.main.component_flags |= ComponentFlags::START_AT_BOOT;
                }
            }
        }
        // Read regions
        if config.regions.is_some() {
            let mut regs = Vec::<CbfHeaderRegion>::new();
            let conf_regs = (&config.regions).as_ref().unwrap();
            for region in conf_regs.iter() {
                regs.push(CbfHeaderRegion {
                    region_base_address: region.base_address,
                    region_size: region.size,
                    region_attributes: convert_config_region_attributes(&region.attributes),
                });
            }
            self.header.regions = Some(regs);
        }
        // Read interrupts
        if config.interrupts.is_some() {
            let mut ints = Vec::<CbfHeaderInterrupt>::new();
            let conf_ints = (&config.interrupts).as_ref().unwrap();
            for interrupt in conf_ints.iter() {
                ints.push(CbfHeaderInterrupt {
                    irq_number: interrupt.irq,
                    notification_mask: interrupt.notification_mask,
                });
            }
            self.header.interrupts = Some(ints);
        }
        // Read dependencies
        if config.dependencies.is_some() {
            let mut deps = Vec::<CbfHeaderDependency>::new();
            let conf_deps = (&config.dependencies).as_ref().unwrap();
            for dependency in conf_deps.iter() {
                deps.push(CbfHeaderDependency {
                    component_id: dependency.component_id as u32,
                    min_version: dependency.min_version,
                    max_version: dependency.max_version,
                });
            }
            self.header.dependencies = Some(deps);
        }
        // Adjust padding
        self.adjust_padding();
    }

    pub fn add_readonly(
        &mut self,
        text_section: &ElfSection,
        rodata_section: Option<&ElfSection>,
        rel_entrypoint: u32,                        // Base start .text
        text_rels: Option<&Vec<RelocationPoint>>,   // Base start .text
        rodata_rels: Option<&Vec<RelocationPoint>>, // Base start .rodata
        data_rels: Option<&Vec<RelocationPoint>>,   // Base start .data
    ) {
        let mut ro_len: u32 = 0;
        let mut rodata_size: u32 = 0;
        // Append section data
        self.payload.text_section = text_section.content.clone();
        ro_len += text_section.size;
        if rodata_section.is_some() {
            let rodata = rodata_section.unwrap();
            self.payload.rodata_section = Some(rodata.content.clone());
            ro_len += rodata.size; // TODO: is aligned? Check addresses and sizes
            rodata_size = rodata.size;
        }
        // Append relocations
        let mut rels = Vec::<CbfHeaderRelocation>::new();
        if text_rels.is_some() {
            let comp_rels = text_rels.unwrap();
            for r in comp_rels {
                rels.push(CbfHeaderRelocation {
                    point: r.shift(0)
                    //address_offset: text_section.size + r, // MUST BE FIXED in finalize + self.header.size()
                });
            }
        }
        if rodata_rels.is_some() {
            let comp_rels = rodata_rels.unwrap();
            for r in comp_rels {
                rels.push(CbfHeaderRelocation {
                    point: r.shift(text_section.size)
                    //address_offset: text_section.size + r, // MUST BE FIXED in finalize + self.header.size()
                });
            }
        }
        if data_rels.is_some() {
            let comp_rels = data_rels.unwrap();
            for r in comp_rels {
                rels.push(CbfHeaderRelocation {
                    point: r.shift(text_section.size + rodata_size)
                    //address_offset: text_section.size + rodata_size + r,
                });
            }
        }
        if rels.len() > 0 {
            self.header.relocations = Some(rels);
        }

        // Adjust padding
        self.adjust_padding();

        // Set offsets
        self.header.main.data_section_offset = self.header.size() + ro_len;
        self.header.main.entry_point_offset = self.header.size() + rel_entrypoint;
        // Check address aligned
        if self.header.main.entry_point_offset % 4 != 0 {
            panic!("Unaligned entry point address!");
        }
        // Finalize jump
        self.header.main.entry_point_offset += 1; // OR 1: ARM Thumb Mode
    }

    pub fn add_data(&mut self, data_section: Option<&ElfSection>, bss_size: u32) {
        let mut data_size = 0u32;
        // Append section data
        if data_section.is_some() {
            let data = data_section.unwrap();
            self.payload.data_section = Some(data.content.clone());
            data_size = data.size;
        } else {
            self.header.main.data_section_offset = 0;
        }
        // Update sizes
        self.header.main.data_section_size = data_size + bss_size;
    }

    fn finalize(&mut self) {
        // Trailer offset
        self.header.base.trailer_offset = self.size() - self.trailer.size();
        // Region structure
        let mut current_pos: u16 = self.header.base.size() as u16 + self.header.main.size() as u16;
        if self.header.regions.is_some() {
            self.header.base.region_offset = current_pos;
            self.header.base.region_count = (&self.header.regions).as_ref().unwrap().len() as u16;
            let region_size = (&self.header.regions).as_ref().unwrap()[0].size() as u16;
            current_pos += region_size * self.header.base.region_count;
        } else {
            self.header.base.region_offset = 0;
            self.header.base.region_count = 0;
        }
        // Interrupt structure
        if self.header.interrupts.is_some() {
            self.header.base.interrupt_offset = current_pos;
            self.header.base.interrupt_count =
                (&self.header.interrupts).as_ref().unwrap().len() as u16;
            let interr_size = (&self.header.interrupts).as_ref().unwrap()[0].size() as u16;
            current_pos += interr_size * self.header.base.interrupt_count;
        } else {
            self.header.base.interrupt_offset = 0;
            self.header.base.interrupt_count = 0;
        }
        // Relocations structure
        if self.header.relocations.is_some() {
            self.header.base.relocation_offset = current_pos;
            self.header.base.relocation_count =
                (&self.header.relocations).as_ref().unwrap().len() as u32;
            let reloc_size = (&self.header.relocations).as_ref().unwrap()[0].size() as u16;
            current_pos += reloc_size * self.header.base.relocation_count as u16;
        } else {
            self.header.base.relocation_offset = 0;
            self.header.base.relocation_count = 0;
        }
        // Dependencies structure
        if self.header.dependencies.is_some() {
            self.header.base.dependency_offset = current_pos;
            self.header.base.dependency_count =
                (&self.header.dependencies).as_ref().unwrap().len() as u16;
            // let dep_size = (&self.header.dependencies).as_ref().unwrap()[0].size() as u16;
            // current_pos += dep_size * self.header.base.dependency_count;
        } else {
            self.header.base.dependency_offset = 0;
            self.header.base.dependency_count = 0;
        }
        // Relocation entries
        // --> Must be put at the end of the header, to get offsets right
        if self.header.relocations.is_some() {
            let header_size = self.header.size();
            let relocs = (&mut self.header.relocations).as_mut().unwrap();
            for r in relocs.iter_mut() {
                //r.address_offset += header_size;
                r.point = r.point.shift(header_size);
            }
        }
        // Total size
        self.header.base.total_size = self.size();
    }

    /// Create the CBF in binary format
    pub fn generate(mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        // Finalize header
        self.finalize();
        // Generate bytes
        let bytes = self.as_bytes();
        let mut bytes_cur = Cursor::new(bytes);
        // Generate checksum
        bytes_cur.seek(SeekFrom::Start(0))?;

        let mut wordbuf = [0_u8; 4];
        let mut checksum: u32 = 0;
        loop {
            let count = bytes_cur.read(&mut wordbuf)?;
            // Combine the bytes back into a word, handling if we don't
            // get a full word.
            let mut word = 0;
            for (i, c) in wordbuf.iter().enumerate().take(count) {
                word |= u32::from(*c) << (8 * i);
            }
            checksum ^= word;
            if count != 4 {
                //TODO: check != 0 plus 0<count<4 panic?
                break;
            }
        }
        // Inject checksum
        let checksum_offset = self.header.base.trailer_offset + 0x00;
        let mut wordbuf = [0_u8; 4];
        bytes_cur.seek(SeekFrom::Start(checksum_offset as u64))?;
        wordbuf[0] = (checksum & 0xFF) as u8;
        wordbuf[1] = ((checksum >> 8) & 0xFF) as u8;
        wordbuf[2] = ((checksum >> 16) & 0xFF) as u8;
        wordbuf[3] = ((checksum >> 24) & 0xFF) as u8;
        bytes_cur.write_all(&wordbuf)?;
        return Ok(bytes_cur.into_inner());
    }
}

/*
    Util methods
*/
fn convert_config_region_attributes(attributes: &Vec<RF>) -> RegionAttributes {
    let mut attr: RegionAttributes = RegionAttributes::NONE;
    for a in attributes.iter() {
        match *a {
            RF::READ => {
                attr |= RegionAttributes::READ;
            }
            RF::WRITE => {
                attr |= RegionAttributes::WRITE;
            }
            RF::EXECUTE => {
                attr |= RegionAttributes::EXECUTE;
            }
            RF::DEVICE => {
                attr |= RegionAttributes::DEVICE;
            }
            RF::DMA => {
                attr |= RegionAttributes::DMA;
            }
        }
    }
    return attr;
}
