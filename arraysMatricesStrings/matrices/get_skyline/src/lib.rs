/*
A city's skyline is the outer contour of the silhouette formed by all the buildings in that city
when viewed from a distance. Now suppose you are given the locations and height of all the
buildings as shown on a cityscape photo (Figure A), write a program to output the skyline formed
by these buildings collectively (Figure B).

The geometric information of each building is represented by a triplet of integers [Li, Ri, Hi],
where Li and Ri are the x coordinates of the left and right edge of the ith building, respectively,
and Hi is its height. It is guaranteed that 0 ≤ Li, Ri ≤ INT_MAX, 0 < Hi ≤ INT_MAX,
and Ri - Li > 0. You may assume all buildings are perfect rectangles grounded on an absolutely flat
surface at height 0.

For instance, the dimensions of all buildings in Figure A are recorded as:
[ [2 9 10], [3 7 15], [5 12 12], [15 20 10], [19 24 8] ] .

The output is a list of "key points" (red dots in Figure B) in the format of
[ [x1,y1], [x2, y2], [x3, y3], ... ]
that uniquely defines a skyline. A key point is the left endpoint of a horizontal line segment.
Note that the last key point, where the rightmost building ends, is merely used to mark the
termination of the skyline, and always has zero height. Also, the ground in between any two
adjacent buildings should be considered part of the skyline contour.

For instance, the skyline in Figure B should be represented as:
[ [2 10], [3 15], [7 12], [12 0], [15 10], [20 8], [24, 0] ].
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn validated_push(point: Vec<i32>, res: &mut Vec<Vec<i32>>) {
    if let Some(last) = res.last_mut() {
        // avoid duplicate heights in adjacent skyline points
        if point[1] == last[1] {
            return;
        }
        // if point is a lower height for the same index, update the last index
        if point[0] == last[0] {
            last[1] = point[1];
            return;
        }
    }
    res.push(point);
}

fn dequeue_buildings(pq: &mut BinaryHeap<(i32, Reverse<i32>)>, res: &mut Vec<Vec<i32>>) {
    // remove the tallest building from the queue
    let (_, Reverse(right)) = pq.pop().unwrap();
    // pop off shorter buildings until the next one ends further to the right
    while let Some(&(_, Reverse(lower_right))) = pq.peek() {
        if lower_right < right {
            pq.pop();
        } else {
            break;
        }
    }
    // get the next building's height (or 0 if the queue is empty)
    let height = pq.peek().unwrap_or(&(0, Reverse(0))).0;
    // push the current building's end index and that height
    validated_push(vec![right, height], res);
}

pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut res: Vec<Vec<i32>> = Vec::new();
    let mut pq: BinaryHeap<(i32, Reverse<i32>)> = BinaryHeap::new();
    for building in buildings {
        let next_left = building[0];
        // while queued buildings have ending points before the next one starts,
        // dequeue them and push relevant points
        while let Some(&(_, Reverse(right))) = pq.peek() {
            if right < next_left {
                dequeue_buildings(&mut pq, &mut res);
            } else {
                break;
            }
        }
        // if the current building is taller than the tallest one in the stack
        // push its start index and height
        if building[2] > pq.peek().unwrap_or(&(0, Reverse(0))).0 {
            validated_push(vec![building[0], building[2]], &mut res);
        }
        // queue the current building's height and ending index
        pq.push((building[2], Reverse(building[1])));
    }
    // after all buildings are queued, dequeue the tallest buildings until the queue is empty
    while !pq.is_empty() {
        dequeue_buildings(&mut pq, &mut res);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![vec![1, 2, 3]], vec![vec![1, 3], vec![2, 0]]),
            (
                vec![vec![0, 3, 1], vec![1, 2, 2]],
                vec![vec![0, 1], vec![1, 2], vec![2, 1], vec![3, 0]],
            ),
            (
                vec![vec![0, 1, 2], vec![1, 2, 2]],
                vec![vec![0, 2], vec![2, 0]],
            ),
            (
                vec![
                    vec![2, 9, 10],
                    vec![3, 7, 15],
                    vec![5, 12, 12],
                    vec![15, 20, 10],
                    vec![19, 24, 8],
                ],
                vec![
                    vec![2, 10],
                    vec![3, 15],
                    vec![7, 12],
                    vec![12, 0],
                    vec![15, 10],
                    vec![20, 8],
                    vec![24, 0],
                ],
            ),
            (
                vec![vec![1, 2, 1], vec![1, 2, 2], vec![1, 2, 3]],
                vec![vec![1, 3], vec![2, 0]],
            ),
        ];
        for (buildings, expected) in test_tuples {
            assert_eq!(get_skyline(buildings), expected);
        }
    }
}
