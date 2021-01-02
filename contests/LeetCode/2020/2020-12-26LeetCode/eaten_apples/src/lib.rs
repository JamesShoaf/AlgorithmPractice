/*
There is a special kind of apple tree that grows apples every day for n days. On the ith day, the
tree grows apples[i] apples that will rot after days[i] days, that is on day i + days[i] the
apples will be rotten and cannot be eaten. On some days, the apple tree does not grow any apples,
which are denoted by apples[i] == 0 and days[i] == 0.

You decided to eat at most one apple a day (to keep the doctors away). Note that you can keep
eating after the first n days.

Given two integer arrays days and apples of length n, return the maximum number of apples you can eat.
*/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
    let mut current_day = apples.len();
    let mut apples_eaten = 0;
    let mut heap: BinaryHeap<(Reverse<usize>, i32)> = BinaryHeap::new();
    for (i, (apple_count, duration)) in apples.into_iter().zip(days.into_iter()).enumerate() {
        let rots = i + duration as usize;
        heap.push((Reverse(rots), apple_count));
        while let Some((Reverse(rots), apple_count)) = heap.pop() {
            if rots > i && apple_count > 0 {
                apples_eaten += 1;
                heap.push((Reverse(rots), apple_count - 1));
                break;
            }
        }
    }
    while let Some((Reverse(rots), apple_count)) = heap.pop() {
        if rots > current_day && apple_count > 0 {
            current_day += 1;
            apples_eaten += 1;
            heap.push((Reverse(rots), apple_count - 1));
        }
    }
    apples_eaten
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
