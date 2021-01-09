pub fn total_money(n: i32) -> i32 {
    let mut res = 0;
    for i in 0..n {
        res += (i / 7) + (i % 7) + 1;
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![(4, 10), (10, 37), (20, 96)];
        for (n, expected) in test_tuples {
            assert_eq!(super::total_money(n), expected);
        }
    }
}
