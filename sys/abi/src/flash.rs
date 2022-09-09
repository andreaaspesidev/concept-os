use zerocopy::{FromBytes, AsBytes, ByteSlice};

/**
 * Flash structures used between the kernel and the flash allocator,
 * are considered part of the abi.
 */

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(u16)]
pub enum BlockType {
    NONE,
    COMPONENT,
    // DATA,
    UNKNOWN(u16),
}

impl From<u16> for BlockType {
    fn from(x: u16) -> Self {
        match x {
            0xFFFF => BlockType::NONE,
            0xFFFE => BlockType::COMPONENT,
            x => BlockType::UNKNOWN(x)
        }
    }
}
impl From<BlockType> for u16 {
    fn from(x: BlockType) -> Self {
        match x {
            BlockType::NONE => 0xFFFF,
            BlockType::COMPONENT => 0xFFFE,
            BlockType::UNKNOWN(x) => x
        }
    }
}

unsafe impl FromBytes for BlockType {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized {
    }

    fn read_from<B: ByteSlice>(bytes: B) -> Option<Self>
    where
        Self: Sized,
    {
        if bytes.len() != 2 {
            return None;
        }
        // Little endian
        let number: u16 = bytes[0] as u16 | ((bytes[1] as u16) << 8);
        Some(Self::from(number))
    }

    fn read_from_prefix<B: ByteSlice>(bytes: B) -> Option<Self>
    where
        Self: Sized,
    {
        if bytes.len() < 2 {
            return None;
        }
        // Little endian
        let number: u16 = bytes[0] as u16 | ((bytes[1] as u16) << 8);
        Some(Self::from(number))
    }

    fn read_from_suffix<B: ByteSlice>(bytes: B) -> Option<Self>
    where
        Self: Sized,
    {
        if bytes.len() < 2 {
            return None;
        }
        // Little endian
        let number: u16 = bytes[bytes.len()-2] as u16 | ((bytes[bytes.len()-1] as u16) << 8);
        Some(Self::from(number))
    }
}

unsafe impl AsBytes for BlockType {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized {
    }
    // TODO: other to add here?
}