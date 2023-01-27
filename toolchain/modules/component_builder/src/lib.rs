use board_config::BoardConfig;
use cargo_metadata::MetadataCommand;
use component_config::{
    read_component_extended_config,
    structures::{Component, ComponentConfig, Interrupt, Region, RegionAttribute},
    write_component_config,
};
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn build_component(
    component_name: &String,
    component_path: &PathBuf,
    component_build_path: &PathBuf,
    target: &String,
    features: &Vec<String>,
    verbose: bool,
) -> Result<PathBuf, ()> {
    println!("Building into '{}'", component_build_path.display());
    println!("Features '{}'", features.join(", "));
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--release");
    if features.len() > 0 {
        cmd.arg("--features").arg(format!("{}", features.join(",")));
    }
    cmd.arg("-Z")
        .arg("build-std=core,panic_abort")
        .arg("-Z")
        .arg("build-std-features=panic_immediate_abort");
    if verbose {
        cmd.arg("-v");
    }
    cmd.current_dir(component_path);
    cmd.env(
        "RUSTFLAGS",
        &format!(
            "-C link-arg=--nmagic \
             -C link-arg=-T{}/link.x \
             -C link-arg=-Map={}/image.map \
             -C panic=abort \
             -C relocation-model=ropi-rwpi \
             -Z emit-stack-sizes \
             --emit=obj
            ",
            component_build_path.display(),
            component_build_path.display()
        ),
    );
    cmd.env("CARGO_TARGET_DIR", &component_build_path); // Location of where to place all generated artifacts, relative to the current working directory.
    cmd.env("CARGO_BUILD_TARGET", target);
    // Launch build
    let status = cmd.status();
    if !status.is_ok() {
        return Err(());
    }
    if !status.unwrap().success() {
        return Err(());
    }
    // Collect result
    let mut source = component_build_path.clone();
    source.push(target);
    source.push("release");
    source.push(component_name);
    let mut dest = component_build_path.clone();
    dest.push("image.elf");
    if std::fs::copy(&source, &dest).is_err() {
        return Err(());
    }
    Ok(dest)
}

fn compute_relocations(
    component_build_path: &PathBuf,
    root_path: &Path,
    artifact_path: &PathBuf,
    verbose: bool,
) -> Result<PathBuf, ()> {
    println!("Scanning relocations for '{}'", artifact_path.display());
    let mut script_path = PathBuf::from(root_path);
    script_path.push("toolchain");
    script_path.push("scripts");
    script_path.push("relocations");
    script_path.push("elf_relocations.py");
    let mut cmd = Command::new("python3");
    cmd.arg(script_path);
    cmd.arg(artifact_path);
    let mut map_path = component_build_path.clone();
    map_path.push("image.map");
    cmd.arg(map_path);
    let mut reloc_path = component_build_path.clone();
    reloc_path.push("image.relocations.toml");
    cmd.arg(reloc_path.display().to_string());
    if verbose {
        cmd.arg("True");
    }
    // Launch script
    let status = cmd.status();
    if !status.is_ok() || !status.unwrap().success() {
        return Err(());
    }
    // Return path
    Ok(reloc_path)
}

fn assemble_hbf(
    root_path: &Path,
    config_path: &PathBuf,
    output_artifact_path: &PathBuf,
    source_artifact_path: &PathBuf,
    component_path: &PathBuf,
    relocations_path: &PathBuf,
) -> Result<(), ()> {
    println!("Generating hbf for '{}'", component_path.display());
    let mut module_path = PathBuf::from(root_path);
    module_path.push("toolchain");
    module_path.push("modules");
    module_path.push("elf2hbf");
    module_path.push("elf2hbf");
    let mut cmd = Command::new(module_path);
    if !config_path.exists() {
        panic!("Cannot find the generated component descriptor 'Component.toml'");
    }
    cmd.arg("-c");
    cmd.arg(config_path);
    cmd.arg("-e");
    cmd.arg(source_artifact_path);
    cmd.arg("-r");
    cmd.arg(relocations_path);
    cmd.arg("-o");
    cmd.arg(output_artifact_path);
    let status = cmd.status();
    if !status.is_ok() || !status.unwrap().success() {
        return Err(());
    }
    Ok(())
}

fn process_component_config(
    component_path: String,
    component_build_path: &PathBuf,
    board_config: &BoardConfig,
) -> PathBuf {
    // Read the extended config
    let mut config_path = PathBuf::from(component_path);
    config_path.push("Component.toml");
    if !config_path.exists() {
        panic!("Cannot find the component descriptor 'Component.toml'");
    }
    let config = read_component_extended_config(config_path.to_str().unwrap())
        .expect("Cannot read the component descriptor 'Component.toml'");

    // Generate regions
    let mut component_regions = Vec::<Region>::new();
    if let Some(peripherals) = &config.component.peripherals {
        for peripheral in peripherals {
            // Search the peripheral in the board config to estract the regions
            let p = board_config
                .peripheral
                .get(peripheral)
                .expect(&format!("Cannot find peripheral {}", peripheral));
            component_regions.push(Region {
                base_address: p.base_address,
                size: p.size,
                attributes: p
                    .attributes
                    .iter()
                    .map(|a| match *a {
                        board_config::RegionAttribute::READ => RegionAttribute::READ,
                        board_config::RegionAttribute::WRITE => RegionAttribute::WRITE,
                        board_config::RegionAttribute::EXECUTE => RegionAttribute::EXECUTE,
                        board_config::RegionAttribute::DMA => RegionAttribute::DMA,
                        board_config::RegionAttribute::DEVICE => RegionAttribute::DEVICE,
                    })
                    .collect(),
            });
        }
    }
    // Generate interrupts
    let mut component_interrupts = Vec::<Interrupt>::new();
    let p_name_regex = Regex::new(r"([A-Za-z0-9]+)\.([A-Za-z0-9]+)").unwrap();
    if let Some(interrupts) = &config.component.interrupts {
        for (peripheral_interrupt, mask) in interrupts {
            // Get the peripheral
            let res = p_name_regex.captures(peripheral_interrupt).unwrap();
            let peripheral_name = res.get(1).unwrap().as_str().to_string();
            let irq_name = res.get(2).unwrap().as_str().to_string();
            // Search the peripheral in the board config to estract the regions
            let p = board_config
                .peripheral
                .get(&peripheral_name)
                .expect(&format!("Cannot find peripheral {}", peripheral_name));
            let p_ints = p
                .interrupts
                .as_ref()
                .expect(&format!("No interrupts found for {}", peripheral_name));
            // Read the IRQ number
            let irq = p_ints.get(&irq_name).expect(&format!(
                "Interrupt {} not found for {}",
                irq_name, peripheral_name
            ));
            // Add the interrupt
            component_interrupts.push(Interrupt {
                irq: *irq,
                notification_mask: *mask,
            });
        }
    }

    // Construct a new component config
    let new_config = ComponentConfig {
        component: Component {
            id: config.component.id,
            version: config.component.version,
            priority: config.component.priority,
            flags: config.component.flags,
            min_ram: config.component.min_ram,
        },
        regions: match component_regions.is_empty() {
            true => None,
            false => Some(component_regions),
        },
        interrupts: match component_interrupts.is_empty() {
            true => None,
            false => Some(component_interrupts),
        },
        dependencies: config.dependencies,
    };
    // Generate the config file
    let mut config_simple_path = component_build_path.clone();
    config_simple_path.push("Component.toml");
    write_component_config(config_simple_path.clone().to_str().unwrap(), &new_config).unwrap();
    return config_simple_path;
}

pub fn build_process(
    component_path: String,
    hbf_output_path: String,
    target_board: String,
    features: &Vec<String>,
    verbose: bool,
    clean_up: bool,
) -> Result<(), ()> {
    // Init paths
    let component_path_buf = PathBuf::from(component_path.clone());
    let mut component_build_path = component_path_buf.clone();
    component_build_path.push("build");
    // Generate build dir if not exists
    if !component_build_path.exists() {
        if std::fs::create_dir(&component_build_path).is_err() {
            panic!("Cannot create build dir");
        }
    }

    // Read cargo metadata of the component
    let mut metadata_path = component_path_buf.clone();
    metadata_path.push("Cargo.toml");
    println!("Reading metadata from '{}'", metadata_path.display());
    let _metadata = MetadataCommand::new().manifest_path(metadata_path).exec();
    if _metadata.is_err() {
        panic!("Cannot read component 'Cargo.toml'. Check the dependencies are correct");
    }
    let metadata = _metadata.unwrap();
    let component_name = metadata.root_package().unwrap().name.clone();

    // Get root path from environm.
    let root_path_res = std::env::var("ROOT_DIR");
    if root_path_res.is_err() {
        panic!("Enviromental variable 'ROOT_DIR' not set");
    }
    let root_path_str = root_path_res.unwrap();
    let root_path = Path::new(&root_path_str);
    if !root_path.exists() {
        panic!("Enviromental variable 'ROOT_DIR' points to non-existing directory");
    }

    println!("Using as root path: {}", root_path.display());

    // Read global board configuration
    let mut board_config_path = PathBuf::from(root_path);
    board_config_path.push("boards");
    board_config_path.push(&target_board);
    board_config_path.push("Board.toml");
    let board_config =
        board_config::read_configuration(&board_config_path.to_str().unwrap()).unwrap();
    // Generate linker script inside the build folder
    let mut linker_input_path = PathBuf::from(root_path);
    linker_input_path.push("boards");
    linker_input_path.push("link_component.x");
    let original_linker = std::fs::read_to_string(linker_input_path);
    if original_linker.is_err() {
        panic!("Cannot find board linker script");
    }
    let linker: String = format!(
        "
MEMORY
{{
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    FLASH : ORIGIN = {}, LENGTH = {}
    RAM : ORIGIN = {}, LENGTH = {}
}}
        
{}",
        board_config.linker.flash_origin,
        board_config.linker.flash_size,
        board_config.linker.ram_origin,
        board_config.linker.ram_size,
        original_linker.unwrap()
    );
    let mut linker_output_path = component_build_path.clone();
    linker_output_path.push("link.x");
    if std::fs::write(linker_output_path, linker).is_err() {
        panic!("Cannot generate linker script for the component");
    }

    // Process the component config to obtain the simplified version
    let config_path = process_component_config(component_path.clone(), &component_build_path, &board_config);

    // Add the board to the features
    let mut feature_list = features.clone();
    feature_list.push(format!("board_{}", target_board));
    // Launch build
    let artifact_path = build_component(
        &component_name,
        &component_path_buf,
        &component_build_path,
        &board_config.board.target,
        &feature_list,
        verbose,
    )?;
    // Compute relocations
    let relocations_path =
        compute_relocations(&component_build_path, root_path, &artifact_path, verbose)?;
    // Launch elf2hbf
    let output_path = PathBuf::from(hbf_output_path);
    assemble_hbf(
        root_path,
        &config_path,
        &output_path,
        &artifact_path,
        &component_path_buf,
        &relocations_path,
    )?;
    // Check if cleanup is required
    if clean_up {
        if std::fs::remove_dir_all(component_build_path).is_err() {
            return Err(());
        }
    }
    Ok(())
}

/*
    Tests
*/
#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::build_process;

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn component1_build() {
        // Set correct env var
        std::env::set_var("ROOT_DIR", "/home/andreaaspesi/GitHub/concept-os");
        // Launch build
        build_process(
            get_test_file_path("component1"),
            get_test_file_path("component1/component1.hbf"),
            "stm32l432kc".to_string(),
            &Vec::<String>::new(),
            false,
            true,
        )
        .unwrap();
    }
}
