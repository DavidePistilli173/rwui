//! Basic button widget.

use cgmath::{Point2, Vector2};
use chrono::Duration;
use rwcommon::animation::Animated;
use rwgfx::sprite::Sprite;
use rwgfx::text::{Text, TextDescriptor};
use winit::event::{ElementState, MouseButton, WindowEvent};

/// Collection of parameters for button creation.
pub struct ButtonDescriptor<T> {
    /// Absolute position of the button.
    pub position: Point2<f32>,
    /// Size of the button.
    pub size: Vector2<f32>,
    /// Lower values are drawn in front of higher ones.
    pub z_index: f32,
    /// Background colour.
    pub back_colour: [f32; 4],
    /// ID of the texture to use as background.
    pub texture_id: Option<u64>,
    /// Label of the button.
    pub label: Option<String>,
    /// Optional callback called when the button is pressed.
    pub on_press: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the button is released.
    pub on_release: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the mouse enters the boundaries of the button.
    pub on_enter: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the mouse leaves the boundaries of the button.
    pub on_exit: Option<fn(&mut Button<T>, &mut T)>,
}

/// Rectangular object that can be interacted with.
pub struct Button<T> {
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
    /// Optional callback called when the button is pressed.
    on_press: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the button is released.
    on_release: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the mouse enters the boundaries of the button.
    on_enter: Option<fn(&mut Button<T>, &mut T)>,
    /// Optional callback called when the mouse leaves the boundaries of the button.
    on_exit: Option<fn(&mut Button<T>, &mut T)>,
    /// Label.
    label: Option<String>,
    /// Actual graphical component of the button.
    sprite: Sprite,
    /// Graphical component for the label.
    text: Text,
}

impl<T> Button<T> {
    /// Process an event.
    /// If the event is directed at this button, true is returned to signal that the event was consumed.
    /// Otherwise, false is returned.
    /// If the event is consumed, all relevant callbacks are called using the provided data.
    pub fn consume_event(&mut self, data: &mut T, event: &WindowEvent) -> bool {
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
                        if let Some(on_enter) = self.on_enter.as_ref() {
                            on_enter(self, data);
                        }
                        event_consumed = true;
                    }
                } else {
                    if self.hovered {
                        self.hovered = false;
                        self.overlay_alpha
                            .set_target(self.overlay_alpha.target() - 0.1);
                        if let Some(on_exit) = self.on_exit.as_ref() {
                            on_exit(self, data);
                        }
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
                            if let Some(on_press) = self.on_press.as_ref() {
                                on_press(self, data);
                            }
                            event_consumed = true;
                        }
                    } else {
                        if self.hovered && *state == ElementState::Pressed {
                            self.pressed = true;
                            self.overlay_alpha
                                .set_target(self.overlay_alpha.target() + 0.1);
                            if let Some(on_release) = self.on_release.as_ref() {
                                on_release(self, data);
                            }
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
    pub fn draw<'a>(
        &'a self,
        render_pass: &mut rwgfx::RenderPass<'a>,
        frame_context: &rwgfx::renderer::FrameContext<'a>,
    ) {
        self.sprite.draw(render_pass, frame_context);
        self.text.draw(render_pass, frame_context);
    }

    /// Create a new button.
    pub fn new(renderer: &mut rwgfx::renderer::Renderer, descriptor: &ButtonDescriptor<T>) -> Self {
        let sprite = Sprite::new(
            renderer,
            descriptor.position,
            descriptor.size,
            descriptor.z_index,
            descriptor.back_colour,
            descriptor.texture_id,
        );

        let text = Text::new(
            renderer,
            &descriptor.label.clone().unwrap_or(String::new()),
            &TextDescriptor {
                bold: false,
                color: rwgfx::color::Decimal {
                    r: 200,
                    g: 0,
                    b: 200,
                    a: 255,
                },
                font_family: "Arial",
                font_size: 14.0,
                italic: false,
                position: descriptor.position,
                size: descriptor.size,
                z: descriptor.z_index - 1.0,
            },
        );

        Self {
            position: Animated::new(descriptor.position, Duration::milliseconds(200)),
            size: Animated::new(descriptor.size, Duration::milliseconds(200)),
            z_index: descriptor.z_index,
            hovered: false,
            pressed: false,
            back_colour: descriptor.back_colour,
            overlay_alpha: Animated::new(0.0, Duration::milliseconds(100)),
            on_press: descriptor.on_press,
            on_release: descriptor.on_release,
            on_enter: descriptor.on_enter,
            on_exit: descriptor.on_exit,
            label: descriptor.label.clone(),
            sprite,
            text,
        }
    }

    /// Set a new absolute position for the button.
    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position.set_target(position);
    }

    /// Set a new position for the button, relative to the target position.
    pub fn set_position_offset(&mut self, offset: Vector2<f32>) {
        self.set_position(self.position.target() + offset);
    }

    /// Set a new absolute size for the button.
    pub fn set_size(&mut self, size: Vector2<f32>) {
        self.size.set_target(size);
    }

    /// Set a new size for the button relative to the target size.
    pub fn set_size_offset(&mut self, offset: Vector2<f32>) {
        self.set_size(self.size.target() + offset);
    }

    /// Set a new z index for the button.
    pub fn set_z_index(&mut self, z_index: f32) {
        self.sprite.set_z_index(z_index);
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
