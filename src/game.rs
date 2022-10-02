use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    draw::{LineHandler, Point},
    HEIGHT, WIDTH,
};

pub struct Game {
    pub lines: LineHandler,
    cur_x: u32,
    cur_y: u32,
    cur_angle: f32,
    current_ms: u128,
    ball: Point,
    vx: f32,
    vy: f32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            lines: LineHandler::new(),
            cur_x: HEIGHT / 2,
            cur_y: WIDTH / 2,
            cur_angle: 0.0,
            current_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            ball: Point { x: 200.0, y: 300.0 },
            vx: 100.0,
            vy: -100.0,
        }
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        self.cur_x = x;
        self.cur_y = y;
        let local_x = self.cur_x as f32 - (WIDTH / 2) as f32;
        let local_y = -1.0 * (self.cur_y as f32 - (HEIGHT / 2) as f32);
        self.cur_angle = local_y.atan2(local_x).to_degrees();
        // println!("x: {}, y: {}, th: {}", self.cur_x, self.cur_y, self.cur_angle);
    }

    pub fn fire(&mut self) {
        println!("Pew pew");
    }

    pub fn draw(&mut self) {
        self.lines.clear_lines();
        let new_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let dt = new_time - self.current_ms;
        self.current_ms = new_time;

        // draw line based on cursor
        self.lines.add_line(
            Point { x: 500.0, y: 500.0 },
            Point {
                x: self.cur_x as f32,
                y: self.cur_y as f32,
            },
        );

        self.ball.x += self.vx * (dt as f32 / 1000.0);
        self.ball.y += self.vy * (dt as f32 / 1000.0);
        if self.ball.x < 0.0 {
            self.ball.x = 0.0;
            self.vx *= -1.0;
        }
        if self.ball.x > WIDTH as f32 {
            self.ball.x = WIDTH as f32;
            self.vx *= -1.0;
        }
        if self.ball.y < 0.0 {
            self.ball.y = 0.0;
            self.vy *= -1.0;
        }
        if self.ball.y > HEIGHT as f32 {
            self.ball.y = HEIGHT as f32;
            self.vy *= -1.0;
        }
        // println!("{}, {}", self.ball.x, self.ball.y);

        self.lines.add_line(Point { x: 500.0, y: 500.0 }, self.ball);

        self.draw_planet();
    }

    fn draw_planet(&mut self) {
        let p1 = Point { x: 400.0, y: 400.0 };
        let p2 = Point { x: 600.0, y: 400.0 };
        let p3 = Point { x: 600.0, y: 600.0 };
        let p4 = Point { x: 400.0, y: 600.0 };

        self.lines.add_line(p1, p2);
        self.lines.add_line(p2, p3);
        self.lines.add_line(p3, p4);
        self.lines.add_line(p4, p1);
    }
}

// struct Box {
//     position: Point,
//     width: u32,
//     height: u32,
//     top_left: Point,
//     top_right: Point,
//     bottom_left: Point,
//     bottom_right: Point,
//     x_vel: u32,
//     y_vel: u32,
// }

// impl Box {
//     pub fn new(position: Point, width: u32, height: u32, x_vel: u32, y_vel: u32) -> Self {
//         Self {
//             position,
//             width,
//             height,
//             top_left: Point {
//                 x: position.x,
//                 y: position.y,
//             },
//             top_right: Point {
//                 x: position.x + width,
//                 y: position.y,
//             },
//             bottom_left: Point {
//                 x: position.x,
//                 y: position.y + height,
//             },
//             bottom_right: Point {
//                 x: position.x + width,
//                 y: position.y + height,
//             },
//             x_vel,
//             y_vel
//         }
//     }

//     pub fn update(&mut self, time: u128) {
//         self.position.x = ;
//     }

//     pub fn push_verts(&mut self, line_handler: &mut LineHandler) {

//     }
// }
