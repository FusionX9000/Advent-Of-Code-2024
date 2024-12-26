use advent_of_code_2024::read_input;

const D: [i32; 3] = [1, 0, -1];

fn get_word_1(vec: &[Vec<char>], i: usize, j: usize, di: i32, dj: i32) -> Option<String> {
    let max_r: i32 = vec.len().try_into().unwrap();
    let max_c: i32 = vec[0].len().try_into().unwrap();

    let ans: String = (0..4)
        .scan((i as i32, j as i32), |(r, c), _| {
            if *r < 0 || *r >= max_r || *c < 0 || *c >= max_c {
                None
            } else {
                let res = (*r as usize, *c as usize);
                *r += di;
                *c += dj;
                Some(res)
            }
        })
        .map(|(r, c)| vec[r][c])
        .collect();
    if ans.len() == 4 {
        Some(ans)
    } else {
        None
    }
}

fn count_1(vec: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            for di in D {
                for dj in D {
                    if di == 0 && dj == 0 {
                        continue;
                    }
                    if get_word_1(vec, i, j, di, dj) == Some("XMAS".to_string()) {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn part1(input: &str) -> String {
    count_1(&parse_input(input)).to_string()
}

fn count_2(vec: &[Vec<char>]) -> u32 {
    let mut count = 0;
    for i in 1..vec.len() - 1 {
        for j in 1..vec[0].len() - 1 {
            if vec[i][j] == 'A' {
                let ldiag = format!("{}{}{}", vec[i - 1][j - 1], 'A', vec[i + 1][j + 1]);
                let rdiag = format!("{}{}{}", vec[i - 1][j + 1], 'A', vec[i + 1][j - 1]);
                if (ldiag == "SAM" || ldiag == "MAS") && (rdiag == "SAM" || rdiag == "MAS") {
                    count += 1;
                }
            }
        }
    }
    count
}

fn part2(input: &str) -> String {
    count_2(&parse_input(input)).to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day4")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", part1(input));
    }

    #[test]
    fn test_part2() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";

        assert_eq!("9", part2(input));
    }
}
