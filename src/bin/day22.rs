use std::{collections::VecDeque, time::Instant};

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<u64> {
    input.lines().map(|n| n.parse().unwrap()).collect()
}

/*
SS = ((SS * 64) ^ SS) % 16777216
SS = ((SS/32) ^ SS) % 16777216
SS = ((SS * 2048) ^ SS) % 16777216
 */

fn next_secret(num: u64) -> u64 {
    let mut n = num;
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}
fn part1(input: &str) -> String {
    parse_input(input)
        .into_iter()
        .map(|num| (0..2000).fold(num, |acc, _| next_secret(acc)))
        .sum::<u64>()
        .to_string()
}

fn solve_2(nums: &[u64]) -> u64 {
    let hash = |a, b, c, d| {
        (a + 10) as usize
            + (b + 10) as usize * 20usize.pow(1)
            + (c + 10) as usize * 20usize.pow(2)
            + (d + 10) as usize * 20usize.pow(3)
    };

    let mut dq = VecDeque::new();
    let mut map = vec![0; 20usize.pow(4)];

    for inital_secret in nums.iter() {
        let mut num = *inital_secret;
        let mut prev_price = num % 10;
        let mut seen = vec![false; 20usize.pow(4)];

        for _ in 0..2000 {
            num = next_secret(num);
            let price = num % 10;

            dq.push_back(price as i64 - prev_price as i64);
            prev_price = price;

            if dq.len() == 4 {
                let hash = hash(dq[0], dq[1], dq[2], dq[3]);
                if !seen[hash] {
                    map[hash] += price;
                    seen[hash] = true;
                }
                dq.pop_front();
            }
        }
    }

    map.into_iter().max().unwrap()
}

fn part2(input: &str) -> String {
    solve_2(&parse_input(input)).to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day22")?;

    let before = Instant::now();
    println!("{}", part1(&input));
    println!("Time taken - {:?}", Instant::now() - before);

    let before = Instant::now();
    println!("{}", part2(&input));
    println!("Time taken - {:?}", Instant::now() - before);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1
10
100
2024";
        assert_eq!("37327623", part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "1
2
3
2024";

        assert_eq!("23", part2(input));
    }
}
