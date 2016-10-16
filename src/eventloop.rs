use std::thread;

use mio;
use unix::eventloop::Message;

#[cfg(unix)]
pub use unix::eventloop::EventLoop;

pub struct EventLoopHandle {
    handle: Option<thread::JoinHandle<()>>,
    tx_msg_channel: mio::channel::Sender<Message>,
}

impl EventLoopHandle {
    pub fn new(
        handle: thread::JoinHandle<()>,
        tx_msg_channel: mio::channel::Sender<Message>) -> EventLoopHandle {
        EventLoopHandle {
            handle: Some(handle),
            tx_msg_channel: tx_msg_channel,
        }
    }

    pub fn shutdown(&mut self) {
        self.tx_msg_channel
            .send(Message::ShutDown)
            .expect("failed to request event loop to shut down");
        if let Some(handle) = self.handle.take() {
            handle.join().expect("failed to shut down event loop");
        }
    }
}