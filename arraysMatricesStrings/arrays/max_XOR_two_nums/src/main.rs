use max_XOR_two_nums::BinaryTrie;

fn main() {
    let mut trie = BinaryTrie::new(3);
    let nums = vec![0, 7];
    println! {"trie: {:?}", trie}
    for num in nums {
        trie.insert(num);
        println!("inserted {}: {:?}", num, trie);
    }
}