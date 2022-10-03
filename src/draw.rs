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

pub fn draw_text(lh: &mut LineHandler, text: &str, mut x: f32, y: f32) {
    for char in text.chars() {
        match char {
            '0' => {
                lh.add_line(Point {x, y}, Point { x, y: y + 40.0 });
                lh.add_line(Point {x, y: y + 40.0 }, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 20.0, y: y + 40.0}, Point {x: x + 20.0, y});
                lh.add_line(Point {x: x + 20.0, y}, Point {x, y});
                // lh.add_line(Point {x, y: y + 40.0}, Point {x: x + 20.0, y});
            },
            '1' => {
                lh.add_line(Point {x, y: y + 40.0}, Point { x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 10.0, y: y + 40.0}, Point { x: x + 10.0, y});
                lh.add_line(Point {x: x + 10.0, y}, Point { x, y: y + 10.0});
            },
            '2' => {
                lh.add_line(Point {x, y}, Point { x: x + 20.0, y });
                lh.add_line(Point {x: x + 20.0, y }, Point { x: x + 20.0, y: y + 20.0 });
                lh.add_line(Point {x: x + 20.0, y: y + 20.0 }, Point { x, y: y + 20.0 });
                lh.add_line(Point {x, y: y + 20.0 }, Point { x, y: y + 40.0 });
                lh.add_line(Point {x, y: y + 40.0 }, Point { x: x + 20.0, y: y + 40.0 });
            },
            '3' => {
                lh.add_line(Point {x, y}, Point { x: x + 20.0, y });
                lh.add_line(Point {x: x + 20.0, y}, Point { x: x + 20.0, y: y + 40.0 });
                lh.add_line(Point {x: x + 20.0, y: y + 40.0}, Point { x, y: y + 40.0 });
                lh.add_line(Point {x: x + 20.0, y: y + 20.0}, Point { x, y: y + 20.0 });
            },
            '4' => {
                lh.add_line(Point {x, y}, Point {x, y: y + 20.0});
                lh.add_line(Point {x, y: y + 20.0}, Point {x: x + 20.0, y: y + 20.0});
                lh.add_line(Point {x: x + 20.0, y}, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 20.0, y}, Point {x: x + 20.0, y: y + 40.0});
            },
            '5' => {
                lh.add_line(Point {x: x + 20.0, y}, Point {x, y});
                lh.add_line(Point {x, y}, Point {x, y: y + 20.0});
                lh.add_line(Point {x, y: y + 20.0}, Point {x: x + 20.0, y: y + 20.0});
                lh.add_line(Point {x: x + 20.0, y: y + 20.0}, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 20.0, y: y + 40.0}, Point {x, y: y + 40.0});
                
            },
            '6' => {
                lh.add_line(Point {x: x + 20.0, y}, Point {x, y});
                lh.add_line(Point {x, y}, Point {x, y: y + 40.0});
                lh.add_line(Point {x, y: y + 40.0}, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 20.0, y: y + 40.0}, Point {x: x + 20.0, y: y + 20.0});
                lh.add_line(Point {x: x + 20.0, y: y + 20.0}, Point {x, y: y + 20.0});
            },
            '7' => {
                lh.add_line(Point {x, y}, Point {x: x + 20.0, y});
                lh.add_line(Point {x: x + 20.0, y}, Point {x, y: y + 40.0});
            },
            '8' => {
                lh.add_line(Point {x, y}, Point { x, y: y + 40.0 });
                lh.add_line(Point {x, y: y + 40.0 }, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point {x: x + 20.0, y: y + 40.0}, Point {x: x + 20.0, y});
                lh.add_line(Point {x: x + 20.0, y}, Point {x, y});
                lh.add_line(Point {x, y: y + 20.0}, Point {x: x + 20.0, y: y + 20.0});
            },
            '9' => {
                lh.add_line(Point{x,y}, Point {x: x + 20.0, y});
                lh.add_line(Point{x: x + 20.0,y}, Point {x: x + 20.0, y: y + 40.0});
                lh.add_line(Point{x: x + 20.0, y: y + 40.0}, Point {x, y: y + 40.0});
                lh.add_line(Point{x,y}, Point {x, y: y + 20.0});
                lh.add_line(Point{x, y: y + 20.0}, Point {x: x + 20.0, y: y + 20.0});
            },
            '.' => {
                lh.add_line(Point {x, y: y + 40.0}, Point { x: x + 2.0, y: y + 40.0 });
                lh.add_line(Point {x: x + 2.0, y: y + 40.0}, Point { x: x + 2.0, y: y + 38.0 });
                lh.add_line(Point {x: x + 2.0, y: y + 38.0}, Point { x, y: y + 38.0 });
                lh.add_line(Point {x, y: y + 38.0}, Point { x, y: y + 40.0 });
                x -= 18.0;
            }
            _ => {
                x -= 30.0;
            },
        };

        x += 30.0;
    }
}