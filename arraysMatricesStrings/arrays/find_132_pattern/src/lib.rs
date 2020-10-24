use std::cmp;

pub fn find_132_pattern(nums: Vec<i32>) -> bool {
    if nums.len() < 3 { return false; }
    // array of min values such that min_vals[i] is the minimum of nums[0]..nums[i] 
    let mut min_vals = vec![nums[0]; nums.len()];
    for i in 1..nums.len() {
        min_vals[i] = cmp::min(min_vals[i - 1], nums[i]);
    }
    // stack of visited indexes from the right
    let mut stack = Vec::new();
    for i in (1..nums.len()).rev() {
        // if the value at the oldest index is less than the value at the current index
        // (_, i: 3, top: 2)
        while !stack.is_empty() && nums[stack[stack.len() - 1]] < nums[i] {
            // check if a minimum with a lower index exists
            if nums[stack.pop().unwrap()] > min_vals[i] { return true; }
        }
        stack.push(i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 3, 4], false),
            (vec![3, 1, 4, 2], true),
            (vec![-1, 3, 2, 0], true),
            (vec![10, 50, 1, 3, 2], true),
            (vec![1, 3, 2, 7, 8, 9], true),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(find_132_pattern(nums), expected);
        }
    }
}

/* 
/*  */
#1 solution for comparison

impl Solution {
    /// True if there is a 132 pattern in nums.
    ///
    /// A 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k]
    /// such that i < j < k and nums[i] < nums[k] < nums[j].
    ///
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<(i32, i32)> = vec![];

        // Loop invariant: The values in `stack` are pairs `(nums[i], nums[j])`
        // for `i <= j < number of loop iterations completed`, sorted in
        // descending order. These ranges are non-overlapping. `stack` may
        // contain some empty ranges. If a later value `nums[k]` falls between
        // `nums[i]` and `nums[j]`, then `i < j < k` and we have found a 132
        // pattern.
        for v in nums {
            let i = bisect(|&(lo, _hi)| lo < v, &stack);
            let lo;
            if i == stack.len() {
                // v is <= everything on the stack.
                lo = v;
            } else if v < stack[i].1 {
                return true;
            } else {
                lo = stack[stack.len() - 1].0;
                stack.truncate(i);
            }
            stack.push((lo, v));
        }
        false
    }
}

// Given a predicate f such that i <= j implies f(&a[i]) <= f(&a[j]),
// find the unique x in `0..=a.len()` such that `f(&a[i])` is true iff `i >= x`.
fn bisect<T, F>(f: F, a: &[T]) -> usize
where
    F: Fn(&T) -> bool
{
    let mut lo = 0;  // invariant: for all i in 0..lo, f(a[i]) == false 
    let mut hi = a.len();  // invariant: for all i in hi..a.len(), f(a[i]) == true 
    while lo < hi {
        let mid = (lo + hi) / 2;
        if f(&a[mid]) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
*/