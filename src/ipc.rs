use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::unix::net::UnixStream;

pub const MIRI_SOCKET_PATH: &str = "/tmp/modal-niri.sock";

#[derive(Debug, Serialize, Deserialize)]
pub struct IPCMessageContainer {
    pub version: String,
    pub message: IPCMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum IPCMessage {
    CliExecute(Command),
}

impl IPCMessageContainer {
    pub fn new(message: IPCMessage) -> Self {
        Self {
            version: "1.0".to_string(),
            message,
        }
    }
}

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum Command {
    Service {
        #[command(subcommand)]
        service_command: MiriServiceCommand,
    },
    Action {
        #[command(subcommand)]
        action: MiriAction,
    },
    Get {
        #[command(subcommand)]
        get: MiriGet,
    },
    Override {
        #[command(subcommand)]
        override_action: MiriOverride,
    },
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum MiriServiceCommand {
    Start,
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum MiriAction {
    CycleFocusedWorkspaceMode,
    SetFocusedWorkspaceMode { mode: Mode },
}

#[derive(Debug, Clone, Subcommand, Serialize, Deserialize)]
pub enum MiriOverride {
    MoveColumnLeft,
    MoveColumnRight,
    MoveColumnToFirst,
    MoveColumnToLast,
    MoveColumnToMonitorUp,
    MoveColumnToMonitorDown,
    MoveColumnToMonitorLeft,
    MoveColumnToMonitorRight,
    MoveColumnToWorkspaceUp,
    MoveColumnToWorkspaceDown,
    MoveColumnToWorkspace { index: u8 },
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum MiriGet {
    FocusedWorkspaceMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Mode {
    Scroll,
    Master,
}

impl Mode {
    pub fn cycle(&mut self) {
        *self = match *self {
            Mode::Scroll => Mode::Master,
            Mode::Master => Mode::Scroll,
        };
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Mode::Scroll => "scroll",
            Mode::Master => "master",
        }
    }
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Scroll
    }
}

#[derive(Debug)]
pub enum MiriServiceError {
    ConnectionFailed(String),
    SerializationFailed,
    SendFailed(String),
}

pub fn send_command_to_miri_service(command: Command) -> Result<(), MiriServiceError> {
    let mut stream =
        UnixStream::connect(MIRI_SOCKET_PATH).map_err(|e| MiriServiceError::ConnectionFailed(e.to_string()))?;

    let container = IPCMessageContainer::new(IPCMessage::CliExecute(command));
    let json = serde_json::to_string(&container).map_err(|_| MiriServiceError::SerializationFailed)?;

    let json_with_newline = format!("{}\n", json);
    stream
        .write_all(json_with_newline.as_bytes())
        .map_err(|e| MiriServiceError::SendFailed(e.to_string()))?;

    Ok(())
}
