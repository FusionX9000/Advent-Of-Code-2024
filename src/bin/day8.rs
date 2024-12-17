use std::collections::{HashMap, HashSet};

use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn as_graph(input: &[Vec<u8>]) -> HashMap<u8, Vec<(i32, i32)>> {
    let mut hmap = HashMap::new();
    for (i, j) in (0..input.len()).cartesian_product(0..input[0].len()) {
        if input[i][j] != b'.' {
            hmap.entry(input[i][j])
                .or_insert_with(|| Vec::new())
                .push((i as i32, j as i32));
        }
    }
    hmap
}

trait Bound {
    fn within_bounds(self, max_0: i32, max_1: i32) -> bool;
}

impl Bound for (i32, i32) {
    fn within_bounds(self, max_0: i32, max_1: i32) -> bool {
        self.0 >= 0 && self.1 >= 0 && self.0 < max_0 && self.1 < max_1
    }
}

fn count_resonance(
    graph: HashMap<u8, Vec<(i32, i32)>>,
    max_row: usize,
    max_col: usize,
    single_resonance: bool,
) -> usize {
    let mut resonance_points: HashSet<(i32, i32)> = HashSet::new();
    for v in graph.values() {
        for (&a, &b) in v.iter().tuple_combinations() {
            let diff = (a.0 - b.0, a.1 - b.1);

            let mut bp = (a.0 + diff.0, a.1 + diff.1);
            while bp.within_bounds(max_row as i32, max_col as i32) {
                resonance_points.insert(bp);
                if single_resonance {
                    break;
                }
                bp = (bp.0 + diff.0, bp.1 + diff.1);
            }
            let mut fp = (b.0 - diff.0, b.1 - diff.1);
            while fp.within_bounds(max_row as i32, max_col as i32) {
                resonance_points.insert(fp);
                if single_resonance {
                    break;
                }
                fp = (fp.0 - diff.0, fp.1 - diff.1);
            }
        }
    }
    if !single_resonance {
        resonance_points.extend(graph.values().filter(|v| v.len() > 1).flatten());
    }
    resonance_points.len()
}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    let graph = as_graph(&input);
    count_resonance(graph, input.len(), input[0].len(), true).to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let graph = as_graph(&input);
    count_resonance(graph, input.len(), input[0].len(), false).to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day8")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part1() {
        assert_eq!("14", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("34", part2(INPUT));
    }
}
