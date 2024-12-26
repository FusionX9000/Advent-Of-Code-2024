use std::{
    collections::{HashMap, HashSet},
    ops::Index,
};

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();
    let order_vec: Vec<(u32, u32)> = sections[0]
        .lines()
        .map(|l| {
            let nums: Vec<u32> = l.split("|").map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    let updates_vec: Vec<Vec<u32>> = sections[1]
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (order_vec, updates_vec)
}

fn create_graph(order_rules: &[(u32, u32)], update: &[u32]) -> HashMap<u32, Vec<u32>> {
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();

    let updates_set = update.iter().copied().collect::<HashSet<u32>>();

    for (l, r) in order_rules {
        if !updates_set.contains(l) || !updates_set.contains(r) {
            continue;
        }
        graph.entry(*l).or_default().push(*r);
    }
    graph
}

fn traverse(
    graph: &HashMap<u32, Vec<u32>>,
    node: u32,
    output: &mut Vec<u32>,
    visited: &mut HashSet<u32>,
) {
    if visited.contains(&node) {
        return;
    }
    for child_node in graph
        .get(&node)
        .into_iter()
        .flatten()
        .collect::<Vec<&u32>>()
    {
        traverse(graph, *child_node, output, visited);
    }
    visited.insert(node);
    output.push(node);
}

fn get_topological_order(graph: HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut output = Vec::new();
    let mut visited = HashSet::new();
    for node in graph.keys() {
        traverse(&graph, *node, &mut output, &mut visited);
    }
    output.reverse();
    output
}

fn valid_update(update: &[u32], topo: &[u32]) -> bool {
    let mut prev = 0;
    update.iter().all(|node| {
        let idx = topo.iter().position(|x| x == node).unwrap();
        if idx < prev {
            return false;
        }
        prev = idx;
        true
    })
}

fn part1(input: &str) -> String {
    let (order_rules, updates) = parse_input(input);

    updates
        .iter()
        .map(|nodes| {
            let graph = create_graph(&order_rules, nodes);
            let topo = get_topological_order(graph);
            if valid_update(nodes, &topo) {
                return *nodes.index(nodes.len() / 2);
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

fn part2(input: &str) -> String {
    let (order_rules, updates) = parse_input(input);

    updates
        .iter()
        .map(|nodes| {
            let graph = create_graph(&order_rules, nodes);
            let topo = get_topological_order(graph);
            if !valid_update(nodes, &topo) {
                return *topo.index(topo.len() / 2);
            }
            0
        })
        .sum::<u32>()
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day5")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part1() {
        assert_eq!("143", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("123", part2(INPUT));
    }
}
