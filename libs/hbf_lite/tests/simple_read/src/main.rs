// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{env, path::PathBuf};

use hbf_lite::BufferReaderImpl;
use hbf_lite::HbfFile;

fn open_file(file_path: PathBuf) -> Result<(), ()> {
    let file_bytes = std::fs::read(file_path).unwrap();

    let reader = BufferReaderImpl::from(&file_bytes);

    let hbf = HbfFile::from_reader(&reader).unwrap();
    // validity
    println!("Is valid: {}", hbf.validate().unwrap());
    // header
    println!("header (base): {:?}", hbf.header_base());
    println!("header (main): {:?}", hbf.header_main());
    // regions
    for region_num in 0..hbf.header_base().unwrap().num_regions() {
        println!("region: {:?}", hbf.region_nth(region_num).unwrap());
    }
    // interrupts
    for interrupt_num in 0..hbf.header_base().unwrap().num_interrupts() {
        println!("interrupt: {:?}", hbf.interrupt_nth(interrupt_num).unwrap());
    }
    // relocations
    for relocation_num in 0..hbf.header_base().unwrap().num_relocations() {
        println!(
            "relocation: {:?}",
            hbf.relocation_nth(relocation_num).unwrap()
        );
    }
    // dependencies
    for dependency_num in 0..hbf.header_base().unwrap().num_dependencies() {
        println!(
            "dependency: {:?}",
            hbf.dependency_nth(dependency_num).unwrap()
        );
    }
    // padding
    println!("padding: {:?}", hbf.header_base().unwrap().padding_bytes());
    // .text + .rodata
    println!(".text + .rodata: {:?}", hbf.get_readonly_payload().unwrap());
    // .data
    println!(".data: {:?}", hbf.get_data_payload());
    // .bss
    println!(".bss size: {:?}", hbf.get_bss_payload());

    // trailer
    println!("trailer: {:?}", hbf.trailer());

    Ok(())
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Need specify file path!");
        return Err(());
    }
    let file_path = PathBuf::from(&args[1]);
    open_file(file_path)
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::open_file;

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("files");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn simple_open1() {
        println!("\r\n\r\n------------------ File 1 ------------------");
        open_file(PathBuf::from(get_test_file_path("component1.hbf"))).unwrap();
    }
    #[test]
    fn simple_open2() {
        println!("\r\n\r\n------------------ File 2 ------------------");
        open_file(PathBuf::from(get_test_file_path("component2.hbf"))).unwrap();
    }
    #[test]
    fn simple_open3() {
        println!("\r\n\r\n------------------ File 3 ------------------");
        open_file(PathBuf::from(get_test_file_path("component3.hbf"))).unwrap();
    }
    #[test]
    fn simple_open4() {
        println!("\r\n\r\n------------------ File 4 ------------------");
        open_file(PathBuf::from(get_test_file_path("component4.hbf"))).unwrap();
    }
}
