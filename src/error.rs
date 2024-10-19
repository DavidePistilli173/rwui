//! Errors for the rwui library.

use std::{error::Error, fmt};

/// Possible errors during context initialisation.
#[derive(Debug, Copy, Clone)]
pub enum WindowCreationError {
    /// Error while creating the event loop.
    EventLoopCreation,
    /// Error while creating the renderer.
    RendererCreation,
}

impl Error for WindowCreationError {}

impl fmt::Display for WindowCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::EventLoopCreation => {
                write!(f, "Failed to create the event loop.")
            }
            Self::RendererCreation => {
                write!(f, "Failed to create the renderer.")
            }
        }
    }
}
