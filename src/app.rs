//! A graphical window in the operating system.

use crate::app_interface::AppInterface;
use crate::command::AppCmd;
use crate::error::WindowCreationError;
use crate::user_app::UserApp;
use chrono::{DateTime, Local};
use crossbeam::channel::{unbounded, Receiver, Sender};
use glium::winit::application::ApplicationHandler;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use glium::winit::window::WindowId;
use rwgfx::renderer::{Renderer, RendererDescriptor};
use rwlog::sender::Logger;

/// Data required for creating a window application.
pub struct WindowAppDescriptor<T: UserApp> {
    /// Logger
    pub logger: Logger,
    /// User application, needed to customise the application's behaviour.
    pub user_app: T,
}

/// Application with a graphical window.
pub struct App<T: UserApp> {
    /// Logger
    logger: Logger,
    /// Renderer.
    renderer: Renderer,
    /// Actual window.
    window: glium::winit::window::Window,
    /// Last time the event processing function was called.
    last_update_time: DateTime<Local>,
    /// User application, needed to customise the application's behaviour.
    user_app: T,
    /// Channel for sending commands to the application. Used when creating new application interfaces.
    sender_channel: Sender<AppCmd>,
    /// Channel for receiving commands to the renderer.
    receiver_channel: Receiver<AppCmd>,
}

impl<T: UserApp> App<T> {
    /// Initialise the application.
    /// Return true on success, false otherwise.
    pub fn init(&mut self) -> bool {
        return self.user_app.on_init(
            AppInterface::new(self.sender_channel.clone()),
            self.renderer.new_interface(),
        );
    }

    /// Get the renderer responsible for this window.
    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    /// Create a new window.
    pub fn new(
        app_descriptor: WindowAppDescriptor<T>,
    ) -> Result<(EventLoop<()>, App<T>), WindowCreationError> {
        let event_loop = glium::winit::event_loop::EventLoop::builder()
            .build()
            .map_err(|_| WindowCreationError::EventLoopCreation)?;
        event_loop.set_control_flow(ControlFlow::Poll);

        let (window, display) =
            glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

        let renderer = Renderer::new(RendererDescriptor {
            display,
            logger: app_descriptor.logger.clone(),
        })
        .map_err(|e| {
            rwlog::err!(
                &app_descriptor.logger,
                "Failed to create the window renderer: {e}."
            );
            WindowCreationError::RendererCreation
        })?;

        let (sender_channel, receiver_channel) = unbounded();

        Ok((
            event_loop,
            App {
                logger: app_descriptor.logger,
                renderer,
                window,
                last_update_time: Local::now(),
                user_app: app_descriptor.user_app,
                sender_channel,
                receiver_channel,
            },
        ))
    }

    pub fn process_commands(&mut self) {}

    /// Get a mutable reference to the user data.
    pub fn user_app(&mut self) -> &mut T {
        &mut self.user_app
    }
}

impl<T: UserApp> ApplicationHandler for App<T> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Update the user data.
        let current_time = chrono::Local::now();
        let delta_time = current_time - self.last_update_time;
        self.last_update_time = current_time;
        self.user_app.on_update(&delta_time);
        self.process_commands();

        // Call the on_event user function.
        self.user_app.on_event(&event);

        // Process the event.
        match event {
            WindowEvent::CloseRequested => {
                rwlog::info!(&self.logger, "Goodbye!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.renderer.process_commands();
                self.user_app.on_draw();
                self.renderer.draw();
            }
            WindowEvent::Resized(window_size) => {
                self.renderer.resize(window_size.into());
            }
            _ => (),
        }

        // Request a redraw for each frame.
        self.window.request_redraw();
    }
}
