// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Debug;

use crate::{
    header::{
        HbfHeaderBaseWrapper, HbfHeaderDependencyIter, HbfHeaderDependencyWrapper,
        HbfHeaderInterruptIter, HbfHeaderInterruptWrapper, HbfHeaderMainWrapper,
        HbfHeaderRegionIter, HbfHeaderRegionWrapper, HbfHeaderRelocationIter,
        HbfHeaderRelocationWrapper,
    },
    section::HbfPayloadSectionWrapper,
    trailer::HbfTrailerWrapper,
};

pub mod hbf;

pub trait HbfFile: Debug {
    fn content(&self) -> &[u8];

    fn header_base(&self) -> HbfHeaderBaseWrapper;
    fn header_main(&self) -> HbfHeaderMainWrapper;
    fn trailer(&self) -> HbfTrailerWrapper;

    fn region_nth(&self, index: usize) -> Option<HbfHeaderRegionWrapper>;
    fn region_iter(&self) -> HbfHeaderRegionIter;

    fn interrupt_nth(&self, index: usize) -> Option<HbfHeaderInterruptWrapper>;
    fn interrupt_iter(&self) -> HbfHeaderInterruptIter;

    fn relocation_nth(&self, index: usize) -> Option<HbfHeaderRelocationWrapper>;
    fn relocation_iter(&self) -> HbfHeaderRelocationIter;

    fn dependency_nth(&self, index: usize) -> Option<HbfHeaderDependencyWrapper>;
    fn dependency_iter(&self) -> HbfHeaderDependencyIter;

    fn read_only_section(&self) -> HbfPayloadSectionWrapper;
    fn data_section(&self) -> Option<HbfPayloadSectionWrapper>;
    fn bss_size(&self) -> u32;
    fn payload_size(&self) -> u32;

    fn checksum_offset(&self) -> u32;

    fn validate(&self) -> bool;
}
