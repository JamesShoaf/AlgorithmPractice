/* 
You are given an array representing a row of seats where seats[i] = 1 represents a person sitting
in the ith seat, and seats[i] = 0 represents that the ith seat is empty (0-indexed).

There is at least one empty seat, and at least one person sitting.

Alex wants to sit in the seat such that the distance between him and the closest person to him is
maximized. 

Return that maximum distance to the closest person.
*/

use std::cmp;

pub fn max_dist_to_closest(seats:Vec<i32>) -> i32 {
    if seats.len() == 0 || !seats.contains(&1) {
        return seats.len() as i32;
    }
    let first_seated = seats.iter().position(|&x| x == 1).unwrap();
    let end = seats.len() - 1;
    // convert binary array to iterator of occupied seat indexes
    let (best_distance, last_seated) = seats.into_iter()
        .enumerate()
        .filter(|&(_, x)| x == 1)
        .map(|(i, _)| i)
        // the index of the first occupied seat is also the distance from the start
        .fold((first_seated, first_seated), |(best_distance, last_seat), i| {
            // best distance between seats
            (cmp::max(best_distance, (i - last_seat) / 2), i)
        });
    // distance from end to last occupied seat
    cmp::max(best_distance, end - last_seated) as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 0, 0, 0, 1, 0, 1], 2),
            (vec![1, 0, 0, 0, 0, 1, 0, 1], 2),
            (vec![1, 0, 0, 0], 3),
            (vec![1, 0, 0, 0, 0], 4),
            (vec![0, 1, 1, 0], 1),
            (vec![0, 1], 1),
            (vec![0, 0, 1], 2),
            (Vec::new(), 0),
            (vec![0, 0], 2),
        ];
        for (seats, expected) in test_tuples {
            assert_eq!(max_dist_to_closest(seats),  expected);
        }
    }
}
