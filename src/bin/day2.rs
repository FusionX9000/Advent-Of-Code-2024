use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[derive(PartialEq, Debug)]
enum Order {
    Increasing,
    Decreasing,
    Equal,
}

impl Order {
    fn build(diff: i32) -> Self {
        match diff {
            ..0 => Self::Decreasing,
            0 => Self::Equal,
            _ => Self::Increasing,
        }
    }
}

fn is_safe_p1(report: &Vec<i32>) -> bool {
    if report.len() <= 1 {
        return true;
    }
    let order = Order::build(report[0] - report[1]);
    for i in 1..report.len() {
        let diff = report[i - 1] - report[i];
        let curr_order = Order::build(diff);
        if diff.abs() > 3 || curr_order != order || curr_order == Order::Equal {
            return false;
        }
    }
    true
}

fn part1(input: &str) -> String {
    let input = parse_input(input);
    input.into_iter().filter(is_safe_p1).count().to_string()
}

fn is_safe_p2(report: &Vec<i32>) -> bool {
    if is_safe_p1(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut new_report = report.clone();
        new_report.remove(i);
        if is_safe_p1(&new_report) {
            return true;
        }
    }
    false
}

// TODO - Optimise to O(N)
fn part2(input: &str) -> String {
    let input = parse_input(input);
    input.into_iter().filter(is_safe_p2).count().to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day2")?;
    println!("Part 1 solution:\n{}", part1(&input));
    println!();
    println!("Part 2 solution:\n{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("4", part2(input));
    }
}
