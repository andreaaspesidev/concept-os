use crate::elf_editor::ElfEditor;
use app_config::AppConfig;
use board_config::BoardConfig;
use clap::Parser;
use std::{path::PathBuf, process::Command, str::FromStr};

mod elf_editor;
mod visualize_stats;

#[derive(Parser)]
#[clap(version, about)]
struct Arguments {
    #[clap(short, long)]
    #[clap(short = 'a')]
    app_config: String,
    #[clap(short, long)]
    #[clap(short = 'o')]
    output_path: String,
    #[clap(short, long)]
    #[clap(short = 'v')]
    #[clap(takes_value = false)]
    verbose: bool,
}

fn process_args() -> i32 {
    // Parse args
    let args = Arguments::parse();
    // Print arguments
    println!(
        "\n--------------------\nConfiguration file: {}\nOutput Path: {}\nVerbose: {}\n--------------------\n",
        args.app_config, args.output_path, args.verbose
    );
    // Launch build
    build_system(args.app_config, args.output_path, args.verbose);
    return 0;
}

fn build_system(app_config_path: String, output_path: String, verbose: bool) {
    // Detect the ROOT_PATH from the configuration file
    let config_path = PathBuf::from_str(&app_config_path).expect("App config path not valid");
    if !config_path.exists() {
        panic!("Cannot find app config");
    }
    let root_path = app_config_path_to_root(&config_path).expect("Impossible to detect root path");
    println!("Root path: {}", root_path.display());
    // Calculate app root path
    let app_root = PathBuf::from(&app_config_path)
        .parent()
        .expect("Cannot find app")
        .to_path_buf();
    println!("App root: {}", app_root.display());
    // Parse the app config
    let app_config = app_config::read_configuration(&config_path.to_str().unwrap()).expect(
        &format!("Cannot read app config from '{}'", config_path.display()),
    );
    // Calculate board config root path
    let board_config_root = get_board_root_path(&app_config.board, &root_path);
    // Parse the board data
    let board_config = read_board_config(&board_config_root, &app_config.board);
    // Now that we know the target, build the kernel
    let kern_elf = build_kernel(
        &root_path,
        &app_config,
        &app_root,
        &board_config,
        &board_config_root,
        verbose,
    );

    let system_out = PathBuf::from(&output_path);
    let mut elf_edit = ElfEditor::new(&system_out, &app_config.board);
    elf_edit.add_kernel(&kern_elf);
    // For each component, build and add
    for (component_name, component_config) in &app_config.components {
        let hbf_path = build_component(
            &root_path,
            &app_config,
            &component_config.features,
            &app_root,
            &board_config,
            &component_name,
        );
        elf_edit.add_component(&hbf_path);
    }
    // Generate ELF
    visualize_stats::visualize(elf_edit.finish(), &app_root);
    println!("Final image placed in: {}", system_out.display());
}

fn app_config_path_to_root(config_path: &PathBuf) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(config_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf())
}

fn get_board_root_path(board_name: &String, root: &PathBuf) -> PathBuf {
    let mut board_config_root_path = PathBuf::from(root);
    board_config_root_path.push("boards");
    board_config_root_path.push(board_name);
    board_config_root_path
}
fn read_board_config(board_config_root: &PathBuf, board_name: &String) -> BoardConfig {
    let mut board_config_path = PathBuf::from(board_config_root);
    board_config_path.push("Board.toml");
    if !board_config_path.exists() {
        panic!(
            "It's not possible to find the configuration file for board '{}' at {}",
            board_name,
            board_config_path.display()
        );
    }
    board_config::read_configuration(&board_config_path.to_str().unwrap()).expect(&format!(
        "Cannot read board configuration for {}",
        board_name
    ))
}

fn build_kernel(
    _root_path: &PathBuf,
    app_config: &AppConfig,
    app_root: &PathBuf,
    board_config: &BoardConfig,
    board_config_root: &PathBuf,
    verbose: bool,
) -> PathBuf {
    // ----- Create output directory
    let mut kernel_out = PathBuf::from(&app_root);
    kernel_out.push("build");
    if !kernel_out.exists() {
        if std::fs::create_dir(&kernel_out).is_err() {
            panic!("Cannot create output dir for kernel build");
        }
    }
    // ----- Create linker script
    // Read the full linker script
    let mut full_kern_linker_path_src = PathBuf::from(&board_config_root);
    full_kern_linker_path_src.push("kernel-link.x");
    let linker_script = std::fs::read_to_string(&full_kern_linker_path_src)
        .expect("Cannot read kernel linker script");
    let kern_linker: String = format!(
        "
MEMORY
{{
    /* NOTE 1 K = 1 KiBi = 1024 bytes */
    FLASH : ORIGIN = {}, LENGTH = {}
    RAM : ORIGIN = {}, LENGTH = {:#010x}
}}
{}",
        board_config.linker.flash_origin,
        board_config.linker.flash_size,
        board_config.linker.ram_origin,
        app_config.kernel_ram,
        linker_script
    );
    // Write this file
    let mut full_kern_linker_path = PathBuf::from(&kernel_out);
    full_kern_linker_path.push("kernel-link.x");
    if std::fs::write(&full_kern_linker_path, kern_linker).is_err() {
        panic!("Cannot generate linker script for kernel");
    }
    // ----- Launch build
    println!("Building kernel into '{}'", kernel_out.display());
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--release");
    if verbose {
        cmd.arg("-v");
    }
    cmd.current_dir(&app_root);
    cmd.env(
        "RUSTFLAGS",
        &format!(
            "-C link-arg=--nmagic \
             -C link-arg=-T{}/kernel-link.x \
             -Z emit-stack-sizes \
             --emit=obj
            ",
            kernel_out.display(),
        ),
    );
    cmd.env("CARGO_TARGET_DIR", &kernel_out); // Location of where to place all generated artifacts, relative to the current working directory.
    cmd.env("CARGO_BUILD_TARGET", &board_config.board.target);
    // Launch build
    let status = cmd.status();
    if !status.is_ok() {
        panic!("Kernel build failed!")
    }
    if !status.unwrap().success() {
        panic!("Kernel build failed!")
    }
    // Collect result
    let mut kern_elf = PathBuf::from(&kernel_out);
    kern_elf.push(&board_config.board.target);
    kern_elf.push("release");
    kern_elf.push(&app_config.name);

    let mut kernel_out_image = PathBuf::from(&kernel_out);
    kernel_out_image.push("kernel.elf");
    if std::fs::copy(&kern_elf, &kernel_out_image).is_err() {
        panic!("Cannot copy final kernel image");
    }
    println!("Kernel ELF: {}", kernel_out_image.display());
    kernel_out_image
}

fn build_component(
    root_path: &PathBuf,
    app_config: &AppConfig,
    features: &Vec<String>,
    _app_root: &PathBuf,
    _board_config: &BoardConfig,
    component_name: &String,
) -> PathBuf {
    std::env::set_var("ROOT_DIR", &root_path.to_str().unwrap());
    // Search this component
    let mut component_root = PathBuf::from(root_path);
    component_root.push("components");
    component_root.push(component_name);
    component_root.push("core");
    if !component_root.exists() {
        panic!("Cannot find component {}", component_name);
    }
    // Output dir
    let mut out_file = PathBuf::from(&component_root);
    out_file.push(component_name.to_owned() + &String::from(".hbf"));
    // Launch build
    if component_builder::build_process(
        String::from(component_root.to_str().unwrap()),
        String::from(out_file.to_str().unwrap()),
        app_config.board.clone(),
        features,
        false,
        false,
    )
    .is_err()
    {
        panic!("Failed to build component: {}", component_name);
    }
    out_file
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

    use crate::build_system;

    #[test]
    fn build_kernel() {
        build_system(
            String::from("/home/andreaaspesi/GitHub/concept-os/app/stm32f303re_demo/App.toml"),
            String::from("/home/andreaaspesi/GitHub/concept-os/app/stm32f303re_demo/App.elf"),
            false,
        );
    }
}
