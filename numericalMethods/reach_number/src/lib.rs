/*
 You are standing at position 0 on an infinite number line. There is a goal at position target.

On each move, you can either go left or right. During the n-th move (starting from 1), you take n
steps.

Return the minimum number of steps required to reach the destination.
*/

pub fn reach_number(target: i32) -> i32 {
    let target = target.abs();
    // find the first n such that n*(n+1)/2 >= target
    let n = ((-1.0 + (1.0 + 8.0 * target as f64).sqrt()) / 2.0).ceil() as i32;
    // find the difference between n*(n+1)/2 and target
    let difference = n * (n + 1) / 2 - target;
    // if the difference is even, flipping the sign of 1 number in the range of 1..n would fix the difference
    if difference % 2 == 0 {
        return n;
    }
    // if the difference is odd, and n+1 is odd, adding n+1 makes the difference even again, so 1 flip would fix it
    if n % 2 == 0 {
        return n + 1;
    }
    // if the difference is odd and n+1 is even, adding n+1 and -(n+2) makes the difference even
    n + 2
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, 0),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 3),
            (5, 5),
            (6, 3),
            (7, 5),
            (8, 4),
            (9, 5),
            (10, 4),
            (11, 5),
            (12, 7),
            (100, 15),
            (1000, 47),
            (1000000000, 44723),
        ];
        for (target, expected) in test_tuples {
            assert_eq!(super::reach_number(target), expected);
        }
    }
}
