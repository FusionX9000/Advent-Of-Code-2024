use std::fmt::{Debug, Display};

use advent_of_code_2024::read_input;

fn parse_input<T>(input: &str) {
    todo!()
}

fn part1(input: &str) -> (impl Display + Eq + Debug) {
    ""
}

fn part2(input: &str) -> (impl Display + Eq + Debug) {
    ""
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("dayX")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";
        assert_eq!(part1(input), todo!());
    }

    #[test]
    fn test_part2() {
        let input = "";

        assert_eq!(part2(input), todo!());
    }
}
