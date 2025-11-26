use crate::ipc::Command;

/// Events that the UI can send to the daemon.
#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    RequestHide,
}

/// Unified event type for the daemon event loop.
/// Combines IPC commands and window events into a single channel.
#[derive(Debug, Clone)]
pub enum DaemonEvent {
    /// IPC command received from external client
    Ipc(Command),
    /// Window event from the UI
    Window(WindowEvent),
}

impl From<Command> for DaemonEvent {
    fn from(cmd: Command) -> Self {
        Self::Ipc(cmd)
    }
}

impl From<WindowEvent> for DaemonEvent {
    fn from(event: WindowEvent) -> Self {
        Self::Window(event)
    }
}

/// Async sender for daemon events.
pub type DaemonEventSender = flume::Sender<DaemonEvent>;

/// Async receiver for daemon events.
pub type DaemonEventReceiver = flume::Receiver<DaemonEvent>;

/// Create an unbounded async channel for daemon events.
pub fn create_daemon_channel() -> (DaemonEventSender, DaemonEventReceiver) {
    flume::unbounded()
}

// Legacy types for backwards compatibility during refactoring
pub type EventSender = flume::Sender<DaemonEvent>;
pub type EventReceiver = flume::Receiver<DaemonEvent>;

pub fn create_event_channel() -> (EventSender, EventReceiver) {
    create_daemon_channel()
}
