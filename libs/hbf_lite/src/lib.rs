#![no_std]

use core::fmt::{Debug, Error, Formatter};

pub use header::{HbfHeaderBase, HbfHeaderMain, HbfHeaderRelocation, HbfHeaderDependency, HbfVersion, HBF_MAGIC};

pub use header::{
    HbfHeaderInterrupt, HbfHeaderRegion, FIXED_HEADER_SIZE,
    HBF_HEADER_MIN_SIZE, INTERRUPT_SIZE, REGION_SIZE, RELOC_SIZE, DEPENDENCY_SIZE
};
pub use trailer::{HbfTrailer,HBF_TRAILER_SIZE};
use trailer::HBF_CHECKSUM_OFFSET;

mod header;
mod trailer;
mod utils;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HbfError {
    ReadError,
    InvalidMagic,
    UnsupportedVersion,
    InvalidRegion,
    InvalidInterrupt,
    InvalidRelocation,
    InvalidDependency,
    InvalidOffset,
}

/// Trait needed by the hbf library to access the content of the hbf.
/// This can be useful in case the whole buffer is not directly accessible
/// in memory, and must be recovered in blocks.
pub trait BufferReader<'a> {
    /// Reads a slice of len bytes starting from the specified offset
    fn read(&self, offset: u32, dest: &mut [u8]) -> Result<(), HbfError>;
}

struct MockReader {}

impl<'a> BufferReader<'a> for MockReader {
    fn read(&self, _: u32, dest: &mut [u8]) -> Result<(), HbfError> {
        for i in 0..dest.len() {
            dest[i] = 0;
        }
        Ok(())
    }
}

/// Simple implementation of the BufferReader for a buffer
#[derive(Clone,Copy,Debug)]
pub struct BufferReaderImpl<'a> {
    buffer: &'a [u8],
}

impl<'a> BufferReaderImpl<'a> {
    pub fn from(buffer: &'a [u8]) -> Self {
        Self { buffer: buffer }
    }
}

impl<'a> BufferReader<'a> for BufferReaderImpl<'a> {
    fn read(&self, offset: u32, dest: &mut [u8]) -> Result<(), HbfError> {
        let off: usize = offset as usize;
        if off >= self.buffer.len() {
            return Err(HbfError::ReadError);
        }
        for i in 0..dest.len() {
            dest[i] = self.buffer[off + i];
        }
        Ok(())
    }
}

static MOCK_READER_INSTANCE: MockReader = MockReader {};

pub struct HbfFile<'a> {
    reader: &'a dyn BufferReader<'a>,
}

pub struct HbfPayloadSection<'a> {
    base_offset: u32,
    size: u32,
    reader: &'a dyn BufferReader<'a>,
}

impl<'a> HbfPayloadSection<'a> {
    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn get_offset(&self) -> u32 {
        self.base_offset
    }

    pub fn get_content(&self, offset: u32, dest_buffer: &mut [u8]) -> Result<(), HbfError> {
        // Check offset is reasonable
        if offset >= self.size {
            return Err(HbfError::InvalidOffset);
        }
        // Read enough straight into the dest buffer
        self.reader.read(self.base_offset + offset, dest_buffer)?;
        Ok(())
    }
}

impl<'a> Debug for HbfPayloadSection<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), core::fmt::Error> {
        f.debug_struct("Payload Section")
            .field("size", &self.size())
            .finish()?;
        f.write_str("\n------------------------\n")?;
        let mut tmp_buff: [u8; 64] = [0; 64];
        let max_to_read: usize;
        if self.size() > 64 {
            max_to_read = 64;
        } else {
            max_to_read = self.size() as usize;
        }
        self.get_content(0, &mut tmp_buff[0..max_to_read]).unwrap();
        utils::dump_section(&tmp_buff[0..max_to_read], f)?;
        f.write_str("\n------------------------\n")?;
        Ok(())
    }
}

impl<'a> HbfFile<'a> {
    pub fn from_reader(reader: &'a dyn BufferReader<'a>) -> Result<Self, HbfError> {
        // Start reading the initial bytes, to check if we find the magic number
        let mut buffer: [u8; 4] = [0x00; 4];
        reader.read(0, &mut buffer)?;
        if buffer != HBF_MAGIC {
            return Err(HbfError::InvalidMagic);
        }
        // Construct a version of self
        let hbf = Self { reader: reader };
        // Check the version of the hbf
        let header_base = hbf.header_base()?;
        if header_base.hbf_version() != HbfVersion::V1 {
            return Err(HbfError::UnsupportedVersion);
        }
        // Return the object
        Ok(hbf)
    }

    pub fn header_base(&self) -> Result<HbfHeaderBase, HbfError> {
        // Read enough to parse
        const SIZE: usize = core::mem::size_of::<HbfHeaderBase>();
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(0, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderBase)) }
    }

    pub fn header_main(&self) -> Result<HbfHeaderMain, HbfError> {
        // Compute offset
        let offset = core::mem::size_of::<HbfHeaderBase>() as u32;
        // Read enough to parse
        const SIZE: usize = core::mem::size_of::<HbfHeaderMain>();
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderMain)) }
    }

    pub fn trailer(&self) -> Result<HbfTrailer, HbfError> {
        // Compute offset
        let offset = self.header_base()?.trailer_offset() as u32;
        // Read enough to parse
        const SIZE: usize = core::mem::size_of::<HbfTrailer>();
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfTrailer)) }
    }

    pub fn checksum_offset(&self) -> Result<u32, HbfError> {
        Ok(self.header_base()?.trailer_offset() + HBF_CHECKSUM_OFFSET as u32)
    }

    pub fn region_nth(&self, region_number: u16) -> Result<HbfHeaderRegion, HbfError> {
        // Check region number
        if region_number >= self.header_base()?.num_regions() {
            return Err(HbfError::InvalidRegion);
        }
        // Compute offset (base + num*size)
        const SIZE: usize = core::mem::size_of::<HbfHeaderRegion>();
        let mut offset = self.header_base()?.offset_regions() as u32;
        offset += (region_number as usize * SIZE) as u32;
        // Read enough to parse
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderRegion)) }
    }

    pub fn interrupt_nth(&self, interrupt_num: u16) -> Result<HbfHeaderInterrupt, HbfError> {
        // Check region number
        if interrupt_num >= self.header_base()?.num_interrupts() {
            return Err(HbfError::InvalidInterrupt);
        }
        // Compute offset (base + num*size)
        const SIZE: usize = core::mem::size_of::<HbfHeaderInterrupt>();
        let mut offset = self.header_base()?.offset_interrupts() as u32;
        offset += (interrupt_num as usize * SIZE) as u32;
        // Read enough to parse
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderInterrupt)) }
    }

    pub fn relocation_nth(&self, relocation_num: u32) -> Result<HbfHeaderRelocation, HbfError> {
        // Check region number
        if relocation_num >= self.header_base()?.num_relocations() {
            return Err(HbfError::InvalidRelocation);
        }
        // Compute offset (base + num*size)
        const SIZE: usize = core::mem::size_of::<HbfHeaderRelocation>();
        let mut offset = self.header_base()?.offset_relocation() as u32;
        offset += (relocation_num as usize * SIZE) as u32;
        // Read enough to parse
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderRelocation)) }
    }

    pub fn dependency_nth(&self, dependency_num: u16) -> Result<HbfHeaderDependency, HbfError> {
        // Check region number
        if dependency_num >= self.header_base()?.num_dependencies() {
            return Err(HbfError::InvalidRelocation);
        }
        // Compute offset (base + num*size)
        const SIZE: usize = core::mem::size_of::<HbfHeaderDependency>();
        let mut offset = self.header_base()?.offset_dependencies() as u32;
        offset += (dependency_num as usize * SIZE) as u32;
        // Read enough to parse
        let mut buffer: [u8; SIZE] = [0x00; SIZE];
        self.reader.read(offset, &mut buffer)?;
        // Convert the buffer into the structure
        unsafe { Ok(*(buffer.as_ptr() as *const HbfHeaderDependency)) }
    }

    fn payload_offset(&self) ->  Result<usize, HbfError> {
        Ok(core::mem::size_of::<HbfHeaderBase>()
            + core::mem::size_of::<HbfHeaderMain>()
            + core::mem::size_of::<HbfHeaderRegion>() * self.header_base()?.num_regions() as usize
            + core::mem::size_of::<HbfHeaderInterrupt>()
                * self.header_base()?.num_interrupts() as usize
            + core::mem::size_of::<HbfHeaderRelocation>()
                * self.header_base()?.num_relocations() as usize
            + core::mem::size_of::<HbfHeaderDependency>()
                * self.header_base()?.num_dependencies() as usize
            + self.header_base()?.padding_bytes() as usize)
    }

    pub fn get_readonly_payload(&self) -> Result<HbfPayloadSection<'a>, HbfError> {
        // Compute offset
        let offset = self.payload_offset()?;
        // Compute size
        let size = self.read_only_payload_size()?;
        // Construct result
        Ok(HbfPayloadSection {
            base_offset: offset as u32,
            size: size,
            reader: self.reader,
        })
    }

    pub fn get_data_payload(&self) -> Result<Option<HbfPayloadSection<'a>>, HbfError> {
        let offset = self.header_main()?.data_offset();
        let size = self.data_size()?;
        match offset {
            0 => Ok(None),
            _ => Ok(Some(HbfPayloadSection {
                base_offset: offset as u32,
                size: size,
                reader: self.reader,
            })),
        }
    }

    pub fn get_bss_payload(&self) -> Result<Option<HbfPayloadSection<'a>>, HbfError> {
        let size = self.bss_size()?;
        if size == 0 {
            return Ok(None);
        }
        Ok(Some(HbfPayloadSection {
            base_offset: 0,
            size: size,
            reader: &MOCK_READER_INSTANCE,
        }))
    }

    /*
        Sizes calcs
    */

    fn read_only_payload_size(&self) -> Result<u32, HbfError> {
        let offset = self.payload_offset()?;
        Ok(match self.header_main()?.data_offset() {
            0 => self.header_base()?.total_size() - core::mem::size_of::<HbfTrailer>() as u32 - offset as u32,
            data_offset => data_offset - offset as u32,
        })
    }

    pub fn payload_size(&self) -> Result<u32, HbfError> {
        let offset = self.payload_offset()?;
        Ok(self.header_base()?.total_size() - core::mem::size_of::<HbfTrailer>() as u32 - offset as u32)
    }

    fn data_size(&self) -> Result<u32, HbfError> {
        let offset = self.header_main()?.data_offset();
        if offset == 0 {
            return Ok(0);
        } else {
            return Ok(self.header_base()?.total_size() - core::mem::size_of::<HbfTrailer>() as u32 - offset);
        }
    }

    fn bss_size(&self) -> Result<u32, HbfError> {
        let cum_size = self.header_main()?.databss_size();
        return Ok(cum_size - self.data_size()?);
    }

    /*
        Validation
    */
    pub fn validate(&self) -> Result<bool, HbfError> {
        let header_base = self.header_base()?;
        let total_size = header_base.total_size();

        let mut index: u32 = 0;
        let mut checksum: u32 = 0;
        let checksum_offset = header_base.trailer_offset() + HBF_CHECKSUM_OFFSET;

        while index < total_size {
            // Read 4 bytes
            let word: u32;
            let mut available: u32 = 4;

            if total_size - index < available {
                available = total_size - index;
            }

            if index == checksum_offset {
                // Skip this field
                word = 0;
            } else {
                let mut buff: [u8; 4] = [0x00; 4];
                self.reader.read(index, &mut buff)?;
                word = u32::from_le_bytes(buff);
            }
            checksum ^= word;
            index += available;
        }
        return Ok(self.trailer()?.checksum() == checksum);
    }
}

impl<'a> Debug for HbfFile<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.debug_struct("Hbf File").finish()
    }
}
