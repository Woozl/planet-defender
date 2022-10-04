use rand::Rng;
use std::{
    f32::consts::PI,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    draw::{draw_text, LineHandler, Point},
    HEIGHT, WIDTH,
};

pub struct Game {
    pub lines: LineHandler,
    cur_x: u32,
    cur_y: u32,
    cur_angle: f32,
    start_time: u128,
    current_ms: u128,
    planet_size: f32,
    distance: f32,
    lasers: Vec<Laser>,
    asteroids: Vec<Asteroid>,
}

impl Game {
    pub fn new() -> Self {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        Self {
            lines: LineHandler::new(),
            cur_x: HEIGHT / 2,
            cur_y: WIDTH / 2,
            cur_angle: 0.0,
            start_time,
            current_ms: start_time,
            planet_size: 100.0,
            distance: 30.0,
            lasers: Vec::new(),
            asteroids: Vec::new(),
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

        self.add_asteroid(self.cur_x as f32, self.cur_y as f32, 0.0, 20.0);
    }

    pub fn draw(&mut self) {
        self.lines.clear_lines();
        let new_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let dt = new_time - self.current_ms;
        self.current_ms = new_time;
        let time_since_start_sec = (self.current_ms - self.start_time) as f64 / 1000.0;

        self.draw_ship();
        self.draw_planet();
        self.draw_lasers(dt);
        self.draw_text(&format!("{:.2}", time_since_start_sec), 10.0, 10.0);
        self.draw_hearts(5, WIDTH as f32 - 10.0, 10.0);
        self.draw_asteroids(dt);
    }

    fn add_asteroid(&mut self, x: f32, y: f32, rot: f32, size: f32) {
        let quadrant = [
            rand::thread_rng().gen_range(0.0..=(PI / 2.0)),
            rand::thread_rng().gen_range((PI / 2.0)..=PI),
            rand::thread_rng().gen_range(PI..=(3.0 * PI / 2.0)),
            rand::thread_rng().gen_range((3.0 * PI / 2.0)..=(2.0 * PI)),
        ];

        let p1 = Point {
            x: size * quadrant[0].cos(),
            y: -size * quadrant[0].sin(),
        };
        let p2 = Point {
            x: size * quadrant[1].cos(),
            y: -size * quadrant[1].sin(),
        };
        let p3 = Point {
            x: size * quadrant[2].cos(),
            y: -size * quadrant[2].sin(),
        };
        let p4 = Point {
            x: size * quadrant[3].cos(),
            y: -size * quadrant[3].sin(),
        };

        self.asteroids.push(Asteroid {
            loc: Point { x, y },
            p1: Point {
                x: p1.x + x,
                y: p1.y + y,
            },
            p2: Point {
                x: p2.x + x,
                y: p2.y + y,
            },
            p3: Point {
                x: p3.x + x,
                y: p3.y + y,
            },
            p4: Point {
                x: p4.x + x,
                y: p4.y + y,
            },
            vx: rand::thread_rng().gen_range(-100.0..100.0),
            vy: rand::thread_rng().gen_range(-100.0..100.0),
        });

    }

    fn draw_asteroids(&mut self, dt: u128) {
        let mut i = 0;
        while i < self.asteroids.len() {
            let Asteroid {
                // mut p1,
                // mut p2,
                // mut p3,
                // mut p4,
                vx,
                vy,
                // mut loc,
                ..
            } = self.asteroids[i];

            self.asteroids[i].loc.x += vx * (dt as f32 / 1000.0);
            self.asteroids[i].loc.y += vy * (dt as f32 / 1000.0);
            self.asteroids[i].p1.x += vx * (dt as f32 / 1000.0);
            self.asteroids[i].p1.y += vy * (dt as f32 / 1000.0);
            self.asteroids[i].p2.x += vx * (dt as f32 / 1000.0);
            self.asteroids[i].p2.y += vy * (dt as f32 / 1000.0);
            self.asteroids[i].p3.x += vx * (dt as f32 / 1000.0);
            self.asteroids[i].p3.y += vy * (dt as f32 / 1000.0);
            self.asteroids[i].p4.x += vx * (dt as f32 / 1000.0);
            self.asteroids[i].p4.y += vy * (dt as f32 / 1000.0);

            self.lines.add_line(self.asteroids[i].p1, self.asteroids[i].p2);
            self.lines.add_line(self.asteroids[i].p2, self.asteroids[i].p3);
            self.lines.add_line(self.asteroids[i].p3, self.asteroids[i].p4);
            self.lines.add_line(self.asteroids[i].p4, self.asteroids[i].p1);

            i += 1;
        }
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        draw_text(&mut self.lines, text, x, y);
    }

    #[rustfmt::skip]
    fn draw_hearts(&mut self, hearts: i32, mut x: f32, y: f32) {
        for _ in 0..hearts {
            self.lines.add_line(Point {x: x - 5.0, y}, Point {x: x - 15.0, y});
            self.lines.add_line(Point {x: x - 15.0, y}, Point {x: x - 20.0, y: y + 5.0});
            self.lines.add_line(Point {x: x - 20.0, y: y + 5.0}, Point {x: x - 25.0, y});
            self.lines.add_line(Point {x: x - 25.0, y}, Point {x: x - 35.0, y});
            self.lines.add_line(Point {x: x - 35.0, y}, Point {x: x - 40.0, y: y + 5.0});
            self.lines.add_line(Point {x: x - 40.0, y: y + 5.0}, Point {x: x - 40.0, y: y + 20.0});
            self.lines.add_line(Point {x: x - 40.0, y: y + 20.0}, Point {x: x - 20.0, y: y + 40.0});
            self.lines.add_line(Point {x: x - 20.0, y: y + 40.0}, Point {x, y: y + 20.0});
            self.lines.add_line(Point {x, y: y + 20.0}, Point {x, y: y + 5.0});
            self.lines.add_line(Point {x, y: y + 5.0}, Point {x: x - 5.0, y});

            x -= 50.0;
        }
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

struct Asteroid {
    loc: Point,
    vx: f32,
    vy: f32,
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
}
