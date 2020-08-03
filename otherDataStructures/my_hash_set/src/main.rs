struct MyHashSet {
    size: usize,
    capacity: usize,
    store: Vec<Vec<i32>>,
}

impl MyHashSet {

    fn new() -> Self {
        MyHashSet {
            size: 0,
            capacity: 8,
            store: vec![Vec::new(); 8],
        }
    }

    fn hash(key: i32, len: usize) -> usize {
        let prime = 1001593; // large prime close to max expected value
        let mut hash: i32 = prime;
        for i in 0..32 {
            let bit = 1 << i;
            hash ^= (hash << 5) % prime + (key & bit) + (hash >> 2);
        }
        hash as usize % len
    }

    fn resize(&mut self, new_len: usize) {
        let mut new_store: Vec<Vec<i32>> = vec![Vec::new(); new_len];
        for index in &self.store {
            for val in index {
                let new_hash = MyHashSet::hash(*val, new_len);
                new_store[new_hash].push(*val);
            }
        }
        self.store = new_store;
    }
    
    fn add(&mut self, key: i32) {
        let index = MyHashSet::hash(key, self.store.len());
        for val in &self.store[index] {
            if *val == key { return; }
        }
        self.store[index].push(key);
        self.size += 1;
        if self.size > self.capacity / 4 {
            self.capacity = self.capacity * 2;
            self.resize(self.capacity);
        }
    }
    
    fn remove(&mut self, key: i32) {
        let index = MyHashSet::hash(key, self.store.len());
        let mut new_bin = self.store[index].clone();
        new_bin.retain(|&val| val != key);
        if self.store[index].len() != new_bin.len() {
            self.size -= 1;
            self.store[index] = new_bin;
            if self.capacity > 8 && self.size < self.capacity / 4 {
                self.capacity = self.capacity / 2;
                self.resize(self.capacity);
            }
        }
    }
    
    fn contains(&self, key: i32) -> bool {
        let capacity = self.capacity;
        let index = MyHashSet::hash(key, capacity);
        for val in &self.store[index] {
            if *val == key { return true; }
        }
        false
    }
}

fn main() {
    let mut set = MyHashSet::new();
    let numbers = vec![10, 100, 100000, 32000, 12345, 8080, 5000, 600, 124, 125];
    for num in numbers.clone() {
        println!("hash {} for size 8: {}", num, MyHashSet::hash(num, 8));
    }
    for num in numbers.clone() {
        println!("adding {}", num);
        set.add(num);
        println!("current size: {}, current capacity: {}", set.size, set.capacity);
    }
    for bin in &set.store {
        println!("{:?}", *bin);
    }
    let test_contains = vec![
        (10, true),
        (12345, true),
        (8080, true),
        (700, false),
        (120, false),
    ];
    for (num, expected) in test_contains {
        println!("expect set.contains({}) to be {}: actual {}", num, expected, set.contains(num));
    }
    for num in numbers {
        println!("removing {}", num);
        set.remove(num);
        println!("current size: {}, current capacity: {}", set.size, set.capacity);
    }
}
