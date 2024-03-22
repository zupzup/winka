use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;

#[derive(Debug)]
pub struct ButtonConfig {
    pub rect_pos: RectPos,
    pub fill_color: [f32; 3],
    pub fill_color_hover: [f32; 3],
    pub border_color: [f32; 3],
    pub border_color_clicked: [f32; 3],
    pub text: &'static str,
    pub text_color: glyphon::Color,
}

#[derive(Debug)]
pub struct Button {
    text: Text,
    rectangle: Rectangle,
}

impl Button {
    pub fn new(cfg: ButtonConfig, font_system: &mut glyphon::FontSystem) -> Self {
        Self {
            rectangle: Rectangle::new(
                cfg.rect_pos,
                cfg.fill_color,
                cfg.fill_color_hover,
                cfg.border_color,
                cfg.border_color_clicked,
            ),
            text: Text::new(font_system, cfg.rect_pos, cfg.text, cfg.text_color),
        }
    }

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn rectangle(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }
}
