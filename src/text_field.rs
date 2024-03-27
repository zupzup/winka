use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;

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
        }
    }
}
