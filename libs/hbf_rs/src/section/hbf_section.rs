// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::HbfPayloadSection;

// Does not exist in the hbf
pub struct HbfPayloadSectionGen {
    pub offset: u32,
    pub size: u32,
}

impl HbfPayloadSection for HbfPayloadSectionGen {
    fn size(&self) -> u32 {
        self.size
    }

    fn offset(&self) -> u32 {
        self.offset
    }
}