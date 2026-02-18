use common::{Command, IPCMessage, IPCMessageContainer, MIRI_SOCKET_PATH};
use std::io::Write;
use std::os::unix::net::UnixStream;

// send a command to the daemon
pub fn send_command_to_miri_service(command: Command) {
    match UnixStream::connect(MIRI_SOCKET_PATH) {
        Ok(mut stream) => {
            let container = IPCMessageContainer::new(IPCMessage::CliExecute(command));
            let json = serde_json::to_string(&container).expect("Failed to serialize command");
            let json_with_newline = format!("{}\n", json);

            if let Err(e) = stream.write_all(json_with_newline.as_bytes()) {
                eprintln!("Failed to send command: {e}");
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to miri service: {e}");
            std::process::exit(1);
        }
    }
}
