/*
There is a biker going on a road trip. The road trip consists of n + 1 points at different
altitudes. The biker starts his trip on point 0 with altitude equal 0.

You are given an integer array gain of length n where gain[i] is the net gain in altitude between
points i​​​​​​ and i + 1 for all (0 <= i < n). Return the highest altitude of a point.
*/

use std::cmp;

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut alt = 0;
    let mut res = 0;
    for num in gain {
        alt += num;
        res = cmp::max(res, alt);
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
