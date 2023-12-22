//! Basic button widget.

use cgmath::{Point2, Vector2};
use chrono::Duration;
use rwgfx::animation::Animated;
use rwgfx::sprite::Sprite;
use winit::event::{ElementState, MouseButton, WindowEvent};

/// Rectangular object that can be interacted with.
pub struct Button {
    /// Position of the button in screen coordinates.
    position: Animated<Point2<f32>>,
    /// Size of the button
    size: Animated<Vector2<f32>>,
    /// Z-index of the button, determines which UI element is drawn on top.
    z_index: f32,
    /// If true, the mouse cursor is hovering over the button.
    hovered: bool,
    /// If true, the user is clicking the button.
    pressed: bool,
    /// Background colour of the button.
    back_colour: [f32; 4],
    /// Alpha value of the white overlay of the button (for hovered-pressed animations).
    overlay_alpha: Animated<f32>,
    /// Actual graphical component of the button.
    sprite: Sprite,
}

impl Button {
    /// Process an event.
    /// If the event is directed at this button, true is returned to signal that the event was consumed.
    /// Otherwise, false is returned.
    pub fn consume_event(&mut self, event: &WindowEvent) -> bool {
        let mut event_consumed = false;

        match event {
            WindowEvent::CursorMoved { position, .. } => {
                let current_button_position = self.position.current();
                let current_button_size = self.size.current();
                let right_coord = current_button_position.x + current_button_size.x;
                let down_coord = current_button_position.y + current_button_size.y;
                // If the cursor is on the button.
                if current_button_position.x <= position.x as f32
                    && position.x as f32 <= right_coord
                    && current_button_position.y <= position.y as f32
                    && position.y as f32 <= down_coord
                {
                    if !self.hovered {
                        self.hovered = true;
                        self.overlay_alpha
                            .set_target(self.overlay_alpha.target() + 0.1);
                        event_consumed = true;
                    }
                } else {
                    if self.hovered {
                        self.hovered = false;
                        self.overlay_alpha
                            .set_target(self.overlay_alpha.target() - 0.1);
                        event_consumed = true;
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // Only process the left mouse button.
                if *button == MouseButton::Left {
                    // If the button is already pressed, check for the mouse release.
                    if self.pressed {
                        if *state == ElementState::Released {
                            self.pressed = false;
                            self.overlay_alpha
                                .set_target(self.overlay_alpha.target() - 0.1);
                            event_consumed = true;
                        }
                    } else {
                        if self.hovered && *state == ElementState::Pressed {
                            self.pressed = true;
                            self.overlay_alpha
                                .set_target(self.overlay_alpha.target() + 0.1);
                            event_consumed = true;
                        }
                    }
                }
            }
            _ => (),
        }

        event_consumed
    }

    /// Draw the button.
    pub fn draw<'a, 'b>(&'a self, frame_context: &'b mut rwgfx::context::FrameContext<'a, 'b>) {
        self.sprite.draw(frame_context);
    }

    /// Create a new button.
    pub fn new(
        context: &rwgfx::context::Context,
        position: Point2<f32>,
        size: Vector2<f32>,
        z_index: f32,
        back_colour: [f32; 4],
    ) -> Self {
        let sprite = Sprite::new(context, position, size, z_index, back_colour);

        Self {
            position: Animated::new(position, Duration::milliseconds(200)),
            size: Animated::new(size, Duration::milliseconds(200)),
            z_index,
            hovered: false,
            pressed: false,
            back_colour,
            overlay_alpha: Animated::new(0.0, Duration::milliseconds(100)),
            sprite,
        }
    }

    /// Update the button's logic.
    pub fn update(&mut self, elapsed: &Duration) {
        // Position update.
        if !self.position.complete() {
            self.position.update(elapsed);
            self.sprite.set_position(*self.position.current());
        }

        // Size update.
        if !self.size.complete() {
            self.size.update(elapsed);
            self.sprite.set_size(*self.size.current());
        }

        // Overlay alpha update.
        if !self.overlay_alpha.complete() {
            self.overlay_alpha.update(elapsed);
            self.sprite.set_overlay_alpha(*self.overlay_alpha.current());
        }
    }
}
