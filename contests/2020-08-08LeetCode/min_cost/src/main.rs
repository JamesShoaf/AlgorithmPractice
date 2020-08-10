/* 
Given a wooden stick of length n units. The stick is labelled from 0 to n. For example, a stick of length 6 is labelled as follows:

Given an integer array cuts where cuts[i] denotes a position you should perform a cut at.

You should perform the cuts in order, you can change the order of the cuts as you wish.

The cost of one cut is the length of the stick to be cut, the total cost is the sum of costs of all cuts. When you cut a stick, it will be split into two smaller sticks (i.e. the sum of their lengths is the length of the stick before the cut). Please refer to the first example for a better explanation.

Return the minimum total cost of the cuts.
*/

struct Solution {}

use std::collections::HashMap;
use std::cmp;

impl Solution {
    fn recursive(cuts: &Vec<i32>, memo: &mut HashMap<usize, HashMap<usize, i32>>, l: usize, r: usize) -> i32 {
        let mut output = i32::MAX;
        if let Some(prev_output) = memo.entry(l).or_insert(HashMap::new()).get(&r) { return *prev_output; }
        // the cost of making a cut is the length of the board
        let cost = cuts[r] - cuts[l];
        // try making each possible cut and store the lowest possible cost to make all cuts
        for i in l + 1..r {
            output = cmp::min(output,
                Solution::recursive(&cuts, memo, l, i) + cost + Solution::recursive(cuts, memo, i, r)
            );
        }
        // base case - there is no cost for no cuts
        if output == i32::MAX { output = 0; }
        memo.get_mut(&l).unwrap().insert(r, output);
        output
    }

    pub fn min_cost(n: i32, mut cuts: Vec<i32>) -> i32 {
        // add the start and end of the stick to the cuts array as starting values
        cuts.append(&mut vec![0, n]);
        // then sort the cuts, since only cuts between the start and end index are possible
        cuts.sort();
        // memoize solutions to subproblems
        let mut memo: HashMap<usize, HashMap<usize, i32>> = HashMap::new();
        Solution::recursive(&cuts, &mut memo, 0, cuts.len() - 1)
    }
}

fn main() {
    println!("{}", i32::MAX);
}
