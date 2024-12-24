use std::collections::VecDeque;

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut line_split = line.split(",").map(|n| n.parse().unwrap());
            (line_split.next().unwrap(), line_split.next().unwrap())
        })
        .collect::<Vec<(usize, usize)>>()
}

fn solve_for_bytes(input: &[(usize, usize)]) -> Option<usize> {
    let height = input.iter().map(|c| c.1).max().unwrap() + 1;
    let width = input.iter().map(|c| c.0).max().unwrap() + 1;

    let mut grid = vec![vec![false; width]; height];

    for (c, r) in input {
        grid[*r][*c] = true;
    }

    let mut q = VecDeque::new();
    q.push_back((0usize, 0usize, 0usize));

    while !q.is_empty() {
        let step = q.pop_front().unwrap();
        if step.0 == height - 1 && step.1 == width - 1 {
            return Some(step.2);
        }
        for dir in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nr = step.0 as i64 + dir.0;
            let nc = step.1 as i64 + dir.1;
            if nr < 0
                || nc < 0
                || nr >= height as i64
                || nc >= width as i64
                || grid[nr as usize][nc as usize]
            {
                continue;
            }
            q.push_back((nr as usize, nc as usize, step.2 + 1));
            grid[nr as usize][nc as usize] = true;
        }
    }
    None
}

fn part1(input: &str) -> String {
    let mut bytes_to_read = 1024;
    if cfg!(test) {
        bytes_to_read = 12;
    }
    let input: &[(usize, usize)] = &parse_input(input)[..bytes_to_read];
    solve_for_bytes(input).unwrap().to_string()
}

// Classic binary search, how I missed thee
fn part2(input: &str) -> String {
    let input: &[(usize, usize)] = &parse_input(input);
    let mut start = 0;
    let mut end = input.len() - 1;
    while start < end {
        let mid = start + (end - start) / 2;
        if solve_for_bytes(&input[..=mid]).is_some() {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    format!("{},{}", input[start].0, input[start].1)
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day18")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part1() {
        assert_eq!("22", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("6,1", part2(INPUT));
    }
}
