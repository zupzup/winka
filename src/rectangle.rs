use crate::Vertex;

pub struct RectPos {
    pub top: u32,
    pub left: u32,
    pub bottom: u32,
    pub right: u32,
}

pub struct Rectangle {
    vertices: [Vertex; 4],
    indices: [u16; 6],
}

//  -1, 1    1,1
//  A--------D
//  |        |
//  |        |
//  |        |
//  B--------C
//  -1, -1   1, -1
impl Rectangle {
    pub fn new(pos: RectPos, color: [f32; 3], size: winit::dpi::PhysicalSize<u32>) -> Self {
        let top = 1.0 - (pos.top as f32 / (size.height as f32 / 2.0));
        let left = (pos.left as f32 / (size.width as f32 / 2.0)) - 1.0;
        let bottom = 1.0 - (pos.bottom as f32 / (size.height as f32 / 2.0));
        let right = (pos.right as f32 / (size.width as f32 / 2.0)) - 1.0;
        println!(
            "top: {top} left: {left} bottom: {bottom} right: {right} size: {}/{}",
            size.width, size.height
        );

        let vertices = [
            Vertex {
                // A
                position: [left, top, 0.0],
                color,
            },
            Vertex {
                // B
                position: [left, bottom, 0.0],
                color,
            },
            Vertex {
                // C
                position: [right, bottom, 0.0],
                color,
            },
            Vertex {
                // D
                position: [right, top, 0.0],
                color,
            },
        ];

        let indices = [0, 1, 2, 0, 2, 3];
        Self { vertices, indices }
    }

    pub fn vertices(&self) -> &[Vertex; 4] {
        &self.vertices
    }

    pub fn indices(&self) -> &[u16; 6] {
        &self.indices
    }

    pub fn num_indices(&self) -> u32 {
        self.indices.len() as u32
    }
}
