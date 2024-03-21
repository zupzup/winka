use crate::Vertex;
use winit::dpi::PhysicalPosition;

pub struct RectPos {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub struct Rectangle {
    position: RectPos,
    color: [f32; 3],
    color_hover: [f32; 3],
    border_color: [f32; 3],
    border_color_clicked: [f32; 3],
}

impl Rectangle {
    pub fn new(
        position: RectPos,
        color: [f32; 3],
        color_hover: [f32; 3],
        border_color: [f32; 3],
        border_color_clicked: [f32; 3],
    ) -> Self {
        Self {
            color,
            color_hover,
            border_color,
            border_color_clicked,
            position,
        }
    }

    pub fn position(&self) -> &RectPos {
        &self.position
    }

    pub fn vertices(
        &mut self,
        mouse_coords: PhysicalPosition<f64>,
        clicked: bool,
        size: winit::dpi::PhysicalSize<u32>,
    ) -> [Vertex; 4] {
        // TODO: memoize these calculations for size
        let top = 1.0 - (self.position.top as f32 / (size.height as f32 / 2.0));
        let left = (self.position.left as f32 / (size.width as f32 / 2.0)) - 1.0;
        let bottom = 1.0 - (self.position.bottom as f32 / (size.height as f32 / 2.0));
        let right = (self.position.right as f32 / (size.width as f32 / 2.0)) - 1.0;

        let rect = [
            self.position.top as f32,
            self.position.left as f32,
            self.position.bottom as f32,
            self.position.right as f32,
        ];
        let mut color = self.color;
        let mut border_color = self.border_color;

        if mouse_coords.x > self.position.left as f64
            && mouse_coords.x < self.position.right as f64
            && mouse_coords.y > self.position.top as f64
            && mouse_coords.y < self.position.bottom as f64
        {
            color = self.color_hover;
            if clicked {
                border_color = self.border_color_clicked;
            }
        }

        //  -1, 1    1,1
        //  A--------D
        //  |        |
        //  |        |
        //  |        |
        //  B--------C
        //  -1, -1   1, -1
        [
            Vertex {
                // A
                position: [left, top, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // B
                position: [left, bottom, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // C
                position: [right, bottom, 0.0],
                color,
                rect,
                border_color,
            },
            Vertex {
                // D
                position: [right, top, 0.0],
                color,
                rect,
                border_color,
            },
        ]
    }

    pub fn indices(&self) -> [u16; 6] {
        [0, 1, 2, 0, 2, 3]
    }

    pub fn num_indices(&self) -> u32 {
        6
    }
}
