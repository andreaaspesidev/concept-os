// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::{Formatter, Error, Write};

const COLUMNS: usize = 4;
const BYTES_PER_ELEMENT: usize = 4;
const BYTES_PER_ROW: usize = BYTES_PER_ELEMENT * COLUMNS;

pub fn dump_section(bytes: &[u8], f: &mut Formatter) -> Result<(), Error> {
    let mut index: usize = 0;
    let mut last_row_start: usize = 0;
    // Read next byte
    let mut byte: u8 = bytes[0];
    f.write_fmt(format_args!("{:02X} ", byte))?;
    index += 1;

    loop {
        // Read next byte
        byte = bytes[index];

        if index % BYTES_PER_ROW == 0 {
            f.write_str("   ")?;
            // Print textual representation
            dump_text(&bytes[last_row_start..index], f)?;
            // Go to next line
            f.write_char('\n')?;
            last_row_start = index;
        }

        // Print byte
        f.write_fmt(format_args!("{:02X} ", byte))?;

        index += 1;

        if index >= bytes.len() {
            f.write_str("   ")?;
            // Print textual representation
            dump_text(&bytes[last_row_start..index], f)?;
            return Ok(());
        }
    }
}

fn dump_text(mut bytes: &[u8], f: &mut Formatter) -> Result<(), Error> {
    loop {
        let byte = bytes[0];
        if byte >= 0x20 && byte <= 0x7E {
            f.write_char(byte as char)?;
        } else {
            f.write_char('.')?;
        }
        if bytes.len() == 1 {
            return Ok(());
        } else {
            bytes = &bytes[1..];
        }
    }
}