/*
Given a singly linked list, return a random node's value from the linked list. Each node must have
the same probability of being chosen.
What if the linked list is extremely large and its length is unknown to you? Could you solve this
efficiently without using extra space?
*/

/*
Reservoir sampling is ideal for this purpose.
https://en.wikipedia.org/wiki/Reservoir_sampling
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type List = Option<Box<ListNode>>;

pub struct Solution {
    list: List,
}

impl Solution {
    pub fn new(list: List) -> Self {
        Solution { list }
    }

    pub fn get_random(&self) -> i32 {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let mut pointer: &List = &self.list;
        assert!(pointer.is_some(), "list contains no elements");
        const RESERVOIR_SIZE: usize = 1024;
        let mut reservoir: Vec<Option<i32>> = vec![None; RESERVOIR_SIZE];
        let mut i = 0;
        while i < RESERVOIR_SIZE && pointer.is_some() {
            if let Some(inner) = pointer {
                reservoir[i] = Some(inner.val);
                pointer = &inner.next;
            }
            i += 1;
        }
        while pointer.is_some() {
            i += 1;
            if let Some(inner) = pointer {
                let random_index = rng.gen_range(0, i);
                if random_index < RESERVOIR_SIZE {
                    reservoir[random_index] = Some(inner.val);
                }
                pointer = &inner.next;
            }
        }
        let reservoir: Vec<i32> = reservoir
            .into_iter()
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect();
        let random_index = rng.gen_range(0, reservoir.len());
        reservoir[random_index]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
