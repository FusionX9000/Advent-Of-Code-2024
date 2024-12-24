use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut sections = input.split("\n\n");
    let patterns = sections.next().unwrap().split(", ").collect();
    let designs = sections.next().unwrap().lines().collect();
    (patterns, designs)
}

fn min_patterns_for_design(
    design: &str,
    patterns: &[&str],
    i: usize,
    cache: &mut [Option<u64>],
) -> u64 {
    if i == design.len() {
        return 1;
    }
    if let Some(v) = cache[i] {
        return v;
    }

    let mut ans = 0;
    for idx in i..design.len() {
        let ndesign = &design[i..=idx];
        if patterns.contains(&ndesign) {
            ans += min_patterns_for_design(design, patterns, idx + 1, cache);
        }
    }
    cache[i] = Some(ans);
    ans
}

fn part1(input: &str) -> String {
    let (patterns, designs) = parse_input(input);
    let mut ans = 0;
    for design in designs {
        let mut cache = vec![None; design.len()];
        if min_patterns_for_design(design, &patterns, 0, &mut cache) > 0 {
            ans += 1;
        }
    }
    ans.to_string()
}

fn part2(input: &str) -> String {
    let (patterns, designs) = parse_input(input);
    let mut ans = 0;
    for design in designs {
        let mut cache = vec![None; design.len()];
        ans += min_patterns_for_design(design, &patterns, 0, &mut cache);
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day19")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part1() {
        assert_eq!("6", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("16", part2(INPUT));
    }
}
