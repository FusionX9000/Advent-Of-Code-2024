use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect()
}

const D: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn solve_part1(grid: &[Vec<u8>]) -> ((usize, usize, usize), usize, Vec<usize>) {
    let mut visit_cost = vec![usize::MAX; grid.len() * grid[0].len() * 4];
    let mut q: VecDeque<(usize, usize, usize, usize)> = VecDeque::new();
    q.push_back((grid.len() - 2, 1, 0, 0));
    let mut min_cost = usize::MAX;
    let mut end = (0, 0, 0);
    while !q.is_empty() {
        let (r, c, dir, cost) = q.pop_front().unwrap();
        if grid[r][c] == b'E' {
            end = (r, c, dir);
            min_cost = cmp::min(min_cost, cost);
        }

        for i in [3, 1] {
            let dir = (dir + i) % 4;
            let hash = ((r * grid[0].len()) + c) * 4 + dir;
            let ncost = cost + 1000;

            if ncost < visit_cost[hash] && ncost < min_cost {
                q.push_back((r, c, dir, ncost));
                visit_cost[hash] = ncost;
            }
        }
        let r = (r as i64 + D[dir].0) as usize;
        let c = (c as i64 + D[dir].1) as usize;
        let hash = ((r * grid[0].len()) + c) * 4 + dir;
        let ncost = cost + 1;

        if ncost < visit_cost[hash] && grid[r][c] != b'#' && ncost < min_cost {
            q.push_back((r, c, dir, ncost));
            visit_cost[hash] = ncost;
        }
    }
    (end, min_cost, visit_cost)
}

fn part1(input: &str) -> String {
    let (_, min_cost, _) = solve_part1(&parse_input(input));
    min_cost.to_string()
}

fn part2(input: &str) -> String {
    let grid = parse_input(input);

    let encode_coord = |r, c| -> usize { (r * grid[0].len()) + c };
    let encode_all = |r, c, dir| -> usize { encode_coord(r, c) * 4 + dir };

    let (end, min_cost, visit_cost) = solve_part1(&grid);

    let mut q = VecDeque::new();
    q.push_back((end.0, end.1, end.2, min_cost));

    let mut set = HashSet::<usize>::new();
    while !q.is_empty() {
        let (r, c, dir, cost) = q.pop_front().unwrap();
        set.insert(encode_coord(r, c));
        if cost == 0 {
            continue;
        }
        for i in [1, 3] {
            let dir = (dir + i) % 4;
            let hash = encode_all(r, c, dir);
            let ncost = cost - 1000;
            if visit_cost[hash] == ncost {
                q.push_back((r, c, dir, ncost));
            }
        }
        let r = (r as i64 - D[dir].0) as usize;
        let c = (c as i64 - D[dir].1) as usize;
        let hash = encode_all(r, c, dir);
        let ncost = cost - 1;
        if visit_cost[hash] == ncost {
            q.push_back((r, c, dir, ncost));
        }
    }

    set.len().to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day16")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_0() {
        let input = "############
#..........#
#.########.#
#.#......#.#
#.#......#.#
#.#......#.#
#.#......#.#
#.#......#.#
#.#....###.#
#.#.####...#
#.##...#.#E#
#S...#...###
############";
        assert_eq!("3028", part1(input));
    }

    #[test]
    fn test_part1_1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!("7036", part1(input));
    }

    #[test]
    fn test_part1_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!("11048", part1(input));
    }

    #[test]
    fn test_part2_1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

        assert_eq!("45", part2(input));
    }

    #[test]
    fn test_part2_2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

        assert_eq!("64", part2(input));
    }
}
