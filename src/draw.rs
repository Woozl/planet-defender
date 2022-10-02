use crate::{HEIGHT, WIDTH, Vertex};

/// screen space representation, coordinates range from 0 .. screen size
# [derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn to_draw_space(&self) -> DrawSpacePoint {
        DrawSpacePoint {
            x: (self.x as f32 - (WIDTH as f32 / 2.0)) / (WIDTH as f32 / 2.0),
            y: -1.0 * ((self.y as f32 - (HEIGHT as f32 / 2.0)) / (HEIGHT as f32 / 2.0)),
        }
    }
}

/// internal representation: coordinates range from -1.0 .. 1.0
# [derive(Clone, Copy)]
pub struct DrawSpacePoint {
    pub x: f32,
    pub y: f32,
}

impl DrawSpacePoint {
    pub fn _to_screen_space(&self) -> Point {
        Point {
            x: ((self.x * WIDTH as f32 / 2.0) + WIDTH as f32 / 2.0),
            y: ((-1.0 * self.y * HEIGHT as f32 / 2.0) + HEIGHT as f32 / 2.0),
        }
    }
}

pub struct LineHandler {
    pub vertices: Vec<Vertex>
}

impl LineHandler {
    pub fn new() -> Self {
        LineHandler {
            vertices: Vec::new(),
        }
    }

    pub fn add_line(&mut self, p1: Point, p2: Point) {
        let p1_ds = p1.to_draw_space();
        let p2_ds = p2.to_draw_space();
        
        self.vertices.push(Vertex {
            position: [p1_ds.x, p1_ds.y, 0.0],
            color: [1.0, 1.0, 1.0]
        });
        self.vertices.push(Vertex {
            position: [p2_ds.x, p2_ds.y, 0.0],
            color: [1.0, 1.0, 1.0]
        });
    }

    pub fn clear_lines(&mut self) {
        self.vertices.clear();
    }
}