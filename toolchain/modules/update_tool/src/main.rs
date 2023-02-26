mod common_messages;
mod crc;
mod utils;

cfg_if::cfg_if! {
    // Check the features are in mutual exclusion
    if #[cfg(all(feature = "uart", feature = "mqtt"))] {
        compile_error!("Both features cannot be enabled at the same time!");
    } else if #[cfg(feature = "uart")] {
        mod debug;
        mod gdb;
        mod flash_system;
    } else if #[cfg(not(feature = "mqtt"))] {
        compile_error!("At least one feature must be enabled!");
    }
}

mod flash_component;
mod info;

use std::{
    io::{self},
    thread::{self, JoinHandle},
};

cfg_if::cfg_if! {
    if #[cfg(feature = "mqtt")] {
        use std::{
            time::Duration,
            str::FromStr
        };
        use rumqttc::{Client, MqttOptions};
    } else if #[cfg(feature = "uart")] {
        use std::{
            fs::File,
            io::{BufReader, BufWriter, Read, Write},
            os::unix::prelude::{FromRawFd, IntoRawFd, RawFd},
        };
    }
}


use clap::{Parser, Subcommand};
use crossbeam_channel::{Receiver, Sender};
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

    #[cfg(feature = "mqtt")]
    #[clap(short, long)]
    #[clap(short = 'i')]
    mqtt_ip: String,

    #[cfg(feature = "mqtt")]
    #[clap(short, long, value_parser)]
    #[clap(short = 'p')]
    mqtt_port: Option<String>,

    #[cfg(feature = "mqtt")]
    #[clap(short, long)]
    #[clap(short = 't')]
    app_root_topic: String,

    #[clap(short, long, value_parser)]
    #[clap(short = 'v')]
    verbose: Option<bool>,
}

#[derive(Subcommand)]
enum Commands {
    #[cfg(feature = "uart")]
    Debug {
        #[clap(short, long, value_parser)]
        #[clap(short = 'c')]
        app_config: String,
    },
    #[cfg(feature = "uart")]
    Gdb {
        #[clap(short, long, value_parser)]
        #[clap(short = 'g')]
        app_config: String,
    },
    #[cfg(feature = "uart")]
    FlashSystem {
        // Flash the whole image
        #[clap(short, long, value_parser)]
        #[clap(short = 'c')]
        app_config: String,
        #[clap(short, long, value_parser)]
        #[clap(short = 'd')]
        image_path: String,
    },
    /// Collect status on the system
    Info {
        #[cfg(feature = "uart")]
        #[clap(short, long)]
        #[clap(short = 's')]
        serial_port: String,
    },
    /// Updates/Insert a new component in the system
    FlashComponent {
        #[cfg(feature = "uart")]
        #[clap(short, long)]
        #[clap(short = 's')]
        serial_port: String,
        #[clap(short, long, value_parser)]
        #[clap(short = 'f')]
        hbf_file: String,
    },
}

fn main() -> Result<(), io::Error> {
    // Parse Args
    let args = Cli::parse();
    let verbose = args.verbose.unwrap_or(false);

    // Create connection abstractions
    let (channel_in_producer, channel_in_consumer) = crossbeam_channel::bounded::<u8>(100);
    let (channel_out_producer, channel_out_consumer) = crossbeam_channel::bounded::<Vec<u8>>(100);

    cfg_if::cfg_if! {
        if #[cfg(feature = "uart")] {
            let port = match args.cmd {
                Commands::Info { ref serial_port} => Some(serial_port.clone()),
                Commands::FlashComponent { ref serial_port, hbf_file:_ } => Some(serial_port.clone()),
                _ => None
            };
            if let Some(serial_port) = port {
                serial_start(
                    serial_port,
                    channel_in_producer,
                    channel_out_consumer,
                );
            }
        } else if #[cfg(feature = "mqtt")] {
            mqtt_start(
                args.mqtt_ip,
                args.mqtt_port.unwrap_or(String::from("1883")),
                args.app_root_topic,
                channel_in_producer.clone(),
                channel_out_consumer.clone(),
            );
        }
    }

    // Execute command
    match args.cmd {
        Commands::Info {#[cfg(feature = "uart")] serial_port: _} => info(channel_in_consumer, channel_out_producer, verbose),
        Commands::FlashComponent { #[cfg(feature = "uart")] serial_port: _, hbf_file } => {
            flash_component(channel_in_consumer, channel_out_producer, hbf_file, verbose)
        }
        #[cfg(feature = "uart")]
        Commands::FlashSystem {
            app_config,
            image_path,
        } => flash_system::flash_system(app_config, image_path, verbose),
        #[cfg(feature = "uart")]
        Commands::Debug { app_config } => debug::debug(app_config, verbose),
        #[cfg(feature = "uart")]
        Commands::Gdb { app_config } => gdb::gdb(app_config, verbose),
    }

    Ok(())
}

#[cfg(feature = "uart")]
pub const SERIAL_BAUDRATE: u32 = 115_200;
#[cfg(feature = "uart")]
fn serial_start(
    serial_port: String,
    channel_in_producer: Sender<u8>,
    channel_out_consumer: Receiver<Vec<u8>>,
) -> (JoinHandle<()>, JoinHandle<()>) {
    let serial_port = serialport::new(&serial_port.clone(), SERIAL_BAUDRATE)
        .open_native()
        .expect("Cannot open serial port");
    // There is no method for waiting data
    let raw_channel: RawFd = serial_port.into_raw_fd();
    let in_raw_file = unsafe { File::from_raw_fd(raw_channel) };
    let out_raw_file = in_raw_file.try_clone().unwrap();

    let mut in_stream = BufReader::new(in_raw_file);
    let mut out_stream = BufWriter::new(out_raw_file);

    let send_handler = thread::spawn(move || {
        loop {
            // Wait for data
            let data_res = channel_out_consumer.recv();
            if data_res.is_err() {
                return; // Exit thread
            }
            // Write data
            let data = data_res.unwrap();
            out_stream.write_all(&data).unwrap();
            out_stream.flush().unwrap();
        }
    });
    let receive_handle = thread::spawn(move || {
        let mut data: [u8; 100] = [0x00; 100];
        loop {
            // Wait for data on the file
            let read = in_stream.read(&mut data);
            if read.is_err() {
                return; // Exit thread
            }
            let to_read = read.unwrap();
            // Copy all this data on the channel
            for b in &data[0..to_read] {
                channel_in_producer.send(*b).unwrap();
            }
        }
    });
    return (send_handler, receive_handle);
}

#[cfg(feature = "mqtt")]
const UPDATE_COMPONENT_ID: u16 = 5;
#[cfg(feature = "mqtt")]
/// Connect to MQTT server
fn mqtt_start(
    server_ip: String,
    server_port: String,
    mqtt_root: String,
    channel_in_producer: Sender<u8>,
    channel_out_consumer: Receiver<Vec<u8>>,
) -> (JoinHandle<()>, JoinHandle<()>) {
    let mut mqtt_options = MqttOptions::new(
        "update-tool",
        server_ip,
        u16::from_str(server_port.as_str()).unwrap(),
    );
    mqtt_options.set_keep_alive(Duration::from_secs(30));
    // Create client and connect
    let (mut client, mut connection) = Client::new(mqtt_options, 10);
    // Now, first subscribe
    client
        .subscribe(
            format!("{}/{}/out", mqtt_root, UPDATE_COMPONENT_ID),
            rumqttc::QoS::ExactlyOnce,
        )
        .unwrap();
    // Then create a new thread for sending data
    let send_handle = thread::spawn(move || {
        loop {
            // Read new data
            let result = channel_out_consumer.recv();
            if result.is_err() {
                //println!("Exiting MQTT sender");
                return; // Exit the current thread
            }
            // Send data
            let data = result.unwrap();
            client
                .publish(
                    format!("{}/{}/in", mqtt_root, UPDATE_COMPONENT_ID),
                    rumqttc::QoS::ExactlyOnce,
                    false,
                    data,
                )
                .unwrap();
        }
    });
    // Finally, create another thread to receive data/send packets
    let receive_handle = thread::spawn(move || {
        for (_, notification) in connection.iter().enumerate() {
            if notification.is_err() {
                panic!("MQTT Connection failed!");
            }
            let event = notification.unwrap();
            if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(data)) = event {
                // Add data to queue
                for b in data.payload {
                    if channel_in_producer.send(b).is_err() {
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

    use crossbeam_channel::{Receiver, Sender};

    use crate::{flash_component::flash_component, info::info, serial_start};

    fn get_test_file_path(name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("tests");
        d.push(name);
        return String::from(d.to_str().unwrap());
    }

    fn init_serial(name: String) -> (Receiver<u8>, Sender<Vec<u8>>) {
        let (channel_in_producer, channel_in_consumer) = crossbeam_channel::bounded::<u8>(100);
        let (channel_out_producer, channel_out_consumer) =
            crossbeam_channel::bounded::<Vec<u8>>(100);
        serial_start(
            name,
            channel_in_producer.clone(),
            channel_out_consumer.clone(),
        );
        return (channel_in_consumer, channel_out_producer);
    }

    #[test]
    fn test_info() {
        let (in_channel, out_channel) = init_serial(String::from("/dev/ttyACM0"));
        info(in_channel, out_channel, false);
    }
}
