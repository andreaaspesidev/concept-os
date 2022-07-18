/**
 * Flash structures used between the kernel and the flash allocator,
 * are considered part of the abi.
 */

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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