mod common_messages;
mod crc;
mod utils;

mod erase_component;
mod flash_component;
mod info;
use crossbeam_channel::{Receiver, Sender};
use rumqttc::{Client, MqttOptions};

use std::{
    io,
    str::FromStr,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};

use clap::{Parser, Subcommand};
use erase_component::erase_component;
use flash_component::flash_component;
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

    #[clap(short, long)]
    #[clap(short = 'i')]
    mqtt_ip: String,

    #[clap(short, long, value_parser)]
    #[clap(short = 'p')]
    mqtt_port: Option<String>,

    #[clap(short, long)]
    #[clap(short = 't')]
    app_root_topic: String,

    #[clap(short, long, value_parser)]
    #[clap(short = 'v')]
    verbose: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    Info {
        // Collects status of the system
    },
    FlashComponent {
        #[clap(short, long, value_parser)]
        #[clap(short = 'f')]
        hbf_file: String,
    },
    EraseComponent {
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
    let mqtt_server_ip = args.mqtt_ip;
    let mqtt_server_port = args.mqtt_port.unwrap_or(String::from("1883"));
    let mqtt_root = args.app_root_topic;

    start(
        mqtt_server_ip,
        mqtt_server_port,
        mqtt_root,
        args.cmd,
        verbose,
    );

    // Exit the application
    Ok(())
}

fn start(
    mqtt_server_ip: String,
    mqtt_server_port: String,
    mqtt_root: String,
    cmd: Commands,
    verbose: bool,
) {
    // Create queues
    let (mut mqtt_in_producer, mut mqtt_in_consumer) = crossbeam_channel::bounded::<u8>(1000);
    let (mut mqtt_out_producer, mut mqtt_out_consumer) = crossbeam_channel::bounded::<Vec<u8>>(10);

    // Establish MQTT connection
    let (h1, h2) = mqtt_start(
        mqtt_server_ip,
        mqtt_server_port,
        mqtt_root,
        mqtt_in_producer.clone(),
        mqtt_out_consumer.clone(),
    );

    // Execute command
    match cmd {
        Commands::Info {} => info(mqtt_in_consumer.clone(), mqtt_out_producer.clone(), verbose),
        Commands::FlashComponent { hbf_file } => flash_component(
            mqtt_in_consumer.clone(),
            mqtt_out_producer.clone(),
            hbf_file,
            verbose,
        ),
        Commands::EraseComponent {
            component_id,
            component_version,
        } => erase_component(
            mqtt_in_consumer.clone(),
            mqtt_out_producer.clone(),
            component_id,
            component_version,
            verbose,
        ),
    }
}

const UPDATE_CLI_ID: u16 = 5;

/// Connect to MQTT server
fn mqtt_start(
    server_ip: String,
    server_port: String,
    mqtt_root: String,
    mqtt_in_producer: Sender<u8>,
    mqtt_out_consumer: Receiver<Vec<u8>>,
) -> (JoinHandle<()>, JoinHandle<()>) {
    let mut mqtt_options = MqttOptions::new(
        "update-cli",
        server_ip,
        u16::from_str(server_port.as_str()).unwrap(),
    );
    mqtt_options.set_keep_alive(Duration::from_secs(30));
    // Create client and connect
    let (mut client, mut connection) = Client::new(mqtt_options, 10);
    // Now, first subscribe
    client
        .subscribe(
            format!("{}/{}/out", mqtt_root, UPDATE_CLI_ID),
            rumqttc::QoS::ExactlyOnce,
        )
        .unwrap();
    // Then create a new thread for sending data
    let send_handle = thread::spawn(move || {
        loop {
            // Read new data
            let result = mqtt_out_consumer.recv();
            if result.is_err() {
                //println!("Exiting MQTT sender");
                return; // Exit the current thread
            }
            // Send data
            let data = result.unwrap();
            client
                .publish(
                    format!("{}/{}/in", mqtt_root, UPDATE_CLI_ID),
                    rumqttc::QoS::ExactlyOnce,
                    false,
                    data,
                )
                .unwrap();
        }
    });
    // Finally, create another thread to receive data/send packets
    let receive_handle = thread::spawn(move || {
        for (i, notification) in connection.iter().enumerate() {
            if notification.is_err() {
                panic!("MQTT Connection failed!");
            }
            let event = notification.unwrap();
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(data)) = event {
                // Add data to queue
                for b in data.payload {
                    if mqtt_in_producer.send(b).is_err() {
                        //println!("Exiting MQTT receiver");
                        return; // Exit thread
                    }
                }
            }
        }
    });
    return (send_handle, receive_handle);
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{
        erase_component::erase_component, flash_component::flash_component, info::info, start,
    };

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    #[test]
    fn test_info() {
        start(
            String::from("192.168.0.3"),
            String::from("1883"),
            String::from("test"),
            crate::Commands::Info {},
            false,
        );
    }
}
