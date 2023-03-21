// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;

/*
    Structures
*/
#[derive(Debug, Clone)]
pub struct CbfError {
    pub msg: String
}
impl fmt::Display for CbfError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl std::error::Error for CbfError {}

