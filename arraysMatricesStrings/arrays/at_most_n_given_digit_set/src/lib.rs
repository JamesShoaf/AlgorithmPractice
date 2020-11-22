/*
Given an array of digits, you can write numbers using each digits[i] as many times as we want.
For example, if digits = ['1','3','5'], we may write numbers such as '13', '551', and '1351315'.

Return the number of positive integers that can be generated that are less than or equal to a
given integer n.

Constraints:

    1 <= digits.length <= 9
    digits[i].length == 1
    digits[i] is a digit from '1' to '9'.
    All the values in digits are unique.
    1 <= n <= 109

*/

// for a number n digits long with k unique digits you can always make k + k^2 + ... + k^n-1
// permutations
fn get_trivial_permutation_count(num_digits: usize, length: u32) -> i32 {
    let num_digits = num_digits as i32;
    let mut count = 0;
    let mut permutations = 1;
    for _ in 0..length - 1 {
        permutations *= num_digits;
        count += permutations;
    }
    count
}

// for a number n digits long, with k digits, you can also make at least k^n-1 * x permutations
// where x is the number of digits less than the most significant digit in n
fn get_nontrivial_permutation_count(digits: &Vec<i32>, n: i32, length: u32) -> i32 {
    // base case - the last digit entered matched the final digit of n
    if length == 0 {
        return 1;
    }
    let tens = 10_i32.pow(length - 1);
    let significant_digit = n / tens;
    let mut count = 0;
    let other_digit_permutations = (digits.len() as i32).pow(length - 1);
    for &digit in digits {
        if digit < significant_digit {
            count += other_digit_permutations;
        }
        if digit == significant_digit {
            count +=
                get_nontrivial_permutation_count(digits, n - significant_digit * tens, length - 1);
            break;
        }
    }
    count
}

pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
    let mut digits: Vec<i32> = digits.into_iter().map(|s| s.parse().unwrap()).collect();
    digits.sort();
    let max_length = ((n as f64).log(10.0) + 1.0) as u32;
    get_trivial_permutation_count(digits.len(), max_length)
        + get_nontrivial_permutation_count(&digits, n, max_length)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![String::from("3"), String::from("4"), String::from("8")],
                4,
                2,
            ),
            (
                vec![
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                ],
                64,
                18,
            ),
            (
                vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                ],
                4321,
                313,
            ),
            (
                vec![
                    String::from("1"),
                    String::from("3"),
                    String::from("5"),
                    String::from("7"),
                ],
                100,
                20,
            ),
            (
                vec![String::from("1"), String::from("4"), String::from("9")],
                1000000000,
                29523,
            ),
            (vec![String::from("7")], 8, 1),
            (vec![String::from("7")], 6, 0),
            (
                vec![
                    String::from("1"),
                    String::from("2"),
                    String::from("3"),
                    String::from("4"),
                    String::from("5"),
                    String::from("6"),
                    String::from("7"),
                    String::from("8"),
                ],
                940860624,
                153391688,
            ),
        ];
        for (digits, n, expected) in test_tuples {
            assert_eq!(at_most_n_given_digit_set(digits, n), expected);
        }
    }
}
