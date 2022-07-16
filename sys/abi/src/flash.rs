/**
 * Flash structures used between the kernel and the flash allocator,
 * are considered part of the abi.
 */

use core::marker::PhantomData;


#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum BlockType {
    NONE,      // 0x0000
    COMPONENT, // 0x0001
    // DATA,
    UNKNOWN(u16),
}

impl BlockType {
    pub fn from_u16(val: u16) -> Self {
        match val {
            0xFFFF => Self::NONE,
            0xFFFE => Self::COMPONENT,
            val => Self::UNKNOWN(val),
        }
    }
    pub fn to_u16(&self) -> u16 {
        match *self {
            Self::NONE => 0xFFFF,
            Self::COMPONENT => 0xFFFE,
            Self::UNKNOWN(val) => val,
        }
    }
}

pub trait BlockHeader<'a, const FLAG_BYTES: usize> {
    fn is_allocated(&self) -> bool;
    fn is_dismissed(&self) -> bool;
    fn is_finalized(&self) -> bool;

    fn block_level(&self) -> u16;
    fn block_type(&self) -> BlockType;
}

/**
 * As the header is very small, and fields are read mutiple times,
 * this implementation copies the header of the block in SRAM when
 * this structure is constructed.
 *
 * It's also possible to avoid this behavior as in hbf_rs
 */
#[allow(dead_code)]
pub struct BlockHeaderGen<'a, const FLAG_BYTES: usize> {
    allocated: bool,
    dismissed: bool,
    finalized: bool,
    block_level: u16,
    block_type: BlockType,
    ph: &'a PhantomData<u8>, // Needed to force the lifetime
}

impl<'a, const FLAG_BYTES: usize> BlockHeaderGen<'a, FLAG_BYTES> {
    pub const HEADER_SIZE: usize = FLAG_BYTES * 4 + 2 + 2;

    pub fn new(header_address: &'a [u8], max_level: u16) -> Self {
        let ptr = header_address.as_ptr();
        // Construct the structure
        let ptr_flag = ptr as *const [u8; FLAG_BYTES];
        let allocated_flag: [u8; FLAG_BYTES] = unsafe { ptr_flag.read_unaligned() }.into();
        let dismissed_flag: [u8; FLAG_BYTES] = unsafe { ptr_flag.add(1).read_unaligned() }.into();
        let finalized_flag: [u8; FLAG_BYTES] = unsafe { ptr_flag.add(2).read_unaligned() }.into();
        let block_level_ptr = unsafe { ptr_flag.add(4) } as *const u16;
        let block_level: u16 = unsafe { block_level_ptr.read_unaligned() }.into();
        let block_type_ptr = unsafe { block_level_ptr.add(1) } as *const u16;
        let block_type: u16 = unsafe { block_type_ptr.read_unaligned() }.into();
        let allocated = allocated_flag == [0x00; FLAG_BYTES];
        Self {
            allocated: allocated,
            dismissed: dismissed_flag == [0x00; FLAG_BYTES],
            finalized: finalized_flag == [0x00; FLAG_BYTES],
            block_level: match allocated {
                true => block_level,
                false => max_level,
            },
            block_type: BlockType::from_u16(block_type),
            ph: &PhantomData,
        }
    }
    fn write_flag<'b>(dest_buffer: &'b mut [u8], offset: usize, flag: bool) {
        for i in 0..FLAG_BYTES {
            dest_buffer[offset + i] = match flag {
                true => 0x00,
                false => 0xFF,
            }
        }
    }
    pub fn write_buffer<'b>(
        allocated: bool,
        dismissed: bool,
        finalized: bool,
        block_level: u16,
        block_type: BlockType,
    ) -> [u8; FLAG_BYTES * 4 + 2 + 2]
    where
        [u8; FLAG_BYTES * 4 + 2 + 2]: Sized,
    {
        let mut buffer: [u8; FLAG_BYTES * 4 + 2 + 2] = [0xFF; FLAG_BYTES * 4 + 2 + 2];
        Self::write_flag(&mut buffer, 0, allocated);
        Self::write_flag(&mut buffer, FLAG_BYTES, dismissed);
        Self::write_flag(&mut buffer, FLAG_BYTES * 2, finalized);
        let level_offset = FLAG_BYTES * 4;
        buffer[level_offset] = block_level.to_le_bytes()[0];
        buffer[level_offset + 1] = block_level.to_le_bytes()[1];
        let flags_offset = level_offset + 2;
        let block_type_u = block_type.to_u16();
        buffer[flags_offset] = block_type_u.to_le_bytes()[0];
        buffer[flags_offset + 1] = block_type_u.to_le_bytes()[1];
        buffer
    }
}

impl<'a, const FLAG_BYTES: usize> BlockHeader<'a, FLAG_BYTES> for BlockHeaderGen<'a, FLAG_BYTES> {
    fn is_allocated(&self) -> bool {
        self.allocated
    }
    fn is_dismissed(&self) -> bool {
        self.dismissed
    }
    fn is_finalized(&self) -> bool {
        self.finalized
    }
    fn block_level(&self) -> u16 {
        self.block_level
    }
    fn block_type(&self) -> BlockType {
        self.block_type
    }
}
