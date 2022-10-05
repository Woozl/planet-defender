use instant::Instant;
use rand::{thread_rng, Rng};
use std::{
    f32::consts::PI,
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
    program_begin: Instant,
    start_time: u128,
    current_ms: u128,
    game_time: u128,
    last_asteroid_time: u128,
    asteroid_spawn_rate: u128,
    planet_size: f32,
    distance: f32,
    lasers: Vec<Laser>,
    asteroids: Vec<Asteroid>,
    asteroids_destroyed: u32,
    lives: u32,
    is_game_over: bool,
}

impl Game {
    pub fn new() -> Self {
        let program_begin = Instant::now();
        let start_time = Instant::now().duration_since(program_begin).as_millis();

        Self {
            lines: LineHandler::new(),
            cur_x: HEIGHT / 2,
            cur_y: WIDTH / 2,
            cur_angle: 0.0,
            program_begin,
            start_time,
            current_ms: start_time,
            game_time: 0,
            last_asteroid_time: start_time,
            asteroid_spawn_rate: 1000,
            planet_size: 100.0,
            distance: 30.0,
            lasers: Vec::new(),
            asteroids: Vec::new(),
            asteroids_destroyed: 0,
            lives: 5,
            is_game_over: false,
        }
    }

    pub fn restart(&mut self) {
        self.is_game_over = false;
        self.start_time = self.current_ms;
        self.game_time = 0;
        self.lives = 5;
        self.asteroids_destroyed = 0;
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
        let new_time = Instant::now().duration_since(self.program_begin).as_millis();
        let dt = new_time - self.current_ms;
        self.current_ms = new_time;

        if self.lives <= 0 {
            self.is_game_over = true;
        }

        if !self.is_game_over {
            self.game_time = self.current_ms - self.start_time;
        }

        if self.asteroids_destroyed < 30 {
            self.asteroid_spawn_rate = 2000;
        } else if self.asteroids_destroyed < 60 {
            self.asteroid_spawn_rate = 1500;
        } else if self.asteroids_destroyed < 80 {
            self.asteroid_spawn_rate = 1200;
        } else if self.asteroids_destroyed < 100 {
            self.asteroid_spawn_rate = 1000;
        } else if self.asteroids_destroyed < 120 {
            self.asteroid_spawn_rate = 800;
        } else if self.asteroids_destroyed < 150 {
            self.asteroid_spawn_rate = 600;
        } else {
            self.asteroid_spawn_rate = 400;
        }

        if !self.is_game_over
            && self.current_ms - self.last_asteroid_time > self.asteroid_spawn_rate
        {
            self.add_asteroid(20.0);
            self.last_asteroid_time = self.current_ms;
        }

        if self.is_game_over {
            self.draw_game_over();
        }
        self.draw_ship();
        self.draw_planet();
        self.draw_lasers(dt);
        self.draw_text(
            &format!("{:.2}", self.game_time as f64 / 1000.0),
            10.0,
            10.0,
        );
        self.draw_text(&format!("{}", self.asteroids_destroyed), 500.0, 10.0);
        self.draw_hearts(self.lives, WIDTH as f32 - 10.0, 10.0);
        self.draw_asteroids(dt);
        self.check_collision();
    }

    fn check_collision(&mut self) {
        let mut a = 0;
        while a < self.asteroids.len() {
            let asteroid = &self.asteroids[a];

            let mut collides_with_laser = false;
            let mut l = 0;
            while l < self.lasers.len() {
                let laser = &self.lasers[l];
                if distance(laser.loc, asteroid.loc) < 20.0 {
                    collides_with_laser = true;
                    if !self.is_game_over {
                        self.asteroids_destroyed += 1;
                    }
                    break;
                } else {
                    l += 1;
                }
            }

            if collides_with_laser {
                // remove asteroid + laser
                self.asteroids.remove(a);
                self.lasers.remove(l);
            } else {
                a += 1;
            }
        }

        fn distance(pt1: Point, pt2: Point) -> f32 {
            ((pt2.x - pt1.x).powf(2.0) + (pt2.y - pt1.y).powf(2.0)).sqrt()
        }
    }

    fn add_asteroid(&mut self, size: f32) {
        let quadrant = [
            rand::thread_rng().gen_range(0.0..=(PI / 2.0)),
            rand::thread_rng().gen_range((PI / 2.0)..=PI),
            rand::thread_rng().gen_range(PI..=(3.0 * PI / 2.0)),
            rand::thread_rng().gen_range((3.0 * PI / 2.0)..=(2.0 * PI)),
        ];

        let angle = rand::thread_rng().gen_range(0.0..(2.0 * PI));
        let distance = 707.0;

        let x = distance * angle.cos() + (WIDTH as f32 / 2.0);
        let y = distance * angle.sin() + (HEIGHT as f32 / 2.0);

        let velocity = rand::thread_rng().gen_range(50.0..150.0);

        let vx = -velocity * angle.cos();
        let vy = -velocity * angle.sin();

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
            vx,
            vy,
            rotation_speed: PI
                / (if rand::thread_rng().gen_bool(0.5) {
                    -1.0
                } else {
                    1.0
                } * thread_rng().gen_range(200.0..500.00)),
            // rotation_speed: PI / rand::thread_rng().gen_range(-100.0..100.0),
        });
    }

    fn asteroid_hit(&self, a: &Asteroid) -> bool {
        let Asteroid { loc, .. } = a;
        let Point { x, y } = loc;

        ((x - WIDTH as f32 / 2.0).powf(2.0) + (y - HEIGHT as f32 / 2.0).powf(2.0)).sqrt()
            < self.planet_size
    }

    fn draw_game_over(&mut self) {
        self.lines
            .add_line(Point { x: 460.0, y: 530.0 }, Point { x: 540.0, y: 530.0 });
        self.lines
            .add_line(Point { x: 460.0, y: 530.0 }, Point { x: 440.0, y: 550.0 });
        self.lines
            .add_line(Point { x: 540.0, y: 530.0 }, Point { x: 560.0, y: 550.0 });
        self.lines
            .add_line(Point { x: 460.0, y: 470.0 }, Point { x: 460.0, y: 440.0 });
        self.lines
            .add_line(Point { x: 540.0, y: 470.0 }, Point { x: 540.0, y: 440.0 });

        self.lines
            .add_line(Point { x: 480.0, y: 200.0 }, Point { x: 480.0, y: 280.0 });
        self.lines
            .add_line(Point { x: 480.0, y: 200.0 }, Point { x: 520.0, y: 200.0 });
        self.lines
            .add_line(Point { x: 520.0, y: 200.0 }, Point { x: 520.0, y: 240.0 });
        self.lines
            .add_line(Point { x: 520.0, y: 240.0 }, Point { x: 480.0, y: 240.0 });
        self.lines
            .add_line(Point { x: 480.0, y: 240.0 }, Point { x: 520.0, y: 280.0 });
        self.lines
            .add_line(Point { x: 460.0, y: 210.0 }, Point { x: 460.0, y: 190.0 });
        self.lines
            .add_line(Point { x: 450.0, y: 210.0 }, Point { x: 450.0, y: 190.0 });
        self.lines
            .add_line(Point { x: 540.0, y: 210.0 }, Point { x: 540.0, y: 190.0 });
        self.lines
            .add_line(Point { x: 550.0, y: 210.0 }, Point { x: 550.0, y: 190.0 });
    }

    fn draw_asteroids(&mut self, dt: u128) {
        let mut i = 0;
        while i < self.asteroids.len() {
            if self.asteroid_hit(&self.asteroids[i]) {
                self.asteroids.remove(i);
                if self.lives > 0 {
                    self.lives -= 1;
                }
            } else {
                self.asteroids[i].update(dt);

                self.lines
                    .add_line(self.asteroids[i].p1, self.asteroids[i].p2);
                self.lines
                    .add_line(self.asteroids[i].p2, self.asteroids[i].p3);
                self.lines
                    .add_line(self.asteroids[i].p3, self.asteroids[i].p4);
                self.lines
                    .add_line(self.asteroids[i].p4, self.asteroids[i].p1);

                i += 1;
            }
        }
    }

    fn draw_text(&mut self, text: &str, x: f32, y: f32) {
        draw_text(&mut self.lines, text, x, y);
    }

    #[rustfmt::skip]
    fn draw_hearts(&mut self, hearts: u32, mut x: f32, y: f32) {
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
    rotation_speed: f32,
}

impl Asteroid {
    pub fn update(&mut self, dt: u128) {
        // update position
        self.loc.x += self.vx * (dt as f32 / 1000.0);
        self.loc.y += self.vy * (dt as f32 / 1000.0);
        self.p1.x += self.vx * (dt as f32 / 1000.0);
        self.p1.y += self.vy * (dt as f32 / 1000.0);
        self.p2.x += self.vx * (dt as f32 / 1000.0);
        self.p2.y += self.vy * (dt as f32 / 1000.0);
        self.p3.x += self.vx * (dt as f32 / 1000.0);
        self.p3.y += self.vy * (dt as f32 / 1000.0);
        self.p4.x += self.vx * (dt as f32 / 1000.0);
        self.p4.y += self.vy * (dt as f32 / 1000.0);

        // rotate points
        fn rotate(mut pt: Point, loc: Point, angle: f32) -> Point {
            pt.x -= loc.x;
            pt.y -= loc.y;

            let new_x = pt.x * angle.cos() - pt.y * angle.sin();
            let new_y = pt.y * angle.cos() + pt.x * angle.sin();

            pt.x = new_x;
            pt.y = new_y;

            pt.x += loc.x;
            pt.y += loc.y;

            Point { x: pt.x, y: pt.y }
        }

        self.p1 = rotate(self.p1, self.loc, self.rotation_speed);
        self.p2 = rotate(self.p2, self.loc, self.rotation_speed);
        self.p3 = rotate(self.p3, self.loc, self.rotation_speed);
        self.p4 = rotate(self.p4, self.loc, self.rotation_speed);
    }
}
