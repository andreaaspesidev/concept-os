// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

mod generic;
pub use self::generic::*;
pub mod device;

use flash_allocator::flash::{page::FlashPage, FlashMethods};

/**
 * STM32 F303RE
 * - Flash Constant
 *
 * Flash: 0x0800 0000 - 0x0807 FFFF
 * Size: 512Kb
 *
 * Notes:
 * - In order for the system to work correcly, the first part of the flash
 *   (starting from address 0x0800 0000) must be reserved to the kernel.
 *   The allocator have two requirements:
 *   - Must have a base address (FLASH_ALLOCATOR_START_ADDR) aligned with its size (FLASH_ALLOCATOR_SIZE).
 *     To alleviate this huge limitation, it's possible to reserve a whole subspace
 *     at the beginning by selecting a  FLASH_ALLOCATOR_START_SCAN_ADDR > FLASH_ALLOCATOR_START_ADDR
 *   - The page containing FLASH_ALLOCATOR_START_SCAN_ADDR must not contain important data (like kernel code).
 *     The allocator will need to erase this page in order to deallocate the first blocks.
 *     For this reason, let's impose that FLASH_ALLOCATOR_START_SCAN_ADDR points to the
 *     beginning of the first free page after the one containing the last kernel code.
 */
pub const FLASH_ALLOCATOR_START_ADDR: u32 = 0x0800_0000; // Page 0
pub const FLASH_ALLOCATOR_END_ADDR: u32 = 0x0807_FFFF;
pub const FLASH_ALLOCATOR_SIZE: usize =
    (FLASH_ALLOCATOR_END_ADDR - FLASH_ALLOCATOR_START_ADDR + 1) as usize; // 0x80000 -> 524288

// This value needs to be fixed (automatically during build),
// and (FLASH_ALLOCATOR_START_SCAN_ADDR - FLASH_ALLOCATOR_START_ADDR) must be a valid multiple of FLASH_PAGE_SIZE
// (even 0 is fine)
pub const FLASH_ALLOCATOR_START_SCAN_ADDR: u32 = 0x0800_4800; // 18432 bytes for the kernel

pub const FLASH_START_ADDR: u32 = 0x0800_0000;
pub const FLASH_END_ADDR: u32 = 0x0807_FFFF;
// pub const FLASH_SIZE: usize = (FLASH_END_ADDR - FLASH_START_ADDR + 1) as usize; // 0x8000 -> 32768

pub const FLASH_BLOCK_SIZE: usize = 2048; // Single page size
pub const FLASH_NUM_BLOCKS: usize = FLASH_ALLOCATOR_SIZE / FLASH_BLOCK_SIZE as usize; // 256
pub const FLASH_TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
pub const FLASH_NUM_NODES: usize = 2 * FLASH_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1
pub const FLASH_FLAG_BYTES: usize = 2; // 16-bits

pub const FLASH_PAGE_SIZE: u32 = 2048;

// Flash operation timings: worst case scenario
pub const FLASH_ERASE_MS: u32 = 40;
pub const FLASH_WRITES_PER_MS: u32 = 1000 / 60 * 2;

// Compile time checks
static_assertions::const_assert_eq!((2 << (FLASH_TREE_MAX_LEVEL +1 - 1)) - 1, FLASH_NUM_NODES);

/**
 * STM32 F303RE
 * - RAM Constant
 *
 * RAM: 0x2000 0000 - 0x2000 FFFF
 * Size: 64Kb
 */
pub const SRAM_START_ADDR: u32 = 0x2000_0000;
pub const SRAM_END_ADDR: u32 = 0x2000_FFFF;
pub const SRAM_SIZE: usize = (SRAM_END_ADDR - SRAM_START_ADDR + 1) as usize; // 64Kb

pub const SRAM_RESERVED: u32 = 4352; // Kernel memory (must be a multiple of 256)
pub const SRAM_BLOCK_SIZE: usize = 256;
pub const SRAM_NUM_BLOCKS: usize = SRAM_SIZE / SRAM_BLOCK_SIZE as usize; // 256
pub const SRAM_TREE_MAX_LEVEL: usize = 8; // log2(num_blocks) = log2(memory_area / block_size)
pub const SRAM_NUM_NODES: usize = 2 * SRAM_NUM_BLOCKS - 1; // 2^(log2(num_blocks) +1) -1 = 2*num_blocks - 1

// Compile time checks
static_assertions::const_assert_eq!((2 << (SRAM_TREE_MAX_LEVEL +1 - 1)) - 1, SRAM_NUM_NODES);

/**
 * STM32 F303RE
 * - Flash Interface
 *
 * Provides methods to read, but expecially write and erase
 * the flash memory.
 */
const FLASH_KEY1: u32 = 0x4567_0123;
const FLASH_KEY2: u32 = 0xCDEF_89AB;

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum FlashError {
    PROGRAM_ERROR,
    WRITE_PROTECTION_ERROR,
}

pub struct Flash<
    'b,
    const FLASH_START_ADDRESS: u32,
    const PAGE_SIZE: u32,
    const FLASH_END_ADDRESS: u32,
> {
    flash: &'b device::flash::RegisterBlock,
    write_buffer: [u8; 2],
    target_address: u32,
    last_error: Option<FlashError>,
}

impl<'b, const FLASH_START_ADDRESS: u32, const PAGE_SIZE: u32, const FLASH_END_ADDRESS: u32>
    Flash<'b, FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    pub fn new() -> Self {
        // To be removed, the code to unlock the flash is put here
        let flash = unsafe { &*device::FLASH::ptr() };
        Self::unlock_flash(flash);
        Self {
            flash: flash,
            write_buffer: [0xFF; 2],
            target_address: 0,
            last_error: None,
        }
    }

    pub fn get_last_error(&self) -> Option<FlashError> {
        self.last_error
    }

    fn unlock_flash(flash: &'b device::flash::RegisterBlock) {
        // Check if already unlocked
        if !flash.cr.read().lock().bit() {
            return;
        }
        // Unlock the main bank
        flash.keyr.write(|w| w.bits(FLASH_KEY1));
        flash.keyr.write(|w| w.bits(FLASH_KEY2));
        // Assert we unlocked the bank
        assert!(!flash.cr.read().lock().bit());
    }

    fn wait_flash_operation(&mut self) -> Result<(), FlashError> {
        let mut error: Option<FlashError> = None;
        // Wait until the operation is completed (reset SR.BSY)
        // (theoretically the program itself can stall)
        loop {
            if self.flash.sr.read().bsy().bit_is_clear() {
                break;
            }
        }
        // Read and clear flags of SR
        self.flash.sr.modify(|r, w| {
            if r.eop().bit() {
                w.eop().set_bit(); // Clear bit
            }
            if r.pgerr().bit() {
                error = Some(FlashError::PROGRAM_ERROR);
                w.pgerr().set_bit(); // Clear bit
            }
            if r.wrprterr().bit() {
                error = Some(FlashError::WRITE_PROTECTION_ERROR);
                w.wrprterr().set_bit(); // Clear bit
            }
            w
        }); // Reset by writing 1 (see pag. 79)

        // Check if errors
        if error.is_some() {
            return Err(error.unwrap());
        }

        return Ok(());
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        let data: u16 = u16::from_le_bytes(self.write_buffer);
        let actual_data: u16 =
            unsafe { core::ptr::read_volatile(self.target_address as *const u16) };
        // Check if no change, do not write just skip
        if data == actual_data {
            // Reset status
            self.target_address = 0;
            return Ok(());
        }
        // Check if we would hard-fauld continuing
        if data != 0x0000 && actual_data != 0xFFFF {
            self.target_address = 0;
            return Err(());
        }
        // Write PROGRAM flag
        self.flash.cr.modify(|_, w| w.pg().set_bit());
        unsafe {
            core::ptr::write_volatile((self.target_address) as *mut u16, data);
        }
        // Wait result of the operation and clear flags
        let result = self.wait_flash_operation();
        // Reset PROGRAM flag
        self.flash.cr.modify(|_, w| w.pg().clear_bit());
        // Save error if any
        if result.is_err() {
            self.last_error = Some(result.unwrap_err());
            self.target_address = 0;
            return Err(());
        }

        // Reset status
        self.target_address = 0;
        Ok(())
    }

    fn write_u8(&mut self, address: u32, value: u8) -> Result<(), ()> {
        // In STM32F303, we must write 16bits at a time. Half writes or other "tricks" does
        // not work, as the flash controller checks the whole word is 0xFFFF before proceding
        // with the write. It's always possible to write 0x0000 in any situation, as the only exception.

        // Every write is then bufferized, and then the buffer flushed automatically whenever possible
        // (on the high byte of the word)
        let is_high_byte: bool = address % 2 > 0;
        let base_address = address - is_high_byte as u32; // Realign base address

        // Check whether we already have a buffer filling up
        if self.target_address > 0 {
            // Check whether we are changing base
            if self.target_address != base_address {
                // In this case force an automatic flush, but then the high byte of the
                // word cannot be written before the next erase. Maybe simply fail?
                self.flush_write_buffer()?;
            }
        }

        // Buffer empty, populate for this write
        if self.target_address == 0 {
            // Fill the buffer with the current data
            let current_word: u16 = unsafe { core::ptr::read_volatile(base_address as *const u16) };
            self.write_buffer[0] = (current_word & 0xFF) as u8;
            self.write_buffer[1] = (current_word >> 8) as u8;
        }

        // Check whether this operation is possible (not strict, just common case)
        if self.write_buffer[is_high_byte as usize] != 0xFF {
            // We can only admit a 0x0000 or the same value (we will skip the write)
            if value != 0x00 && value != self.write_buffer[is_high_byte as usize] {
                return Err(());
            }
        }

        // Set the base
        self.target_address = base_address;
        // Set the new byte
        self.write_buffer[is_high_byte as usize] = value;
        // Automatic flush when we get enough data
        if is_high_byte {
            self.flush_write_buffer()?;
        }
        Ok(())
    }
    pub fn page_from_address(address: u32) -> Option<FlashPage> {
        if address <= FLASH_END_ADDRESS {
            let offset = address - FLASH_START_ADDRESS;
            let page_num = offset / PAGE_SIZE;
            let base_addr = FLASH_START_ADDRESS + page_num * PAGE_SIZE;
            return Some(FlashPage::new(page_num as u16, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    pub fn page_from_number(page_num: u16) -> Option<FlashPage> {
        let max_num: u32 = (FLASH_END_ADDRESS - FLASH_START_ADDRESS + 1) / PAGE_SIZE;
        if page_num < max_num as u16 {
            let base_addr = FLASH_START_ADDRESS + page_num as u32 * PAGE_SIZE;
            return Some(FlashPage::new(page_num, base_addr, PAGE_SIZE as u16));
        }
        return None;
    }
    pub fn prev_page(page_num: u16) -> Option<FlashPage> {
        let prev_num = page_num - 1;
        Self::page_from_number(prev_num)
    }
}

impl<
        'a,
        'b,
        const FLASH_START_ADDRESS: u32,
        const PAGE_SIZE: u32,
        const FLASH_END_ADDRESS: u32,
    > FlashMethods<'a> for Flash<'b, FLASH_START_ADDRESS, PAGE_SIZE, FLASH_END_ADDRESS>
{
    fn read(&self, address: u32, buffer: &mut [u8]) -> Result<(), ()> {
        // Validate read address
        if address < FLASH_START_ADDRESS || address + (buffer.len() as u32) > FLASH_END_ADDRESS {
            return Err(());
        }
        // Negate write if this includes pending writes
        // TODO: maybe read considering the buffer? How to compose the abstraction?
        if self.target_address > 0 {
            if self.target_address >= address
                && self.target_address <= address + (buffer.len() as u32)
            {
                return Err(());
            }
        }
        // Actually perform the operation
        buffer.copy_from_slice(unsafe {
            core::slice::from_raw_parts(address as *const u8, buffer.len())
        });
        Ok(())
    }

    fn write(&mut self, address: u32, data: &[u8]) -> Result<(), ()> {
        // Write the bytes singularly, using the legacy method.
        // In this way we easily take into account odd lengths
        for i in 0..data.len() {
            self.write_u8(address + i as u32, data[i])?;
        }
        Ok(())
    }

    fn flush_write_buffer(&mut self) -> Result<(), ()> {
        // Nothing to flush
        if self.target_address == 0 {
            return Ok(());
        }
        // Perform flush
        self.flush_write_buffer()
    }
    fn erase(&mut self, page_num: u16) -> Result<(), ()> {
        // Check the page exists
        let page = self.page_from_number(page_num).ok_or(())?;

        // Write PER bit in CR
        self.flash.cr.modify(|_, w| w.per().set_bit());
        // Set page FLASH_AR
        self.flash.ar.write(|w| w.bits(page.base_address()));
        // Launch erase operation
        self.flash.cr.modify(|_, w| w.strt().set_bit());
        // Wait result of the operation and clear flags
        let result = self.wait_flash_operation();
        // Reset bit
        self.flash
            .cr
            .modify(|_, w| w.strt().clear_bit().per().clear_bit());

        // Check for errors
        if result.is_err() {
            self.last_error = Some(result.unwrap_err());
            return Err(());
        }

        Ok(())
    }
    fn page_from_address(&self, address: u32) -> Option<FlashPage> {
        Self::page_from_address(address)
    }
    fn page_from_number(&self, page_num: u16) -> Option<FlashPage> {
        Self::page_from_number(page_num)
    }
    fn prev_page(&self, page_num: u16) -> Option<FlashPage> {
        Self::prev_page(page_num)
    }
}
