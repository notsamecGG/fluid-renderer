use crate::Vertex;

pub trait Shape {
    const VERTICES: &'static [Vertex];
    const INDICES: &'static [u16];

    fn scale(&self, factor: f32, aspect_ratio: f32) -> Vec<Vertex> {
        let vertices = Self::VERTICES.iter().map(|vertex| {
            let mut position = vertex.position.clone();
            position[0] *= aspect_ratio;

            Vertex {
                position: position.map(|x| {x * factor}),
                tex_coords: vertex.tex_coords,
            }
        }).collect::<Vec<_>>();

        vertices
    }
}


pub struct Pentagon;

impl Shape for Pentagon {
    const VERTICES: &'static [Vertex] = &[
        Vertex {
            position: [-0.0868241, 0.49240386, 0.0],
            tex_coords: [0.4131759, 0.00759614],
        }, // A
        Vertex {
            position: [-0.49513406, 0.06958647, 0.0],
            tex_coords: [0.0048659444, 0.43041354],
        }, // B
        Vertex {
            position: [-0.21918549, -0.44939706, 0.0],
            tex_coords: [0.28081453, 0.949397],
        }, // C
        Vertex {
            position: [0.35966998, -0.3473291, 0.0],
            tex_coords: [0.85967, 0.84732914],
        }, // D
        Vertex {
            position: [0.44147372, 0.2347359, 0.0],
            tex_coords: [0.9414737, 0.2652641],
        }, // E
    ];

    const INDICES: &'static [u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4, /* padding */ 0];
}



pub struct Quad;

impl Shape for Quad {
    const VERTICES: &'static [Vertex] = &[
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coords: [0.0, 0.0],
        }, // A
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coords: [1.0, 0.0],
        }, // B
        Vertex {
            position: [0.5, 0.5, 0.0],
            tex_coords: [1.0, 1.0],
        }, // C
        Vertex {
            position: [-0.5, 0.5, 0.0],
            tex_coords: [0.0, 1.0],
        }, // D
    ];

    const INDICES: &'static [u16] = &[0, 1, 3, 1, 2, 3];
}
