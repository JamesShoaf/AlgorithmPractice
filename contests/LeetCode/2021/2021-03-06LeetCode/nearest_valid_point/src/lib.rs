/*
You are given two integers, x and y, which represent your current location on a Cartesian grid: (x, y).
You are also given an array points where each points[i] = [ai, bi] represents that a point exists at
(ai, bi). A point is valid if it shares the same x-coordinate or the same y-coordinate as your location.

Return the index (0-indexed) of the valid point with the smallest Manhattan distance from your current
location. If there are multiple, return the valid point with the smallest index. If there are no valid
points, return -1.

The Manhattan distance between two points (x1, y1) and (x2, y2) is abs(x1 - x2) + abs(y1 - y2).
*/

pub fn nearest_valid_point(x: i32, y: i32, points: Vec<Vec<i32>>) -> i32 {
    let mut distances: Vec<(usize, i32)> = points
        .into_iter()
        .enumerate()
        .filter(|(_, vec)| vec[0] == x || vec[1] == y)
        .map(|(i, vec)| (i, (vec[0] - x).abs() + (vec[1] - y).abs()))
        .collect();
    distances.sort_unstable_by_key(|&(i, d)| (d, i));
    if let Some(&(i, _)) = distances.first() {
        i as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
