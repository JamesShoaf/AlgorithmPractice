/*
Given n cuboids where the dimensions of the ith cuboid is cuboids[i] = [widthi, lengthi, heighti]
(0-indexed). Choose a subset of cuboids and place them on each other.

You can place cuboid i on cuboid j if widthi <= widthj and lengthi <= lengthj and
heighti <= heightj. You can rearrange any cuboid's dimensions by rotating it to put it on another
cuboid.

Return the maximum height of the stacked cuboids.
*/

pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
