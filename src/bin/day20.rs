use std::collections::VecDeque;

use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.as_bytes().to_vec()).collect()
}

type Point = (usize, usize);

fn get_coords(grid: &[Vec<u8>]) -> (Point, Point) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == b'S' {
                start = (r, c);
            } else if grid[r][c] == b'E' {
                end = (r, c);
            }
        }
    }
    (start, end)
}

fn bfs(grid: &[Vec<u8>], start: Point, end: Point) -> Vec<Vec<u64>> {
    let check_bounds =
        |r, c| r >= 0 && c >= 0 && (r as usize) < grid.len() && (c as usize) < grid[0].len();

    let max_cost: u64 = (grid[0].len() * grid.len() * 100) as u64;
    let mut time: Vec<Vec<u64>> = vec![vec![max_cost; grid[0].len()]; grid.len()];

    let mut q: VecDeque<(Point, u64)> = VecDeque::new();
    q.push_back((start, 0));

    while !q.is_empty() {
        let ((r, c), sec) = q.pop_front().unwrap();
        if r == end.0 && c == end.1 {
            continue;
        }
        for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nr = r as i64 + dir.0;
            let nc = c as i64 + dir.1;
            let nsec = sec + 1;
            if !check_bounds(nr, nc)
                || grid[nr as usize][nc as usize] == b'#'
                || time[nr as usize][nc as usize] <= nsec
            {
                continue;
            }
            q.push_back(((nr as usize, nc as usize), nsec));
            time[nr as usize][nc as usize] = nsec;
        }
    }
    time
}

/*
cheat duration = 3
#############
######3######
#####323#####
####32123####
###321O123###
####32123####
#####323#####
######3######
#############

For each point (x,y) within distance of "ps" from centre (r,c) where r in 0..H and c in 0..W,
    If both (x,y) and (r,c) are valid and unblocked points,
        Then cost_saved = time_to_end[r][c] - dis_between(x_y,r_c) - time_to_end[x][y]

TC = O(HxWxPS^2)
 */

fn solve(grid: Vec<Vec<u8>>, time_to_end: Vec<Vec<u64>>, ps: usize) -> u64 {
    let min_time_saved = if !cfg!(test) { 100 } else { 50 };
    let check_bounds =
        |r, c| r >= 0 && c >= 0 && (r as usize) < grid.len() && (c as usize) < grid[0].len();

    let mut ans = 0;
    let ps = ps as i64;
    // O(HW)
    for r in 1..grid.len() - 1 {
        for c in 1..grid[0].len() - 1 {
            if grid[r][c] == b'#' {
                continue;
            }
            // Costs O(2PS^2)
            for (dx, dy) in (-ps..=ps).cartesian_product(-ps..=ps) {
                if dx.abs() + dy.abs() > ps || dx.abs() + dy.abs() == 0 {
                    continue;
                }

                let nr = r as i64 + dx;
                let nc = c as i64 + dy;

                if !check_bounds(nr, nc) || grid[nr as usize][nc as usize] == b'#' {
                    continue;
                }

                let time_without_cheat = time_to_end[r][c];
                let time_with_cheat =
                    (dx.abs() + dy.abs()) as u64 + time_to_end[nr as usize][nc as usize];
                if (time_without_cheat as i64) - (time_with_cheat as i64) >= min_time_saved {
                    ans += 1;
                }
            }
        }
    }
    ans
}

fn part1(input: &str) -> String {
    let grid = parse_input(input);
    let (start, end) = get_coords(&grid);
    let time_to_end = bfs(&grid, end, start);
    solve(grid, time_to_end, 2).to_string()
}

fn part2(input: &str) -> String {
    let grid = parse_input(input);
    let (start, end) = get_coords(&grid);
    let time_to_end = bfs(&grid, end, start);
    solve(grid, time_to_end, 20).to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day20")?;
    let before = std::time::Instant::now();
    println!("{}", part1(&input));
    println!(
        "Total time taken - {:?}",
        std::time::Instant::now() - before
    );

    let before = std::time::Instant::now();
    println!("{}", part2(&input));
    println!(
        "Total time taken - {:?}",
        std::time::Instant::now() - before
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part2() {
        assert_eq!("284", part2(INPUT));
    }
}
