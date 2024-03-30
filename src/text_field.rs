use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;
use glyphon::FontSystem;
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
    text: Text,
    rectangle: Rectangle,
    content: String,
    active: bool,
}

impl TextField {
    pub fn new(cfg: TextFieldConfig, font_system: &mut glyphon::FontSystem) -> Self {
        Self {
            rectangle: Rectangle::new(
                cfg.rect_pos,
                cfg.fill_color,
                cfg.fill_color_active,
                cfg.border_color,
                cfg.border_color_active,
            ),
            text: Text::new(
                font_system,
                cfg.rect_pos,
                "",
                cfg.text_color,
                cfg.text_color,
            ),
            content: String::new(),
            active: false,
        }
    }

    pub fn set_text(&mut self, font_system: &mut FontSystem, text: &str) {
        if self.active {
            self.content = text.to_owned();
            self.text.set_text(font_system, &self.content);
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_active(&mut self) {
        self.active = true
    }

    pub fn set_inactive(&mut self) {
        self.active = false
    }

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn rectangle(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn is_hovered(&self, mouse_coords: PhysicalPosition<f64>) -> bool {
        self.rectangle.is_hovered(mouse_coords)
    }
}
