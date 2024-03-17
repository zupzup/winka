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
        let top = -1.0;
        let left = 1.0;
        let width = 0.25;
        let height = 0.125;

        let vertices = [
            Vertex {
                // A
                position: [-1.0, 1.0, 0.0],
                color,
            },
            Vertex {
                // B
                position: [-1.0, 0.875, 0.0],
                color,
            },
            Vertex {
                // C
                position: [0.750, 0.875, 0.0],
                color,
            },
            Vertex {
                // D
                position: [0.750, 1.0, 0.0],
                color,
            },
        ];
        // let vertices = [
        //     Vertex {
        //         // A
        //         position: [left, top, 0.0],
        //         color,
        //     },
        //     Vertex {
        //         // B
        //         position: [left, top + height, 0.0],
        //         color,
        //     },
        //     Vertex {
        //         // C
        //         position: [left + width, top, 0.0],
        //         color,
        //     },
        //     Vertex {
        //         // D
        //         position: [left + width, top + height, 0.0],
        //         color,
        //     },
        // ];

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
