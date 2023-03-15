// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::BTreeMap;

use serde::Deserialize;

/**
 * Structures
 */
#[derive(Deserialize, PartialEq, Debug)]
pub struct ComponentConfig {
    pub features: Vec<String>
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct AppConfig {
    pub name: String,
    pub board: String,
    pub kernel_ram: u32,
    pub clock_speed: u32,
    pub strip_panics: bool,
    pub components: BTreeMap<String, ComponentConfig>
}