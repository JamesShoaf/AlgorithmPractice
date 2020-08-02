// Given an array of integers arr, and three integers a, b and c. You need to find the number of good triplets.

// A triplet (arr[i], arr[j], arr[k]) is good if the following conditions are true:

//     0 <= i < j < k < arr.length
//     |arr[i] - arr[j]| <= a
//     |arr[j] - arr[k]| <= b
//     |arr[i] - arr[k]| <= c

// Where |x| denotes the absolute value of x.

// Return the number of good triplets.

struct Solution {}

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let mut count: i32 = 0;
        for i in 0..arr.len() - 2 {
            for j in i + 1..arr.len() - 1 {
                if (arr[i] - arr[j]).abs() <= a {
                    for k in j + 1..arr.len() {
                        if (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }
}

fn main() {
    let test_tuples = vec![
        (vec![3, 0, 1, 1, 9, 7], 7, 2, 3, 4),
        (vec![1, 1, 2, 2, 3], 0, 0, 1, 0),
    ];
    for (arr, a, b, c, expected) in test_tuples {
        assert_eq!(Solution::count_good_triplets(arr, a, b, c), expected);
    }
}
