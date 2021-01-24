/*
You are given a 2D integer array, queries. For each queries[i], where queries[i] = [ni, ki],
find the number of different ways you can place positive integers into an array of size ni
such that the product of the integers is ki. As the number of ways may be too large, the
answer to the ith query is the number of ways modulo 109 + 7.

Return an integer array answer where answer.length == queries.length, and answer[i] is the
answer to the ith query.
*/

/*
    1 <= queries.length <= 104
    1 <= ni, ki <= 104
*/

pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
    Vec::new()
    // memoized recursive approach
    // for each factor of n, f, add together the possibilities [n/f, k - 1]
    // if k == 1, return 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
