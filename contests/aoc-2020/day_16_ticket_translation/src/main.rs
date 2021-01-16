fn main() {
    let (rules, your_ticket, other_tickets) = parse_input(
        include_str!("../input.txt")
            .lines()
            .map(|line| line.to_string())
            .collect(),
    );
    let tree = get_interval_tree(&rules);
    println!("error rate: {}", get_error_rate(&tree, &other_tickets));
    // part 2
    let valid_tickets = filter_tickets(&tree, other_tickets);
    let valid_rule_indices = valid_rule_indices(&rules, &valid_tickets);
    if let Some(assigned) = assign_indices(&valid_rule_indices) {
        println!("{:?}", assigned);
        // the first 6 fields start with departure
        let product: u64 = assigned
            .iter()
            .take(6)
            .map(|&i| your_ticket[i] as u64)
            .product();
        println!("product: {}", product);
    }
}

use intervaltree;
use std::iter::FromIterator;
type Rule = Vec<(u32, u32)>;
type IntervalTree = intervaltree::IntervalTree<u32, usize>;

fn parse_input(input: Vec<String>) -> (Vec<Rule>, Vec<u32>, Vec<Vec<u32>>) {
    let mut rules: Vec<Rule> = Vec::new();
    let your_ticket: Vec<u32>;
    let mut other_tickets: Vec<Vec<u32>> = Vec::new();

    let mut input = input.into_iter();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }
        let mut line = line.split(": ");
        line.next();
        let mut bounds: Vec<(u32, u32)> = Vec::new();
        for bound in line.next().unwrap().split(" or ") {
            let mut nums = bound.split("-");
            let low: u32 = nums.next().unwrap().parse().unwrap();
            let high: u32 = nums.next().unwrap().parse().unwrap();
            bounds.push((low, high));
        }
        rules.push(bounds);
    }

    input.next();
    your_ticket = input
        .next()
        .unwrap()
        .split(",")
        .map(|string| string.parse().unwrap())
        .collect();
    input.next();
    input.next();

    for ticket in input {
        let ticket = ticket.split(",").map(|num| num.parse().unwrap()).collect();
        other_tickets.push(ticket);
    }
    (rules, your_ticket, other_tickets)
}

fn get_interval_tree(rules: &Vec<Rule>) -> IntervalTree {
    IntervalTree::from_iter(
        rules
            .iter()
            .map(|vec| vec.iter())
            .flatten()
            .enumerate()
            .map(|(i, &(s, e))| ((s..e + 1), i)),
    )
}

fn get_error_rate(tree: &IntervalTree, other_tickets: &Vec<Vec<u32>>) -> u32 {
    other_tickets
        .iter()
        .map(|vec| vec.iter())
        .flatten()
        .filter(|&&val| tree.query_point(val).peekable().peek().is_none())
        .sum()
}

fn filter_tickets(tree: &IntervalTree, other_tickets: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    other_tickets
        .into_iter()
        .filter(|vec| {
            vec.iter()
                .all(|&val| tree.query_point(val).peekable().peek().is_some())
        })
        .collect()
}

// another cool approach I saw was to instead assemble a Vec<HashSet<usize>> of possible rule
// assignments, then iterate through the vec repeatedly looking for rules with unique assignments
// and removing them from all other rules. If this does not uniquely assign all indices, it at
// least reduces n in the O(2^n) backtracking approach significantly

fn valid_rule_indices(rules: &Vec<Rule>, valid_tickets: &Vec<Vec<u32>>) -> Vec<Vec<usize>> {
    let field_count = rules.len();
    let ticket_count = valid_tickets.len();
    let mut res = Vec::new();
    for i in 0..field_count {
        let mut valid_indices = Vec::new();
        let min_1 = rules[i][0].0;
        let max_1 = rules[i][0].1;
        let min_2 = rules[i][1].0;
        let max_2 = rules[i][1].1;
        for j in 0..field_count {
            if (0..ticket_count).all(|k| {
                let num = valid_tickets[k][j];
                (num >= min_1 && num <= max_1) || (num >= min_2 && num <= max_2)
            }) {
                valid_indices.push(j);
            }
        }
        res.push(valid_indices);
    }
    res
}

use std::collections::HashSet;

fn backtrack_assign_indices(
    i: usize,
    rule_indices: &Vec<Vec<usize>>,
    res: &mut Vec<usize>,
    set: &mut HashSet<usize>,
) -> bool {
    if i == rule_indices.len() {
        return true;
    }
    for &index in &rule_indices[i] {
        if !set.contains(&index) {
            set.insert(index);
            res.push(index);
            if backtrack_assign_indices(i + 1, rule_indices, res, set) {
                return true;
            }
            set.remove(&index);
            res.pop();
        }
    }
    false
}

fn assign_indices(rule_indices: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let mut res = Vec::new();
    let mut set: HashSet<usize> = HashSet::new();
    if backtrack_assign_indices(0, rule_indices, &mut res, &mut set) {
        return Some(res);
    }
    None
}
