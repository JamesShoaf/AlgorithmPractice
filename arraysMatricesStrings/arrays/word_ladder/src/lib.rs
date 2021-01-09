/*
Given two words beginWord and endWord, and a dictionary wordList, return the length of the shortest
transformation sequence from beginWord to endWord, such that:

    Only one letter can be changed at a time.
    Each transformed word must exist in the word list.

Return 0 if there is no such transformation sequence.
*/

use std::collections::{HashMap, HashSet};

// generate graph of words that are off from eachother by 1 letter
fn generate_word_graph(word_list: Vec<String>) -> HashMap<String, HashSet<String>> {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    for i in 0..word_list.len() {
        for j in i + 1..word_list.len() {
            if word_list[i]
                .chars()
                .zip(word_list[j].chars())
                .filter(|(ci, cj)| ci != cj)
                .count()
                == 1
            {
                graph
                    .entry(word_list[i].clone())
                    .or_insert(HashSet::new())
                    .insert(word_list[j].clone());
                graph
                    .entry(word_list[j].clone())
                    .or_insert(HashSet::new())
                    .insert(word_list[i].clone());
            }
        }
    }
    graph
}

// dfs to find shortest route from start to end
pub fn ladder_length(start: String, end: String, mut word_list: Vec<String>) -> i32 {
    word_list.push(start.clone());
    let graph = generate_word_graph(word_list);
    let mut level = vec![start.clone()];
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(start);
    let mut res = 0;
    while !level.is_empty() {
        res += 1;
        let mut next_level = Vec::new();
        while let Some(word) = level.pop() {
            if end == word {
                return res;
            }
            if let Some(connected) = graph.get(&word) {
                for next_word in connected.iter() {
                    if !visited.contains(next_word) {
                        visited.insert(next_word.clone());
                        next_level.push(next_word.clone());
                    }
                }
            }
        }
        level = next_level;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (
                "hit".to_string(),
                "cog".to_string(),
                vec![
                    "hot".to_string(),
                    "dot".to_string(),
                    "dog".to_string(),
                    "lot".to_string(),
                    "log".to_string(),
                    "cog".to_string(),
                ],
                5,
            ),
            (
                "hot".to_string(),
                "dog".to_string(),
                vec!["hot".to_string(), "dot".to_string()],
                0,
            ),
        ];
        for (start, end, word_list, expected) in test_tuples {
            assert_eq!(ladder_length(start, end, word_list), expected);
        }
    }
}
