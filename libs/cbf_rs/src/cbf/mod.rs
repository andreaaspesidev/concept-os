// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use core::fmt::Debug;

use crate::{
    header::{
        CbfHeaderBaseWrapper, CbfHeaderDependencyIter, CbfHeaderDependencyWrapper,
        CbfHeaderInterruptIter, CbfHeaderInterruptWrapper, CbfHeaderMainWrapper,
        CbfHeaderRegionIter, CbfHeaderRegionWrapper, CbfHeaderRelocationIter,
        CbfHeaderRelocationWrapper,
    },
    section::CbfPayloadSectionWrapper,
    trailer::CbfTrailerWrapper,
};

pub mod cbf;

pub trait CbfFile: Debug {
    fn content(&self) -> &[u8];

    fn header_base(&self) -> CbfHeaderBaseWrapper;
    fn header_main(&self) -> CbfHeaderMainWrapper;
    fn trailer(&self) -> CbfTrailerWrapper;

    fn region_nth(&self, index: usize) -> Option<CbfHeaderRegionWrapper>;
    fn region_iter(&self) -> CbfHeaderRegionIter;

    fn interrupt_nth(&self, index: usize) -> Option<CbfHeaderInterruptWrapper>;
    fn interrupt_iter(&self) -> CbfHeaderInterruptIter;

    fn relocation_nth(&self, index: usize) -> Option<CbfHeaderRelocationWrapper>;
    fn relocation_iter(&self) -> CbfHeaderRelocationIter;

    fn dependency_nth(&self, index: usize) -> Option<CbfHeaderDependencyWrapper>;
    fn dependency_iter(&self) -> CbfHeaderDependencyIter;

    fn read_only_section(&self) -> CbfPayloadSectionWrapper;
    fn data_section(&self) -> Option<CbfPayloadSectionWrapper>;
    fn bss_size(&self) -> u32;
    fn payload_size(&self) -> u32;

    fn checksum_offset(&self) -> u32;

    fn validate(&self) -> bool;
}
