use crate::ipc::commands::{Command, Response};
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;

pub struct IpcServer {
    listener: UnixListener,
    socket_path: PathBuf,
}

impl IpcServer {
    pub fn new() -> anyhow::Result<Self> {
        let socket_path = get_socket_path();

        if socket_path.exists() {
            if UnixStream::connect(&socket_path).is_ok() {
                anyhow::bail!("Another instance is already running");
            }
            std::fs::remove_file(&socket_path)?;
        }

        let listener = UnixListener::bind(&socket_path)?;
        listener.set_nonblocking(true)?;

        Ok(Self {
            listener,
            socket_path,
        })
    }

    pub fn poll_command(&self) -> Option<Command> {
        match self.listener.accept() {
            Ok((mut stream, _)) => {
                let mut buf = [0u8; 1024];
                let n = stream.read(&mut buf).ok()?;
                let cmd: Command = serde_json::from_slice(&buf[..n]).ok()?;

                let response = Response::Ok;
                let response_bytes = serde_json::to_vec(&response).ok()?;
                let _ = stream.write_all(&response_bytes);

                Some(cmd)
            }
            Err(_) => None,
        }
    }
}

impl Drop for IpcServer {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.socket_path);
    }
}

pub fn get_socket_path() -> PathBuf {
    std::env::var("XDG_RUNTIME_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/tmp"))
        .join("zlaunch.sock")
}
