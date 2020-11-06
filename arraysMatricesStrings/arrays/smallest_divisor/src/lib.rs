/* 
Given an array of integers nums and an integer threshold, we will choose a positive integer divisor
and divide all the array by it and sum the result of the division. Find the smallest divisor such
that the result mentioned above is less than or equal to threshold.

Each result of division is rounded to the nearest integer greater than or equal to that element.
(For example: 7/3 = 3 and 10/2 = 5).

It is guaranteed that there will be an answer.
*/

pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
    assert!(!nums.is_empty(), "nums must contain at least one value");
    assert!(!nums.iter().any(|&num| num < 1), "all numbers must be positive");
    let mut low = 1;
    let mut high = *nums.iter().max().unwrap();
    while low < high {
        let mid = (low + high) / 2;
        if threshold < nums.iter().map(|&num| num / mid + (num % mid != 0) as i32).sum::<i32>() {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 5, 9], 6, 5),
            (vec![2, 3, 5, 7, 11], 11, 3),
            (vec![19], 5, 4),
            (vec![81488,78770,69737,14847,36203,85812,62771,25584,40434,55397,95643,48474,63802,
                63203,69258,16397,62134,68311,48931,4317,488,76613,70301,24052,67069,8018,61725,
                98992,86206,54407,33175,65597,5215,11866,9420,93851,66858,78097,70255,92021,50353,
                91451,52417,411,81332,27928,51842,83525,74458,10995,18792,14219,31190,86907,412,
                45731,9730,1258,94589,84673,20469,479,71501,68751,18230,77410,41662,9005,72532,
                36514,5203,528,59746,45580,72611,30386,40609,30783,70776,78765,97187,19997,16257,
                86367,80891,43219,55020,4753,2785,70141,86103,25092,50595,73151,69139,92893,18011,
                94848,65111,5657], 988, 5317),
        ];
        for (nums, threshold, expected) in test_tuples {
            assert_eq!(smallest_divisor(nums, threshold), expected);
        }
    }

    #[test]
    #[should_panic]
    fn negative_numbers() {
        smallest_divisor(vec![-1], 1);
    }

    #[test]
    #[should_panic]
    fn empty_input() {
        smallest_divisor(Vec::new(), 1);
    }
}
