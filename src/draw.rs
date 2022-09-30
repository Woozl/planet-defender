use crate::{HEIGHT, WIDTH, Vertex};

pub fn screen_space_to_draw_space(x: u32, y: u32) -> (f32, f32) {
    (
        ((x as f32 - (WIDTH as f32 / 2.0)) / (WIDTH as f32 / 2.0)),
        -1.0 * ((y as f32 - (HEIGHT as f32 / 2.0)) / (HEIGHT as f32 / 2.0)),
    )
}

pub struct Lines {
    pub vertices: Vec<Vertex>
}

impl Lines {
    pub fn new() -> Self {
        Lines {
            vertices: Vec::new(),
        }
    }

    pub fn add_line(&mut self, p1: (u32, u32), p2: (u32, u32)) {
        let (p1_x, p1_y) = screen_space_to_draw_space(p1.0, p1.1);
        let (p2_x, p2_y) = screen_space_to_draw_space(p2.0, p2.1);
        
        self.vertices.push(Vertex {
            position: [p1_x, p1_y, 0.0],
            color: [1.0, 1.0, 1.0]
        });
        self.vertices.push(Vertex {
            position: [p2_x, p2_y, 0.0],
            color: [1.0, 1.0, 1.0]
        });
    }

    pub fn clear_lines(&mut self) {
        self.vertices.clear();
    }
}