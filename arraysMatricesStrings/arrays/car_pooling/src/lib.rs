/* 
You are driving a vehicle that has capacity empty seats initially available for passengers.  The
vehicle only drives east (ie. it cannot turn around and drive west.)

Given a list of trips, trip[i] = [num_passengers, start_location, end_location] contains
information about the i-th trip: the number of passengers that must be picked up, and the locations
to pick them up and drop them off.  The locations are given as the number of kilometers due east
from your vehicle's initial location.

Return true if and only if it is possible to pick up and drop off all passengers for all the given
trips. 
*/

use std::collections::BinaryHeap;

pub fn car_pooling(trips: Vec<Vec<i32>>, capacity: i32) -> bool {
    let mut current_riders = 0;
    // (stop mile marker, embarking/disembarking count)
    let mut heap: BinaryHeap<(i32, i32)> = BinaryHeap::new();
    for trip in trips {
        // since heap produces a MaxHeap, swap signs to make it a MinHeap
        heap.push((-trip[1], -trip[0]));
        heap.push((-trip[2], trip[0]));
    }
    while let Some((_, passengers)) = heap.pop() {
        current_riders += passengers;
        if current_riders < -capacity {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![
                    vec![2, 1, 5],
                    vec![3, 3, 7],
                ],
                4,
                false,
            ),
            (
                vec![
                    vec![2, 1, 5],
                    vec![3, 3, 7],
                ],
                5,
                true,
            ),
            (
                vec![
                    vec![2, 1, 5],
                    vec![3, 5, 7],
                ],
                3,
                true,
            ),
            (
                vec![
                    vec![3, 2, 7],
                    vec![3, 7, 9],
                    vec![8, 3, 9],
                ],
                11,
                true,
            ),
        ];
        for (trips, capacity, expected) in test_tuples {
            assert_eq!(car_pooling(trips, capacity), expected);
        }
    }
}
