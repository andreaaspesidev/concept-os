extern crate hbf_rs;
use std::{path::PathBuf, env};

use crate::hbf_rs::HbfFile;


fn main () -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need specify file path!");
        return Err(());
    }
    let file_path = PathBuf::from(&args[1]);
    let file_bytes = std::fs::read(file_path).unwrap();
    let hbf = hbf_rs::parse_hbf(&file_bytes).unwrap();
    // validity
    println!("Is valid: {}", hbf.validate());
    // header
    println!("header (base): {:?}", hbf.header_base());
    println!("header (main): {:?}", hbf.header_main());
    // regions
    for region in hbf.region_iter() {
        println!("region: {:?}", region);
    }
    // interrupts
    for interrupt in hbf.interrupt_iter() {
        println!("interrupt: {:?}", interrupt);
    }
    // relocations
    for relocation in hbf.relocation_iter() {
        println!("relocation: {:?}", relocation);
    }
    // .text + .rodata
    println!(".text + .rodata: {:?}", hbf.read_only_section());
    // .data
    println!(".data: {:?}", hbf.data_section());
    // .bss
    println!(".bss size: {:?}", hbf.bss_size());

    Ok(())
}