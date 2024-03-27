use crate::rectangle::RectPos;
use glyphon::{Attrs, Buffer, Color, Family, FontSystem, Metrics, Shaping, TextArea, TextBounds};

#[derive(Debug)]
pub struct Text {
    buffer: Buffer,
    rect_pos: RectPos,
    color: Color,
    color_active: Color,
}

impl Text {
    pub fn new(
        font_system: &mut FontSystem,
        rect_pos: RectPos,
        text: &str,
        color: Color,
        color_active: Color,
    ) -> Self {
        let mut buffer = Buffer::new(font_system, Metrics::new(30.0, 42.0));
        buffer.set_size(
            font_system,
            (rect_pos.right - rect_pos.left) as f32,
            (rect_pos.bottom - rect_pos.top) as f32,
        );
        buffer.set_text(
            font_system,
            text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
        buffer.lines.iter_mut().for_each(|line| {
            line.set_align(Some(glyphon::cosmic_text::Align::Center));
        });
        buffer.shape_until_scroll(font_system);
        Self {
            buffer,
            rect_pos,
            color,
            color_active,
        }
    }

    fn set_text(&mut self, font_system: &mut FontSystem, text: String) {
        self.buffer.set_text(
            font_system,
            &text,
            Attrs::new().family(Family::SansSerif),
            Shaping::Advanced,
        );
    }

    fn top(&self) -> f32 {
        (self.rect_pos.bottom - (self.rect_pos.bottom - self.rect_pos.top) / 2) as f32
            - (self.buffer.metrics().line_height / 2.0)
    }

    fn bounds(&self) -> TextBounds {
        TextBounds {
            left: self.rect_pos.left as i32,
            top: self.rect_pos.top as i32,
            right: self.rect_pos.right as i32,
            bottom: self.rect_pos.bottom as i32,
        }
    }

    pub fn text_area(&self, is_active: bool) -> TextArea {
        TextArea {
            buffer: &self.buffer,
            left: self.rect_pos.left as f32,
            top: self.top(),
            scale: 1.0,
            bounds: self.bounds(),
            default_color: if is_active {
                self.color_active
            } else {
                self.color
            },
        }
    }
}
