use crate::Vertex;

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
    pub fn new(
        top: u32,
        left: u32,
        width: u32,
        height: u32,
        color: [f32; 3],
        size: winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        // TODO: fix calculation and clean up
        let top = 1.0 - (top as f32 / (size.height as f32 / 2.0));
        let left = (left as f32 / (size.width as f32 / 2.0)) - 1.0;
        let width = (width as f32 / (size.width as f32 / 2.0)) - 1.0;
        let height = 1.0 - (height as f32 / (size.height as f32 / 2.0));
        println!(
            "top: {top} left: {left} width: {width} height: {height} size: {}/{}, A: {}, {}, B: {}, {}, C: {}, {}, D: {}, {}",
            size.width, size.height, left, top, left, top-height, left-width, top-height, left-width, top
        );

        let vertices = [
            Vertex {
                // A
                position: [left, top, 0.0],
                color,
            },
            Vertex {
                // B
                position: [left, top + height - 1.0, 0.0],
                color,
            },
            Vertex {
                // C
                position: [left + width + 1.0, top + height - 1.0, 0.0],
                color,
            },
            Vertex {
                // D
                position: [left + width + 1.0, top, 0.0],
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
