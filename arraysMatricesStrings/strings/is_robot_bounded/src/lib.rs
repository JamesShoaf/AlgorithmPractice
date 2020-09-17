/* 
On an infinite plane, a robot initially stands at (0, 0) and faces north.  The robot can receive one of three instructions:

    "G": go straight 1 unit;
    "L": turn 90 degrees to the left;
    "R": turn 90 degress to the right.

The robot performs the instructions given in order, and repeats them forever.

Return true if and only if there exists a circle in the plane such that the robot never leaves the circle.
*/

#[derive(PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Move {
    direction: Direction,
    distance_north: i32,
    distance_east: i32,
}

impl Move {
    pub fn new() -> Self {
        Move {
            direction: Direction::North,
            distance_north: 0,
            distance_east: 0,
        }
    }

    pub fn go(&mut self) {
        match &self.direction {
            Direction::North => self.distance_north += 1,
            Direction::South => self.distance_north -= 1,
            Direction::East => self.distance_east += 1,
            Direction::West => self.distance_east -= 1,
        }
    }
    
    pub fn left(&mut self) {
        match &self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
        }
    }
    
    pub fn right(&mut self) {
        match &self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        }
    }
}

pub fn is_robot_bounded(instructions: String) -> bool {
    let mut total_move = Move::new();
    for c in instructions.chars() {
        match c {
            'G' => total_move.go(),
            'L' => total_move.left(),
            'R' => total_move.right(),
            _ => ()
        }
    }
    total_move.direction != Direction::North
        || (total_move.distance_north.abs() + total_move.distance_east.abs()) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), true),
            (String::from("GGLLGG"), true),
            (String::from("GG"), false),
            (String::from("LL"), true),
            (String::from("GL"), true),
        ];
        for (instructions, expected) in test_tuples {
            assert_eq!(is_robot_bounded(instructions), expected);
        }
    }
}
