//! Errors for the rwui library.

use std::{error::Error, fmt};

/// Possible errors during graphical widget creation.
#[derive(Debug, Copy, Clone)]
pub enum WidgetCreationError {
    /// Error while creating the sprite.
    SpriteCreation,
}

impl Error for WidgetCreationError {}

impl fmt::Display for WidgetCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::SpriteCreation => {
                write!(f, "Failed to create widget sprite.")
            }
        }
    }
}

/// Possible errors during window initialisation.
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
