use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

type Point = (usize, usize);

const NUMERIC_KEYPAD: [[u8; 3]; 4] = [
    [b'7', b'8', b'9'],
    [b'4', b'5', b'6'],
    [b'1', b'2', b'3'],
    [b'#', b'0', b'A'],
];

const DIRECTIONAL_KEYPAD: [[u8; 3]; 2] = [[b'#', b'^', b'A'], [b'<', b'v', b'>']];

#[derive(Clone, Copy, Debug)]
enum Keypad {
    Directional,
    Numeric,
}

impl Keypad {
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
            Keypad::Directional => &DIRECTIONAL_KEYPAD,
            Keypad::Numeric => &NUMERIC_KEYPAD,
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
}

fn dfs(pos: Point, keypad: Keypad, times: usize, code: &[u8], running: &mut Vec<u8>) -> usize {
    match keypad {
        Keypad::Numeric => {
            if code.is_empty() {
                return dfs(
                    Keypad::Directional.start_pos(),
                    Keypad::Directional,
                    2,
                    running,
                    &mut Vec::new(),
                );
            }
        }
        Keypad::Directional => {
            if times == 0 {
                return code.len();
            }
            if code.is_empty() {
                return dfs(
                    Keypad::Directional.start_pos(),
                    Keypad::Directional,
                    times - 1,
                    running,
                    &mut Vec::new(),
                );
            }
        }
    }

    let mut ans: usize = usize::MAX;
    if pos == keypad.invalid_pos() {
        return ans;
    }

    let next_pos = keypad.position(code[0]);
    if pos == next_pos {
        running.push(b'A');
        let ans = dfs(pos, keypad, times, &code[1..], running);
        running.pop();
        return ans;
    }

    let r_diff = next_pos.0 as i64 - pos.0 as i64;
    let c_diff = next_pos.1 as i64 - pos.1 as i64;

    let (ch, npos) = match r_diff {
        0 => (b'#', (0, 0)),
        ..0 => (b'^', (pos.0 - 1, pos.1)),
        1.. => (b'v', (pos.0 + 1, pos.1)),
    };
    if ch != b'#' {
        running.push(ch);
        ans = dfs(npos, keypad, times, code, running);
        running.pop();
    }

    let (ch, npos) = match c_diff {
        0 => (b'#', (0, 0)),
        ..0 => (b'<', (pos.0, pos.1 - 1)),
        1.. => (b'>', (pos.0, pos.1 + 1)),
    };
    if ch != b'#' {
        running.push(ch);
        ans = ans.min(dfs(npos, keypad, times, code, running));
        running.pop();
    }

    ans
}

fn solve(code: &[u8]) -> usize {
    let keypad = Keypad::Numeric;
    dfs(keypad.start_pos(), keypad, 0, code, &mut Vec::new())
}

fn part1(input: &str) -> String {
    let codes = parse_input(input);
    let mut ans = 0usize;
    for code in codes {
        let val = solve(&code);
        let code_str = code.iter().map(|x| *x as char).collect::<String>();
        let code_int: usize = code_str[..code_str.len() - 1].parse().unwrap();
        ans += val * code_int;
    }
    ans.to_string()
}

fn part2(input: &str) -> String {
    String::new()
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
