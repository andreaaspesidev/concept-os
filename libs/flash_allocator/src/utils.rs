use crate::{buddy, flash::{header::BlockHeader, FlashMethods}};

pub fn read_block_header<
    'a,
    const FLAG_BYTES: usize,
    const START_ADDR: u32,
    const NUM_SLOTS: usize,
>(
    flash: &dyn FlashMethods<'a>,
    offset: u32,
) -> BlockHeader<'a, FLAG_BYTES> {
    let header_buffer = flash.read(
        START_ADDR + offset,
        BlockHeader::<FLAG_BYTES>::HEADER_SIZE,
    );
    let block_header: BlockHeader<FLAG_BYTES> =
        BlockHeader::<FLAG_BYTES>::new(header_buffer, buddy::get_max_level::<NUM_SLOTS>() as u16);
    return block_header;
}

pub fn get_block_size<
    'a,
    const START_ADDR: u32,
    const END_ADDR: u32,
    const BLOCK_SIZE: usize,
    const FLAG_BYTES: usize,
>(
    block_header: &BlockHeader<'a, FLAG_BYTES>,
) -> usize {
    let size = (END_ADDR - START_ADDR + 1) as usize;
    if block_header.is_allocated() {
        size >> block_header.block_level()
    } else {
        BLOCK_SIZE
    }
}
