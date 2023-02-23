#![no_std]

use userlib::sys_log;

pub trait RelocatorMethods {
    /// Reads dst.len() relocations starting from start_index into dst,
    /// and returns the actual number of relocations copied
    fn read_relocations(&self, start_index: usize, dst: &mut [u32]) -> Result<usize,()>;

    /// Flushes (to flash/disk) the src buffer, at the position supplied.
    /// The position is intended in bytes, considered the current_file_pos passed 
    /// to the relocator constructor
    fn flush(&mut self, position: usize, src: &[u8]) -> Result<(),()>;
}

/// This object implements the relocator, able to process at runtime
/// the relocation points supplied via an external function.
///
/// All the relocations are assumed to be accessible at any moment, even in random order.
/// To improve performance, they are generally requested in blocks.
///
/// The ELF/HBF instead is available only for the part stored in the buffer inside the structure,
/// and in particular it's assumed not possible to search/write both after or before this buffer.
/// 
/// NOTE: BUFF_SIZE and RELOC_BUFF_SIZE must be powers of two.
///
pub struct Relocator<const LINKED_FLASH_ADDR: u32, const LINKED_SRAM_ADDR: u32, const BUFF_SIZE: usize, const RELOC_BUFF_SIZE: usize> {
    /// Buffer where we store the data in a sliding window fashion
    working_buffer: RingBuffer<u8, BUFF_SIZE>,
    current_file_pos: usize,
    /// Buffer where we store a block of relocations, to avoid asking
    /// for relocations (considered an expensive operation)
    relocation_buffer: RelocationBuff<RELOC_BUFF_SIZE>,

    current_buffer_pos: usize,

    new_flash_base: u32,
    new_sram_base: u32,
}

impl<'a, const LINKED_FLASH_ADDR: u32, const LINKED_SRAM_ADDR: u32, const BUFF_SIZE: usize, const RELOC_BUFF_SIZE: usize>
    Relocator<LINKED_FLASH_ADDR, LINKED_SRAM_ADDR, BUFF_SIZE, RELOC_BUFF_SIZE>
{
    pub fn new(
        new_flash_base: u32,
        new_sram_base: u32,
        current_file_pos: usize,
        total_relocations_available: usize,
    ) -> Self {
        Self {
            new_flash_base: new_flash_base,
            new_sram_base: new_sram_base,
            working_buffer: RingBuffer::<_, BUFF_SIZE>::new(),
            current_file_pos: current_file_pos,
            relocation_buffer: RelocationBuff::new(total_relocations_available),
            current_buffer_pos: 0,
        }
    }

    fn flush_working_buffer(
        &mut self,
        num_bytes: usize,
        relocator_methods: &mut dyn RelocatorMethods,
    ) -> Result<usize,()> {
        // As we cannot directly get a slice for our ring buffer,
        // let's call it multiple times
        let mut buff: [u8; BUFF_SIZE] = [0x00; BUFF_SIZE];

        let mut total_flushed: usize = 0;
        while total_flushed < num_bytes {
            let available_to_flush: usize =
                core::cmp::min(self.working_buffer.read_capacity(), BUFF_SIZE);
            let to_flush: usize = core::cmp::min(available_to_flush, num_bytes - total_flushed);
            // Check if it makes sense to continue
            if to_flush == 0 {
                break;
            }
            let total = self.working_buffer.read(0, &mut buff[0..to_flush]);
            // Call the flush method
            relocator_methods.flush(self.current_file_pos, &buff[0..total])?;
            self.current_file_pos += total;
            total_flushed += total;
            // Check if it makes sense to continue
            if total == 0 {
                break;
            }
        }
        return Ok(total_flushed);
    }

    fn flush_bytes(
        &mut self,
        num_bytes: usize,
        current_buffer: &[u8],
        relocator_methods: &mut dyn RelocatorMethods,
    ) -> Result<bool,()> {
        let mut total_flushed: usize = 0;
        while total_flushed < num_bytes {
            // Try to flush everything we have in the working buffer
            total_flushed += self.flush_working_buffer(num_bytes - total_flushed, relocator_methods)?;
            // Read more data in the working buffer
            let total_filled = self
                .working_buffer
                .extend_from(&current_buffer[self.current_buffer_pos..current_buffer.len()]);
            self.current_buffer_pos += total_filled;
            // Continue flushing until we have no more to fill the buffer with
            if self.current_buffer_pos == current_buffer.len() {
                // Perform last flush and return
                total_flushed += self.flush_working_buffer(num_bytes - total_flushed, relocator_methods)?;

                if total_flushed == num_bytes {
                    // We flushed enough, we may fix the relocation now
                    return Ok(true);
                } else {
                    // We need to wait for more data, the relocation point is not in the working area
                    return Ok(false);
                }
            }
        }
        return Ok(true);
    }

    /// This methods blocks untill the content of the current buffer is consumed.
    /// It must be called upon the current_buffer is refilled with new data.
    pub fn consume_current_buffer(
        &mut self,
        current_buffer: &[u8],
        relocator_methods: &mut dyn RelocatorMethods,
    ) -> Result<(),()> {
        // Avoid processing if we have finished
        if self.relocation_buffer.no_more_relocs() {
            // Just flush the whole current_buffer as it is
            relocator_methods.flush(self.current_file_pos, current_buffer)?;
            self.current_file_pos += current_buffer.len();
            return Ok(());
        }
        self.current_buffer_pos = 0; // Start from the beginning

        // Until we finish consuming the current buffer
        while self.current_buffer_pos < current_buffer.len() || self.working_buffer.read_capacity() > 0 {
            // First, load the next relocation, as it may be out of reach for now
            let next_reloc_option = self.relocation_buffer.get_next(relocator_methods)?;
            if next_reloc_option.is_none() {
                // No more relocations
                //assert!(self.relocation_buffer.no_more_relocs());
                // Flush all we can
                self.flush_bytes(usize::MAX, current_buffer, relocator_methods)?;
                //assert!(self.current_buffer_pos == current_buffer.len());
                return Ok(());
            }
            let next_reloc = next_reloc_option.unwrap();
            // Calculate distance with the relocation
            let distance = next_reloc.get_target_file_pos() - self.current_file_pos;

            // ---------- Optimize -------------
            // If the target is out of reach, it makes no sense to fill and empty the working buffer
            // This happens when we won't reach the distance even in the optimistic case we flush the whole
            // current content of the working buffer and add all the content of the current_buffer. 
            //
            if distance > current_buffer.len() + self.working_buffer.read_capacity() {
                // Direct flush everything (also the working buffer or we could have strange behaviours!)
                self.flush_working_buffer(BUFF_SIZE, relocator_methods)?;
                relocator_methods.flush(self.current_file_pos, &current_buffer[self.current_buffer_pos..current_buffer.len()])?;
                // Advance counters!
                self.current_file_pos += current_buffer.len() - self.current_buffer_pos;
                return Ok(());
            }
            // ---------------------------------


            // Try to copy how as much data as possible into the working buffer
            let total_filled = self
                .working_buffer
                .extend_from(&current_buffer[self.current_buffer_pos..current_buffer.len()]);
            self.current_buffer_pos += total_filled;

            // Flush up to this point.
            if !self.flush_bytes(distance, current_buffer, relocator_methods)? {
                // We have consumed everything, yet did not manage to reach the position
                //assert!(self.current_buffer_pos == current_buffer.len());
                return Ok(());
            }
            //assert!(self.current_file_pos == next_reloc.get_target_file_pos());
            // Check the type of relocation
            match next_reloc.get_type() {
                RelocationType::AbsAddress => {
                    // We need to have at least 4 bytes in the buffer
                    if self.working_buffer.read_capacity() < 4 {
                        //assert!(self.current_buffer_pos == current_buffer.len());
                        return Ok(()); // Wait for more data to come
                    }

                    // We have everything we need. Just launch the relocation
                    self.process_abs_reloc(next_reloc);
                    // Now for sure something can be flushed. But continue with the iteration to allow flushing.
                    // For now, just mark this reloc as consumed
                    self.relocation_buffer.consume_relocation(0);
                }
                RelocationType::MovW | RelocationType::MovT => {
                    // These relocations needs the pair to be visible in both buffers (relocation and working)
                    // assert!(next_reloc.get_paired_offset() > 0);
                    // Search the pair
                    let paired_index = next_reloc.get_paired_offset() as usize; // the current one is the one at 0
                    let paired_reloc = *self.relocation_buffer.peek(paired_index);

                    // Assert it's inside the working buffer
                    let paired_distance =
                        paired_reloc.get_target_file_pos() - self.current_file_pos;

                    // We need to have at least the 4 bytes of the paired relocation in the buffer
                    if self.working_buffer.read_capacity() < paired_distance + 4 {
                        //assert!(self.current_buffer_pos == current_buffer.len());
                        return Ok(()); // Wait for more data to come
                    }
                    // Fix both the relocations
                    self.process_paired_reloc(next_reloc, paired_reloc);
                    // Mark both as completed
                    self.relocation_buffer.consume_relocation(0);
                    self.relocation_buffer.consume_relocation(paired_index);
                }
            }
        }
        Ok(())
    }

    /// Closes the relocator, flushing everything that remained in the buffers
    pub fn finish(mut self, relocator_methods: &mut dyn RelocatorMethods) -> Result<(),()> {
        // Flushes everything that remained in the working buffer
        self.flush_working_buffer(usize::MAX, relocator_methods).map(|_| ())
    }

    fn is_flash_addr(&self, addr: u32) -> bool {
        addr & 0xFF000000 == LINKED_FLASH_ADDR & 0xFF000000
    }

    fn is_sram_addr(&self, addr: u32) -> bool {
        addr & 0xFF000000 == LINKED_SRAM_ADDR & 0xFF000000
    }

    fn fix_address(&self, addr: u32) -> u32 {
        if self.is_flash_addr(addr) {
            return addr - LINKED_FLASH_ADDR + self.new_flash_base;
        } else if self.is_sram_addr(addr) {
            return addr - LINKED_SRAM_ADDR + self.new_sram_base;
        } else {
            sys_log!("Unknown memory area to relocate");
            panic!();
        }
    }

    /**
     * Standard ABS relocations
     */
    fn process_abs_reloc(&mut self, rel: Relocation) {
        // assert!(rel.get_type() == RelocationType::AbsAddress);
        // Compute offset
        let offset = rel.get_target_file_pos() - self.current_file_pos;
        // Read address
        let mut addr_bytes: [u8; 4] = [0x00; 4];
        self.working_buffer.read_into(offset, &mut addr_bytes);
        let linked_addr = u32::from_le_bytes(addr_bytes);
        // Compute new address
        let new_addr_bytes = self.fix_address(linked_addr).to_le_bytes();
        // Write back address in the working buffer
        for i in 0..4usize {
            self.working_buffer.change_at(offset + i, new_addr_bytes[i]);
        }
    }

    /**
     * MOV relocations
     */
    fn process_paired_reloc(&mut self, rel1: Relocation, rel2: Relocation) {
        let lower_rel;
        let upper_rel;
        // Here we could assume those relocation comes first MOVW then MOVT, but it's kept generic.
        if rel1.get_type() == RelocationType::MovW {
            lower_rel = rel1;
            upper_rel = rel2;
        } else {
            lower_rel = rel2;
            upper_rel = rel1;
        }
        // Read the two instructions
        let lower_mov_pos = lower_rel.get_target_file_pos() - self.current_file_pos;
        let upper_mov_pos = upper_rel.get_target_file_pos() - self.current_file_pos;
        let lower_encoded_mov = self.read_mov(lower_mov_pos);
        let upper_encoded_mov = self.read_mov(upper_mov_pos);

        // Extract the linked address from the two movs
        let lower_linked = Self::get_mov_immediate(lower_encoded_mov) as u32;
        let upper_linked = Self::get_mov_immediate(upper_encoded_mov) as u32;
        let linked_addr = upper_linked << 16 | lower_linked;

        // Compute the new address
        let new_addr = self.fix_address(linked_addr);

        // Generate the new instructions
        let new_lower = (new_addr & 0xFFFF) as u16;
        let new_upper = (new_addr >> 16) as u16;
        let new_encoded_lower = Self::fix_mov_immediate(lower_encoded_mov, new_lower);
        let new_encoded_upper = Self::fix_mov_immediate(upper_encoded_mov, new_upper);

        // Write back both
        self.write_mov(lower_mov_pos, new_encoded_lower);
        self.write_mov(upper_mov_pos, new_encoded_upper);
    }

    fn read_mov(&self, offset: usize) -> u32 {
        let mut word_bytes: [u8; 4] = [0x00; 4];
        self.working_buffer.read_into(offset, &mut word_bytes);
        // assert_eq!(self.working_buffer.read_into(offset, &mut word_bytes), 4);
        let upper_half_word = u16::from_le_bytes(word_bytes[0..2].try_into().unwrap()) as u32;
        let lower_half_word = u16::from_le_bytes(word_bytes[2..4].try_into().unwrap()) as u32;
        return upper_half_word << 16 | lower_half_word;
    }

    fn write_mov(&mut self, offset: usize, mov: u32) {
        let lower_half_word = ((mov & 0xFFFF) as u16).to_le_bytes();
        let upper_half_word = ((mov >> 16) as u16).to_le_bytes();
        for i in 0..2usize {
            self.working_buffer.change_at(offset + i, upper_half_word[i]);
        }
        for i in 0..2usize {
            self.working_buffer.change_at(offset + i + 2, lower_half_word[i]);
        }
    }

    fn get_mov_immediate(mov: u32) -> u16 {
        // Extract parts
        let i = (mov >> 26) & 0b1;
        let imm4 = (mov >> 16) & 0b1111;
        let imm3 = (mov >> 12) & 0b111;
        let imm8 = mov & 0b11111111;
        return (imm8 | imm3 << 8 | i << 11 | imm4 << 12) as u16;
    }

    fn fix_mov_immediate(mov: u32, value: u16) -> u32 {
        // Extract parts from new value
        let value = value as u32;
        let imm8 = value & 0b11111111;
        let imm3 = (value >> 8) & 0b111;
        let i = (value >> 11) & 0b1;
        let imm4 = (value >> 12) & 0b1111;
        // Unset all mov operand bits
        let mov_empty = mov & 0b11111011111100001000111100000000;
        // Fix word
        return mov_empty | i << 26 | imm4 << 16 | imm3 << 12 | imm8;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum RelocationType {
    AbsAddress = 0,
    MovW = 1,
    MovT = 2,
}

impl From<u32> for RelocationType {
    fn from(x: u32) -> Self {
        match x {
            0 => Self::AbsAddress,
            1 => Self::MovW,
            2 => Self::MovT,
            _ => {
                sys_log!("Relocation not supported!");
                panic!();
            },
        }
    }
}

struct RelocationBuff<const N: usize> {
    buff: RingBuffer<Relocation, N>,
    total_used_relocs: usize,
    total_available_relocs: usize,
}

impl<'a, const N: usize> RelocationBuff<N> {
    pub fn new(
        total_available_relocs: usize,
    ) -> Self {
        Self {
            buff: RingBuffer::<_, N>::new(),
            total_used_relocs: 0,
            total_available_relocs: total_available_relocs,
        }
    }
    pub fn no_more_relocs(&self) -> bool {
        self.is_empty() && self.total_used_relocs == self.total_available_relocs
    }
    fn is_empty(&self) -> bool {
        self.buff.read_capacity() == 0
    }

    fn refill(&mut self, relocator_methods: &mut dyn RelocatorMethods) -> Result<bool,()> {
        let yet_to_consume: usize = self.total_available_relocs - self.total_used_relocs;
        if yet_to_consume == 0 {
            return Ok(false); // No more relocs available
        }
        // Ask the most we can
        let to_transfer: usize = core::cmp::min(yet_to_consume, self.buff.write_capacity());
        if to_transfer == 0 {
            // We were not able to load more relocations
            return Ok(true);
        }
        // Create a temp buffer to store these relocations
        let mut reloc_buff: [u32; N] = [0x00; N];
        let _total_read = relocator_methods.read_relocations(self.total_used_relocs, &mut reloc_buff[0..to_transfer])?;
        // assert_eq!(to_transfer, total_read);
        self.total_used_relocs += to_transfer;
        // Convert each point into a relocation and store it
        for i in 0..to_transfer {
            self.buff.extend_from(&[Relocation::from(reloc_buff[i])]);
            //assert_eq!(self.buff.extend_from(&[Relocation::from(reloc_buff[i])]), 1);
        }
        return Ok(true);
    }
    fn get_next_free(&mut self) -> Option<Relocation> {
        // First, free all the consumed relocation at the beginning
        while let Some(rel) = self.buff.peek(0) {
            if rel.is_consumed() {
                self.buff.free(1);
            } else {
                break;
            }
        }
        if self.buff.read_capacity() == 0 {
            // We have consumed everything
            return None;
        } else {
            // We have the free one at the beginning
            let r = self.buff.peek(0);
            if r.is_none() {
                sys_log!("Err 2");
            }
            return Some(*r.unwrap());
        }
    }
    pub fn get_next(&mut self, relocator_methods: &mut dyn RelocatorMethods) -> Result<Option<Relocation>,()> {
        // Check if refill is necessary
        if !self.refill(relocator_methods)? && self.is_empty() {
            return Ok(None); // No more relocation available
        }
        // Just read the next not consumed relocation
        if let Some(reloc) = self.get_next_free() {
            return Ok(Some(reloc));
        }
        if !self.is_empty() {
            sys_log!("Relocation buffer too small");
            panic!();
        }
        // We have finished all the relocations
        return Ok(None);
    }
    pub fn peek(&mut self, reloc_index: usize) -> &Relocation {
        self.buff
            .peek(reloc_index)
            .expect("Cannot find paired relocation. Buffer too small?")
    }
    pub fn consume_relocation(&mut self, reloc_index: usize) {
        let r = self.buff.peek_mut(reloc_index);
        if r.is_none() {
            sys_log!("err 1");
        }
        r.unwrap().set_consumed();
    }
}

#[derive(Debug, Clone, Copy)]
struct Relocation {
    rel_type: RelocationType,
    paired_rel_offset: isize,
    file_offset: usize,
    consumed: bool,
}

impl Relocation {
    fn get_type(&self) -> RelocationType {
        self.rel_type
    }
    fn get_target_file_pos(&self) -> usize {
        self.file_offset
    }
    fn get_paired_offset(&self) -> isize {
        self.paired_rel_offset
    }
    fn is_consumed(&self) -> bool {
        self.consumed
    }
    fn set_consumed(&mut self) {
        self.consumed = true;
    }
}

/// Extends the sign of a 2's complement int with bitsize different
/// from 32
fn sign_extend(value: u32, bits: usize) -> isize {
    let sign_bit = 1 << (bits - 1);
    let result = (value as isize & (sign_bit - 1)) - (value as isize & sign_bit);
    return result;
    // i.e.  11111 -> -1
    //       sign_extend(31, 5)
    //       sign_bit = 1 << 4
    //       31 & 15  - 16
    //       15 - 16 = -1
}

impl From<u32> for Relocation {
    fn from(x: u32) -> Self {
        let paired_offset = sign_extend((x >> 24) & 0x1F, 5);
        Self {
            rel_type: RelocationType::from(x >> 30),
            paired_rel_offset: paired_offset,
            file_offset: x as usize & 0xFF_FFFF,
            consumed: false,
        }
    }
}

impl Default for Relocation {
    fn default() -> Self {
        Self {
            rel_type: RelocationType::AbsAddress,
            paired_rel_offset: 0,
            file_offset: 0,
            consumed: true,
        }
    }
}

/// This structure implements a circular buffers that
/// allows the user to consider as the buffer is linear with size N,
/// but new data can be added at the end without shifting all the elements
pub struct RingBuffer<T, const N: usize>
where
    T: Sized + Default + Copy,
{
    buff: [T; N],
    /// next position to read
    read_ptr: usize,
    /// next position to write
    write_ptr: usize,
}

impl<T: Sized, const N: usize> RingBuffer<T, N>
where
    T: Sized + Default + Copy,
{
    pub fn new() -> Self {
        // assert!(N != 0, "Capacity is not allowed to be zero");
        // assert!(N.is_power_of_two(), "Capacity must be a power of two");
        Self {
            buff: [T::default(); N],
            read_ptr: 0,
            write_ptr: 0,
        }
    }
    /// Returns how much data can be currently written
    /// in the buffer
    pub fn write_capacity(&self) -> usize {
        if self.write_ptr > self.read_ptr {
            return N - 1 - self.write_ptr + self.read_ptr;
        } else if self.write_ptr < self.read_ptr {
            return self.read_ptr - self.write_ptr - 1;
        } else {
            // Only at the beginning
            return N - 1;
        }
        // es.
        //      +---+---+---+---+---+---+---+---+
        //      | _ | 1 | 2 | x | _ | _ | _ | _ |
        //      +---+---+---+---+---+---+---+---+
        //            ^           ^
        //           write       read
        //             1           4
        //  to_write: 4 - 1 - 1 = 2
        // es.
        //      +---+---+---+---+---+---+---+---+
        //      | x | _ | _ | _ | 1 | 2 | 3 | 4 |
        //      +---+---+---+---+---+---+---+---+
        //            ^           ^
        //           read       write
        //             1           4
        //  to_write: 8 - 1 - 4 + 1 = 4
    }
    /// Returns how much data can be read from the buffer
    pub fn read_capacity(&self) -> usize {
        // This value is:
        // let diff = self.read_ptr - self.write_ptr;
        // > 0, then this is the exact number of bytes we can read
        // < 0, we can now read till the tail, then from the beginning to the write ptr
        return (N + self.write_ptr - self.read_ptr) % N;
        // es.
        //      +---+---+---+---+---+---+---+---+
        //      | 5 | _ | _ | _ | 1 | 2 | 3 | 4 |
        //      +---+---+---+---+---+---+---+---+
        //            ^           ^
        //           write       read
        //             1           4
        //  to_read: (8 + 1 - 4) % 8 = 5 % 8 = 5
        // es.
        //      +---+---+---+---+---+---+---+---+
        //      | _ | 1 | 2 | 3 | _ | _ | _ | _ |
        //      +---+---+---+---+---+---+---+---+
        //            ^           ^
        //           read       write
        //             1           4
        //  to_read: (8 + 4 - 1) % 8 = 11 % 8 = 3
    }
    /// Copies min(write_capacity(),src.len()) elements from the src buffer,
    /// returning how many elements it has copied
    pub fn extend_from(&mut self, src: &[T]) -> usize {
        let to_be_copied: usize = core::cmp::min(src.len(), self.write_capacity());
        for i in 0..to_be_copied {
            self.write_t(src[i]);
        }
        return to_be_copied;
    }
    fn write_t(&mut self, value: T) {
        self.buff[self.write_ptr] = value;
        self.write_ptr = (self.write_ptr + 1) % N;
    }

    /// Puts into the dest buffer min(read_capacity()-offset,dst.len()),
    /// returning how many elements it has copied. It does not consume the buffer though.
    pub fn read_into(&self, offset: usize, dst: &mut [T]) -> usize {
        let to_be_copied = core::cmp::min(dst.len(), self.read_capacity() - offset);
        let mut read_ptr = (self.read_ptr + offset) % N;
        for i in 0..to_be_copied {
            dst[i] = self.buff[read_ptr];
            read_ptr = (read_ptr + 1) % N;
        }
        return to_be_copied;
    }
    /// Peeks at single element at the offset.
    pub fn peek(&self, offset: usize) -> Option<&T> {
        if offset >= self.read_capacity() {
            return None;
        }
        let read_ptr = (self.read_ptr + offset) % N;
        return Some(&self.buff[read_ptr]);
    }
    pub fn peek_mut(&mut self, offset: usize) -> Option<&mut T> {
        if offset >= self.read_capacity() {
            return None;
        }
        let read_ptr = (self.read_ptr + offset) % N;
        return Some(&mut self.buff[read_ptr]);
    }
    pub fn change_at(&mut self, offset: usize, new_val: T) {
        // assert!(offset < self.read_capacity());
        let read_ptr = (self.read_ptr + offset) % N;
        self.buff[read_ptr] = new_val;
    }
    // Discard num_elements from the read buffer, in a FIFO fashion.
    // It returns how many bytes it's able to free
    pub fn free(&mut self, num_elements: usize) -> usize {
        let to_be_freed: usize = core::cmp::min(self.read_capacity(), num_elements);
        self.read_ptr = (self.read_ptr + num_elements) % N;
        return to_be_freed;
    }
    // Discard num_elements from the read buffer, in a FIFO fashion.
    // It returns how many bytes it's able to free
    pub fn read(&mut self, offset: usize, dst: &mut [T]) -> usize {
        let read_data = self.read_into(offset, dst);
        self.free(read_data);
        return read_data;
    }
}
