use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<(u64, Vec<u64>)> {
    input
        .lines()
        .map(|l| {
            let sections: Vec<&str> = l.split(": ").collect();
            let res: u64 = sections[0].trim().parse().unwrap();
            let nums: Vec<u64> = sections[1]
                .split_ascii_whitespace()
                .map(|n| n.trim().parse().unwrap())
                .collect();
            (res, nums)
        })
        .collect()
}

fn concat(a: u64, b: u64) -> u64 {
    let mut digits = 0;
    let mut bb = b;
    loop {
        if bb == 0 {
            break;
        }
        bb /= 10;
        digits += 1;
    }

    a * 10_u64.pow(digits) + b
}

fn solveable(ref_value: u64, value: u64, nums: &[u64], third_op: bool) -> bool {
    if value == ref_value && nums.is_empty() {
        return true;
    }
    if nums.is_empty() || value > ref_value {
        return false;
    }

    solveable(ref_value, value + nums[0], &nums[1..], third_op)
        || solveable(ref_value, value * nums[0], &nums[1..], third_op)
        || (third_op && solveable(ref_value, concat(value, nums[0]), &nums[1..], third_op))
}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    input
        .into_iter()
        .filter(|q| solveable(q.0, 0, &q.1[..], false))
        .map(|q| q.0)
        .sum::<u64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    input
        .into_iter()
        .filter(|q| solveable(q.0, 0, &q.1[..], true))
        .map(|q| q.0)
        .sum::<u64>()
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day7")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_part1() {
        assert_eq!("3749", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("11387", part2(INPUT));
    }
}
