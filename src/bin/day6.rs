use std::{i32, usize};

use advent_of_code_2024::read_input;
use itertools::Itertools;

const DIRECTION: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect()
}

// As per part 1, guard will always leave mapped area
fn traverse_1(map_vec: &mut Vec<Vec<char>>, r: i32, c: i32, dir: usize) -> () {
    if r < 0 || c < 0 || (r as usize) >= map_vec.len() || (c as usize) >= map_vec[0].len() {
        return;
    }
    if map_vec[r as usize][c as usize] == '#' {
        traverse_1(
            map_vec,
            r - DIRECTION[dir].0,
            c - DIRECTION[dir].1,
            (dir + 1) % 4,
        );
        return;
    }
    map_vec[r as usize][c as usize] = 'X';
    traverse_1(map_vec, r + DIRECTION[dir].0, c + DIRECTION[dir].1, dir);
}

fn guard_index(map_vec: &Vec<Vec<char>>) -> (usize, usize) {
    for r in 0..map_vec.len() {
        for c in 0..map_vec[0].len() {
            if map_vec[r][c] == '^' {
                return (r, c);
            }
        }
    }
    panic!("Cannnot find start point")
}

fn part1(input: &str) -> String {
    let mut input = parse_input(input);
    let (r, c) = guard_index(&input);
    traverse_1(&mut input, r as i32, c as i32, 0);
    input
        .into_iter()
        .flatten()
        .filter(|c| *c == 'X')
        .count()
        .to_string()
}

fn outside_bounds(pos: (i32, i32), map_vec: &Vec<Vec<char>>) -> bool {
    !(pos.0 >= 0
        && pos.1 >= 0
        && (pos.0 as usize) < map_vec.len()
        && (pos.1 as usize) < map_vec[0].len())
}

fn is_blocked(pos: (i32, i32), map_vec: &Vec<Vec<char>>) -> bool {
    map_vec[pos.0 as usize][pos.1 as usize] == '#'
}

fn next_pos(pos: (usize, usize), dir: usize) -> (i32, i32) {
    (
        pos.0 as i32 + DIRECTION[dir].0,
        pos.1 as i32 + DIRECTION[dir].1,
    )
}

fn visited_2(cache: &Vec<Vec<u32>>, pos: (usize, usize), dir: usize) -> bool {
    let curr_bitset = cache[pos.0][pos.1];

    let bitmask = 1 << dir;

    (curr_bitset & bitmask) > 0
}

fn set_bit(cache: &mut Vec<Vec<u32>>, pos: (usize, usize), dir: usize) {
    let curr_bitset = cache[pos.0][pos.1];

    let bitmask = 1 << dir;
    let bitset = curr_bitset | bitmask;

    cache[pos.0][pos.1] = bitset;
}

fn traverse_2(map_vec: &Vec<Vec<char>>, pos: (usize, usize)) -> bool {
    let mut cache: Vec<Vec<u32>> = (0..map_vec.len())
        .into_iter()
        .map(|_| vec![0; map_vec[0].len()])
        .collect();

    let mut dir = 0;
    let mut pos = pos;

    loop {
        let npos = next_pos(pos, dir);
        if outside_bounds(npos, map_vec) {
            return false;
        }

        if is_blocked(npos, map_vec) {
            if visited_2(&cache, pos, dir) {
                return true;
            }
            set_bit(&mut cache, pos, dir);
            dir = (dir + 1) % 4;
        } else {
            pos = (npos.0 as usize, npos.1 as usize);
        }
    }
}

fn part2(input: &str) -> String {
    let mut input = parse_input(input);
    let (r, c) = guard_index(&input);

    let before_all = std::time::Instant::now();

    traverse_1(&mut input, r as i32, c as i32, 0);

    let a = (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|(i, j)| input[*i][*j] == 'X')
        .filter(|(i, j)| !(*i == r && *j == c))
        .collect::<Vec<(usize, usize)>>()
        .iter()
        .filter(|(i, j)| {
            input[*i][*j] = '#';
            let ans = traverse_2(&input, (r, c));
            input[*i][*j] = 'X';
            ans
        })
        .count();

    println!(
        "total time taken: {:?}",
        std::time::Instant::now() - before_all
    );
    a.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day6")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part1() {
        assert_eq!("41", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("6", part2(INPUT));
    }
}
