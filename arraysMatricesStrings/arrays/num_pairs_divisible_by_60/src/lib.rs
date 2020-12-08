/*
You are given a list of songs where the ith song has a duration of time[i] seconds.

Return the number of pairs of songs for which their total duration in seconds is divisible by 60.
Formally, we want the number of indices i, j such that i < j with (time[i] + time[j]) % 60 == 0.
*/

pub fn num_pairs_divisible_by_60(time: Vec<i32>) -> i32 {
    let mut counts = [0; 60];
    let mut res = 0;
    for t in time {
        let t = t as usize % 60;
        res += counts[(60 - t) % 60];
        counts[t] += 1;
    }
    res
}

pub fn num_pairs_divisible_by_n(nums: Vec<i32>, n: usize) -> usize {
    assert!(n != 0, "Cannot divide by zero");
    if n == 1 && !nums.is_empty() {
        return nums.len() * 2 - 1;
    }
    let mut counts = vec![0; n];
    let mut res = 0;
    for num in nums {
        let num = num as usize % n;
        res += counts[(n - num) % n];
        counts[num] += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::num_pairs_divisible_by_60;
    #[test]
    fn it_works() {
        let test_tuples = vec![(vec![30, 20, 150, 100, 40], 3), (vec![60, 60, 60], 3)];
        for (time, expected) in test_tuples {
            assert_eq!(num_pairs_divisible_by_60(time), expected);
        }
    }
}
