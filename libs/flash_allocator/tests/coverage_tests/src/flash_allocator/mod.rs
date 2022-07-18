#![no_std]
#![feature(generic_const_exprs)]

mod buddy;
mod utils;
pub mod flash;

#[cfg(feature = "swap")]
pub mod swap;
