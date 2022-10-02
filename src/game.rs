use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    draw::{LineHandler, Point},
    HEIGHT, WIDTH,
};

pub struct Game {
    pub lines: LineHandler,
    cur_x: u32,
    cur_y: u32,
    current_ms: u128,
    ball: Point,
    vx: i32,
    vy: i32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            lines: LineHandler::new(),
            cur_x: HEIGHT / 2,
            cur_y: WIDTH / 2,
            current_ms: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            ball: Point { x: 200, y: 200 },
            vx: 100,
            vy: -100,
        }
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        self.cur_x = x;
        self.cur_y = y;
        println!("x: {}, y: {}", self.cur_x, self.cur_y);
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
            &Point { x: 500, y: 500 },
            &Point {
                x: self.cur_x,
                y: self.cur_y,
            },
        );

        let mut actual_x = self.ball.x as i32 + ((self.vx as f32) * (dt as f32 / 100.0)) as i32;
        let mut actual_y = self.ball.y as i32 + ((self.vy as f32) * (dt as f32 / 100.0)) as i32;
        if actual_x < 0 {
            actual_x = 0;
            self.vx *= -1;
        }
        if actual_x > WIDTH as i32 {
            actual_x = WIDTH as i32;
            self.vx *= -1;
        }
        if actual_y < 0 {
            actual_y = 0;
            self.vy *= -1;
        }
        if actual_y > HEIGHT as i32 {
            actual_y = HEIGHT as i32;
            self.vy *= -1;
        }
        self.ball.x = actual_x as u32;
        self.ball.y = actual_y as u32;
        // println!("{}, {}", self.ball.x, self.ball.y);

        self.lines.add_line(&Point { x: 500, y: 500 }, &self.ball);

        self.draw_planet();
    }

    fn draw_planet(&mut self) {
        let p1 = Point { x: 400, y: 400 };
        let p2 = Point { x: 600, y: 400 };
        let p3 = Point { x: 600, y: 600 };
        let p4 = Point { x: 400, y: 600 };

        self.lines.add_line(&p1, &p2);
        self.lines.add_line(&p2, &p3);
        self.lines.add_line(&p3, &p4);
        self.lines.add_line(&p4, &p1);
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
