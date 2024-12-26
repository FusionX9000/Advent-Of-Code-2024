use advent_of_code_2024::read_input;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

#[derive(Debug)]
struct Robot {
    p: Coord,
    v: Coord,
}

const WIDTH: usize = 101;
const HEIGHT: usize = 103;

const WIDTH_TEST: usize = 11;
const HEIGHT_TEST: usize = 7;

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .map(|line| {
            let pattern = "p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)";
            let re = regex::Regex::new(pattern).unwrap();
            let caps = re
                .captures(line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|cap| cap.unwrap().as_str().parse().unwrap())
                .collect::<Vec<i64>>();
            Robot {
                p: Coord {
                    x: caps[0],
                    y: caps[1],
                },
                v: Coord {
                    x: caps[2],
                    y: caps[3],
                },
            }
        })
        .collect::<Vec<Robot>>()
}

fn part1(input: &str) -> String {
    let mut width = WIDTH as i64;
    let mut height = HEIGHT as i64;

    if cfg!(test) {
        width = WIDTH_TEST as i64;
        height = HEIGHT_TEST as i64;
    }

    let input = parse_input(input);
    let vals = input
        .iter()
        .map(|robot| {
            (
                (robot.p.x + 100 * robot.v.x).rem_euclid(width),
                (robot.p.y + 100 * robot.v.y).rem_euclid(height),
            )
        })
        .map(|(x, y)| {
            let mid_x = width / 2;
            let mid_y = height / 2;
            let mut ans = [[0, 0], [0, 0]];
            if x != mid_x && y != mid_y {
                ans[(x < mid_x) as usize][(y < mid_y) as usize] = 1;
            }
            ans
        })
        .reduce(|acc, e| {
            [
                [acc[0][0] + e[0][0], acc[0][1] + e[0][1]],
                [acc[1][0] + e[1][0], acc[1][1] + e[1][1]],
            ]
        })
        .unwrap();
    let ans = vals[0][0] * vals[0][1] * vals[1][0] * vals[1][1];
    ans.to_string()
}

fn calc_variance(v: &[(i64, i64)], in_x: bool) -> f64 {
    let n = v.len();
    let mut total = 0;
    for (x, y) in v.iter() {
        if in_x {
            total += x;
        } else {
            total += y;
        }
    }
    let mean = (total as f64) / n as f64;
    let mut num = 0f64;
    for (x, y) in v.iter() {
        if in_x {
            num += (*x as f64 - mean) * (*x as f64 - mean);
        } else {
            num += (*y as f64 - mean) * (*y as f64 - mean);
        }
    }
    num / n as f64
}

fn part2(input: &str) -> String {
    let width = WIDTH as i64;
    let height = HEIGHT as i64;

    let input = parse_input(input);

    (1..10000)
        .map(|i| {
            let vals = input
                .iter()
                .map(|robot| {
                    (
                        (robot.p.x + i * robot.v.x).rem_euclid(width),
                        (robot.p.y + i * robot.v.y).rem_euclid(height),
                    )
                })
                .collect::<Vec<(i64, i64)>>();

            let var = calc_variance(&vals, true) * calc_variance(&vals, false);
            (var, i)
        })
        .min_by(|(var1, i1), (var2, i2)| var1.partial_cmp(var2).unwrap().then_with(|| i1.cmp(i2)))
        .unwrap()
        .1
        .to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day14")?;
    println!("{}", part1(&input));

    println!("{}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("12", part1(input));
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = "";

        assert_eq!("", part2(input));
    }
}
