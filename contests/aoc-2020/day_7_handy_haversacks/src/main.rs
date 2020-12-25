use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    if let Ok(f) = File::open("input.txt") {
        let f = BufReader::new(f);
        let rules: Vec<(String, Vec<(String, u32)>)> = f
            .lines()
            .filter(|line| line.is_ok())
            .map(|line| line.unwrap())
            .map(|rule| parse_rule(&rule))
            .collect();
        let containment_tree = get_containment_tree(&rules);
        println!(
            "Types that could contain a shiny gold bag: {}",
            count_containing_types("shiny gold", &containment_tree)
        );
        let containing_tree: HashMap<String, Vec<(String, u32)>> = rules.into_iter().collect();
        println!(
            "Total bags inside a shiny gold bag: {}",
            count_contained_bags("shiny gold", &containing_tree)
        );
    }
}

fn parse_rule(rule: &String) -> (String, Vec<(String, u32)>) {
    let mut rule = rule.split(" bags contain ");
    let bag_type = rule.next().unwrap().to_string();
    let contained = rule.next().unwrap();
    let mut held: Vec<(String, u32)> = Vec::new();
    if contained != "no other bags." {
        for bag in contained.split(',') {
            let mut bag = bag.split_whitespace();
            let count: u32 = bag.next().unwrap().parse::<u32>().unwrap();
            let mut held_bag_type = String::new();
            held_bag_type.push_str(bag.next().unwrap());
            held_bag_type.push(' ');
            held_bag_type.push_str(bag.next().unwrap());
            held.push((held_bag_type, count));
        }
    }
    (bag_type, held)
}

fn get_containment_tree(
    rules: &Vec<(String, Vec<(String, u32)>)>,
) -> HashMap<String, HashMap<String, u32>> {
    let mut res = HashMap::new();
    for (outer_type, inner) in rules.iter() {
        for (inner_type, count) in inner.iter() {
            res.entry(inner_type.to_string())
                .or_insert(HashMap::new())
                .entry(outer_type.to_string())
                .or_insert(*count);
        }
    }
    res
}

// bfs through all containing types for target
fn count_containing_types(
    contained_type: &str,
    tree: &HashMap<String, HashMap<String, u32>>,
) -> usize {
    let mut queue: VecDeque<String> = VecDeque::new();
    queue.push_back(contained_type.to_string());
    let mut visited: HashSet<String> = HashSet::new();
    while let Some(bag_type) = queue.pop_front() {
        if let Some(containers) = tree.get(&bag_type) {
            for (containing_type, _) in containers.iter() {
                if !visited.contains(containing_type) {
                    queue.push_back(containing_type.to_string());
                }
            }
        }
        visited.insert(bag_type);
    }
    // subtract 1 to exclude target type
    visited.len() - 1
}

fn count_contained_bags(containing_type: &str, tree: &HashMap<String, Vec<(String, u32)>>) -> u32 {
    let mut memo: HashMap<String, u32> = HashMap::new();
    dfs(containing_type, tree, &mut memo) - 1
}

fn dfs(
    containing_type: &str,
    tree: &HashMap<String, Vec<(String, u32)>>,
    memo: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(&prev) = memo.get(containing_type) {
        return prev;
    }
    let mut total = 1;
    for (contained, count) in tree.get(containing_type).unwrap().iter() {
        total += count * dfs(contained, tree, memo);
    }
    memo.insert(containing_type.to_string(), total);
    total
}

mod tests {
    #[test]
    fn test_parse_rule() {
        let test_tuples = vec![
            (
                "posh black bags contain no other bags.".to_string(),
                ("posh black".to_string(), Vec::new()),
            ),
            (
                "shiny purple bags contain 5 faded aqua bags.".to_string(),
                ("shiny purple".to_string(), vec![
                    ("faded aqua".to_string(), 5),
                ]),
            ),
            (
                "clear orange bags contain 1 posh orange bag, 4 bright orange bags, 5 clear indigo bags.".to_string(),
                ("clear orange".to_string(), vec![
                    ("posh orange".to_string(), 1),
                    ("bright orange".to_string(), 4),
                    ("clear indigo".to_string(), 5),
                ]),
            ),
        ];
        for (rule, expected) in test_tuples {
            assert_eq!(super::parse_rule(&rule), expected);
        }
    }
}
