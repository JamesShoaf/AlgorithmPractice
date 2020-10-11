/* 
There are some spherical balloons spread in two-dimensional space. For each balloon, provided input
is the start and end coordinates of the horizontal diameter. Since it's horizontal, y-coordinates
don't matter, and hence the x-coordinates of start and end of the diameter suffice. The start is
always smaller than the end.

An arrow can be shot up exactly vertically from different points along the x-axis. A balloon with
xstart and xend bursts by an arrow shot at x if xstart ≤ x ≤ xend. There is no limit to the number
of arrows that can be shot. An arrow once shot keeps traveling up infinitely.

Given an array points where points[i] = [xstart, xend], return the minimum number of arrows that
must be shot to burst all balloons.


    0 <= points.length <= 104
    points.length == 2
    -231 <= xstart < xend <= 231 - 1

*/

use std::collections::HashMap;
use std::cmp;

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() { return 0; }
    let mut closest_ends: HashMap<i32, i32> = HashMap::new();
    points.sort();
    for point in points.iter() {
        if !closest_ends.contains_key(&point[0]) {
            closest_ends.insert(point[0], point[1]);
        }
    }
    points.into_iter().scan(i32::MAX, |end, x| {
        if x[0] > *end {
            *end = *closest_ends.get(&x[0]).unwrap();
            Some(1)
        } else {
            *end = cmp::min(*end, *closest_ends.get(&x[0]).unwrap());
            Some(0)
        }
    }).fold(1, |acc, val| acc + val)
}

/* 
#1 solution for comparison

pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    if points.is_empty() {
        return 0;
    }
    points.sort_by_key(|v| v[1]);
    let mut arrow_pos = points[0][1];
    let mut ans = 1;
    for v in points.iter().skip(1) {
        if arrow_pos >= v[0] {
            continue;
        }
        ans += 1;
        arrow_pos = v[1];
    }
    ans
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![
                vec![3, 5],
                vec![3, 6],
                vec![1, 3],
                vec![1, 2],
                vec![3, 4],
                vec![1, 4],
            ],
            2,),
            (vec![
                vec![10, 16],
                vec![2, 8],
                vec![1, 6],
                vec![7, 12],
            ],
            2,),
            (vec![
                vec![1, 2],
                vec![3, 4],
                vec![5, 6],
                vec![7, 8],
            ],
            4,),
            (vec![
                vec![1, 2],
                vec![2, 3],
                vec![3, 4],
                vec![4, 5],
            ],
            2,),
            (vec![
                vec![1, 2],
            ],
            1,),
            (vec![
                vec![2, 3],
                vec![3, 4],
            ],
            1,),
            (vec![
                vec![9, 12],
                vec![1, 10],
                vec![4, 11],
                vec![8, 12],
                vec![3, 9],
                vec![6, 9],
                vec![6, 7],
            ],
            2,),
            (vec![
                vec![i32::MIN, i32::MAX],
            ],
            1,),
            (Vec::new(), 0),
        ];
        for (points, expected) in test_tuples {
            assert_eq!(find_min_arrow_shots(points.clone()), expected, "{:?}", points);
        }
    }
}
