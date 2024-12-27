use advent_of_code_2024::read_input;
use itertools::Itertools;
use std::time::Instant;

const MAX_NODES: usize = 26 * 26;

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
    let starts_with_t = |n: u32| (n / 26) as u8 == b't' - b'a';
    let connections = parse_input(input);
    let mut edges = vec![vec![false; MAX_NODES]; MAX_NODES];
    for edge in connections.iter() {
        edges[edge.0 as usize][edge.1 as usize] = true;
    }
    let mut ans = 0;
    for connection in connections {
        for node_3 in (connection.1 as usize)..(MAX_NODES) {
            if edges[connection.0 as usize][node_3]
                && edges[connection.1 as usize][node_3]
                && (starts_with_t(node_3 as u32)
                    || starts_with_t(connection.0)
                    || starts_with_t(connection.1))
            {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

fn should_connect(edges: &[Vec<bool>], connected_component: &[u32], new_node: u32) -> bool {
    connected_component.iter().all(|connected_node| {
        new_node != *connected_node && edges[*connected_node as usize][new_node as usize]
    })
}

// Optimisation inspired by - https://www.reddit.com/r/adventofcode/comments/1hkgj5b/2024_day_23_solutions/m3fbw3t/
fn part2(input: &str) -> String {
    let connections = parse_input(input);
    let mut edges = vec![vec![false; MAX_NODES]; MAX_NODES];
    for edge in connections.iter() {
        edges[edge.0 as usize][edge.1 as usize] = true;
    }

    let mut interconnections: Vec<Vec<u32>> = connections.iter().map(|&c| vec![c.0, c.1]).collect();

    for connection in interconnections.iter_mut() {
        for node in *connection.last().unwrap()..(MAX_NODES as u32) {
            if should_connect(&edges, connection, node) {
                connection.push(node);
            }
        }
    }

    interconnections
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
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
