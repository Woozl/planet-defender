use rand::Rng;
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
    planet_size: f32,
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
            planet_size: 100.0,
        }
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        self.cur_x = x;
        self.cur_y = y;
        let local_x = self.cur_x as f32 - (WIDTH / 2) as f32;
        let local_y = -1.0 * (self.cur_y as f32 - (HEIGHT / 2) as f32);
        self.cur_angle = local_y.atan2(local_x);
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
        self.planet_size = 100.0 + 10.0 * (self.current_ms as f64 / 500.0).sin() as f32;

        let mut first_point = Point {
            x: self.planet_size * 0.0f32.to_radians().cos() + (WIDTH / 2) as f32,
            y: -self.planet_size * 0.0f32.to_radians().sin() + (HEIGHT / 2) as f32,
        };
        let mut last_point = first_point;
        let mut current_point = first_point;
        for theta in (0..360).step_by(6) {
            let point_deviance = self.planet_size + rand::thread_rng().gen_range(-5.0..5.0);

            current_point = Point {
                x: point_deviance * (theta as f32).to_radians().cos() + (WIDTH / 2) as f32,
                y: -1.0 * point_deviance * (theta as f32).to_radians().sin() + (HEIGHT / 2) as f32,
            };

            // save the first point to connect the last point up to it to complete the path
            if theta == 0 {
                first_point = current_point;
            } else {
                self.lines.add_line(last_point, current_point)
            }

            last_point = current_point;
        }
        // finish path
        self.lines.add_line(current_point, first_point);
    }
}