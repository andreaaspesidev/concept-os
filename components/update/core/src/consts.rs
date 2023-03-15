// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub const READ_TIMEOUT_TICKS: u32 = 50_000;
pub const PACKET_BUFFER_SIZE: usize = 64;
pub const LINKED_FLASH_BASE: u32 = 0x0800_0000;
pub const LINKED_SRAM_BASE: u32 = 0x2000_0000;
pub const BUFF_SIZE: usize = 128;
pub const RELOC_BUFF_SIZE: usize = 16;

#[cfg(feature = "multi-support")]
pub const CHANNEL_ID: u16 = 5;