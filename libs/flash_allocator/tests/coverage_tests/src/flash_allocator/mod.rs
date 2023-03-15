// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![no_std]

/// Flag bytes is the minimum write granularity allowed on the target board.
/// As operations with generics are not stable yet in Rust and leads to strange
/// behaviours, it's hardcoded here, equal to the maximum supported granularity of the target board.
const FLAG_BYTES: usize = 8; // 64 bits

pub mod flash;

#[cfg(feature = "swap")]
pub mod swap;