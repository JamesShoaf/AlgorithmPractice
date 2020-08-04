struct Solution {}

impl Solution {
    pub fn is_power_of_four(num: i32) -> bool {
        if num <= 0 { return false; }
        let mut four_flag = false;
        for i in 0..16 {
            if num & 1 << (2 * i) > 0 {
                if four_flag { return false; }
                four_flag = true;
            }
            if num & 1 << (2 * i + 1) > 0 { return false; }
        }
        true
    }
}

fn main() {
    let test_tuples = vec![
        (-1, false),
        (1, true),
        (2, false),
        (4, true),
        (5, false),
        (16, true),
        (20, false),
        (64, true),
        (256, true),
        (1024, true),
        (1073741824, true),
        (-2147483648, false),
    ];
    for (num, expected) in test_tuples {
        assert_eq!(Solution::is_power_of_four(num), expected);
    }
}
