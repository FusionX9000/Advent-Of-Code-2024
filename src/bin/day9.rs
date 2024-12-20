use std::{
    cmp::{Ordering, Reverse},
    collections::BinaryHeap,
};

use advent_of_code_2024::read_input;

fn parse_input(input: &str) -> Vec<i64> {
    input.as_bytes().iter().map(|n| (n - b'0') as i64).collect()
}

fn asum(a_1: i64, a_n: i64, n: i64) -> i64 {
    (n as i64) * (a_1 as i64 + a_n as i64) / 2
}

// O(N)
fn part1(input: &str) -> String {
    let mut input = parse_input(input);

    let mut ans = 0;
    let mut free_idx = 0;
    let mut i = 0;
    let mut j = input.len() - 1;

    while i <= j {
        if i % 2 == 0 {
            let blocks = input[i];
            let val = asum(free_idx, free_idx + blocks - 1, blocks) * (i / 2) as i64;

            ans += val;
            free_idx += blocks;
            i += 1;
        } else {
            let blocks = std::cmp::min(input[j], input[i]);
            let val = asum(free_idx, free_idx + blocks - 1, blocks) * (j / 2) as i64;

            ans += val;
            free_idx += blocks;
            input[i] -= blocks;
            input[j] -= blocks;

            if input[i] <= 0 {
                i += 1;
            }
            if input[j] <= 0 {
                j -= 2;
            }
        }
    }

    ans.to_string()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Block {
    id: i64,
    pos: i64,
    size: i64,
}

impl Ord for Block {
    fn cmp(&self, other: &Block) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Block) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part2_brute(input: &str) -> String {
    let input = parse_input(input);
    let mut blocks: Vec<Block> = Vec::new();
    let mut pos = 0;

    for (i, &b) in input.iter().enumerate() {
        if i % 2 == 0 {
            blocks.push(Block {
                id: (i as i64) / 2,
                pos,
                size: b,
            });
        }
        pos += b;
    }

    let mut ans = 0;

    let mut final_blocks: Vec<Block> = Vec::new();
    while blocks.len() > 1 {
        // println!("{:?}", blocks);
        let &block = blocks.last().unwrap();
        let mut npos = block.pos;
        let mut flag = false;
        for i in 1..blocks.len() {
            let a = blocks[i - 1].pos + blocks[i - 1].size;
            let b = blocks[i].pos;
            let free_space = b - a;
            if free_space >= block.size {
                blocks.insert(
                    i,
                    Block {
                        id: block.id,
                        pos: a,
                        size: block.size,
                    },
                );
                npos = a;
                flag = true;
                blocks.pop();
                break;
            }
        }
        // println!("{:?}", flag);
        if !flag {
            blocks.pop();
            final_blocks.push(Block {
                id: block.id,
                pos: npos,
                size: block.size,
            });
            for j in npos..(npos + block.size) {
                ans += block.id * j;
            }
        } else {
        }
    }
    ans.to_string()
}

// O(N log N)
fn part2(input: &str) -> String {
    let input = parse_input(input);

    let mut pqs: Vec<BinaryHeap<Reverse<Block>>> = vec![BinaryHeap::new(); 11];
    let blocks: Vec<Block> = input
        .clone()
        .iter()
        .scan(0, |pos, &size| {
            let opos = *pos;
            *pos += size;
            Some((opos, size))
        })
        .map(|(pos, size)| Block { id: 0, pos, size })
        .collect();

    for i in (1..input.len()).step_by(2) {
        let ublock = blocks[i];
        pqs[ublock.size as usize].push(Reverse(ublock));
    }

    let mut ans = 0;

    let mut final_blocks: Vec<Block> = Vec::new();

    for (i, ublock) in blocks.into_iter().enumerate().step_by(2).rev() {
        if ublock.size == 0 {
            continue;
        }
        let id = (i / 2) as i64;
        let mut new_pos = ublock.pos;

        let mut lowest_heap_idx: Option<usize> = None;
        for j in (ublock.size as usize)..=10 {
            let pq = &mut pqs[j];
            while let Some(Reverse(fblock)) = pq.peek() {
                if ublock.pos < fblock.pos {
                    pq.pop();
                } else {
                    break;
                }
            }
            let pq = &pqs[j];
            if let Some(Reverse(free_block)) = pq.peek() {
                if lowest_heap_idx.is_none() {
                    lowest_heap_idx = Some(j);
                } else {
                    if let Some(Reverse(lowest_pos_block)) = pqs[lowest_heap_idx.unwrap()].peek() {
                        if free_block.pos < lowest_pos_block.pos {
                            lowest_heap_idx = Some(j);
                        }
                    }
                }
            }
        }
        if let Some(idx) = lowest_heap_idx {
            let free_block = pqs[idx].pop().unwrap().0;
            new_pos = free_block.pos;

            let rem_size = free_block.size - ublock.size;
            if rem_size > 0 {
                pqs[rem_size as usize].push(Reverse(Block {
                    id,
                    pos: free_block.pos + ublock.size,
                    size: rem_size,
                }));
            }
        }

        final_blocks.push(Block {
            id,
            pos: new_pos,
            size: ublock.size,
        });

        let a_i = new_pos;
        let a_n = new_pos + ublock.size - 1;
        let n = ublock.size;
        let val = asum(a_i, a_n, n) * id;
        ans += val;
    }
    ans.to_string()
}

fn main() -> Result<(), std::io::Error> {
    let input = read_input("day9")?;
    println!("{}", part1(&input));

    let before = std::time::Instant::now();
    println!("{}", part2(&input));
    println!("Time: {:?}", before.elapsed());
    let before = std::time::Instant::now();
    println!("{}", part2_brute(&input));
    println!("Time: {:?}", before.elapsed());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        assert_eq!("1928", part1(input));
    }

    #[test]
    fn test_part2() {
        //01..
        let input = "2333133121414131402";

        assert_eq!("2858", part2(input));
    }

    #[test]
    fn test_part2_1() {
        let input = "2333133121414131402";

        assert_eq!("2858", part2_brute(input));
    }
}
