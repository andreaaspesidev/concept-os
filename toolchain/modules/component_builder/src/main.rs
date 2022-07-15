mod structures;

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use cargo_metadata::MetadataCommand;
use clap::Parser;

use crate::structures::BoardConfig;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    #[clap(short, long)]
    #[clap(short = 's')]
    component_source_dir: String,
    #[clap(short, long)]
    #[clap(short = 'o')]
    hbf_output_path: String,
    #[clap(short, long)]
    #[clap(short = 'b')]
    target_board: String,
    #[clap(short, long)]
    #[clap(short = 'v')]
    #[clap(takes_value = false)]
    verbose: bool,
    #[clap(short, long)]
    #[clap(short = 'c')]
    #[clap(takes_value = false)]
    clean_up: bool,
}

fn process_args() -> i32 {
    // Parse args
    let args = Arguments::parse();
    // Print arguments
    println!(
        "\n--------------------\nSource dir: {}\nOutput Path: {}\nTarget Board: {}\nVerbose: {}\nClean-up: {}\n--------------------\n",
        args.component_source_dir, args.hbf_output_path, args.target_board, args.verbose, args.clean_up
    );
    // Launch build process
    if build_process(
        args.component_source_dir,
        args.hbf_output_path,
        args.target_board,
        args.verbose,
        args.clean_up,
    )
    .is_ok()
    {
        return 0;
    } else {
        return 1;
    }
}

fn build_component(
    component_name: &String,
    component_path: &PathBuf,
    component_build_path: &PathBuf,
    target: &String,
    target_board: &String,
    verbose: bool,
) -> Result<PathBuf, ()> {
    println!("Building into '{}'", component_build_path.display());
    let mut cmd = Command::new("cargo");
    cmd.arg("build")
        .arg("--release")
        .arg("--features")
        .arg(target_board)
        .arg("-Z")
        .arg("build-std=std,panic_abort")
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
    let mut config_path = component_path.clone();
    config_path.push("Component.toml");
    if !config_path.exists() {
        panic!("Cannot find the component descriptor 'Component.toml'");
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

fn build_process(
    component_path: String,
    hbf_output_path: String,
    target_board: String,
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

    // Read global board configuration
    let mut board_config_path = PathBuf::from(root_path);
    board_config_path.push("boards");
    board_config_path.push(&target_board);
    board_config_path.push("board.toml");
    let file_content = std::fs::read_to_string(board_config_path);
    if file_content.is_err() {
        panic!("Cannot find the specified board");
    }
    let board_config: BoardConfig = toml::from_str(&file_content.unwrap()).unwrap();
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
    // Launch build
    let artifact_path = build_component(
        &component_name,
        &component_path_buf,
        &component_build_path,
        &board_config.board.target,
        &target_board,
        verbose,
    )?;
    // Compute relocations
    let relocations_path =
        compute_relocations(&component_build_path, root_path, &artifact_path, verbose)?;
    // Launch elf2hbf
    let output_path = PathBuf::from(hbf_output_path);
    assemble_hbf(
        root_path,
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

fn main() {
    let res = process_args();
    std::process::exit(res);
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
            "stm32f303e".to_string(),
            false,
            true,
        )
        .unwrap();
    }
}
