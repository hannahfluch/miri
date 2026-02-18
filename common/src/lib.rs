use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod config;

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
    Action {
        #[command(subcommand)]
        action: MiriAction,
    },
    Get {
        #[command(subcommand)]
        get: MiriGet,
    },
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum MiriAction {
    CycleFocusedWorkspaceMode,
    Spawn,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum MiriGet {
    FocusedWorkspaceMode,
    OtherThing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    Scroll,
    Master,
}

impl Mode {
    pub fn cycle(self) -> Mode {
        match self {
            Mode::Scroll => Mode::Master,
            Mode::Master => Mode::Scroll,
        }
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

pub struct WorkspaceModes {
    // output name and index used as key
    // FIXME: solve case of output name being the same
    modes: HashMap<(String, u8), Mode>,
}

impl WorkspaceModes {
    pub fn new() -> Self {
        WorkspaceModes { modes: HashMap::new() }
    }

    pub fn get_mode(&self, output: &str, index: u8) -> Mode {
        self.modes
            .get(&(output.to_string(), index))
            .copied()
            .unwrap_or_default()
    }

    pub fn set_mode(&mut self, output: &str, index: u8, mode: Mode) {
        self.modes.insert((output.to_string(), index), mode);
    }

    pub fn cycle_mode(&mut self, output: &str, index: u8) -> Mode {
        let current = self.get_mode(output, index);
        let new_mode = current.cycle();
        self.set_mode(output, index, new_mode);
        new_mode
    }
}

impl Default for WorkspaceModes {
    fn default() -> Self {
        Self::new()
    }
}
