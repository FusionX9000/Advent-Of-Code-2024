use std::collections::HashMap;

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split_ascii_whitespace()
        .map(|n| n.trim().parse().unwrap())
        .collect()
}

fn count_digits(num: u64) -> u32 {
    let mut count = 0;
    let mut n = num;
    while n > 0 {
        count += 1;
        n /= 10;
    }
    count
}
fn split_num_middle(num: u64) -> (u64, u64) {
    let count = count_digits(num);
    let mid = count / 2;
    let left = num / 10u64.pow(mid);
    let right = num % 10u64.pow(mid);

    (left, right)
}

fn count_stones(num: u64, rem: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    let ans;
    if let Some(&val) = cache.get(&(num, rem)) {
        return val;
    }
    if rem == 0 {
        ans = 1;
    } else if num == 0 {
        ans = count_stones(1, rem - 1, cache);
    } else if count_digits(num) % 2 == 0 {
        let (left, right) = split_num_middle(num);
        ans = count_stones(left, rem - 1, cache) + count_stones(right, rem - 1, cache);
    } else {
        ans = count_stones(num * 2024, rem - 1, cache);
    }
    cache.insert((num, rem), ans);
    ans
}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    input
        .into_iter()
        .map(|num| count_stones(num, 25, &mut cache))
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    input
        .into_iter()
        .map(|num| count_stones(num, 75, &mut cache))
        .sum::<u64>()
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day11")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        assert_eq!("55312", part1(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "125 17";

        assert_eq!("", part2(input));
    }
}
