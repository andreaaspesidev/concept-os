// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::CbfPayloadSection;

// Does not exist in the cbf
pub struct CbfPayloadSectionGen {
    pub offset: u32,
    pub size: u32,
}

impl CbfPayloadSection for CbfPayloadSectionGen {
    fn size(&self) -> u32 {
        self.size
    }

    fn offset(&self) -> u32 {
        self.offset
    }
}