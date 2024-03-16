use crate::Vertex;

pub struct Rectangle {
    vertices: [Vertex; 4],
    indices: [u16; 6],
}

impl Rectangle {
    pub fn new(/*_x: u32, _y: u32, _width: u32, _height: u32, */ color: [f32; 3]) -> Self {
        let vertices = [
            Vertex {
                // A
                position: [0.0, 0.1, 0.0],
                color,
            },
            Vertex {
                // B
                position: [0.0, 0.0, 0.0],
                color,
            },
            Vertex {
                // C
                position: [0.1, 0.0, 0.0],
                color,
            },
            Vertex {
                // D
                position: [0.1, 0.1, 0.0],
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
