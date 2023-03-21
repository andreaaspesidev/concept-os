// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::{mem::MaybeUninit, slice};

use crate::{
    log::sys_log,
    startup::{with_irq_table, HUBRIS_STORAGE_ANALYZE_NOTIFICATION},
    task::{NotificationSet, Task},
};
use abi::{
    flash::BlockType, u32_from_le_bytes_raw, InterruptOwner, RegionAttributes,
    RegionDescriptor, TaskDescriptor, TaskFlags, TaskId, HUBRIS_MAX_IRQS,
    HUBRIS_MAX_SUPPORTED_TASKS, REGIONS_PER_TASK,
};
use flash_allocator::flash::FlashBlock;
use cbf_lite::{BufferReaderImpl, CbfFile};
use unwrap_lite::UnwrapLite;

/**
 * Custom kernel structures (based partially on heapless)
 */

/// KVec: wrapper around a fixed size array, where items can only be added. It
///       provides an iterator over the elements
#[derive(Debug)]
pub struct KVec<T, const N: usize> {
    /// Current number of elements contained
    len: usize,
    /// Buffer where data is stored
    buffer: [MaybeUninit<T>; N],
}
impl<T, const N: usize> KVec<T, N> {
    const ELEM: MaybeUninit<T> = MaybeUninit::uninit();
    const INIT: [MaybeUninit<T>; N] = [Self::ELEM; N];
    /// Constructs a new, empty vector with a fixed capacity of `N`
    pub const fn new() -> Self {
        Self {
            len: 0,
            buffer: Self::INIT,
        }
    }

    /// Removes all the elements from the vector
    pub fn clear(&mut self) {
        // We drop each element used in the vector by turning into a &mut[T]
        unsafe {
            core::ptr::drop_in_place(self.as_mut_slice());
        }
        self.len = 0;
    }

    /// Extracts a mutable slice containing the entire vector.
    ///
    /// Equivalent to `&s[..]`.
    pub fn as_slice(&self) -> &[T] {
        // NOTE(unsafe) avoid bound checks in the slicing operation
        // &buffer[..self.len]
        unsafe {
            core::slice::from_raw_parts(
                self.buffer.as_ptr() as *const T,
                self.len,
            )
        }
    }

    pub fn first(&self) -> Option<&T> {
        if self.len > 0 {
            return Some(unsafe { self.buffer[0].assume_init_ref() });
        }
        return None;
    }

    /// Extracts a mutable slice containing the entire vector.
    pub(crate) fn as_mut_slice(&mut self) -> &mut [T] {
        // NOTE(unsafe) avoid bound checks in the slicing operation
        // &mut buffer[..self.len]
        unsafe {
            slice::from_raw_parts_mut(
                self.buffer.as_mut_ptr() as *mut T,
                self.len,
            )
        }
    }

    /// Returns the maximum number of elements the vector can hold.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Appends an `item` to the back of the collection
    ///
    /// # Safety
    ///
    /// This assumes the vec is not full.
    pub unsafe fn push_unchecked(&mut self, item: T) {
        // NOTE(ptr::write) the memory slot that we are about to write to is uninitialized. We
        // use `ptr::write` to avoid running `T`'s destructor on the uninitialized memory
        unsafe {
            *self.buffer.get_unchecked_mut(self.len) = MaybeUninit::new(item)
        };

        self.len += 1;
    }
    /// Appends an `item` to the back of the collection
    ///
    /// Returns back the `item` if the vector is full
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.len < self.capacity() {
            unsafe { self.push_unchecked(item) }
            Ok(())
        } else {
            Err(item)
        }
    }
    /// Returns true if the vec is full
    #[inline]
    pub fn is_full(&self) -> bool {
        self.len == self.capacity()
    }

    /// Returns true if the vec is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T: PartialEq, const N: usize> KVec<T, N> {
    /// Searches if the element is in the vector,
    /// returning its index
    pub fn search(&self, value: T) -> Option<usize> {
        for i in 0..self.len {
            if unsafe { self.buffer[i].assume_init_read() } == value {
                return Some(i);
            }
        }
        return None;
    }
}

impl<T: Copy, const N: usize> KVec<T, N> {
    pub fn extend_from(&mut self, v: KVec<T, N>) -> Result<(), ()> {
        // Check if we have enough space
        if v.len > self.capacity() - self.len {
            return Err(());
        }
        // Add the elements
        for e in v.into_iter() {
            unsafe { self.push_unchecked(*e) };
        }
        return Ok(());
    }
}

// Trait implementations
impl<T, const N: usize> Default for KVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T, const N: usize> Drop for KVec<T, N> {
    fn drop(&mut self) {
        // We drop each element used in the vector by turning into a &mut[T]
        unsafe {
            core::ptr::drop_in_place(self.as_mut_slice());
        }
    }
}
impl<'a, T, const N: usize> IntoIterator for &'a KVec<T, N> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().iter()
    }
}
impl<'a, T, const N: usize> IntoIterator for &'a mut KVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}

/// KHash: simple hash-map with linear probing using integer keys
/// To reduce the code produced, it was chosen to use u16 as key regardless.
struct KBucket<T> {
    /// Key associated to the element. If present, then the value was initialized
    key: Option<u16>,
    /// Value contained in the bucket
    value: MaybeUninit<T>,
}

impl<T> KBucket<T> {
    const INIT: Self = Self {
        key: None,
        value: MaybeUninit::uninit(),
    };
    pub fn is_free(&self) -> bool {
        self.key.is_none()
    }
    pub fn init(&mut self, key: u16, value: T) {
        assert!(self.key.is_none()); // Otherwise memory leak, as we do not drop the value
        self.key = Some(key);
        self.value = MaybeUninit::new(value);
    }
    pub fn reset(&mut self) {
        assert!(self.key.is_some());
        self.key = None;
        // Manually drop the element
        unsafe { core::ptr::drop_in_place(self.value.assume_init_mut()) };
        self.value = MaybeUninit::uninit();
    }
    pub fn take_ref(&self) -> &T {
        return unsafe { self.value.assume_init_ref() };
    }
    pub fn take_mut_ref(&mut self) -> &mut T {
        return unsafe { self.value.assume_init_mut() };
    }
    pub fn matches(&self, key: u16) -> bool {
        if let Some(k) = self.key {
            return k == key;
        }
        return false;
    }
}

pub struct KHash<T, const N: usize> {
    /// Buffer where data is stored
    buffer: [KBucket<T>; N],
}

impl<T, const N: usize> KHash<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: [KBucket::<T>::INIT; N],
        }
    }
    pub const fn capacity(&self) -> usize {
        N
    }
    /// Inserts an element associated with this key, returning the element
    /// on failure. If an old element is present, a bool true is returned
    pub fn insert(&mut self, key: u16, value: T) -> Result<bool, T> {
        // Start searching from the index associated with the key
        let start_index = key as usize % self.capacity();
        let mut index = start_index;
        // Scan for free space up to the end of the buffer
        while index < self.capacity() {
            if self.buffer[index].is_free() {
                self.buffer[index].init(key, value);
                return Ok(false);
            } else {
                // Check it does not contain this key
                if self.buffer[index].matches(key) {
                    // Just replace the value and return the old one
                    self.buffer[index].reset();
                    self.buffer[index].init(key, value);
                    return Ok(true);
                }
            }
            index += 1;
        }
        // Scan for free space from the start of buffer up to start_index
        index = 0;
        while index < start_index {
            if self.buffer[index].is_free() {
                self.buffer[index].init(key, value);
                return Ok(false);
            } else {
                // Check it does not contain this key
                if self.buffer[index].matches(key) {
                    // Just replace the value and return the old one
                    self.buffer[index].reset();
                    self.buffer[index].init(key, value);
                    return Ok(true);
                }
            }
            index += 1;
        }
        // Failed
        return Err(value);
    }
    /// Searches an element based on the key
    pub fn get(&self, key: u16) -> Option<&T> {
        // Start searching from the index associated with the key
        let start_index = key as usize % self.capacity();
        let mut index = start_index;
        // Scan for free space up to the end of the buffer
        while index < self.capacity() {
            if self.buffer[index].matches(key) {
                return Some(self.buffer[index].take_ref());
            }
            index += 1;
        }
        // Scan for free space from the start of buffer up to start_index
        index = 0;
        while index < start_index {
            if self.buffer[index].matches(key) {
                return Some(self.buffer[index].take_ref());
            }
            index += 1;
        }
        // Failed
        return None;
    }
    pub fn get_mut(&mut self, key: u16) -> Option<&mut T> {
        // Start searching from the index associated with the key
        let start_index = key as usize % self.capacity();
        let mut index = start_index;
        // Scan for free space up to the end of the buffer
        while index < self.capacity() {
            if self.buffer[index].matches(key) {
                return Some(self.buffer[index].take_mut_ref());
            }
            index += 1;
        }
        // Scan for free space from the start of buffer up to start_index
        index = 0;
        while index < start_index {
            if self.buffer[index].matches(key) {
                return Some(self.buffer[index].take_mut_ref());
            }
            index += 1;
        }
        // Failed
        return None;
    }
    pub fn keys(&self) -> KVec<u16, N> {
        let mut result = KVec::new();
        for bucket in &self.buffer {
            if !bucket.is_free() {
                unsafe { result.push_unchecked(bucket.key.unwrap_lite()) };
            }
        }
        return result;
    }
    pub fn into_iter(&self) -> KHashIter<T, N> {
        KHashIter {
            khash: &self,
            pos: 0,
        }
    }
    pub fn first(&self) -> Option<(u16, &T)> {
        let mut index = 0;
        while index < self.capacity() {
            if !self.buffer[index].is_free() {
                return Some((
                    self.buffer[index].key.unwrap_lite(),
                    self.buffer[index].take_ref(),
                ));
            }
            index += 1;
        }
        return None;
    }
}

impl<T: Clone + Copy, const N: usize> KHash<T, N> {
    /// Returns an array of all the values currently stored in the structure
    pub fn values(&self) -> KVec<T, N> {
        let mut result = KVec::<T, N>::new();
        for bucket in &self.buffer {
            if !bucket.is_free() {
                unsafe { result.push_unchecked(*bucket.take_ref()) };
            }
        }
        return result;
    }
    /// Removes an element, returning if it's found
    pub fn remove(&mut self, key: u16) -> Result<T, ()> {
        // Start searching from the index associated with the key
        let start_index = key as usize % self.capacity();
        let mut index = start_index;
        // Scan for free space up to the end of the buffer
        while index < self.capacity() {
            if self.buffer[index].matches(key) {
                let value = *self.buffer[index].take_ref();
                self.buffer[index].reset();
                return Ok(value);
            }
            index += 1;
        }
        // Scan for free space from the start of buffer up to start_index
        index = 0;
        while index < start_index {
            if self.buffer[index].matches(key) {
                let value = *self.buffer[index].take_ref();
                self.buffer[index].reset();
                return Ok(value);
            }
            index += 1;
        }
        // Failed
        return Err(());
    }
}

pub struct KHashIter<'a, T, const N: usize> {
    khash: &'a KHash<T, N>,
    pos: usize,
}

impl<'a, T, const N: usize> Iterator for KHashIter<'a, T, N> {
    type Item = (u16, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        // Get the next non-free element
        for i in self.pos..self.khash.capacity() {
            if !self.khash.buffer[i].is_free() {
                self.pos = i + 1;
                return Some((
                    self.khash.buffer[i].key.unwrap_lite(),
                    self.khash.buffer[i].take_ref(),
                ));
            }
        }
        return None;
    }
}

/**
 * Kernel structures operations
 */
/// This structure is responsible for the mapping "component id -> component index"
pub struct TaskIndexes {
    /// Hashmap component_id => component_index
    hash: KHash<usize, HUBRIS_MAX_SUPPORTED_TASKS>,
    /// Array of bits, each is true if the corresponding index is associated
    /// to a component index currently in use
    indexes_mask: [bool; HUBRIS_MAX_SUPPORTED_TASKS],
    /// This structure and the above one could be merged into a single one,
    /// but here are kept separated for simplicity
    valid_ordered_indexes: [usize; HUBRIS_MAX_SUPPORTED_TASKS],
    valid_ordered_indexes_len: usize
}

impl TaskIndexes {
    pub fn new() -> Self {
        Self {
            hash: KHash::new(),
            indexes_mask: [false; HUBRIS_MAX_SUPPORTED_TASKS],
            valid_ordered_indexes: [0; HUBRIS_MAX_SUPPORTED_TASKS],
            valid_ordered_indexes_len: 0
        }
    }

    /// Drops and recreate the list starting from the indexes.
    /// More advanced alg. can be used, but remember we do not need to optimize
    /// the insertion and removal, but the access times.
    fn recreate_ordered_indexes(&mut self) {
        let mut curr_pos: usize = 0;
        for i in 0..self.indexes_mask.len() {
            let in_use = self.indexes_mask[i];
            if in_use {
                self.valid_ordered_indexes[curr_pos] = i;
                curr_pos += 1;
            }
        }
        self.valid_ordered_indexes_len = curr_pos;
    }

    /// Gets the index (if any) associated to the current component id
    pub fn get_task_index(&self, component_id: u16) -> Option<usize> {
        self.hash.get(component_id).map(|e| *e)
    }

    pub fn change_id_of_index(
        &mut self,
        old_id: u16,
        new_id: u16,
    ) -> Result<(), ()> {
        // First, get the old index
        if let Some(old_index) = self.get_task_index(old_id) {
            // Now, remove this association
            self.hash.remove(old_id).unwrap_lite();
            // Insert under the new id
            self.hash.insert(new_id, old_index).unwrap_lite();
            // No need to change the indexes_mask, as we are not changing index
            return Ok(());
        }
        return Err(());
    }

    /// Gets the next free index and associate it to a component id. Returns the index on success
    pub fn get_free_index(&mut self, component_id: u16) -> Option<usize> {
        // Just scan the mask to get the first free element
        for i in 0..self.indexes_mask.len() {
            if self.indexes_mask[i] == false {
                // Got the index: i
                if self.hash.insert(component_id, i).is_ok() {
                    // Mark as used
                    self.indexes_mask[i] = true;
                    self.recreate_ordered_indexes();
                    return Some(i);
                }
                return None;
            }
        }
        // No more available ids
        return None;
    }
    /// Release an index currently associated to a component. Returns the index on success
    pub fn set_free_index(&mut self, component_id: u16) -> Option<usize> {
        // Search the element
        if let Ok(i) = self.hash.remove(component_id) {
            // Set as free
            self.indexes_mask[i] = false;
            self.recreate_ordered_indexes();
            return Some(i);
        }
        return None;
    }

    pub fn ids(&self) -> KVec<u16, HUBRIS_MAX_SUPPORTED_TASKS> {
        self.hash.keys()
    }

    pub fn indexes(&self) -> KVec<usize, HUBRIS_MAX_SUPPORTED_TASKS> {
        self.hash.values()
    }

    pub fn indexes_mask(&self) -> &[bool; HUBRIS_MAX_SUPPORTED_TASKS] {
        &self.indexes_mask
    }

    pub fn valid_indexes(&self) -> &[usize] {
        &self.valid_ordered_indexes[0..self.valid_ordered_indexes_len]
    }

    pub fn into_iter(&self) -> KHashIter<usize, HUBRIS_MAX_SUPPORTED_TASKS> {
        self.hash.into_iter()
    }
    pub fn first_index(&self) -> Option<usize> {
        self.hash.first().map(|(_id, index)| *index)
    }
}

pub fn populate_kernel_structures(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
) {
    // Get an iterator for the flash
    let flash_walker = crate::arch::get_flash_walker();
    // Iterate, to find CBFs
    for b in flash_walker {
        if !b.is_finalized() {
            sys_log!("Not finalized block found at: {}", b.get_base_address());
            continue;
        }
        // Look into only finalized blocks of components
        if b.is_finalized() && b.get_type() == BlockType::COMPONENT {
            // Load the component
            let (task_descr, task_regions, task_data) =
                get_task_from_block(b, true).unwrap_lite();
            let task_id = task_descr.component_id();
            add_task_to_system(
                task_list,
                task_map,
                irq_map,
                &task_descr,
                &task_regions,
                task_data,
                task_id,
            )
            .unwrap_lite();
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum LoadError {
    InvalidBlockPointer,
    InvalidBlock,
    MalformedCBF,
    TooManyIRQs,
    TooManyTasks,
}

fn get_task_from_block(
    block: FlashBlock,
    validate: bool,
) -> Result<
    (
        TaskDescriptor,
        KVec<RegionDescriptor, REGIONS_PER_TASK>,
        &'static [u8],
    ),
    LoadError,
> {
    // Let's create an abstraction to read its bytes
    let raw_block_bytes = unsafe {
        core::slice::from_raw_parts(
            (block.get_base_address() + 8) as *const u8,
            block.get_size() as usize,
        )
    };
    let block_reader = BufferReaderImpl::from(raw_block_bytes);
    // Let's read the cbf
    let cbf_parse = cbf_lite::CbfFile::from_reader(&block_reader);
    if cbf_parse.is_err() {
        sys_log!("Malformed CBF at {:#010x}", block.get_base_address());
        return Err(LoadError::MalformedCBF);
    }
    let cbf = cbf_parse.unwrap_lite();
    if validate {
        // Validate this cbf
        let cbf_valid = cbf.validate().unwrap_or(false);
        if !cbf_valid {
            sys_log!("Malformed CBF at {:#010x}", block.get_base_address());
            return Err(LoadError::MalformedCBF);
        }
    }
    Ok(process_cbf(
        &cbf,
        block.get_nominal_base_address(),
        block.get_base_address(),
        block.get_nominal_size(),
    ))
}

fn process_cbf(
    cbf: &CbfFile,
    block_nominal_base_address: u32,
    block_base_address: u32,
    block_nominal_size: u32,
) -> (
    TaskDescriptor,
    KVec<RegionDescriptor, REGIONS_PER_TASK>,
    &'static [u8],
) {
    // Create a new instance of Task
    let task_desc = TaskDescriptor::new(block_base_address, block_nominal_size); // Nominal size is actually bigger than needed. It's only used for cbf reading so here it's okay
    let mut regions: KVec<RegionDescriptor, REGIONS_PER_TASK> = KVec::new();
    // Create a region for the SRAM
    let sram_base: u32 = unsafe { u32_from_le_bytes_raw(block_base_address) };
    let sram_size: u32 =
        unsafe { u32_from_le_bytes_raw(block_base_address + 4) };
    let sram_region = RegionDescriptor {
        base: sram_base,
        size: sram_size,
        attributes: RegionAttributes::READ
            | RegionAttributes::WRITE
            | RegionAttributes::EXECUTE,
    };
    regions.push(sram_region).unwrap_lite();
    // Create a sregion for the FLASH
    let flash_region = RegionDescriptor {
        base: block_nominal_base_address,
        size: block_nominal_size,
        attributes: RegionAttributes::READ
            | RegionAttributes::WRITE
            | RegionAttributes::EXECUTE,
    };
    regions.push(flash_region).unwrap_lite();
    let cbf_base = cbf.header_base().unwrap_lite();
    // Append all the other regions
    for region_num in 0..cbf_base.num_regions() {
        // TODO: check regions alignment, or we will have an hard fault in the kernel
        // when setting the MPU
        let region = cbf.region_nth(region_num).unwrap_lite();
        regions
            .push(RegionDescriptor {
                base: region.base_address(),
                size: region.size(),
                attributes: unsafe {
                    RegionAttributes::from_bits_unchecked(
                        region.attributes().bits(),
                    )
                },
            })
            .unwrap_lite();
    }
    // Extract the data section
    let data_section = cbf.get_data_payload().unwrap_lite();
    let mut data_section_slice: &'static [u8] = &[];
    if data_section.is_some() {
        let ds = data_section.unwrap_lite();
        let data_address = block_base_address + 8 + ds.get_offset();
        data_section_slice = unsafe {
            core::slice::from_raw_parts(
                data_address as *const u8,
                ds.size() as usize,
            )
        };
    }
    // Create the task structure
    return (task_desc, regions, data_section_slice);
}

fn remove_task_from_system(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: Option<&mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>>,
    task_id: u16,
) {
    // Start by flushing IRQs
    let task_search = task_map.get_task_index(task_id);
    if task_search.is_none() {
        return; // Simply ignore
    }
    let task = &task_list[task_search.unwrap_lite()];
    if let Some(irq_map) = irq_map {
        for interrupt_num in 0..task.descriptor().num_interrupts() {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            irq_map
                .remove(interrupt.irq_num as u16)
                .unwrap_lite();
        }
    }
    // Mark the corresponding block for removal
    unsafe {
        crate::arch::dismiss_block(task.descriptor().get_descriptor_block())
            .unwrap_lite();
    }
    // Remove the task from the map (and to be sure, replace the corresponding element)
    let old_index = task_map.set_free_index(task_id).unwrap_lite();
    // Clear this element
    unsafe { task_list[old_index].reset_element() };
}

fn add_task_to_system(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
    task_descriptor: &TaskDescriptor,
    task_regions: &KVec<RegionDescriptor, REGIONS_PER_TASK>,
    task_data: &'static [u8],
    use_id: u16,
) -> Result<usize, LoadError> {
    // First, check if this component already exists
    let search_result = task_map.get_task_index(use_id);
    if search_result.is_some() {
        // Check the versions, if this is newer let's override everything
        let other_index = search_result.unwrap_lite();
        let other_task = &task_list[other_index];
        if task_descriptor.component_version()
            > other_task.descriptor().component_version()
        {
            sys_log!("Found an newer task for {}", use_id);
            // Delete the old task
            remove_task_from_system(task_list, task_map, Some(irq_map), use_id);
        } else {
            sys_log!("Found an older task for {}", use_id);
            // Ignore this task
            return Ok(other_index); // TODO: maybe an error is better here?
        }
    }
    // Get a space for the structure
    if let Some(new_index) = task_map.get_free_index(use_id) {
        // Initialize the corresponding task structure
        // No need to set the ID, as the temp one will be set when .begin_update() is called
        task_list[new_index].init_from_descriptor(
            task_descriptor,
            task_regions,
            task_data,
        );
        // Add the IRQs
        let num_irqs = task_list[new_index].descriptor().num_interrupts();
        for interrupt_num in 0..num_irqs {
            let interrupt = task_list[new_index]
                .descriptor()
                .interrupt_nth(interrupt_num);
            // Append the IRQ
            match irq_map.insert(
                interrupt.irq_num as u16,
                InterruptOwner {
                    task_id: use_id,
                    notification: interrupt.notification,
                },
            ) {
                Ok(old_val) => {
                    if old_val {
                        // Another component registered this IRQ, panic!
                        panic!("Duplicated IRQ: {}", interrupt.irq_num);
                    }
                }
                Err(_) => {
                    // TODO: clean-up
                    return Err(LoadError::TooManyIRQs);
                }
            };
        }
        // Return the index
        return Ok(new_index);
    }
    return Err(LoadError::TooManyTasks);
}

/// Insert a component in the system, and executes it
pub fn load_component_at(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    irq_map: &mut KHash<InterruptOwner, HUBRIS_MAX_IRQS>,
    block_base_address: u32,
) -> Result<usize, LoadError> {
    // Try to read this header (already checked for addresses)
    let block_header_search =
        crate::arch::get_flash_block(block_base_address, false);
    if block_header_search.is_none() {
        return Err(LoadError::InvalidBlock);
    }
    let block = block_header_search.unwrap_lite();
    // Check if this block has a component (but allow unfinalized blocks!)
    if block.get_type() != BlockType::COMPONENT {
        return Err(LoadError::InvalidBlock);
    }
    // Load the component
    let load_res = get_task_from_block(block, false);
    if let Ok((task_descr, task_regions, task_data)) = load_res {
        // Assume already validated
        // Check if an older component with this ID exist
        let nominal_id = task_descr.component_id();
        let old_task_index = task_map.get_task_index(nominal_id);
        if old_task_index.is_some() {
            let old_task = &mut task_list[old_task_index.unwrap_lite()];
            // Remove all its irqs, after disabling them
            for interrupt_num in 0..old_task.descriptor().num_interrupts() {
                let interrupt =
                    old_task.descriptor().interrupt_nth(interrupt_num);
                crate::arch::disable_irq(interrupt.irq_num);
                irq_map
                    .remove(interrupt.irq_num as u16)
                    .unwrap_lite();
            }
            // If the old component support it, now it can state transfer.
            // Otherwise is simply stopped.
            old_task.begin_state_transfer();
        }
        // Add the new task to the system
        let res = add_task_to_system(
            task_list,
            task_map,
            irq_map,
            &task_descr,
            &task_regions,
            task_data,
            abi::UPDATE_TEMP_ID,
        );
        if res.is_ok() {
            let task_index = res.unwrap_lite();
            // Initialize the task for update
            task_list[task_index].begin_update();
            // Setup internal
            crate::arch::reinitialize(&mut task_list[task_index]);
            return Ok(task_index);
        }
        return Err(res.unwrap_err());
    }
    return Err(load_res.unwrap_err());
}

pub fn revert_update(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
) {
    sys_log!("Reverting update for timeout");
    // Search for the new component
    let task_new = task_map.get_task_index(abi::UPDATE_TEMP_ID);
    if task_new.is_none() {
        return; // Ignore
    }
    let new_task = &task_list[task_new.unwrap_lite()];
    let nominal_id = new_task.descriptor().component_id();
    // Delete the new task
    with_irq_table(|irq_map| {
        remove_task_from_system(
            task_list,
            task_map,
            Some(irq_map),
            abi::UPDATE_TEMP_ID,
        );
    });
    // Get the old one
    let old_task_index = task_map.get_task_index(nominal_id);
    if old_task_index.is_none() {
        // Remove the new version
        let storage_task_index =
            task_map.get_task_index(abi::STORAGE_ID).unwrap_lite();
        let storage_task = &mut task_list[storage_task_index];
        storage_task.post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
        return; // Ignore, we have nothing to revert to
    }
    let task_index = task_map.get_task_index(nominal_id).unwrap_lite();
    // Re-map IRQs of the old one. Do not reenable them, as the task will restart
    // and enable them itself.
    with_irq_table(|irq_map| {
        let task = &task_list[task_index];
        for interrupt_num in 0..task.descriptor().num_interrupts() {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            irq_map
                .insert(
                    interrupt.irq_num as u16,
                    InterruptOwner {
                        task_id: nominal_id,
                        notification: interrupt.notification,
                    },
                )
                .unwrap_lite();
        }
    });
    // Reset the old one
    let old_task = &mut task_list[task_index];
    old_task.reinitialize();
    if old_task
        .descriptor()
        .flags()
        .contains(TaskFlags::START_AT_BOOT)
    {
        old_task.set_healthy_state(abi::SchedState::Runnable);
    }
    // Remove the new version
    let storage_task_index = task_map.get_task_index(abi::STORAGE_ID).unwrap_lite();
    let storage_task = &mut task_list[storage_task_index];
    storage_task.post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
}

pub fn activate_component(
    task_list: &mut [Task; HUBRIS_MAX_SUPPORTED_TASKS],
    task_map: &mut TaskIndexes,
    caller_index: usize,
    _nominal_id: u16,
) -> bool {
    // Read the nominal id of the task
    let nominal_id = task_list[caller_index].descriptor().component_id();
    // Process the old component, if it exists
    let old_task_index = task_map.get_task_index(nominal_id);
    let mut old_identifier: Option<TaskId> = None;
    if old_task_index.is_some() {
        let old_task_index = old_task_index.unwrap_lite();
        // Before removing, save the generation
        old_identifier = Some(task_list[old_task_index].current_identifier());
        // Remove the component
        remove_task_from_system(task_list, task_map, None, nominal_id);
        // Do not schedule the block for removal here, as we might not have the task
        // yet, if we are updating the storage component
    }
    // Update the task, and remap it under the new id
    let task = &mut task_list[caller_index];
    task_map
        .change_id_of_index(abi::UPDATE_TEMP_ID, nominal_id)
        .unwrap_lite();
    // Switch the mode on the task
    task.end_update(old_identifier.map(|id| id.generation().next()));
    // Redirect all IRQs
    with_irq_table(|irq_map| {
        let task = &task_list[caller_index];
        let tot_irqs = task.descriptor().num_interrupts();
        for interrupt_num in 0..tot_irqs {
            let interrupt = task.descriptor().interrupt_nth(interrupt_num);
            let entry = irq_map
                .get_mut(interrupt.irq_num as u16)
                .unwrap_lite();
            entry.task_id = nominal_id;
        }
    });
    let mut storage_woken: bool = false;
    if let Some(old_id) = old_identifier {
        // Restart pending tasks
        crate::task::restart_pending_tasks(
            task_list, task_map, caller_index, old_id,
        );
        // Now is safe to schedule the old block for removal
        let storage_index =
            task_map.get_task_index(abi::STORAGE_ID).unwrap_lite();
        storage_woken = task_list[storage_index]
            .post(NotificationSet(HUBRIS_STORAGE_ANALYZE_NOTIFICATION));
    }
    // Finalize block
    crate::arch::finalize_block(
        task_list[caller_index].descriptor().get_descriptor_block(),
    )
    .unwrap_lite();
    return storage_woken;
}
