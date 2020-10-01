/* 
You have a RecentCounter class which counts the number of recent requests within a certain time frame.

Implement the RecentCounter class:

    RecentCounter() Initializes the counter with zero recent requests.
    int ping(int t) Adds a new request at time t, where t represents some time in milliseconds, and
    returns the number of requests that has happened in the past 3000 milliseconds (including the
        new request). Specifically, return the number of requests that have happened in the
        inclusive range [t - 3000, t].

It is guaranteed that every call to ping uses a strictly larger value of t than the previous call.
*/

// initial BTreeSet implementation
// use std::collections::BTreeSet;

// pub struct RecentCounter {
//     pings: BTreeSet<i32>,
// }

// impl RecentCounter {
//     pub fn new() -> Self {
//         RecentCounter {
//             pings: BTreeSet::new(),
//         }
//     }

//     pub fn ping(&mut self, t: i32) -> i32 {
//         self.pings.insert(t);
//         self.pings.range(t - 3000..=t)
//             .fold(0, |acc, _| acc + 1)
//     }
// }

// optimized VecDeque solution

use std::collections::VecDeque;

pub struct RecentCounter {
    pings: VecDeque<i32>
}

impl RecentCounter {

    pub fn new() -> Self {
        RecentCounter {
            pings: VecDeque::new(),
        }
    }
    
    pub fn ping(&mut self, t: i32) -> i32 {
        self.pings.push_back(t);
        while self.pings[0] < t - 3000 {
            self.pings.pop_front();
        }
        self.pings.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![
                (1, 1),
                (100, 2),
                (3001, 3),
                (3002, 3),
            ]),
        ];
        for pings in test_tuples {
            let mut counter = RecentCounter::new();
            for (t, expected) in pings {
                assert_eq!(counter.ping(t), expected);
            }
        }
    }
}