/* 
    You are given a sorted unique integer array nums.

    Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That
    is, each element of nums is covered by exactly one of the ranges, and there is no integer x
    such that x is in one of the ranges but not in nums.

    Each range [a,b] in the list should be output as:

        "a->b" if a != b
        "a" if a == b
*/

fn format_range(current_start: i32, last: i32) -> String {
    if last == current_start {
        format!("{}", last)
    } else {
        format!("{}->{}", current_start, last)
    }
}

pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
    if nums.is_empty() { return Vec::new(); }
    let mut output = Vec::new();
    let mut current_start = nums[0];
    let mut last = nums[0];
    for num in nums {
        if num > last + 1 {
            output.push(format_range(current_start, last));
            current_start = num;
        }
        last = num;
    }
    output.push(format_range(current_start, last));
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                vec![0, 1, 2, 4, 5, 7],
                vec![String::from("0->2"), String::from("4->5"), String::from("7")]
            ),
            (
                Vec::new(),
                Vec::new(),
            ),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(summary_ranges(nums), expected);
        }
    }
}
