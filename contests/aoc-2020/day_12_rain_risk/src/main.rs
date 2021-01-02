fn main() {
    let instructions: Vec<(char, i32)> = include_str!("../input.txt")
        .lines()
        .map(|string| {
            let chars: Vec<char> = string.chars().collect();
            let num: i32 = (1..chars.len())
                .map(|i| chars[i])
                .collect::<String>()
                .parse()
                .unwrap();
            (chars[0], num)
        })
        .collect();
    let ferry = instructions
        .iter()
        .fold(Ferry::new(), |mut ferry, &(ins, num)| {
            ferry.follow_instruction(ins, num);
            ferry
        });
    println!(
        "manhattan distance: {}",
        ferry.north.abs() + ferry.east.abs()
    );
    let waypoint = instructions
        .iter()
        .fold(Waypoint::new(), |mut waypoint, &(ins, num)| {
            waypoint.follow_instruction(ins, num);
            waypoint
        });
    println!(
        "manhattan distance: {}",
        waypoint.n.abs() + waypoint.e.abs()
    );
    println!("Hello, world!");
}

enum Dir {
    N,
    E,
    W,
    S,
}

struct Ferry {
    north: i32,
    east: i32,
    dir: Dir,
}

impl Ferry {
    pub fn new() -> Self {
        Ferry {
            north: 0,
            east: 0,
            dir: Dir::E,
        }
    }
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
        }
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Dir::N => Dir::E,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
            Dir::S => Dir::W,
        }
    }
    fn turn_180(&mut self) {
        self.dir = match self.dir {
            Dir::N => Dir::S,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
            Dir::S => Dir::N,
        }
    }
    fn turn(&mut self, instruction: char, num: i32) {
        match num {
            90 => match instruction {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => (),
            },
            180 => self.turn_180(),
            270 => match instruction {
                'L' => self.turn_right(),
                'R' => self.turn_left(),
                _ => (),
            },
            _ => (),
        }
    }
    pub fn follow_instruction(&mut self, instruction: char, num: i32) {
        match instruction {
            'N' => self.north += num,
            'E' => self.east += num,
            'W' => self.east -= num,
            'S' => self.north -= num,
            'L' | 'R' => self.turn(instruction, num),
            'F' => match self.dir {
                Dir::N => self.north += num,
                Dir::E => self.east += num,
                Dir::W => self.east -= num,
                Dir::S => self.north -= num,
            },
            _ => (),
        }
    }
}

struct Waypoint {
    rel_n: i32,
    rel_e: i32,
    n: i32,
    e: i32,
}

use std::mem;

impl Waypoint {
    fn new() -> Self {
        Waypoint {
            rel_n: 1,
            rel_e: 10,
            n: 0,
            e: 0,
        }
    }
    fn advance(&mut self, num: i32) {
        self.n += num * self.rel_n;
        self.e += num * self.rel_e;
    }
    fn turn_left(&mut self) {
        mem::swap(&mut self.rel_n, &mut self.rel_e);
        self.rel_e *= -1;
    }
    fn turn_right(&mut self) {
        mem::swap(&mut self.rel_n, &mut self.rel_e);
        self.rel_n *= -1;
    }
    fn turn_180(&mut self) {
        self.rel_n *= -1;
        self.rel_e *= -1;
    }
    fn move_waypoint(&mut self, ins: char, num: i32) {
        match ins {
            'N' => self.rel_n += num,
            'E' => self.rel_e += num,
            'W' => self.rel_e -= num,
            'S' => self.rel_n -= num,
            _ => (),
        }
    }
    fn turn(&mut self, ins: char, num: i32) {
        match num {
            90 => match ins {
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => (),
            },
            180 => self.turn_180(),
            270 => match ins {
                'L' => self.turn_right(),
                'R' => self.turn_left(),
                _ => (),
            },
            _ => (),
        }
    }
    pub fn follow_instruction(&mut self, ins: char, num: i32) {
        match ins {
            'N' | 'E' | 'W' | 'S' => self.move_waypoint(ins, num),
            'L' | 'R' => self.turn(ins, num),
            'F' => self.advance(num),
            _ => (),
        }
    }
}
