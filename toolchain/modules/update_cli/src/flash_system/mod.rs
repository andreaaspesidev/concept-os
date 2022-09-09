use crate::utils::*;
use std::{path::PathBuf, process::Command};

pub fn flash_system(app_config: String, ihex_file: String, verbose: bool) {
    if verbose {
        println!("---> Flashing System");
    }
    // Validate paths
    let app_config_path = PathBuf::from(app_config);
    if !app_config_path.exists() {
        panic!("Cannot find app config at '{}'", app_config_path.display());
    }
    let ihex_path = PathBuf::from(ihex_file);
    if !ihex_path.exists() {
        panic!("Cannot find image at '{}'", ihex_path.display());
    }
    // Read app config
    let app_config = app_config::read_configuration(&app_config_path.to_str().unwrap())
        .expect("Cannot read app config");
    // Extract needed information
    let target_chip = openocd_board_to_chip(&app_config.board);
    // -> Step 1: halt and erase flash
    if verbose {
        println!("---> Erasing Flash");
    }
    openocd_flash_erase(&target_chip, verbose);
    // -> Step 2: upload program
    if verbose {
        println!("---> Flashing Image");
    }
    openocd_program(&target_chip, &ihex_path.to_str().unwrap(), verbose);
    // -> Step 3: sw reset
    println!("Success!");
}

fn openocd_flash_erase(target_chip: &String, verbose: bool) {
    let mut cmd = Command::new("openocd");
    cmd.arg("-f")
        .arg("interface/stlink.cfg")
        .arg("-f")
        .arg(&format!("target/{}.cfg", target_chip))
        .arg("-c")
        .arg("init")
        .arg("-c")
        .arg("halt")
        .arg("-c")
        .arg("wait_halt")
        .arg("-c")
        .arg(&format!("{} mass_erase 0", target_chip))
        .arg("-c")
        .arg("exit");

    if !verbose {
        // Avoid stdout
        cmd.stdout(std::process::Stdio::null());
    }

    let status = cmd.status();
    if !status.is_ok() {
        panic!("Cannot erase flash");
    }
    if !status.unwrap().success() {
        panic!("Cannot erase flash");
    }
}

fn openocd_program(target_chip: &String, ihex_path: &str, verbose: bool) {
    let mut cmd = Command::new("openocd");
    cmd.arg("-f")
        .arg("interface/stlink.cfg")
        .arg("-f")
        .arg(&format!("target/{}.cfg", target_chip))
        .arg("-c")
        .arg(&format!("program {} verify reset", ihex_path))
        .arg("-c")
        .arg("exit");
    if !verbose {
        // Avoid stdout
        cmd.stdout(std::process::Stdio::null());
    }
    let status = cmd.status();
    if !status.is_ok() {
        panic!("Cannot flash");
    }
    if !status.unwrap().success() {
        panic!("Cannot flash");
    }
}
