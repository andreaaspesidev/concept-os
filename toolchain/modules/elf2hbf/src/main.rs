mod hbf;
mod parse_elf;
mod structures;
mod relocations;
use std::{error::Error, fs};
use component_config::structures::ComponentConfig;
use hbf::HbfFile;
use clap::Parser;

/// Generate an HBF file for the given component
pub fn generate_hbf(
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
    // Initialize hbf
    let mut component_hbf = HbfFile::new();
    // Init header
    component_hbf.initialize_header(&component_config);
    // Init read-only (.text, .rodata)
    component_hbf.add_readonly(
        &elf_result.text_section, 
        (&elf_result.rodata_section).as_ref(), 
        elf_result.rel_entrypoint, 
        relocs.rodata.as_ref(), 
        relocs.data.as_ref()
    );
    // Init .data
    component_hbf.add_data(
        (&elf_result.data_section).as_ref(), 
        elf_result.bss_size
    );
    // Generate
    let hbf_bytes = component_hbf.generate()?;
    return Ok(hbf_bytes);
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
    hbf_output_path: String
}

fn process_args() -> i32 {
    let args = Arguments::parse();
    let hbf = generate_hbf(
        &args.component_config_file,
        &args.component_elf_file,
        &args.component_relocations_file
    );
    if hbf.is_err() {
        println!("Error: \n{}", hbf.unwrap_err());
        return 1;
    }
    let write_result = fs::write(args.hbf_output_path, hbf.unwrap());
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

    use crate::generate_hbf;
    use crate::parse_elf::bss_size;
    use crate::parse_elf::entrypoint_bytes;
    use crate::relocations;

    use hbf_rs::HbfFile;
    use component_config::structures::ComponentConfig;

    use component_config::structures::ComponentFlag as CF;
    use hbf_rs::ComponentFlags as HBF_CF;

    use component_config::structures::RegionAttribute as RF;
    use hbf_rs::RegionAttributes as HBF_RF;

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("examples");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    fn convert_component_flag(flags: &Vec<CF>) -> HBF_CF {
        let mut attr: HBF_CF = HBF_CF::NONE;
        for a in flags.iter() {
            match *a {
                CF::START_AT_BOOT => {
                    attr |= HBF_CF::START_AT_BOOT;
                }
            }
        }
        return attr;
    }

    fn convert_region_attributes(attributes: &Vec<RF>) -> HBF_RF {
        let mut attr: HBF_RF = HBF_RF::NONE;
        for a in attributes.iter() {
            match *a {
                RF::READ => {
                    attr |= HBF_RF::READ;
                },
                RF::WRITE => {
                    attr |= HBF_RF::WRITE;
                },
                RF::EXECUTE => {
                    attr |= HBF_RF::EXECUTE;
                },
                RF::DEVICE => {
                    attr |= HBF_RF::DEVICE;
                },
                RF::DMA => {
                    attr |= HBF_RF::DMA;
                }
            }
        }
        return attr;
    }

    fn check_header(
        component_config_file: &str, 
        component_relocations_file: &str, 
        hbf_parsed: &impl HbfFile
    ) {
        // Parse component config
        let component_config: ComponentConfig = component_config::read_component_config(component_config_file).unwrap();
        // Parse relocations
        let relocs = relocations::parse_relocations(component_relocations_file).unwrap();
        // [component]
        assert_eq!(component_config.component.id, hbf_parsed.header_base().component_id());
        assert_eq!(component_config.component.version, hbf_parsed.header_base().component_version());
        assert_eq!(component_config.component.priority, hbf_parsed.header_main().component_priority());
        assert_eq!(convert_component_flag(&component_config.component.flags), hbf_parsed.header_main().component_flags());
        assert_eq!(component_config.component.min_ram, hbf_parsed.header_main().component_min_ram());
        // [[regions]]
        assert_eq!(component_config.regions.is_some(), hbf_parsed.header_base().num_regions() > 0);
        assert_eq!(component_config.regions.is_none(), hbf_parsed.header_base().num_regions() == 0);
        if component_config.regions.is_some() {
            let regions = (&component_config.regions).as_ref().unwrap();
            assert_eq!(regions.len(), hbf_parsed.header_base().num_regions() as usize);
            for (i, r) in regions.iter().enumerate() {
                assert_eq!(r.base_address, hbf_parsed.region_nth(i).unwrap().base_address());
                assert_eq!(r.size, hbf_parsed.region_nth(i).unwrap().size());
                assert_eq!(convert_region_attributes(&r.attributes), hbf_parsed.region_nth(i).unwrap().attributes());
            }
        }
        // [[interrupts]]
        assert_eq!(component_config.interrupts.is_some(), hbf_parsed.header_base().num_interrupts() > 0);
        assert_eq!(component_config.interrupts.is_none(), hbf_parsed.header_base().num_interrupts() == 0);
        if component_config.interrupts.is_some() {
            let interrupts = (&component_config.interrupts).as_ref().unwrap();
            assert_eq!(interrupts.len(), hbf_parsed.header_base().num_interrupts() as usize);
            for (i, r) in interrupts.iter().enumerate() {
                assert_eq!(r.irq, hbf_parsed.interrupt_nth(i).unwrap().irq_number());
                assert_eq!(r.notification_mask, hbf_parsed.interrupt_nth(i).unwrap().notification_mask());
            }
        }
        // [[relocations]]
        // TODO: more extensive check
        let total_relocs: usize = relocs.rodata.unwrap_or_else(|| vec![]).len() + relocs.data.unwrap_or_else(|| vec![]).len();
        assert_eq!(total_relocs, hbf_parsed.header_base().num_relocations() as usize);
    }

    fn check_payload(
        text_section_file: &str, 
        ro_section_file: &str, 
        data_section_file: &str, 
        hbf_parsed: &impl HbfFile
    ) {
        // Binary read all these files, and construct sections
        let text = std::fs::read(text_section_file).unwrap();
        let rodata = std::fs::read(ro_section_file).unwrap();
        let data = std::fs::read(data_section_file).unwrap();
        // Merge sections
        let mut read_only = text.clone();
        read_only.extend(rodata);
        // Check readonly
        assert_eq!(hbf_parsed.read_only_section().size() as usize, read_only.len());
        assert_eq!(hbf_parsed.read_only_section().content(), read_only);
        // Check data
        assert_eq!(hbf_parsed.data_section().is_some(), data.len() > 0);
        assert_eq!(hbf_parsed.data_section().is_none(), data.len() == 0);
        if data.len() > 0 {
            assert_eq!(hbf_parsed.data_section().unwrap().size() as usize, data.len());
            assert_eq!(hbf_parsed.data_section().unwrap().content(), data);
        }
    }

    #[test]
    fn gen_test1() {
        let component_config_file = get_test_file_path("component1/Component.toml");
        let component_elf_file = get_test_file_path("component1/output/image.elf");
        let component_relocations_file = get_test_file_path("component1/output/relocations.toml");
        let hbf_out_file = get_test_file_path("component1/output/component.hbf");
        let hbf = generate_hbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(hbf_out_file, &hbf).unwrap();
        // Parse back the hbf, and compare
        let parsed_hbf = hbf_rs::parse_hbf(&hbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_hbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component1/output/image.text"),
            &get_test_file_path("component1/output/image.rodata"),
            &get_test_file_path("component1/output/image.data"),
            &parsed_hbf
        );
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_hbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_hbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_hbf.bss_size());
    }
    #[test]
    fn gen_test2() {
        let component_config_file = get_test_file_path("component2/Component.toml");
        let component_elf_file = get_test_file_path("component2/output/image.elf");
        let component_relocations_file = get_test_file_path("component2/output/relocations.toml");
        let hbf_out_file = get_test_file_path("component2/output/component.hbf");
        let hbf = generate_hbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(hbf_out_file, &hbf).unwrap();
        // Parse back the hbf, and compare
        let parsed_hbf = hbf_rs::parse_hbf(&hbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_hbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component2/output/image.text"),
            &get_test_file_path("component2/output/image.rodata"),
            &get_test_file_path("component2/output/image.data"),
            &parsed_hbf
        );
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_hbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_hbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_hbf.bss_size());
    }
    #[test]
    fn gen_test3() {
        let component_config_file = get_test_file_path("component3/Component.toml");
        let component_elf_file = get_test_file_path("component3/output/image.elf");
        let component_relocations_file = get_test_file_path("component3/output/relocations.toml");
        let hbf_out_file = get_test_file_path("component3/output/component.hbf");
        let hbf = generate_hbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(hbf_out_file, &hbf).unwrap();
        // Parse back the hbf, and compare
        let parsed_hbf = hbf_rs::parse_hbf(&hbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_hbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component3/output/image.text"),
            &get_test_file_path("component3/output/image.rodata"),
            &get_test_file_path("component3/output/image.data"),
            &parsed_hbf
        );
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_hbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_hbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_hbf.bss_size());
    }
    #[test]
    fn gen_test4() {
        let component_config_file = get_test_file_path("component4/Component.toml");
        let component_elf_file = get_test_file_path("component4/output/image.elf");
        let component_relocations_file = get_test_file_path("component4/output/relocations.toml");
        let hbf_out_file = get_test_file_path("component4/output/component.hbf");
        let hbf = generate_hbf(
            &component_config_file,
            &component_elf_file,
            &component_relocations_file
        ).unwrap();
        std::fs::write(hbf_out_file, &hbf).unwrap();
        // Parse back the hbf, and compare
        let parsed_hbf = hbf_rs::parse_hbf(&hbf).unwrap();
        // Check header
        check_header(
            &component_config_file, 
            &component_relocations_file, 
            &parsed_hbf
        );
        // check payload
        check_payload(
            &get_test_file_path("component4/output/image.text"),
            &get_test_file_path("component4/output/image.rodata"),
            &get_test_file_path("component4/output/image.data"),
            &parsed_hbf
        );
        // Check entrypoint
        let entry_point_bytes = entrypoint_bytes(&component_elf_file, 4).unwrap();
        let start = (parsed_hbf.header_main().entry_point_offset()-1) as usize;
        assert_eq!(entry_point_bytes, parsed_hbf.content()[start..start+4]);
        // Check bss
        let bss_size = bss_size(&component_elf_file).unwrap();
        assert_eq!(bss_size, parsed_hbf.bss_size());
    }
}