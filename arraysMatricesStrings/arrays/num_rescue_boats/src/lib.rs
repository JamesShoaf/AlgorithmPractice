/*
The i-th person has weight people[i], and each boat can carry a maximum weight of limit.

Each boat carries at most 2 people at the same time, provided the sum of the weight of those people is at most limit.

Return the minimum number of boats to carry every given person.  (It is guaranteed each person can be carried by a boat.)
*/

use std::cell::RefCell;
use std::rc::Rc;
type VEBTree = Option<Rc<RefCell<VEB>>>;
#[derive(Debug)]
struct VEB {
    val: i32,
    significant_bit: i32,
    count: i32,
    left: VEBTree,
    right: VEBTree,
}

// returns position of first set bit, 0-indexed, or 0 if x is 0
fn most_significant_bit_place(mut x: i32) -> u32 {
    if x < 0 {
        return 32;
    }
    let mut res = 0;
    while x > 1 {
        x >>= 1;
        res += 1;
    }
    res
}

impl VEB {
    // creates a new VEB tree rooted at the maximum value which uses the same number of bits
    pub fn new(max: i32) -> Self {
        // first number of binary form 11...11 >= max
        let significant_bit = 1 << most_significant_bit_place(max);
        VEB {
            val: significant_bit * 2 - 1,
            significant_bit,
            count: 0,
            left: None,
            right: None,
        }
    }
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }
    pub fn insert(&mut self, val: i32) {
        self.count += 1;
        if self.significant_bit > 0 {
            if val & self.significant_bit == 0 {
                if self.left.is_none() {
                    self.left.replace(Rc::new(RefCell::new(VEB {
                        val: self.val & !self.significant_bit,
                        significant_bit: self.significant_bit >> 1,
                        count: 0,
                        left: None,
                        right: None,
                    })));
                }
                self.left.as_mut().unwrap().borrow_mut().insert(val);
            } else {
                if self.right.is_none() {
                    self.right.replace(Rc::new(RefCell::new(VEB {
                        val: self.val,
                        significant_bit: self.significant_bit >> 1,
                        count: 0,
                        left: None,
                        right: None,
                    })));
                }
                self.right.as_mut().unwrap().borrow_mut().insert(val);
            }
        }
    }
    // returns the highest number in the tree num <= val and trims empty branches
    fn trimming_remove(&mut self, val: i32) -> (Option<i32>, bool) {
        if self.significant_bit == 0 {
            self.count -= 1;
            return (Some(self.val), self.count == 0);
        }
        let mut res: Option<i32> = None;
        if val > self.val - self.significant_bit {
            let mut empty_flag = false;
            if let Some(right) = &mut self.right {
                let mut right = right.borrow_mut();
                let (o, f) = right.trimming_remove(val);
                res = o;
                empty_flag = f;
            }
            if empty_flag {
                self.right.take();
            }
        }
        if res.is_none() {
            let mut empty_flag = false;
            if let Some(left) = &mut self.left {
                let mut left = left.borrow_mut();
                let (o, f) = left.trimming_remove(val);
                res = o;
                empty_flag = f;
            }
            if empty_flag {
                self.left.take();
            }
        }
        if res.is_some() {
            self.count -= 1;
        }
        (res, self.count == 0)
    }
    // returns an option containing the highest number in the tree such that num <= val
    pub fn remove(&mut self, val: i32) -> Option<i32> {
        self.trimming_remove(val).0
    }
}

pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let max = *people.iter().max().unwrap_or(&0);
    if max > limit {
        panic!("Invalid entry: 1 or more elements above limit")
    }
    let mut res = 0;
    let mut veb = VEB::new(max);
    for weight in people {
        veb.insert(weight);
    }
    // while there are people to rescue
    while !veb.is_empty() {
        // increase the boat count by 1
        res += 1;
        // rescue the heaviest person
        let boat_capacity = limit - veb.remove(max).unwrap();
        // rescue the heaviest person that will fit in the boat
        veb.remove(boat_capacity);
    }
    res
}

/*
#1 solution for comparison
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    //println!("rescue {:?} {}", people, limit);
    // greedy
    // pair heavy person first
    people.sort_by_key(|w| std::cmp::Reverse(*w));
    let mut cnt = 0;
    let mut boat = 0;

    for someone in &people {
        //println!("someone {} {:?}", someone, boats);
        if boat > 0 {
            let person = &people[boat-1];
            if person + someone <= limit {
                // take pair
                cnt += 1;
                boat -= 1;
                continue;
            }
        }

        // new boat
        boat += 1;
    }

    return cnt + boat as i32;
}
*/

// if boats can carry an unlimited number of passengers, we can simply pair the heavy passengers
// with the light passengers
use std::collections::VecDeque;

pub fn num_rescue_boats_unlimited(mut people: Vec<i32>, limit: i32) -> i32 {
    if *people.iter().max().unwrap_or(&0) > limit {
        panic!("Invalid entry: 1 or more elements above limit")
    }
    people.sort_unstable();
    let mut people: VecDeque<i32> = VecDeque::from(people);
    let mut res = 0;
    while !people.is_empty() {
        println!("{:?}", people);
        res += 1;
        let mut boat_capacity = limit;
        while let Some(&heavy) = people.back() {
            if heavy > boat_capacity {
                break;
            }
            boat_capacity -= heavy;
            people.pop_back();
        }
        while let Some(&light) = people.front() {
            if light > boat_capacity {
                break;
            }
            boat_capacity -= light;
            people.pop_front();
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_veb() {
        let mut veb = VEB::new(7);
        assert!(veb.is_empty());
        veb.insert(1);
        assert!(!veb.is_empty());
        assert_eq!(veb.remove(0), None);
        veb.insert(0);
        assert_eq!(veb.remove(1), Some(1));
        assert_eq!(veb.remove(1), Some(0));

        let mut veb = VEB::new(5);
        for num in vec![3, 5, 3, 4].into_iter() {
            veb.insert(num);
        }
        assert_eq!(veb.count, 4);
        assert_eq!(veb.remove(5), Some(5));
        assert_eq!(veb.remove(0), None);
        assert_eq!(veb.count, 3);
        assert_eq!(veb.remove(5), Some(4));
        assert_eq!(veb.remove(1), None);
        assert_eq!(veb.count, 2);
        assert_eq!(veb.remove(5), Some(3));
        assert_eq!(veb.remove(2), None);
        assert_eq!(veb.count, 1);
        assert_eq!(veb.remove(5), Some(3));
        assert_eq!(veb.remove(2), None);
        assert_eq!(veb.count, 0);

        let mut veb = VEB::new(2);
        for num in vec![2, 1].into_iter() {
            veb.insert(num);
        }
        assert_eq!(veb.count, 2);
        assert_eq!(veb.remove(2), Some(2));
        assert_eq!(veb.remove(1), Some(1));
        assert_eq!(veb.count, 0);
    }

    #[test]
    fn test_num_rescue_boats() {
        let test_tuples = vec![
            (vec![1, 2], 3, 1),
            (vec![3, 2, 2, 1], 3, 3),
            (vec![3, 5, 3, 4], 5, 4),
            (vec![1, 1, 2, 2, 2, 3, 4, 5], 5, 5),
            (vec![3, 2, 3, 2, 2], 6, 3),
        ];
        for (people, limit, expected) in test_tuples {
            assert_eq!(num_rescue_boats(people, limit), expected);
        }
    }

    #[test]
    fn test_num_rescue_boats_unlimited() {
        let test_tuples = vec![
            (vec![1, 2], 3, 1),
            (vec![3, 2, 2, 1], 3, 3),
            (vec![3, 5, 3, 4], 5, 4),
            (vec![1, 1, 2, 2, 2, 3, 4, 5], 5, 4),
            (vec![3, 2, 3, 2, 2], 6, 2),
        ];
        for (people, limit, expected) in test_tuples {
            assert_eq!(num_rescue_boats_unlimited(people, limit), expected);
        }
    }
}
