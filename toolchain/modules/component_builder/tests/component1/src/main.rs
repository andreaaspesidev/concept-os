#![no_std]
#![no_main]

mod constants;
mod flash;

use constants::*;
use flash::Flash;
use flash_allocator::flash::FlashAllocatorImpl;
use userlib::*;

#[export_name = "main"]
fn main() -> ! {
    // Inst. the flash operators
    let mut flash = Flash::<FLASH_START_ADDR, FLASH_PAGE_SIZE, FLASH_END_ADDR>::new();
    // Inst. the allocator
    let mut allocator = FlashAllocatorImpl::<
        ALLOCATOR_START_ADDR,
        ALLOCATOR_END_ADDR,
        BLOCK_SIZE,
        NUM_BLOCKS,
        NUM_SLOTS,
        FLAG_SIZE,
    >::new(&mut flash);
    // Main loop
    let mut buffer = [0; 4];
    loop {
        // Wait for a command
        let rec_result = sys_recv_closed(&mut buffer, 0x0000_0000, TaskId(2));
        if rec_result.is_ok() {
            let message = rec_result.unwrap();
            if message.operation == 0x0001 {
                // Parse size
                let requested_size = u32::from_be_bytes(buffer);
                // Get the address
                let result = allocator.allocate(requested_size);
                if result.is_none() {
                    sys_reply(TaskId(2), 0x0, &[]);
                } else {
                    let addr = result.unwrap();
                    let buff = addr.to_be_bytes();
                    sys_reply(TaskId(2), 0x1, &buff);
                }
            } else if message.operation == 0x0002 {
                // Parse allocation address
                let address = u32::from_be_bytes(buffer);
                // Launch the deallocation
                allocator.deallocate(address);
                sys_reply(TaskId(2), 0x1, &[]);
            }
        }
    }
}
