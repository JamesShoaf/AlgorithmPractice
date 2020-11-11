/* 
Given the coordinates of four points in 2D space, return whether the four points could construct a square.

The coordinate (x,y) of a point is represented by an integer array with two integers.
*/

// get square distance between two points (to avoid floating point errors)
fn square_distance(p1: &Vec<i32>, p2: &Vec<i32>) -> i32 {
    (p1[0] - p2[0]).abs().pow(2) + (p1[1] - p2[1]).abs().pow(2)
}

// compare square distances between each of three points
fn is_square_corner(p1: &Vec<i32>, p2: &Vec<i32>, p3: &Vec<i32>) -> bool {
    let mut distances = vec![
        square_distance(&p1, p2),
        square_distance(&p2, p3),
        square_distance(&p1, p3),
    ];
    distances.sort();
    // distances must be nonzero
    distances[0] > 0
    // shortest two distances will be equal in a square
    && distances[0] == distances[1]
    // A^2 + B^2 == C^2 iff ABC is a right triangle
    && distances[0] + distances[1] == distances[2]
}

pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
    is_square_corner(&p1, &p2, &p3)
        && is_square_corner(&p1, &p2, &p4)
        && is_square_corner(&p1, &p3, &p4)
        && is_square_corner(&p2, &p3, &p4)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![0, 0],
                vec![0, 0],
                vec![0, 0],
                vec![0, 0],
                false
            ),
            (
                vec![0, 0],
                vec![0, 1],
                vec![1, 0],
                vec![1, 1],
                true
            ),
            (
                vec![0, 0],
                vec![0, 0],
                vec![1, 1],
                vec![1, 1],
                false
            ),
            (
                vec![1, 0],
                vec![1, 0],
                vec![0, 1],
                vec![0, 1],
                false
            ),
            (
                vec![0, 0],
                vec![4, 3],
                vec![7, -1],
                vec![3, -4],
                true
            ),
        ];
        for (p1, p2, p3, p4, expected) in test_tuples {
            assert_eq!(valid_square(p1, p2, p3, p4), expected);
        }
    }
}
