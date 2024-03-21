use crate::rectangle::Rectangle;
use crate::text::Text;

pub struct Button {
    text: Text,
    rectangle: Rectangle,
}

impl Button {
    pub fn new(rectangle: Rectangle, text: Text) -> Self {
        Self { rectangle, text }
    }

    pub fn text(&self) -> &Text {
        &self.text
    }

    pub fn rectangle(&mut self) -> &mut Rectangle {
        &mut self.rectangle
    }
}
