use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::net::{UnixListener, UnixStream};

use crate::ipc::miri_socket_path;

// TODO: this is an ai generated async socket reader. please redo this
pub struct MiriListener {
    listener: UnixListener,
}

pub struct MiriSocket {
    pub reader: BufReader<tokio::io::ReadHalf<UnixStream>>,
}

impl MiriListener {
    pub async fn bind() -> Self {
        let _ = std::fs::remove_file(miri_socket_path());
        let listener = UnixListener::bind(miri_socket_path()).expect("Failed to bind miri socket");
        Self { listener }
    }

    pub async fn accept(&self) -> MiriSocket {
        let (stream, _) = self.listener.accept().await.expect("Failed to accept connection");
        let (read_half, _) = tokio::io::split(stream);
        MiriSocket {
            reader: BufReader::new(read_half),
        }
    }
}

impl MiriSocket {
    pub async fn read(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line).await {
            Ok(0) => None,
            Ok(_) => Some(line),
            Err(e) => {
                eprintln!("Error reading from miri socket: {}", e);
                None
            }
        }
    }
}
