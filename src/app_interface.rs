//! Interface to an application.

use crate::command::AppCmd;
use crossbeam::channel::Sender;

/// Application interface that can be cloned throughout the program.
#[derive(Clone)]
pub struct AppInterface {
    /// Channel used for sending commands to the renderer.
    channel: Sender<AppCmd>,
}

impl AppInterface {
    /// Create a new application interface from a given sender data channel.
    pub fn new(channel: Sender<AppCmd>) -> Self {
        Self { channel }
    }
}
