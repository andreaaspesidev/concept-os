// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use userlib::{sys_send, TaskId};

#[no_mangle]
#[inline(never)]
pub fn print(text: &str) {
    // Mock call
    sys_send(
        TaskId(1), 
        1, 
        text.as_bytes(), 
        &mut [],
        &[],
    );
}