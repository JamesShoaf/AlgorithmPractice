/*
Given an integer array nums sorted in non-decreasing order, return an array of the squares of each
number sorted in non-decreasing order.
*/

// square each element in order for trivial cases
fn square_vals(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        nums[i] *= nums[i];
    }
    nums
}

// binary search for index of first nonnegative value
fn get_first_non_negative_index(nums: &Vec<i32>) -> usize {
    let mut l = 0;
    let mut h = nums.len();
    while l < h {
        let m = (l + h) / 2;
        if nums[m] >= 0 {
            h = m;
        } else {
            l = m + 1;
        }
    }
    l
}

// The linear O(n) approach
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    // if there are no negative values, return the square of each element in order
    if nums.is_empty() || nums[0] >= 0 {
        return square_vals(nums);
    }
    // if there are no positive values, reverse the array first, then return the square
    if *nums.last().unwrap() <= 0 {
        nums.reverse();
        return square_vals(nums);
    }
    // binary search to find the index of the first nonnegative value
    let mut pos = get_first_non_negative_index(&nums);
    // and index of the last negative value
    let mut neg = Some(pos - 1);
    let mut res = Vec::new();
    // iterate forwards and backwards adding the square of the element with lower absolute value
    while neg.is_some() && pos < nums.len() {
        let neg_index = neg.unwrap();
        if nums[neg_index].abs() < nums[pos] {
            res.push(nums[neg_index].pow(2));
            neg = neg_index.checked_sub(1);
        } else {
            res.push(nums[pos].pow(2));
            pos += 1;
        }
    }
    // if positive numbers are exhausted first continue on the negative side
    while let Some(neg_index) = neg {
        res.push(nums[neg_index].pow(2));
        neg = neg_index.checked_sub(1);
    }
    // and vice versa
    while pos < nums.len() {
        res.push(nums[pos].pow(2));
        pos += 1;
    }
    res
}

// Simple idiomatic O(nln(n)) time sorting approach
// pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
//     let mut res: Vec<i32> = nums.into_iter().map(|val| val.pow(2)).collect();
//     res.sort();
//     res
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
