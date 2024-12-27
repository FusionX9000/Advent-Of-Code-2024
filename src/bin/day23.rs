use std::time::Instant;

use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u32, u32)> {
    input
        .lines()
        .map(|line| {
            let [left, right]: [u32; 2] = line
                .split("-")
                .map(|id_str| {
                    (id_str.as_bytes()[0] - b'a') as u32 * 26 + (id_str.as_bytes()[1] - b'a') as u32
                })
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap();
            if left < right {
                (left, right)
            } else {
                (right, left)
            }
        })
        .collect()
}

fn to_string(node: &u32) -> String {
    (((node / 26) as u8 + b'a') as char).to_string()
        + &(((node % 26) as u8 + b'a') as char).to_string()
}

fn part1(input: &str) -> String {
    let connections = parse_input(input);
    let mut edges = vec![vec![false; 26 * 26]; 26 * 26];
    for edge in connections.iter() {
        edges[edge.0 as usize][edge.1 as usize] = true;
        edges[edge.1 as usize][edge.0 as usize] = true;
    }

    let interconnections: Vec<Vec<u32>> = connections.iter().map(|&c| vec![c.0, c.1]).collect();

    let three_interconnections = get_next_interconnections(&interconnections, &edges);
    three_interconnections
        .into_iter()
        .filter(|v| v.iter().any(|node| to_string(node).starts_with('t')))
        .count()
        .to_string()
}

fn should_connect(edges: &[Vec<bool>], connected_component: &[u32], new_node: u32) -> bool {
    for connected_node in connected_component.iter() {
        if new_node == *connected_node || !edges[new_node as usize][*connected_node as usize] {
            return false;
        }
    }
    true
}

fn get_next_interconnections(interconnections: &[Vec<u32>], edges: &[Vec<bool>]) -> Vec<Vec<u32>> {
    let mut next_interconnections = Vec::new();

    for interconnection in interconnections.iter() {
        for new_node in (*interconnection.last().unwrap())..(26 * 26) {
            if should_connect(edges, interconnection, new_node) {
                let mut next_connection = interconnection.clone();
                next_connection.push(new_node);
                next_interconnections.push(next_connection);
            }
        }
    }

    next_interconnections
}

fn part2(input: &str) -> String {
    let connections = parse_input(input);
    let mut edges = vec![vec![false; 26 * 26]; 26 * 26];
    for edge in connections.iter() {
        edges[edge.0 as usize][edge.1 as usize] = true;
        edges[edge.1 as usize][edge.0 as usize] = true;
    }

    let mut interconnections: Vec<Vec<u32>> = connections.iter().map(|&c| vec![c.0, c.1]).collect();
    let mut next_interconnections: Vec<Vec<u32>> =
        get_next_interconnections(&interconnections, &edges);

    while !next_interconnections.is_empty() {
        interconnections = next_interconnections;
        next_interconnections = get_next_interconnections(&interconnections, &edges);
    }

    interconnections
        .first()
        .unwrap()
        .iter()
        .map(to_string)
        .join(",")
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day23")?;
    let before = Instant::now();
    println!("{}", part1(&input));
    println!("Time taken - {:?}", Instant::now() - before);

    println!();

    let before = Instant::now();
    println!("{}", part2(&input));
    println!("Time taken - {:?}", Instant::now() - before);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part1() {
        assert_eq!("7", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("co,de,ka,ta", part2(INPUT));
    }
}
