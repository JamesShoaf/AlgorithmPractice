/*
Instead of zones or groups, this airline uses binary space partitioning to seat people. A seat
might be specified like FBFBBFFRLR, where F means "front", B means "back", L means "left", and R
means "right".

The first 7 characters will either be F or B; these specify exactly one of the 128 rows on the
plane (numbered 0 through 127). Each letter tells you which half of a region the given seat is in.
Start with the whole list of rows; the first letter indicates whether the seat is in the front
(0 through 63) or the back (64 through 127). The next letter indicates which half of that region
the seat is in, and so on until you're left with exactly one row.
*/

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let boarding_passes: Vec<String> = f
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap())
            .collect();
        let mut boarding_passes: Vec<i32> = boarding_passes
            .into_iter()
            .map(|pass| get_seat_number(&pass))
            .collect();
        boarding_passes.sort_unstable();
        let highest = *boarding_passes.last().unwrap_or(&-1);
        println!("Highest seat: {}", highest);
        if let Some(i_plus_one) =
            (1..boarding_passes.len()).find(|&i| boarding_passes[i] - 1 != boarding_passes[i - 1])
        {
            println!("Missing seat: {}", boarding_passes[i_plus_one] - 1)
        }
    }
}

fn get_seat_number(s: &String) -> i32 {
    let mut row_min = 0;
    let mut row_max = 128;
    let mut col_min = 0;
    let mut col_max = 8;
    for c in s.chars() {
        match c {
            'F' => row_max = (row_min + row_max) / 2,
            'B' => row_min = (row_min + row_max) / 2,
            'L' => col_max = (col_min + col_max) / 2,
            'R' => col_min = (col_min + col_max) / 2,
            _ => (),
        }
    }
    row_min * 8 + col_min
}

mod tests {
    #[test]
    fn test_get_seat_number() {
        let test_tuples = vec![
            ("BFFFBBFRRR".to_string(), 567),
            ("FFFBBBFRRR".to_string(), 119),
            ("BBFFBBFRLL".to_string(), 820),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(super::get_seat_number(&s), expected);
        }
    }
}
