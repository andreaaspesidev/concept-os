use crate::utils::*;
use std::path::PathBuf;
use std::process::{Child, Command};

pub fn gdb(app_config: String, verbose: bool) {
    // Validate paths
    let app_config_path = PathBuf::from(app_config);
    if !app_config_path.exists() {
        panic!("Cannot find app config at '{}'", app_config_path.display());
    }
    // Read app config
    let app_config = app_config::read_configuration(&app_config_path.to_str().unwrap())
        .expect("Cannot read app config");
    // Extract needed information
    let target_chip = openocd_board_to_chip(&app_config.board);
    // Start openocd
    let mut openocd = openocd_start(&target_chip, verbose);
    // Start gdb
    let mut gdb = gdb_start();
    // Wait gdb
    gdb.wait().unwrap();
    // Terminate childs
    openocd.kill().unwrap();
}

fn openocd_start(target_chip: &String, verbose: bool) -> Child {
    let mut cmd = Command::new("openocd");
    cmd.arg("-f")
        .arg("interface/stlink.cfg")
        .arg("-f")
        .arg(&format!("target/{}.cfg", target_chip));
    if !verbose {
        // Avoid stdout
        cmd.stderr(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
    }
    let status = cmd.spawn();
    if !status.is_ok() {
        panic!("Cannot open openocd");
    }
    status.unwrap()
}

fn gdb_start() -> Child {
    let mut cmd = Command::new("gdb-multiarch");
    cmd.arg("-ex")
        .arg("target extended-remote localhost:3333");
    let status = cmd.spawn();
    if !status.is_ok() {
        panic!("Cannot open gdb");
    }
    status.unwrap()
}
