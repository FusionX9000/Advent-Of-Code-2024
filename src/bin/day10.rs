use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.trim().bytes().map(|b| (b - b'0') as u32).collect())
        .collect()
}

const D: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn trailheads(input: &[Vec<u32>], r: usize, c: usize, cache: &mut [i32]) -> u32 {
    let mut ans = 0;
    let hash = r * input[0].len() + c;
    if input[r][c] == 9 {
        ans = 1;
    } else if cache[hash] != -1 {
        return cache[hash] as u32;
    } else {
        for (di, dj) in D {
            let i = r as i32 + di;
            let j = c as i32 + dj;
            if (0..input.len() as i32).contains(&i) && (0..input[0].len() as i32).contains(&j) {
                if input[i as usize][j as usize] == input[r][c] + 1 {
                    ans += trailheads(input, i as usize, j as usize, cache)
                }
            }
        }
    }
    cache[hash] = ans as i32;
    ans
}

// TC for part1 can be improved with HashSet by a factor of N, but I'm simply re-using the code from part2
fn part1(input: &str) -> String {
    let input = parse_input(input);
    let h = input.len();
    let w = input[0].len();
    let mut ans = 0;
    for (r, c) in (0..h)
        .cartesian_product(0..w)
        .filter(|(i, j)| input[*i][*j] == 0)
    {
        let mut cache = vec![-1; h * w];
        trailheads(&input, r, c, &mut cache);

        for (ni, nj) in (0..h).cartesian_product(0..w) {
            let hash = ni * w + nj;
            if input[ni][nj] == 9 && cache[hash] > 0 {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

fn part2(input: &str) -> String {
    let input = parse_input(input);
    let h = input.len();
    let w = input[0].len();
    let mut ans = 0;
    for (r, c) in (0..h)
        .cartesian_product(0..w)
        .filter(|(i, j)| input[*i][*j] == 0)
    {
        let mut cache = vec![-1; h * w];
        let val = trailheads(&input, r, c, &mut cache);
        ans += val;
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day10")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part1() {
        assert_eq!("36", part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!("81", part2(INPUT));
    }
}
