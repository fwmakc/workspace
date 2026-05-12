//! Application state and business logic.

use winit::dpi::PhysicalPosition;

pub const CURSOR_COLOR: [f32; 4] = [0.0, 0.9, 1.0, 1.0];
pub const CURSOR_SIZE_MIN: f32 = 4.0;
pub const CURSOR_SIZE_MAX: f32 = 64.0;
pub const CURSOR_SIZE_DEFAULT: f32 = 16.0;

pub const CIRCLE_COLORS: [[f32; 4]; 5] = [
    [1.0, 0.2, 0.2, 1.0],
    [0.2, 1.0, 0.2, 1.0],
    [0.2, 0.2, 1.0, 1.0],
    [1.0, 1.0, 0.2, 1.0],
    [1.0, 0.2, 1.0, 1.0],
];

pub const FONT_SIZE: f32 = 32.0;
pub const CMD_FONT_SIZE: f32 = 24.0;
pub const TEXT_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const CMD_PANEL_COLOR: [f32; 4] = [0.1, 0.12, 0.18, 0.95];
pub const CMD_PANEL_HEIGHT: f32 = 48.0;

#[derive(Clone, Copy, Debug)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
    pub color: [f32; 4],
}

/// Pure application state. Knows nothing about wgpu.
pub struct AppState {
    pub cursor_pos: PhysicalPosition<f64>,
    pub cursor_size: f32,
    pub circles: Vec<Circle>,
    next_color_idx: usize,
    pub command_bar_visible: bool,
    pub command_text: String,
    pub typed_text: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            cursor_pos: PhysicalPosition::new(0.0, 0.0),
            cursor_size: CURSOR_SIZE_DEFAULT,
            circles: Vec::new(),
            next_color_idx: 0,
            command_bar_visible: false,
            command_text: String::new(),
            typed_text: String::new(),
        }
    }
}

impl AppState {
    pub fn add_circle(&mut self) {
        let color = CIRCLE_COLORS[self.next_color_idx];
        self.next_color_idx = (self.next_color_idx + 1) % CIRCLE_COLORS.len();
        self.circles.push(Circle {
            x: self.cursor_pos.x as f32,
            y: self.cursor_pos.y as f32,
            radius: self.cursor_size,
            color,
        });
    }

    pub fn clear_circles(&mut self) {
        self.circles.clear();
        self.next_color_idx = 0;
    }

    pub fn resize_cursor(&mut self, delta: f32) {
        self.cursor_size = (self.cursor_size + delta).clamp(CURSOR_SIZE_MIN, CURSOR_SIZE_MAX);
    }

    pub fn toggle_command_bar(&mut self) {
        self.command_bar_visible = !self.command_bar_visible;
    }

    pub fn type_char(&mut self, ch: char, target: TextTarget) {
        match target {
            TextTarget::Typed => self.typed_text.push(ch),
            TextTarget::Command => self.command_text.push(ch),
        }
    }

    pub fn backspace(&mut self, target: TextTarget) {
        match target {
            TextTarget::Typed => {
                self.typed_text.pop();
            }
            TextTarget::Command => {
                self.command_text.pop();
            }
        }
    }

    pub fn execute_command(&mut self) -> Option<String> {
        if self.command_text.is_empty() {
            return None;
        }
        let cmd = self.command_text.clone();
        self.command_text.clear();
        Some(cmd)
    }
}

/// Which text buffer to modify.
#[derive(Clone, Copy, Debug)]
pub enum TextTarget {
    Typed,
    Command,
}
