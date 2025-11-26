use crate::desktop::DesktopEntry;
use crate::ipc::{Command, IpcServer};

pub struct AppState {
    pub entries: Vec<DesktopEntry>,
    pub ipc_server: Option<IpcServer>,
    pub window_visible: bool,
}

impl AppState {
    pub fn new(entries: Vec<DesktopEntry>, ipc_server: Option<IpcServer>) -> Self {
        Self {
            entries,
            ipc_server,
            window_visible: false,
        }
    }

    pub fn poll_ipc_command(&self) -> Option<Command> {
        self.ipc_server.as_ref()?.poll_command()
    }
}
