/* 
In LOL world, there is a hero called Teemo and his attacking can make his enemy Ashe be in poisoned condition. Now, given the Teemo's attacking ascending time series towards Ashe and the poisoning time duration per Teemo's attacking, you need to output the total time that Ashe is in poisoned condition.

You may assume that Teemo attacks at the very beginning of a specific time point, and makes Ashe be in poisoned condition immediately.
*/

pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    if time_series.is_empty() { return 0; }
    time_series.iter().scan(time_series[0], |end_poison, &time| {
        let time_poisoned = std::cmp::min(time + duration - *end_poison, duration);
        *end_poison = time + duration;
        Some(time_poisoned)
    }).fold(0, |acc, val| acc + val)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 2, 0),
            (vec![1, 4], 2, 4),
            (vec![1, 2], 2, 3),
            (vec![1, 2, 6], 2, 5),
        ];
        for (time_series, duration, expected) in test_tuples {
            assert_eq!(find_poisoned_duration(time_series, duration), expected);
        }
    }
}
