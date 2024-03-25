use crate::rectangle::{RectPos, Rectangle};
use crate::text::Text;
use winit::dpi::PhysicalPosition;

#[derive(Debug)]
pub struct ButtonConfig {
    pub rect_pos: RectPos,
    pub fill_color: [f32; 3],
    pub fill_color_active: [f32; 3],
    pub border_color: [f32; 3],
    pub border_color_active: [f32; 3],
    pub text: &'static str,
    pub text_color: glyphon::Color,
    pub text_color_active: glyphon::Color,
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
                cfg.fill_color_active,
                cfg.border_color,
                cfg.border_color_active,
            ),
            text: Text::new(
                font_system,
                cfg.rect_pos,
                cfg.text,
                cfg.text_color,
                cfg.text_color_active,
            ),
        }
    }

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn rectangle(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }

    pub fn is_hovered(&self, mouse_coords: PhysicalPosition<f64>) -> bool {
        let rect_pos = self.rectangle.position();
        mouse_coords.x > rect_pos.left as f64
            && mouse_coords.x < rect_pos.right as f64
            && mouse_coords.y > rect_pos.top as f64
            && mouse_coords.y < rect_pos.bottom as f64
    }
}
