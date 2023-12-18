use std::collections::{BinaryHeap, HashMap};

fn dijkstra(grid: &[&[u8]], minstep: isize, maxstep: isize) -> i64 {
    let mut dists = HashMap::new();
    let mut q = BinaryHeap::from_iter([(0, (0, 0, (0, 0)))]);
    while let Some((cost, (r, c, d))) = q.pop() {
        if (r, c) == (grid.len() - 1, grid[0].len() - 1) {
            return -cost;
        }
        if dists.get(&(r, c, d)).is_some_and(|&c| -cost > c) {
            continue;
        }
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if d == (dr, dc) || d == (-dr, -dc) {
                continue;
            }
            let mut next_cost = -cost;
            for dist in 1..=maxstep {
                let rr = (r as isize + dr * dist) as usize;
                let cc = (c as isize + dc * dist) as usize;
                if rr >= grid.len() || cc >= grid[0].len() {
                    continue;
                }
                next_cost += (grid[rr][cc] - b'0') as i64;
                let key = (rr, cc, (dr, dc));
                if minstep <= dist && next_cost < *dists.get(&key).unwrap_or(&10000000) {
                    dists.insert(key, next_cost);
                    q.push((-next_cost, key));
                }
            }
        }
    }
    unreachable!()
}

pub fn part1(input: &str) -> i64 {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let total_cost = dijkstra(&grid, 1, 3);

    println!("{total_cost}");
    total_cost
}

pub fn part2(input: &str) -> i64 {
    let grid = input.lines().map(str::as_bytes).collect::<Vec<_>>();
    let total_cost = dijkstra(&grid, 4, 10);

    println!("{total_cost}");
    total_cost
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 94);

        let input = include_str!("../input_bad.txt");
        assert_eq!(part2(input), 71);
    }
}