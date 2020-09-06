/* 
Two images A and B are given, represented as binary, square matrices of the same size.

We translate one image however we choose (sliding it left, right, up, or down any number of units),
and place it on top of the other image.  After, the overlap of this translation is the number of
positions that have a 1 in both images.

(Note also that a translation does not include any kind of rotation.)

What is the largest possible overlap?
*/

// (O(h^2*w^2)) time complexity, O(1) space complexity

pub mod largest_overlap {
    use std::cmp;

    fn helper(a: &Vec<Vec<i32>>, b: &Vec<Vec<i32>>) -> i32 {
        let mut best_overlap = 0;
        for x_offset in 0..b.len() {
            for y_offset in 0..b[x_offset].len() {
                let mut overlap = 0;
                for x in x_offset..cmp::min(a.len(), b.len()) {
                    for y in y_offset..cmp::min(a[x].len(), b[x_offset].len()) {
                        if a[x][y] == 1 && b[x - x_offset][y - y_offset] == 1 {
                            overlap += 1
                        }
                    }
                }
                best_overlap = cmp::max(best_overlap, overlap);
            }
        }
        best_overlap
    }

    pub fn largest_overlap(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> i32 {
        cmp::max(helper(&a, &b), helper(&b, &a))
    }
}

#[cfg(test)]
mod tests {
    use crate::largest_overlap;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                Vec::new(),
                Vec::new(),
                0,
            ),
            (
                Vec::new(),
                vec![
                    vec![1],
                ],
                0,
            ),
            (
                vec![
                    vec![1, 1],
                    vec![0, 1],
                    vec![0, 1],
                ],
                vec![
                    vec![1, 1, 0],
                    vec![1, 1, 0],
                ],
                3,
            ),
            (
                vec![
                    vec![1, 1, 1],
                    vec![0, 1, 1],
                    vec![0, 1, 1],
                ],
                vec![
                    vec![1, 0, 0],
                    vec![1, 1, 0],
                    vec![1, 1, 0],
                ],
                5,
            ),
            (
                vec![
                    vec![1, 1, 0],
                    vec![0, 1, 0],
                    vec![0, 1, 0],
                ],
                vec![
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 1],
                ],
                3,
            ),
            (
                vec![
                    vec![0, 0, 0],
                    vec![0, 1, 1],
                    vec![0, 0, 1],
                ],
                vec![
                    vec![1, 1, 0],
                    vec![0, 1, 0],
                    vec![0, 1, 0],
                ],
                3,
            ),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(largest_overlap::largest_overlap(a, b), expected);
        }
    }
}
