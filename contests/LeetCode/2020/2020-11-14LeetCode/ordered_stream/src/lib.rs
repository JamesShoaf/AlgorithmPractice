/*
There are n (id, value) pairs, where id is an integer between 1 and n and value is a string. No two pairs have the same id.

Design a stream that takes the n pairs in an arbitrary order, and returns the values over several calls in increasing order of their ids.

Implement the OrderedStream class:

    OrderedStream(int n) Constructs the stream to take n values and sets a current ptr to 1.
    String[] insert(int id, String value) Stores the new (id, value) pair in the stream. After storing the pair:
        If the stream has stored a pair with id = ptr, then find the longest contiguous incrementing sequence of ids starting with
        id = ptr and return a list of the values associated with those ids in order. Then, update ptr to the last id + 1.
        Otherwise, return an empty list.


*/

pub struct OrderedStream {
    vec: Vec<Option<String>>,
    index: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {
    pub fn new(n: i32) -> Self {
        OrderedStream {
            vec: vec![None; n as usize],
            index: 1,
        }
    }
    pub fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let mut output: Vec<String> = Vec::new();
        self.vec[id as usize - 1] = Some(value);
        while self.index <= self.vec.len() {
            if let Some(string) = &self.vec[self.index - 1] {
                output.push(string.clone());
            } else {
                break;
            }
            self.index += 1;
        }
        output
    }
}
