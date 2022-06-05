use core::fmt;

use buddy_allocator::{BuddyAllocator, BuddyAllocatorImpl};

// Workaround to get a formatter
pub struct Fmt<F>(pub F) where F: Fn(&mut fmt::Formatter) -> fmt::Result;
impl<F> fmt::Debug for Fmt<F>
    where F: Fn(&mut fmt::Formatter) -> fmt::Result
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        (self.0)(f)
    }
}

fn main() {
    let mut bd = init_stm32f303e();
    // Create some allocations
    println!("Starting allocations at: 0x{:02X}", bd.start_addr());
    let size1: usize = 1024 as usize;
    let alloc1 = bd.alloc(size1).unwrap();
    println!("Allocated {} at 0x{:02X}", size1, alloc1);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
    let size2: usize = 3*1024 as usize;
    let alloc2 = bd.alloc(size2).unwrap();
    println!("Allocated {} at 0x{:02X}", size2, alloc2);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
    let size3: usize = 1024 as usize;
    let alloc3 = bd.alloc(size3).unwrap();
    println!("Allocated {} at 0x{:02X}", size3, alloc3);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
    bd.dealloc(alloc1, size1);
    println!("Deallocated {} at 0x{:02X}", size1, alloc1);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
    bd.dealloc(alloc2, size2);
    println!("Deallocated {} at 0x{:02X}", size2, alloc2);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
    bd.dealloc(alloc3, size3);
    println!("Deallocated {} at 0x{:02X}", size3, alloc3);
    println!("{:?}", &Fmt(|f| bd.dump(f)));
}

fn init_stm32f303e() -> impl BuddyAllocator {
    // SRAM: 0x2000 0000 - 0x2000 FFFF
    // Size: 64Kb
    const START_ADDR: u32 = 0x2000_0000;
    const END_ADDR: u32 = 0x2000_FFFF;
    const SIZE: usize = (END_ADDR - START_ADDR + 1) as usize;  // 0x10000 -> 2^16 -> 65536
    const BLOCK_SIZE: u16 = 1024;
    assert!(SIZE % BLOCK_SIZE as usize == 0);
    const NUM_BLOCKS: usize = SIZE / BLOCK_SIZE as usize; // 64
    const NUM_SLOTS: usize = 6 + 1;   // clog2(NUM_BLOCKS) + 1
    let bd = BuddyAllocatorImpl::<START_ADDR,END_ADDR,BLOCK_SIZE,NUM_BLOCKS,NUM_SLOTS>::new();
    println!("Required memory bytes: {}", core::mem::size_of::<BuddyAllocatorImpl::<START_ADDR,END_ADDR,BLOCK_SIZE,NUM_BLOCKS,NUM_SLOTS>>());
    return bd;
}