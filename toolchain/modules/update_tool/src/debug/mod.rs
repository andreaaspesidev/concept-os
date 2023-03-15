// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use signal_hook::consts::SIGTERM;

use crate::utils::*;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn debug(app_config: String, verbose: bool) {
    // Validate paths
    let app_config_path = PathBuf::from(app_config);
    if !app_config_path.exists() {
        panic!("Cannot find app config at '{}'", app_config_path.display());
    }
    // Setup signal hooks
    let should_terminate = Arc::new(AtomicBool::new(false));
    signal_hook::flag::register(SIGTERM, Arc::clone(&should_terminate)).unwrap();
    // Read app config
    let app_config = app_config::read_configuration(&app_config_path.to_str().unwrap())
        .expect("Cannot read app config");
    // Extract needed information
    let target_chip = openocd_board_to_chip(&app_config.board);
    let target_freq = app_config.clock_speed as u32;
    // Create the channel
    let channel = create_channel(verbose);
    // Start openocd
    let mut openocd = openocd_start(&target_chip, verbose);
    // Start gdb
    let mut gdb = gdb_start(&channel, target_freq, verbose);
    // Parsing loop
    let mut reader = BufReader::new(File::open(channel).expect("Cannot open itm channel pipe"));
    let mut tmp_buff: Vec<u8> = Vec::new();
    reader.read_to_end(&mut tmp_buff).unwrap();
    let mut decoder = itm::Decoder::new(reader, true);
    while !should_terminate.load(Ordering::Relaxed) {
        let packet_result = decoder.read_packet();
        if packet_result.is_err() {
            continue;
        }
        let pkt = packet_result.unwrap();
        match pkt.kind() {
            itm::packet::Kind::Instrumentation(data) => match data.port() {
                0 => {
                    let message =
                        std::str::from_utf8(data.payload()).expect("Cannot parse ITM packet");
                    print!("{}", message);
                }
                _ => {
                    let message =
                        std::str::from_utf8(data.payload()).expect("Cannot parse ITM packet");
                    print!("{}", message);
                }
            },
            _ => panic!("Unknown ITM packet"),
        }
    }
    // Terminate childs
    gdb.kill().unwrap();
    openocd.kill().unwrap();
}

fn create_channel(verbose: bool) -> PathBuf {
    let channel_file = PathBuf::from("/tmp/concept-os-itm-fifo");
    if !channel_file.exists() {
        if verbose {
            println!("Creating a new temp file at {}", channel_file.display());
        }
        std::fs::File::create(&channel_file).expect("Cannot create itm fifo");
    }
    channel_file
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
        panic!("Cannot flash");
    }
    status.unwrap()
}

fn gdb_start(channel: &PathBuf, target_freq: u32, verbose: bool) -> Child {
    let mut cmd = Command::new("gdb-multiarch");
    cmd.arg("-ex")
        .arg("target extended-remote localhost:3333")
        .arg("-ex")
        .arg(format!(
            "monitor tpiu config internal {} uart off {}",
            channel.display(),
            target_freq
        ))
        .arg("-ex")
        .arg("monitor itm ports on");
    if !verbose {
        // Avoid stdout
        cmd.stderr(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());
    }
    let status = cmd.spawn();
    if !status.is_ok() {
        panic!("Cannot flash");
    }
    status.unwrap()
}
