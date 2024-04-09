use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;
use glyphon::FontSystem;
use std::time::SystemTime;
use winit::dpi::PhysicalPosition;

#[derive(Debug)]
pub struct TextFieldConfig {
    pub rect_pos: RectPos,
    pub fill_color: [f32; 3],
    pub fill_color_active: [f32; 3],
    pub border_color: [f32; 3],
    pub border_color_active: [f32; 3],
    pub text_color: glyphon::Color,
}

#[derive(Debug)]
pub struct TextField {
    pub text: Text,
    pub rectangle: Rectangle,
    pub content: String,
    pub active: bool,
    pub last_cursor_blink: Option<SystemTime>,
}

const PADDING: u32 = 10;

impl TextField {
    pub fn new(cfg: TextFieldConfig, font_system: &mut glyphon::FontSystem) -> Self {
        let padded_rect = RectPos {
            top: cfg.rect_pos.top + PADDING,
            left: cfg.rect_pos.left + PADDING,
            right: cfg.rect_pos.right - PADDING,
            bottom: cfg.rect_pos.bottom - PADDING,
        };
        Self {
            rectangle: Rectangle::new(
                cfg.rect_pos,
                cfg.fill_color,
                cfg.fill_color_active,
                cfg.border_color,
                cfg.border_color_active,
            ),
            text: Text::new(font_system, padded_rect, "", cfg.text_color, cfg.text_color),
            content: String::new(),
            active: false,
            last_cursor_blink: None,
        }
    }

    pub fn get_last_cursor_blink(&self) -> &Option<SystemTime> {
        &self.last_cursor_blink
    }

    pub fn set_last_cursor_blink(&mut self) {
        self.last_cursor_blink = Some(SystemTime::now());
    }

    pub fn get_cursor(&self) -> Rectangle {
        let text_width = self.text.get_text_width();
        let rect_pos = self.rectangle.position();
        let left = if text_width.width > text_width.buffer_width {
            rect_pos.right - 10
        } else {
            rect_pos.left + text_width.width as u32 + 10
        };
        Rectangle::new(
            RectPos {
                top: rect_pos.top + 10,
                left,
                right: left + 2,
                bottom: rect_pos.bottom - 10,
            },
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
        )
    }

    pub fn add_text(&mut self, font_system: &mut FontSystem, text: &str) {
        if self.active {
            self.content.push_str(text);
            self.text.set_text(font_system, &self.content);
        }
    }

    pub fn remove_character(&mut self, font_system: &mut FontSystem) {
        if self.active {
            self.content.pop();
            self.text.set_text(font_system, &self.content);
        }
    }

    pub fn set_active(&mut self) {
        self.active = true;
        if self.last_cursor_blink.is_none() {
            self.last_cursor_blink = Some(SystemTime::now());
        }
    }

    pub fn set_inactive(&mut self) {
        self.active = false;
        self.last_cursor_blink = None;
    }

    pub fn is_hovered(&self, mouse_coords: PhysicalPosition<f64>) -> bool {
        self.rectangle.is_hovered(mouse_coords)
    }
}
