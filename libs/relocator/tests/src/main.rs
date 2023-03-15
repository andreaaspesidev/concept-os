// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![feature(test)]

mod relocations;

fn main() {

}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use std::{path::PathBuf, fs::File, io::{Write, Read, Seek}, thread, time::Duration};

    use relocator::{RingBuffer, RelocatorMethods};

    use crate::relocations::parse_relocations;

    const NEW_FLASH_BASE: u32 = 0x0800_1500;
    const NEW_SRAM_BASE: u32 = 0x2000_1000;
    const LINKED_FLASH_BASE: u32 = 0x0800_0480;
    const LINKED_SRAM_BASE: u32 = 0x2000_0000;
    const BUFF_SIZE: usize = 64; // must be large enough to store paired relocations instructions
    const RELOC_BUFF_SIZE: usize = 16; // must be large enough to store paired relocations


    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    struct FileRelocationMethods<'a> {
        points: &'a Vec<u32>,
        output_buff: &'a mut Vec<u8>
    }

    impl<'a> RelocatorMethods for FileRelocationMethods<'a> {
        fn read_relocations(&self, start_index: usize, dst: &mut [u32]) -> usize {
            // Simulate latency
            thread::sleep(Duration::from_millis(5));
            for i in 0..dst.len() {
                dst[i] = self.points[start_index + i];
            }
            return dst.len();
        }

        fn flush(&mut self, position: usize, src: &[u8]) {
            // Simulate latency
            thread::sleep(Duration::from_micros(91* src.len() as u64 / 64));
            assert!(position >= self.output_buff.len());
            self.output_buff.extend_from_slice(src);
        }
    }

    #[test]
    pub fn relocation_test_example1() {
        let file_path = get_test_file_path("example1/relocations.toml");
        let points = parse_relocations(&file_path).unwrap().points;
        let total_relocs_available = points.len();
        // Copy file in another location
        let mut src_elf_file = File::open(get_test_file_path("example1/image.elf")).unwrap();
        let mut dst_elf_file = File::create(get_test_file_path("example1/image_relocated.elf")).unwrap();
        
        let mut output_buff: Vec<u8> = vec![];

        // Create the relocator
        let mut relocator = relocator::Relocator::<LINKED_FLASH_BASE,LINKED_SRAM_BASE,BUFF_SIZE,RELOC_BUFF_SIZE>::new(
            NEW_FLASH_BASE,
            NEW_SRAM_BASE,
            0,
            total_relocs_available,
        );
        // Read data from the file in chunks, then pass it to the relocator
        let mut curr_read_pos: u64 = 0;
        let total_length = src_elf_file.metadata().unwrap().len();
        while curr_read_pos < total_length {
            // Read a chunk of 32 bytes
            let mut buff: [u8; 32] = [0x00; 32];
            src_elf_file.seek(std::io::SeekFrom::Start(curr_read_pos)).unwrap();
            let to_read = core::cmp::min(total_length - curr_read_pos, 32) as usize;
            src_elf_file.read(&mut buff[0..to_read]).unwrap();
            curr_read_pos += to_read as u64;
            // Create a temp object to supply methods to the relocator
            let mut relocator_methods = FileRelocationMethods {
                points: &points,
                output_buff: &mut output_buff,
            };
            relocator.consume_current_buffer(&mut buff[0..to_read], &mut relocator_methods);
        }
        let mut relocator_methods = FileRelocationMethods {
            points: &points,
            output_buff: &mut output_buff,
        };
        relocator.finish(&mut relocator_methods);

        // println!("Written {} bytes", output_buff.len());
        // Write all to dest file
        dst_elf_file.write(&output_buff).unwrap();
    }

    #[bench]
    fn bench_example1(b: &mut Bencher){
        // 213,618,933 ns/iter (+/- 2,446,152)
        b.iter(|| relocation_test_example1());
    }

    #[test]
    fn ring_buffer_test() {
        let mut ring_buff = RingBuffer::<u8, 16>::new();
        assert_eq!(ring_buff.read_capacity(), 0);
        assert_eq!(ring_buff.write_capacity(), 15);
        // add 4 elements
        assert_eq!(ring_buff.extend_from(&[1,2,3,4]),4);
        assert_eq!(ring_buff.read_capacity(), 4);
        assert_eq!(ring_buff.write_capacity(), 11);
        // add the other 12 (11 actually)
        assert_eq!(ring_buff.extend_from(&[5,6,7,8,9,10,11,12,13,14,15,16]),11);
        assert_eq!(ring_buff.read_capacity(), 15);
        assert_eq!(ring_buff.write_capacity(), 0);
        // check we cannot add anymore
        assert_eq!(ring_buff.extend_from(&[16]),0);
        // read back all the 15 elements
        let mut buff: [u8; 15] = [0x00; 15];
        assert_eq!(ring_buff.read_into(0, &mut buff), 15);
        assert_eq!(&buff[0..15],&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
        // random access to the elements (read 8,9,10,11)
        let mut addr: [u8; 4] = [0x00; 4];
        assert_eq!(ring_buff.read_into(7, &mut addr), 4);
        assert_eq!(&addr[0..4],&[8,9,10,11]);
        // free the first 4 elements
        assert_eq!(ring_buff.free(4),4);
        assert_eq!(ring_buff.read_capacity(), 15 - 4);
        assert_eq!(ring_buff.write_capacity(), 4);
        // add 4 new elements
        assert_eq!(ring_buff.extend_from(&[16,17,18,19]),4);
        assert_eq!(ring_buff.read_into(0, &mut buff), 15);
        assert_eq!(&buff[0..15],&[5,6,7,8,9,10,11,12,13,14,15,16,17,18,19]);
        // random access again 8,9,10,11,12
        let mut elements: [u8; 5] = [0x00; 5];
        assert_eq!(ring_buff.read_into(3, &mut elements), 5);
        assert_eq!(&elements[0..5], &[8,9,10,11,12]);
    }
}