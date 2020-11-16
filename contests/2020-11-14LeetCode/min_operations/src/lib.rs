/*
You are given an integer array nums and an integer x. In one operation, you can either remove the
leftmost or the rightmost element from the array nums and subtract its value from x. Note that this
modifies the array for future operations.

Return the minimum number of operations to reduce x to exactly 0 if it's possible, otherwise,
return -1.
*/

pub fn min_operations(nums: Vec<i32>, mut x: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    let mut res = None;

    while l <= r && x >= nums[l] {
        x -= nums[l];
        l += 1;
    }

    if x == 0 {
        res = Some(l);
    }

    while l < r {
        while x >= nums[r] {
            x -= nums[r];
            r -= 1;
        }

        if x == 0 {
            let ops = l + nums.len() - 1 - r;
            if let Some(min) = res {
                if ops < min {
                    res = Some(ops);
                }
            } else {
                res = Some(ops);
            }
        }
        if l == 0 {
            break;
        }
        l -= 1;
        x += nums[l];
    }

    if let Some(min) = res {
        min as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
