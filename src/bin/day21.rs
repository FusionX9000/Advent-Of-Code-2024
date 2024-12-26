use std::{collections::HashMap, iter::repeat};

use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

type Point = (usize, usize);

#[derive(Clone, Copy, Debug)]
enum Keypad {
    Directional,
    Numeric,
}

impl Keypad {
    const NUMERIC_KEYPAD: [[u8; 3]; 4] = [
        [b'7', b'8', b'9'],
        [b'4', b'5', b'6'],
        [b'1', b'2', b'3'],
        [b'#', b'0', b'A'],
    ];

    const DIRECTIONAL_KEYPAD: [[u8; 3]; 2] = [[b'#', b'^', b'A'], [b'<', b'v', b'>']];

    fn start_pos(&self) -> Point {
        match self {
            Keypad::Directional => (0, 2),
            Keypad::Numeric => (3, 2),
        }
    }
    fn invalid_pos(&self) -> Point {
        match self {
            Keypad::Directional => (0, 0),
            Keypad::Numeric => (3, 0),
        }
    }
    fn array(&self) -> &[[u8; 3]] {
        match self {
            Keypad::Directional => &Self::DIRECTIONAL_KEYPAD,
            Keypad::Numeric => &Self::NUMERIC_KEYPAD,
        }
    }
    fn position(&self, search_val: u8) -> Point {
        let keypad = self.array();
        for (i, row) in keypad.iter().enumerate() {
            for (j, ch) in row.iter().enumerate() {
                if search_val == *ch {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }
    fn get_dir(ch: u8) -> (i64, i64) {
        let chars = [b'>', b'<', b'^', b'v'];
        let dirs = [(0, 1), (0, -1), (-1, 0), (1, 0)];
        let i = chars.iter().position(|x| *x == ch).unwrap();
        dirs[i]
    }
}

fn gen_path(start: Point, end: Point, keypad: Keypad) -> Vec<Vec<u8>> {
    fn is_invalid_code(code: &[u8], start: Point, keypad: Keypad) -> bool {
        let mut pos = start;
        for &next_ch in code {
            let dir = Keypad::get_dir(next_ch);
            pos = (
                (pos.0 as i64 + dir.0) as usize,
                (pos.1 as i64 + dir.1) as usize,
            );
            if keypad.invalid_pos() == pos {
                return false;
            }
        }
        true
    }

    let (r_diff, c_diff) = (end.0 as i64 - start.0 as i64, end.1 as i64 - start.1 as i64);

    let mut chars = Vec::new();

    chars.extend(repeat(b'v').take(r_diff.max(0) as usize));
    chars.extend(repeat(b'^').take((-r_diff).max(0) as usize));
    chars.extend(repeat(b'>').take(c_diff.max(0) as usize));
    chars.extend(repeat(b'<').take((-c_diff).max(0) as usize));

    let len = chars.len();
    chars
        .into_iter()
        .permutations(len)
        .unique()
        .filter(|perm| is_invalid_code(perm, start, keypad))
        .map(|mut perm| {
            perm.push(b'A');
            perm
        })
        .collect::<Vec<Vec<u8>>>()
}

fn u8_to_string(v: &[u8]) -> String {
    v.iter().map(|v| *v as char).collect()
}

fn solve(keypad: Keypad, code: &[u8], times: u64, cache: &mut HashMap<(String, u64), u64>) -> u64 {
    let mut times = times;
    if matches!(keypad, Keypad::Directional) {
        if times == 0 {
            return code.len() as u64;
        }
        times -= 1;
    }

    let hash = (u8_to_string(code), times);
    if let Some(v) = cache.get(&hash) {
        return *v;
    }

    let mut pos = keypad.start_pos();
    let mut cost = 0;

    for ch_to in code {
        let to = keypad.position(*ch_to);
        let mut min_cost = u64::MAX;
        for path in gen_path(pos, to, keypad) {
            min_cost = min_cost.min(solve(Keypad::Directional, &path, times, cache))
        }
        pos = to;
        assert_ne!(min_cost, u64::MAX);
        cost += min_cost;
    }

    cache.insert(hash, cost);
    cost
}

fn part1(input: &str) -> String {
    let codes = parse_input(input);
    let mut ans = 0usize;
    let mut cache = HashMap::new();
    for code in codes {
        let val = solve(Keypad::Numeric, &code, 2, &mut cache);
        let code_str = code.iter().map(|x| *x as char).collect::<String>();
        let code_int: usize = code_str[..code_str.len() - 1].parse().unwrap();
        ans += val as usize * code_int;
    }
    ans.to_string()
}

fn part2(input: &str) -> String {
    let codes = parse_input(input);
    let mut ans = 0usize;
    let mut cache = HashMap::new();
    for code in codes {
        let val = solve(Keypad::Numeric, &code, 25, &mut cache);
        let code_str = code.iter().map(|x| *x as char).collect::<String>();
        let code_int: usize = code_str[..code_str.len() - 1].parse().unwrap();
        ans += val as usize * code_int;
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day21")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", part1(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "";

        assert_eq!("", part2(input));
    }
}
