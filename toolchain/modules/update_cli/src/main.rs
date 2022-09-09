mod common_messages;
mod crc;
mod utils;

mod debug;
mod erase_component;
mod flash_component;
mod flash_system;
mod info;

use std::io;

use clap::{Parser, Subcommand};
use debug::debug;
use erase_component::erase_component;
use flash_component::flash_component;
use flash_system::flash_system;
use info::info;

/**
 * Command line arguments
 */

#[derive(Parser)]
#[clap(name = "Concept-OS Update Client")]
#[clap(author = "Andrea Aspesi <andrea1.aspesi@mail.polimi.it>")]
#[clap(version)]
#[clap(about = "A tool to enable update of Concept OS.", long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    cmd: Commands,

    #[clap(short, long, value_parser)]
    #[clap(short = 'v')]
    verbose: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    Info {
        // Collects status of the system
        #[clap(short, long, value_parser)]
        #[clap(short = 's')]
        serial_port: String,
    },
    Debug {
        #[clap(short, long, value_parser)]
        #[clap(short = 'c')]
        app_config: String,
    },
    FlashSystem {
        // Flash the whole image
        #[clap(short, long, value_parser)]
        #[clap(short = 'c')]
        app_config: String,
        #[clap(short, long, value_parser)]
        #[clap(short = 'p')]
        image_path: String,
    },
    FlashComponent {
        #[clap(short, long, value_parser)]
        #[clap(short = 's')]
        serial_port: String,
        #[clap(short, long, value_parser)]
        #[clap(short = 'f')]
        hbf_file: String,
    },
    EraseComponent {
        #[clap(short, long, value_parser)]
        #[clap(short = 's')]
        serial_port: String,
        #[clap(short, long, value_parser)]
        #[clap(short = 'i')]
        component_id: u16,
        #[clap(short, long, value_parser)]
        #[clap(short = 'n')]
        component_version: u32,
    },
}

fn main() -> Result<(), io::Error> {
    // Parse Args
    let args = Cli::parse();
    let verbose = args.verbose.unwrap_or(false);
    // Execute command
    match args.cmd {
        Commands::Info { serial_port } => info(serial_port, verbose),
        Commands::FlashSystem {
            app_config,
            image_path,
        } => flash_system(app_config, image_path, verbose),
        Commands::FlashComponent {
            serial_port,
            hbf_file,
        } => flash_component(serial_port, hbf_file, verbose),
        Commands::EraseComponent {
            serial_port,
            component_id,
            component_version,
        } => erase_component(serial_port, component_id, component_version, verbose),
        Commands::Debug { app_config } => debug(app_config, verbose),
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{erase_component::erase_component, flash_component::flash_component, info::info};

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn test1() {
        let hbf = get_test_file_path("rcc.hbf");
        flash_component(String::from("/dev/ttyACM0"), hbf, true);
    }

    #[test]
    fn test2() {
        let hbf = get_test_file_path("uart-channel.hbf");
        flash_component(String::from("/dev/ttyACM0"), hbf, false);
    }

    #[test]
    fn test3() {
        let hbf = get_test_file_path("storage.hbf");
        flash_component(String::from("/dev/ttyACM0"), hbf, false);
    }

    #[test]
    fn test_info() {
        info(String::from("/dev/ttyACM0"), true);
    }

    #[test]
    fn test_erase() {
        erase_component(String::from("/dev/ttyACM0"), 3, 1, true);
    }
}
