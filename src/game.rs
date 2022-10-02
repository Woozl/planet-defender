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
    planet_size: f32,
    distance: f32,
    lasers: Vec<Laser>,
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
            planet_size: 100.0,
            distance: 30.0,
            lasers: Vec::new(),
        }
    }

    pub fn set_cursor(&mut self, x: u32, y: u32) {
        self.cur_x = x;
        self.cur_y = y;
        let local_x = self.cur_x as f32 - (WIDTH / 2) as f32;
        let local_y = -1.0 * (self.cur_y as f32 - (HEIGHT / 2) as f32);
        self.cur_angle = local_y.atan2(local_x);
    }

    pub fn fire(&mut self) {
        self.lasers.push(Laser {
            loc: Point {
                x: (self.distance + 30.0 + self.planet_size) * self.cur_angle.cos()
                    + (WIDTH / 2) as f32,
                y: -(self.distance + 30.0 + self.planet_size) * self.cur_angle.sin()
                    + (HEIGHT / 2) as f32,
            },
            vx: 500.0 * self.cur_angle.cos(),
            vy: -500.0 * self.cur_angle.sin(),
        });
    }

    pub fn draw(&mut self) {
        self.lines.clear_lines();
        let new_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let dt = new_time - self.current_ms;
        self.current_ms = new_time;

        self.draw_ship();
        self.draw_planet();
        self.draw_lasers(dt);
    }

    fn draw_lasers(&mut self, dt: u128) {
        fn laser_out_of_bounds(l: &Laser, epsilon: f32) -> bool {
            l.loc.x < -epsilon
            || l.loc.x > WIDTH as f32 + epsilon
            || l.loc.y < -epsilon
            || l.loc.y > HEIGHT as f32 + epsilon
        }

        let mut i = 0;
        while i < self.lasers.len() {
            // remove lasers outside the screen
            if laser_out_of_bounds(&self.lasers[i], 30.0) {
                self.lasers.remove(i);
            } else {
                let laser = &mut self.lasers[i];
                
                // update and draw laser if still in screen
                laser.loc.x += laser.vx * (dt as f32 / 1000.0);
                laser.loc.y += laser.vy * (dt as f32 / 1000.0);
                
                self.lines.add_line(
                    Point {
                        x: laser.loc.x + laser.vx * -30.0 / 500.0,
                        y: laser.loc.y + laser.vy * -30.0 / 500.0,
                    },
                    Point {
                        x: laser.loc.x,
                        y: laser.loc.y,
                    },
                );

                i += 1;
            }
        }
    }

    fn draw_ship(&mut self) {
        let angle_deg = self.cur_angle.to_degrees();
        let side1_deg = angle_deg - 6.0;
        let side2_deg = angle_deg + 6.0;

        let pt1 = Point {
            x: (self.distance + self.planet_size) * side1_deg.to_radians().cos()
                + (WIDTH / 2) as f32,
            y: -(self.distance + self.planet_size) * side1_deg.to_radians().sin()
                + (HEIGHT / 2) as f32,
        };
        let pt2 = Point {
            x: (self.distance + self.planet_size + 30.0) * angle_deg.to_radians().cos()
                + (WIDTH / 2) as f32,
            y: -(self.distance + self.planet_size + 30.0) * angle_deg.to_radians().sin()
                + (HEIGHT / 2) as f32,
        };
        let pt3 = Point {
            x: (self.distance + self.planet_size) * side2_deg.to_radians().cos()
                + (WIDTH / 2) as f32,
            y: -(self.distance + self.planet_size) * side2_deg.to_radians().sin()
                + (HEIGHT / 2) as f32,
        };

        self.lines.add_line(pt1, pt2);
        self.lines.add_line(pt2, pt3);
        self.lines.add_line(pt3, pt1);
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

#[derive(Debug)]
struct Laser {
    loc: Point,
    vx: f32,
    vy: f32,
}
