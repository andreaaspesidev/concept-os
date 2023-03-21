// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod cbf;
mod parse_elf;
mod structures;
mod relocations;
use std::{error::Error, fs};
use component_config::structures::ComponentConfig;
use cbf::CbfFile;
use clap::Parser;

/// Generate an CBF file for the given component
pub fn generate_cbf(
    component_config_file: &str, 
    component_elf_file: &str,
    component_relocations_file: &str
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Parse component config
    let component_config: ComponentConfig = component_config::read_component_config(component_config_file)?;
    // Parse ELF
    let elf_result = parse_elf::parse(component_elf_file)?;
    // Parse relocations
    let relocs = relocations::parse_relocations(component_relocations_file)?;
    // Initialize cbf
    let mut component_cbf = CbfFile::new();
    // Init header
    component_cbf.initialize_header(&component_config);
    // Init read-only (.text, .rodata)
    component_cbf.add_readonly(
        &elf_result.text_section, 
        (&elf_result.rodata_section).as_ref(), 
        elf_result.rel_entrypoint, 
        relocs.text.as_ref(),
        relocs.rodata.as_ref(), 
        relocs.data.as_ref()
    );
    // Init .data
    component_cbf.add_data(
        (&elf_result.data_section).as_ref(), 
        elf_result.bss_size
    );
    // Generate
    let cbf_bytes = component_cbf.generate()?;
    return Ok(cbf_bytes);
}


#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    #[clap(long)]
    #[clap(short = 'c')]
    component_config_file: String,
    #[clap(short, long)]
    #[clap(short = 'e')]
    component_elf_file: String,
    #[clap(short, long)]
    #[clap(short = 'r')]
    component_relocations_file: String,
    #[clap(short, long)]
    #[clap(short = 'o')]
    cbf_output_path: String
}

fn process_args() -> i32 {
    let args = Arguments::parse();
    let cbf = generate_cbf(
        &args.component_config_file,
        &args.component_elf_file,
        &args.component_relocations_file
    );
    if cbf.is_err() {
        println!("Error: \n{}", cbf.unwrap_err());
        return 1;
    }
    let write_result = fs::write(args.cbf_output_path, cbf.unwrap());
    if write_result.is_err() {
        println!("Error: \n{}", write_result.unwrap_err());
        return 2;
    }
    return 0;
}

fn main() {
    let res = process_args();
    std::process::exit(res);
}


#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::generate_cbf;
    use crate::parse_elf::bss_size;
    use crate::parse_elf::entrypoint_bytes;
    use crate::relocations;

    use cbf_rs::CbfFile;
    use component_config::structures::ComponentConfig;

    use component_config::structures::ComponentFlag as CF;
    use cbf_rs::ComponentFlags as CBF_CF;

    use component_config::structures::RegionAttribute as RF;
    use cbf_rs::RegionAttributes as CBF_RF;

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    fn convert_component_flag(flags: &Vec<CF>) -> CBF_CF {
        let mut attr: CBF_CF = CBF_CF::NONE;
        for a in flags.iter() {
            match *a {
                CF::START_AT_BOOT => {
                    attr |= CBF_CF::START_AT_BOOT;
                }
            }
        }
        return attr;
    }

    fn convert_region_attributes(attributes: &Vec<RF>) -> CBF_RF {
        let mut attr: CBF_RF = CBF_RF::NONE;
        for a in attributes.iter() {
            match *a {
                RF::READ => {
                    attr |= CBF_RF::READ;
                },
                RF::WRITE => {
                    attr |= CBF_RF::WRITE;
                },
                RF::EXECUTE => {
                    attr |= CBF_RF::EXECUTE;
                },
                RF::DEVICE => {
                    attr |= CBF_RF::DEVICE;
                },
                RF::DMA => {
                    attr |= CBF_RF::DMA;
                }
            }
        }
        return attr;
    }

    fn check_header(
        component_config_file: &str, 
        component_relocations_file: &str, 
        cbf_parsed: &impl CbfFile
    ) {
        // Parse component config
        let component_config: ComponentConfig = component_config::read_component_config(component_config_file).unwrap();
        // Parse relocations
        let relocs = relocations::parse_relocations(component_relocations_file).unwrap();
        // [component]
        assert_eq!(component_config.component.id, cbf_parsed.header_base().component_id());
        assert_eq!(component_config.component.version, cbf_parsed.header_base().component_version());
        assert_eq!(component_config.component.priority, cbf_parsed.header_main().component_priority());
        assert_eq!(convert_component_flag(&component_config.component.flags), cbf_parsed.header_main().component_flags());
        assert_eq!(component_config.component.min_ram, cbf_parsed.header_main().component_min_ram());
        // [[regions]]
        assert_eq!(component_config.regions.is_some(), cbf_parsed.header_base().num_regions() > 0);
        assert_eq!(component_config.regions.is_none(), cbf_parsed.header_base().num_regions() == 0);
        if component_config.regions.is_some() {
            let regions = (&component_config.regions).as_ref().unwrap();
            assert_eq!(regions.len(), cbf_parsed.header_base().num_regions() as usize);
            for (i, r) in regions.iter().enumerate() {
                assert_eq!(r.base_address, cbf_parsed.region_nth(i).unwrap().base_address());
                assert_eq!(r.size, cbf_parsed.region_nth(i).unwrap().size());
                assert_eq!(convert_region_attributes(&r.attributes), cbf_parsed.region_nth(i).unwrap().attributes());
            }
        }
        // [[interrupts]]
        assert_eq!(component_config.interrupts.is_some(), cbf_parsed.header_base().num_interrupts() > 0);
        assert_eq!(component_config.interrupts.is_none(), cbf_parsed.header_base().num_interrupts() == 0);
        if component_config.interrupts.is_some() {
            let interrupts = (&component_config.interrupts).as_ref().unwrap();
            assert_eq!(interrupts.len(), cbf_parsed.header_base().num_interrupts() as usize);
            for (i, r) in interrupts.iter().enumerate() {
                assert_eq!(r.irq, cbf_parsed.interrupt_nth(i).unwrap().irq_number());
                assert_eq!(r.notification_mask, cbf_parsed.interrupt_nth(i).unwrap().notification_mask());
            }
        }
        // [[relocations]]
        // TODO: more extensive check
        let total_relocs: usize = relocs.text.unwrap_or_else(|| vec![]).len() + relocs.rodata.unwrap_or_else(|| vec![]).len() + relocs.data.unwrap_or_else(|| vec![]).len();
        assert_eq!(total_relocs, cbf_parsed.header_base().num_relocations() as usize);
        // [[dependencies]]
        assert_eq!(component_config.dependencies.is_some(), cbf_parsed.header_base().num_dependencies() > 0);
        assert_eq!(component_config.dependencies.is_none(), cbf_parsed.header_base().num_dependencies() == 0);
        if component_config.dependencies.is_some() {
            let dependencies = (&component_config.dependencies).as_ref().unwrap();
            assert_eq!(dependencies.len(), cbf_parsed.header_base().num_dependencies() as usize);
            for (i, d) in dependencies.iter().enumerate() {
                assert_eq!(d.component_id, cbf_parsed.dependency_nth(i).unwrap().component_id());
                assert_eq!(d.min_version, cbf_parsed.dependency_nth(i).unwrap().min_version());
                assert_eq!(d.max_version, cbf_parsed.dependency_nth(i).unwrap().max_version());
            }
        }
    }

    fn check_payload(
        text_section_file: &str, 
        ro_section_file: &str, 
        data_section_file: &str, 
        cbf_parsed: &impl CbfFile
    ) {
        // Binary read all these files, and construct sections
        let text = std::fs::read(text_section_file).unwrap();
        let rodata = std::fs::read(ro_section_file).unwrap();
        let data = std::fs::read(data_section_file).unwrap();
        // Merge sections
        let mut read_only = text.clone();
        read_only.extend(rodata);
        // Check readonly
        assert_eq!(cbf_parsed.read_only_section().size() as usize, read_only.len());
        assert_eq!(cbf_parsed.read_only_section().content(), read_only);
        // Check data
        assert_eq!(cbf_parsed.data_section().is_some(), data.len() > 0);
        assert_eq!(cbf_parsed.data_section().is_none(), data.len() == 0);
        if data.len() > 0 {
            assert_eq!(cbf_parsed.data_section().unwrap().size() as usize, data.len());
            assert_eq!(cbf_parsed.data_section().unwrap().content(), data);
        }
    }

    #[test]
    fn gen_test1() {
        let component_config_file = get_test_file_path("component1/Component.toml");
        let component_elf_file = get_test_file_path("component1/output/image.elf");
        let component_relocations_file = get_test_file_path("component1/output/relocations.toml");
        let cbf_out_file = get_test_file_path("component1/output/component.cbf");
        let cbf = generate_cbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(cbf_out_file, &cbf).unwrap();
        // Parse back the cbf, and compare
        let parsed_cbf = cbf_rs::parse_cbf(&cbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_cbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component1/output/image.text"),
            &get_test_file_path("component1/output/image.rodata"),
            &get_test_file_path("component1/output/image.data"),
            &parsed_cbf
        );
        // Check checksum
        assert!(parsed_cbf.validate());
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_cbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_cbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_cbf.bss_size());
    }
    #[test]
    fn gen_test2() {
        let component_config_file = get_test_file_path("component2/Component.toml");
        let component_elf_file = get_test_file_path("component2/output/image.elf");
        let component_relocations_file = get_test_file_path("component2/output/relocations.toml");
        let cbf_out_file = get_test_file_path("component2/output/component.cbf");
        let cbf = generate_cbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(cbf_out_file, &cbf).unwrap();
        // Parse back the cbf, and compare
        let parsed_cbf = cbf_rs::parse_cbf(&cbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_cbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component2/output/image.text"),
            &get_test_file_path("component2/output/image.rodata"),
            &get_test_file_path("component2/output/image.data"),
            &parsed_cbf
        );
        // Check checksum
        assert!(parsed_cbf.validate());
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_cbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_cbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_cbf.bss_size());
    }
    #[test]
    fn gen_test3() {
        let component_config_file = get_test_file_path("component3/Component.toml");
        let component_elf_file = get_test_file_path("component3/output/image.elf");
        let component_relocations_file = get_test_file_path("component3/output/relocations.toml");
        let cbf_out_file = get_test_file_path("component3/output/component.cbf");
        let cbf = generate_cbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(cbf_out_file, &cbf).unwrap();
        // Parse back the cbf, and compare
        let parsed_cbf = cbf_rs::parse_cbf(&cbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_cbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component3/output/image.text"),
            &get_test_file_path("component3/output/image.rodata"),
            &get_test_file_path("component3/output/image.data"),
            &parsed_cbf
        );
        // Check checksum
        assert!(parsed_cbf.validate());
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_cbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_cbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_cbf.bss_size());
    }
    #[test]
    fn gen_test4() {
        let component_config_file = get_test_file_path("component4/Component.toml");
        let component_elf_file = get_test_file_path("component4/output/image.elf");
        let component_relocations_file = get_test_file_path("component4/output/relocations.toml");
        let cbf_out_file = get_test_file_path("component4/output/component.cbf");
        let cbf = generate_cbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(cbf_out_file, &cbf).unwrap();
        // Parse back the cbf, and compare
        let parsed_cbf = cbf_rs::parse_cbf(&cbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_cbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component4/output/image.text"),
            &get_test_file_path("component4/output/image.rodata"),
            &get_test_file_path("component4/output/image.data"),
            &parsed_cbf
        );
        // Check checksum
        assert!(parsed_cbf.validate());
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_cbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_cbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_cbf.bss_size());
    }
}