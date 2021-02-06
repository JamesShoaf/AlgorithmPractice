use std::collections::HashMap;

pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
    let mut counts: HashMap<i32, u32> = HashMap::new();
    for num in nums {
        *counts.entry(num).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter(|&(_, v)| v == 1)
        .map(|(k, _)| k)
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
