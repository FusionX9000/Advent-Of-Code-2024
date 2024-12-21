use advent_of_code_2024::read_input;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct ClawMachine {
    a: Coord,
    b: Coord,
    p: Coord,
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let parse_button = |line| {
        let pattern = "Button [A,B]: X\\+([0-9]+), Y\\+([0-9]+)";
        let re = regex::Regex::new(pattern).unwrap();
        let caps = re.captures(line).unwrap();
        Coord {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
        }
    };
    let parse_prize = |line| {
        let pattern = "Prize: X=([0-9]+), Y=([0-9]+)";
        let re = regex::Regex::new(pattern).unwrap();
        let caps = re.captures(line).unwrap();
        Coord {
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
        }
    };

    input
        .trim()
        .split("\n\n")
        .map(|cm| {
            let mut itr = cm.trim().split("\n");
            ClawMachine {
                a: parse_button(itr.next().unwrap()),
                b: parse_button(itr.next().unwrap()),
                p: parse_prize(itr.next().unwrap()),
            }
        })
        .collect()
}

// Find intersection of straight lines formed by
// alpha * A_vector + beta * B_vector = Prize_vector

fn solve_machine(m: &ClawMachine) -> i64 {
    let beta = (m.a.x * m.p.y - m.a.y * m.p.x) / (m.a.x * m.b.y - m.a.y * m.b.x);
    let alpha = (m.p.x - m.b.x * beta) / m.a.x;

    // Non-integer solutions will fail
    if m.a.x * alpha + m.b.x * beta != m.p.x || m.a.y * alpha + m.b.y * beta != m.p.y {
        return 0;
    }
    3 * alpha + beta
}

fn part1(input: &str) -> String {
    let input: Vec<ClawMachine> = parse_input(input);
    input
        .iter()
        .map(|m| solve_machine(m))
        .sum::<i64>()
        .to_string()
}

fn part2(input: &str) -> String {
    let input: Vec<ClawMachine> = parse_input(input);
    input
        .into_iter()
        .map(|m| ClawMachine {
            p: Coord {
                x: m.p.x + 10_000_000_000_000,
                y: m.p.y + 10_000_000_000_000,
            },
            ..m
        })
        .map(|m| solve_machine(&m))
        .sum::<i64>()
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day13")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
    #[test]
    fn test_part1() {
        assert_eq!("480", part1(INPUT));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!("", part2(INPUT));
    }
}
