use std::sync::mpsc::{Receiver, Sender, channel};

#[derive(Debug, Clone, Copy)]
pub enum WindowEvent {
    RequestHide,
}

pub type EventSender = Sender<WindowEvent>;
pub type EventReceiver = Receiver<WindowEvent>;

pub fn create_event_channel() -> (EventSender, EventReceiver) {
    channel()
}
