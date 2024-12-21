use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.trim().bytes().collect()).collect()
}

const D: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn traverse_1(input: &[Vec<u8>], r: usize, c: usize, visited: &mut Vec<Vec<bool>>) -> (u64, u64) {
    let in_bounds = |i, j| i >= 0 && j >= 0 && i < input.len() as i32 && j < input[0].len() as i32;

    visited[r][c] = true;
    let mut area = 1;
    let mut per = 0;
    for (di, dj) in D {
        let i = r as i32 + di;
        let j = c as i32 + dj;

        if !in_bounds(i, j) || input[i as usize][j as usize] != input[r][c] {
            per += 1;
        } else if !visited[i as usize][j as usize] {
            let (narea, np) = traverse_1(input, i as usize, j as usize, visited);
            area += narea;
            per += np;
        }
    }
    (area, per)
}

fn part1(input: &str) -> String {
    let input = parse_input(input);

    let mut ans = 0u64;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    for (r, c) in (0..input.len()).cartesian_product(0..input[0].len()) {
        if !visited[r][c] {
            let (area, perimeter) = traverse_1(&input, r, c, &mut visited);
            ans += area * perimeter;
        }
    }
    ans.to_string()
}

fn traverse_2(input: &[Vec<u8>], r: usize, c: usize, visited: &mut Vec<Vec<bool>>) -> (u64, u64) {
    let in_bounds = |i, j| i >= 0 && j >= 0 && i < input.len() as i32 && j < input[0].len() as i32;
    let matches = |(di, dj): (i32, i32)| -> bool {
        let i = r as i32 + di;
        let j = c as i32 + dj;
        in_bounds(i, j) && input[i as usize][j as usize] == input[r][c]
    };

    visited[r][c] = true;

    let mut area = 1;
    let mut sides = 0;

    for d in 0..4 {
        let d1 = D[d];
        let d2 = D[(d + 1) % 4];

        if !matches(d1) && !matches(d2) {
            sides += 1;
        }
        if matches(d1) && matches(d2) && !matches((d1.0 + d2.0, d1.1 + d2.1)) {
            sides += 1;
        }

        let i = r as i32 + D[d].0;
        let j = c as i32 + D[d].1;

        if matches(D[d]) && !visited[i as usize][j as usize] {
            let (narea, nsides) = traverse_2(input, i as usize, j as usize, visited);
            area += narea;
            sides += nsides;
        }
    }
    (area, sides)
}

// "Inspired" by errichto's solution. Count convex and concave corners.
fn part2(input: &str) -> String {
    let input = parse_input(input);

    let mut ans = 0u64;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; input[0].len()]; input.len()];

    for (r, c) in (0..input.len()).cartesian_product(0..input[0].len()) {
        if !visited[r][c] {
            let (area, sides) = traverse_2(&input, r, c, &mut visited);
            ans += area * sides;
        }
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day12")?;
    let before = std::time::Instant::now();
    println!("{}", part1(&input));
    println!("Time taken: {:?}", std::time::Instant::now() - before);
    let before = std::time::Instant::now();
    println!("{}", part2(&input));
    println!("Time taken: {:?}", std::time::Instant::now() - before);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", part1(input));
    }

    #[test]
    fn test_part2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        assert_eq!("1206", part2(input));
    }
}
