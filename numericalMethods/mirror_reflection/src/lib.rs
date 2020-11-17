/*
There is a special square room with mirrors on each of the four walls.  Except for the southwest
corner, there are receptors on each of the remaining corners, numbered 0, 1, and 2.
(SE - 0, NE - 1, NW - 2)

The square room has walls of length p, and a laser ray from the southwest corner first meets the
east wall at a distance q from the 0th receptor.

Return the number of the receptor that the ray meets first.  (It is guaranteed that the ray will
    meet a receptor eventually.)
*/

pub fn mirror_reflection(p: i32, q: i32) -> i32 {
    assert!(p >= 1 && p <= 1000);
    assert!(q >= 0 && q <= p);
    if q == 0 {
        return 0;
    }
    let mut ascending = true;
    let mut travelling_right = true;
    let mut height_from_corner = 0;
    loop {
        height_from_corner += q;
        if height_from_corner == p {
            if !ascending {
                return 0;
            }
            if travelling_right {
                return 1;
            }
            return 2;
        }
        travelling_right = !travelling_right;
        if height_from_corner > p {
            height_from_corner -= p;
            ascending = !ascending;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (6, 0, 0),
            (6, 1, 2),
            (6, 2, 1),
            (6, 3, 2),
            (6, 4, 0),
            (6, 5, 2),
            (6, 6, 1),
        ];
        for (p, q, expected) in test_tuples {
            assert_eq!(mirror_reflection(p, q), expected);
        }
    }
}
