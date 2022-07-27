#![no_std]
#![feature(generic_const_exprs)]

pub mod flash;

#[cfg(feature = "swap")]
pub mod swap;
