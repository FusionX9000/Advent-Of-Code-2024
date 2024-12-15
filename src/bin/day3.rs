use advent_of_code_2024::read_input;
use regex::Regex;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(String::from).collect()
}

fn get_valid_instructions_1(corr_memory: &String) -> Vec<(i64, i64)> {
    let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
    re.captures_iter(&corr_memory)
        .into_iter()
        .map(|capture| {
            let first = capture.get(1).unwrap().as_str();
            let second = capture.get(2).unwrap().as_str();
            (first.parse().unwrap(), second.parse().unwrap())
        })
        .collect()
}

fn part1(input: &str) -> String {
    let corr_memories = parse_input(input);
    get_valid_instructions_1(&corr_memories.join(""))
        .iter()
        .map(|(a, b)| a * b)
        .sum::<i64>()
        .to_string()
}

fn get_valid_instructions_2(corr_memory: &String) -> Vec<(i64, i64)> {
    let re = Regex::new("(?:mul\\(([0-9]{1,3}),([0-9]{1,3})\\)|do\\(\\)|don't\\(\\))").unwrap();
    let mut flag = true;
    re.captures_iter(&corr_memory)
        .into_iter()
        .map(|capture| {
            let capture_str = capture.get(0).unwrap().as_str();
            if capture_str == "do()" {
                flag = true;
                (1, 0)
            } else if capture_str == "don't()" {
                flag = false;
                (1, 0)
            } else {
                if !flag {
                    return (1, 0);
                }
                let first = capture.get(1).unwrap().as_str();
                let second = capture.get(2).unwrap().as_str();
                (first.parse().unwrap(), second.parse().unwrap())
            }
        })
        .collect()
}

fn part2(input: &str) -> String {
    let corr_memories = parse_input(input);
    get_valid_instructions_2(&corr_memories.join(""))
        .iter()
        .map(|(a, b)| a * b)
        .sum::<i64>()
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day3")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", part1(input));
    }

    #[test]
    fn test_part2() {
        let input: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!("48", part2(input));
    }
}
