use advent_of_code_2024::read_input;

fn parse_input(input: &str) {
    todo!()
}

fn part1(input: &str) -> String {
    String::new()
}

fn part2(input: &str) -> String {
    String::new()
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
        assert_eq!("", part1(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "";

        assert_eq!("", part2(input));
    }
}
