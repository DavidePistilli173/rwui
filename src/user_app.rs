use chrono::Duration;

use crate::app_interface::AppInterface;
use glium::winit::event::WindowEvent;
use rwgfx::render_interface::RenderInterface;

/// User graphical applications.
pub trait UserApp {
    /// Function called when the application is initialised.
    /// Must return true on success and false otherwise.
    fn on_init(&mut self, app_interface: AppInterface, render_interface: RenderInterface) -> bool;

    /// Function called when a new event has arrived.
    fn on_event(&mut self, event: &WindowEvent);

    /// Function called before drawing each frame.
    fn on_draw(&mut self);

    /// Update the application, depending on the time elapsed since the last call.
    fn on_update(&mut self, elapsed: &Duration);
}
