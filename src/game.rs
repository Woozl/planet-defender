use crate::{draw::{LineHandler, Point}, HEIGHT, WIDTH};

pub struct Game {
    pub lines: LineHandler,
    cur_x: u32,
    cur_y: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            lines: LineHandler::new(),
            cur_x: HEIGHT / 2,
            cur_y: WIDTH / 2,
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

        // update state based on input
        self.lines.add_line(&Point {x: 500, y: 500}, &Point {x: self.cur_x, y: self.cur_y});

        self.draw_planet();
    }

    fn draw_planet(&mut self) {
        let p1 = Point {x: 400, y: 400};
        let p2 = Point {x: 600, y: 400};
        let p3 = Point {x: 600, y: 600};
        let p4 = Point {x: 400, y: 600};

        self.lines.add_line(&p1, &p2);
        self.lines.add_line(&p2, &p3);
        self.lines.add_line(&p3, &p4);
        self.lines.add_line(&p4, &p1);
    }
}