/*
Given an integer n, return true if it is possible to represent n as the sum of distinct powers of three.
Otherwise, return false.

An integer y is a power of three if there exists an integer x such that y == 3x.
*/

fn recurse(i: usize, exp: u32, dp: &mut Vec<bool>) {
    if i >= dp.len() {
        return;
    }
    dp[i] = true;
    let pow = 3_usize.pow(exp);
    if pow >= dp.len() {
        return;
    }
    recurse(i, exp + 1, dp);
    recurse(i + pow, exp + 1, dp);
}

pub fn check_powers_of_three(n: i32) -> bool {
    let mut dp = vec![false; n as usize + 1];
    recurse(0, 0, &mut dp);
    dp[n as usize]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
