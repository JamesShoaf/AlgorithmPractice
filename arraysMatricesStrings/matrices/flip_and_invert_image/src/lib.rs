/* 
Given a binary matrix A, we want to flip the image horizontally, then invert it, and return the
resulting image.

To flip an image horizontally means that each row of the image is reversed.  For example, flipping
[1, 1, 0] horizontally results in [0, 1, 1].

To invert an image means that each 0 is replaced by 1, and each 1 is replaced by 0. For example,
inverting [0, 1, 1] results in [1, 0, 0].
*/

pub fn flip_and_invert_image(mut a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    for i in 0..a.len() {
        a[i].reverse();
        for j in 0..a[i].len() {
            a[i][j] ^= 1;
        }
    }
    a
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
