use advent_of_code_2024::read_input;
use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<Vec<u8>>, Vec<(i64, i64)>) {
    let mut sections = input.trim().split("\n\n");
    let grid: Vec<Vec<u8>> = sections
        .next()
        .unwrap()
        .lines()
        .map(|l| l.trim().bytes().collect())
        .collect();
    let directions = sections
        .next()
        .unwrap()
        .lines()
        .join("")
        .trim()
        .chars()
        .map(|ch| match ch {
            '>' => (0, 1),
            '^' => (-1, 0),
            '<' => (0, -1),
            'v' => (1, 0),
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<(i64, i64)>>();
    (grid, directions)
}

fn get_robot_index(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'@' {
                return (i, j);
            }
        }
    }
    panic!("Robot not found")
}

// Simulate as-is. Simple casework. No ingenuity
fn part1(input: &str) -> String {
    let (mut grid, directions) = parse_input(input);
    let (mut ri, mut rj) = get_robot_index(&grid);
    grid[ri][rj] = b'.';

    for d in directions {
        let mut nri = ri as i64 + d.0;
        let mut nrj = rj as i64 + d.1;

        // try to find empty spot
        while grid[nri as usize][nrj as usize] != b'#' && grid[nri as usize][nrj as usize] != b'.' {
            nri += d.0;
            nrj += d.1
        }

        // if no empty spot found, continue
        if grid[nri as usize][nrj as usize] != b'.' {
            continue;
        }

        // else if empty spot found, move robot ahead
        ri = (ri as i64 + d.0) as usize;
        rj = (rj as i64 + d.1) as usize;
        grid[ri][rj] = b'.';

        // fill spaces with 'O'
        while nri as usize != ri || nrj as usize != rj {
            grid[nri as usize][nrj as usize] = b'O';
            nri -= d.0;
            nrj -= d.1;
        }
    }
    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'O' {
                ans += 100 * i + j;
            }
        }
    }
    ans.to_string()
}

fn check_vertical(
    grid: &mut Vec<Vec<u8>>,
    dir: i64,
    r: usize,
    c: usize,
    visited: &mut Vec<Vec<bool>>,
) -> Option<Vec<(usize, usize)>> {
    if grid[r][c] == b'#' {
        return None;
    } else if grid[r][c] == b'.' || visited[r][c] {
        return Some(vec![]);
    }
    println!("{r},{c}");
    visited[r][c] = true;
    let mut vec = Vec::new();
    if grid[r][c] == b'[' || grid[r][c] == b']' {
        let nc = if grid[r][c] == b'[' { c + 1 } else { c - 1 };
        if !visited[r][nc] {
            if let Some(v) = check_vertical(grid, dir, r, nc, visited) {
                vec.extend(v);
            } else {
                return None;
            }
        }
    }
    let nr = (r as i64 + dir) as usize;
    if let Some(v) = check_vertical(grid, dir, nr, c, visited) {
        vec.extend(v);
        vec.push((r, c));
        return Some(vec);
    }
    None
}

fn check_horizontal(
    grid: &mut Vec<Vec<u8>>,
    dir: i64,
    r: usize,
    c: usize,
) -> Option<Vec<(usize, usize)>> {
    if grid[r][c] == b'#' {
        return None;
    } else if grid[r][c] == b'.' {
        return Some(vec![]);
    }
    let nc = (c as i64 + dir) as usize;
    if let Some(mut v) = check_horizontal(grid, dir, r, nc) {
        v.push((r, c));
        return Some(v);
    }
    None
}

// Simulate as-is. Use DFS to move connected components (if possible).
fn part2(input: &str) -> String {
    let (grid, directions) = parse_input(input);

    // Convert input to be doubly width
    let mut grid: Vec<Vec<u8>> = grid
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|ch| match ch {
                    b'#' => vec![b'#', b'#'],
                    b'.' => vec![b'.', b'.'],
                    b'O' => vec![b'[', b']'],
                    b'@' => vec![b'@', b'.'],
                    _ => panic!("Wait what?"),
                })
                .collect()
        })
        .collect();

    // Initial robot index
    let (mut ri, mut rj) = get_robot_index(&grid);

    // Convert robot char to blank char to make life easier. We won't be checking for it in future.
    grid[ri][rj] = b'.';

    for d in directions {
        if d.0 != 0 {
            let nri = (ri as i64 + d.0) as usize;
            let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
            if let Some(v) = check_vertical(&mut grid, d.0, nri, rj, &mut visited) {
                for (r, c) in v {
                    grid[(r as i64 + d.0) as usize][c] = grid[r][c];
                    grid[r][c] = b'.';
                }
                ri = nri;
            }
        } else {
            let nrj = (rj as i64 + d.1) as usize;
            if let Some(v) = check_horizontal(&mut grid, d.1, ri, nrj) {
                for (r, c) in v {
                    grid[r][(c as i64 + d.1) as usize] = grid[r][c];
                    grid[r][c] = b'.';
                }
                rj = nrj;
            }
        }
    }

    let mut ans = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == b'[' {
                ans += 100 * i + j;
            }
        }
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day15")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        assert_eq!("2028", part1(input));
    }

    #[test]
    fn test_part1_2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        assert_eq!("10092", part1(input));
    }

    #[test]
    fn test_part2_1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!("9021", part2(input));
    }
}
