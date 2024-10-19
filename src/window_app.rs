//! A graphical window in the operating system.

pub use crate::error::WindowCreationError;
use glium::winit::application::ApplicationHandler;
use glium::winit::event::WindowEvent;
use glium::winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use glium::winit::window::WindowId;
use rwgfx::renderer::{Renderer, RendererDescriptor};
use rwlog::sender::Logger;

/// Data required for creating a window application.
pub struct WindowAppDescriptor<T> {
    /// Logger
    pub logger: Logger,
    /// Data needed for the user to customise the application's behaviour..
    pub data: T,
    /// Function called before new events are processed.
    pub on_before_events: Option<fn(&mut WindowApp<T>)>,
    /// Function called after new events are processed.
    pub on_after_events: Option<fn(&mut WindowApp<T>)>,
    /// Function called before drawing each frame.
    pub on_before_draw: Option<fn(&mut WindowApp<T>)>,
    /// Function called after drawing each frame.
    pub on_after_draw: Option<fn(&mut WindowApp<T>)>,
}

/// Application with a graphical window.
pub struct WindowApp<T> {
    /// Logger
    logger: Logger,
    /// Renderer.
    renderer: Renderer,
    /// Actual window.
    window: glium::winit::window::Window,
    /// Data needed for the user to customise the application's behaviour..
    data: T,
    /// Function called before new events are processed.
    on_before_events: Option<fn(&mut WindowApp<T>)>,
    /// Function called after new events are processed.
    on_after_events: Option<fn(&mut WindowApp<T>)>,
    /// Function called before drawing each frame.
    on_before_draw: Option<fn(&mut WindowApp<T>)>,
    /// Function called after drawing each frame.
    on_after_draw: Option<fn(&mut WindowApp<T>)>,
}

impl<T> WindowApp<T> {
    /// Get a mutable reference to the user data.
    pub fn data(&mut self) -> &mut T {
        &mut self.data
    }

    /// Draw a frame.
    fn draw(&mut self) {
        if let Some(fun) = self.on_before_draw {
            fun(self);
        }

        self.renderer.draw();

        if let Some(fun) = self.on_after_draw {
            fun(self);
        }
    }

    /// Get the renderer responsible for this window.
    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }

    /// Create a new window.
    pub fn new(
        app_descriptor: WindowAppDescriptor<T>,
    ) -> Result<(EventLoop<()>, WindowApp<T>), WindowCreationError> {
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

        Ok((
            event_loop,
            WindowApp {
                logger: app_descriptor.logger,
                renderer,
                window,
                data: app_descriptor.data,
                on_before_events: app_descriptor.on_before_events,
                on_after_events: app_descriptor.on_after_events,
                on_before_draw: app_descriptor.on_before_draw,
                on_after_draw: app_descriptor.on_after_draw,
            },
        ))
    }
}

impl<T> ApplicationHandler for WindowApp<T> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        if let Some(fun) = self.on_before_events {
            fun(self);
        }

        match event {
            WindowEvent::CloseRequested => {
                rwlog::info!(&self.logger, "Goodbye!");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.draw();
            }
            _ => (),
        }

        self.window.request_redraw();

        if let Some(fun) = self.on_after_events {
            fun(self);
        }
    }
}
