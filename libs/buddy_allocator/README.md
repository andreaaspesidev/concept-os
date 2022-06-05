# Buddy Allocator
This library provides a standard implementation of a binary buddy allocator,
that can be useful in various situations (for allocating RAM, Flash, Heap Allocator, ...).

The peculiarity of this implementation is that stores the information about the allocations outside of the memory itself and the library is `no_std`, `no_heap`.

Based on the implementation of [`Rust OS`](https://nfil.dev/kernel/rust/coding/rust-buddy-allocator/)
