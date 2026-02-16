pub mod socket;

use clap::{Parser, Subcommand};

use crate::socket::MIRI_SOCKET_PATH;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;

// send a command to the daemon and read the response
pub fn send_command_to_miri_service(command: &str) {
    match UnixStream::connect(MIRI_SOCKET_PATH) {
        Ok(stream) => {
            let mut stream = stream;
            let command_with_newline = format!("{}\n", command);

            if let Err(e) = stream.write_all(command_with_newline.as_bytes()) {
                eprintln!("Failed to send command: {e}");
                std::process::exit(1);
            }

            let mut response = String::new();
            let mut reader = BufReader::new(stream);
            if let Err(e) = reader.read_line(&mut response) {
                eprintln!("Failed to read response: {e}");
                std::process::exit(1);
            }

            println!("{}", response.trim());
        }
        Err(e) => {
            eprintln!("Failed to connect to modal-niri: {e}");
            eprintln!("Make sure modal-niri is running");
            std::process::exit(1);
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Action {
        #[command(subcommand)]
        action: MiriAction,
    },
    Get {
        #[command(subcommand)]
        get: MiriGet,
    },
}

#[derive(Subcommand)]
pub enum MiriAction {
    CycleFocusedWorkspaceMode,
    Spawn,
}

#[derive(Subcommand)]
pub enum MiriGet {
    FocusedWorkspaceMode,
    OtherThing,
}
