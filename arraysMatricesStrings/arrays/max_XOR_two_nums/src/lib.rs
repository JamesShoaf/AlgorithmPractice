use std::cmp;
use std::rc::Rc;
use std::cell::RefCell;

pub fn find_maximum_xor_n_squared(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            max = cmp::max(max, nums[i] ^ nums[j]);
        }
    }
    max
}

fn most_significant_bit_place(num: i32) -> i32 {
    if num == 0 { return 0; }
    let mut i = 30;
    while num & 1 << i == 0 {
        i -= 1;
    }
    i
}

// a Trie containing only 0, or 0 and 1 has a height of 1
fn find_trie_height(nums: &Vec<i32>) -> i32 {
    nums.iter().fold(1, |acc, &num| cmp::max(acc, most_significant_bit_place(num) + 1))
}

#[derive(Debug)]
pub struct BinaryTrie {
    height: i32,
    zero: Option<Rc<RefCell<BinaryTrie>>>,
    one: Option<Rc<RefCell<BinaryTrie>>>
}

impl BinaryTrie {
    pub fn new(height: i32) -> Self {
        BinaryTrie {
            height,
            zero: None,
            one: None
        }
    }

    pub fn insert(&mut self, num: i32) {
        let insert_helper = |choice: &mut Option<Rc<RefCell<BinaryTrie>>>, new_height| {
            if choice.is_none() {
                choice.replace(Rc::new(RefCell::new(BinaryTrie::new(new_height))));
            }
            if let Some(inner) = choice.as_ref() {
                inner.borrow_mut().insert(num);
            }
        };
        if self.height <= 0 { return; }
        match num & 1 << (self.height - 1) {
            0 => insert_helper(&mut self.zero, self.height - 1),
            _ => insert_helper(&mut self.one, self.height - 1)
        }
    }

    pub fn best_xor(&self, num: i32, prev: i32) -> i32 {
        if self.height == 0 { return prev; }
        let next = prev << 1;
        return match num & 1 << (self.height - 1) {
            0 => {
                if let Some(one) = &self.one {
                    one.borrow().best_xor(num, next + 1)
                } else if let Some(zero) = &self.zero {
                    zero.borrow().best_xor(num, next)
                } else {
                    0
                }
            }
            _ => {
                if let Some(zero) = &self.zero {
                    zero.borrow().best_xor(num, next + 1)
                } else if let Some(one) = &self.one {
                    one.borrow().best_xor(num, next)
                } else {
                    0
                }
            }
        }
    }
}

pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut trie = BinaryTrie::new(find_trie_height(&nums));
    for &num in nums.iter() {
        trie.insert(num);
    }
    for num in nums {
        max = cmp::max(max, trie.best_xor(num, 0));
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_maximum_xor_n_squared() {
        let test_tuples = vec![
            (vec![3, 10, 5, 25, 2, 8], 28),
            (vec![10, 5, 25], 28),
            (vec![0, 0], 0),
            (vec![0, 1], 1),
            (vec![0], 0),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(find_maximum_xor_n_squared(nums), expected);
        }
    }

    fn test_find_trie_height() {
        let test_tuples = vec![
            (vec![3, 10, 5, 25, 2, 8], 5),
            (vec![10, 5, 25], 5),
            (vec![0, 0], 1),
            (vec![0, 1], 1),
            (vec![0], 0),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(find_trie_height(&nums), expected);
        }
    }

    fn test_maximum_xor() {
        let test_tuples = vec![
            (vec![3, 10, 5, 25, 2, 8], 28),
            (vec![10, 5, 25], 28),
            (vec![0, 0], 0),
            (vec![0, 1], 1),
            (vec![0], 0),
        ];
        for (nums, expected) in test_tuples {
            assert_eq!(find_maximum_xor(nums), expected);
        }
    }
}

// #1 solution for comparison

// use std::collections::HashSet;
// use std::iter::FromIterator;

// impl Solution {
//     pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
//         (0..=31)
//             .rev()
//             .scan(0, |mask, i| {
//                 *mask = *mask | 1 << i;
//                 Some((*mask, 1 << i))
//             })
//             .fold(0, |result, (mask, bit)| {
//                 let greedy = result | bit;
//                 let s: HashSet<i32> = HashSet::from_iter(nums.iter().map(|n| n & mask));
//                 s.iter()
//                     .find(|&left_part| s.contains(&(left_part ^ greedy)))
//                     .map(|_| greedy)
//                     .unwrap_or(result)
//             })
//     }
// }