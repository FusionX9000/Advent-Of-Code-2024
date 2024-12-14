use std::env;

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            let mut pair_iter = l.split("   ").map(|n| n.trim().parse::<i32>().unwrap());
            (pair_iter.next().unwrap(), pair_iter.next().unwrap())
        })
        .unzip()
}

fn part1(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (*a - *b).abs())
        .sum()
}

fn part2(input: &str) -> i32 {
    let (mut left, mut right) = parse_input(input);
    left.iter()
        .map(|lval| *lval * (right.iter().filter(|rval| **rval == *lval).count() as i32))
        .sum()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day1")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";

        assert_eq!(part2(input), 31);
    }
}
