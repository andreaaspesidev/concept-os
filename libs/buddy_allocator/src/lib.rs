// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]

use core::fmt::Formatter;

mod binary_buddy;
mod legacy;

pub use binary_buddy::BinaryBuddyImpl;
pub use legacy::ListBuddyImpl;

/// Offers an interface for the chosen implementation of the allocator,
/// permitting the abstraction of the generics of that implementation.
pub trait BuddyAllocator {
    /// Returns the first memory address managed by this allocator
    fn start_addr(&self) -> u32;
    /// Maximum size managed by this allocator
    fn max_size(&self) -> usize;
    /// Whether a given physical address belongs to this allocator
    fn contains(&self, addr: u32) -> bool;
    /// Allocates memory for the requested size, 
    /// returning the address of the base of this allocation
    fn alloc(&mut self, size: usize) -> Option<u32>;

    /// Deallocated memory prev. allocated. 
    /// Considered unsafe as it's not possible to check here if at this address
    /// there is actually an allocated block.
    unsafe fn dealloc(&mut self, addr: u32, size: usize);

    /// Dump a structure representation to the specified formatter
    fn dump(&self, f: &mut Formatter) -> Result<(), core::fmt::Error>;

    /// Converts the requested size into the corresponding level 
    fn size_to_level(&self, size: usize) -> Option<usize>;

    /// Directly adds as free a basic block (size = 1)
    fn add_free_block(&mut self, block_num: usize) -> Option<()>;

    /// Checks whether all the reserved space is available for allocation
    fn is_all_available(&self) -> bool;
}