fn mirrors(ns: &[u32], i: usize, flex: u32) -> bool {
    (0..i)
        .rev()
        .zip(i..ns.len())
        .map(|(a, b)| (ns[a] ^ ns[b]).count_ones())
        .sum::<u32>()
        == flex
}

fn summarize(grid: &str, flex: u32) -> Option<usize> {
    let mut rows = Vec::new();
    let mut cols = Vec::new();
    for line in grid.lines() {
        cols.resize(line.len(), 0);
        let mut row = 0;
        for (c, v) in line.bytes().enumerate() {
            cols[c] = (cols[c] << 1) | ((v == b'#') as u32);
            row = (row << 1) | ((v == b'#') as u32);
        }
        rows.push(row);
    }
    for c in 1..cols.len() {
        if mirrors(&cols, c, flex) {
            return Some(c);
        }
    }
    for r in 1..rows.len() {
        if mirrors(&rows, r, flex) {
            return Some(100 * r);
        }
    }
    None
}

fn solve(input: &str, flex: u32) -> usize {
    input
        .split("\n\n")
        .map(|grid| summarize(grid, flex).unwrap())
        .sum()
}

pub fn part1(input: &str) -> usize {
    let result = solve(input, 0);
    println!("{result}");
    result
}

pub fn part2(input: &str) -> usize {
    let result = solve(input, 1);
    println!("{result}");
    result
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 405);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 400);
    }
}