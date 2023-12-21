use std::collections::{HashMap, VecDeque};

use memoize::memoize;

const MOVES: &[(i32, i32)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

#[memoize]
fn solve(steps: usize, v: usize, max_steps: usize, rows: usize) -> usize {
    if steps > max_steps {
        return 0;
    }
    let amt = (max_steps - steps) / rows;
    let mut ret = 0;

    for x in 1..amt + 1 {
        if steps + rows * x <= max_steps && (steps + rows * x) % 2 == (max_steps % 2) {
            ret += if v == 2 { x + 1 } else { 1 };
        }
    }

    ret
}

fn find_reachable(
    row: i32,
    col: i32,
    max: (i32, i32),
    grid: &[Vec<char>],
) -> HashMap<(i32, i32, i32, i32), usize> {
    let mut reachable = HashMap::new();
    let mut queue = VecDeque::from([(0_i32, 0_i32, row, col, 0)]);

    while let Some((mut tr, mut tc, mut r, mut c, steps)) = queue.pop_front() {
        if r < 0 {
            tr -= 1;
            r += max.0;
        }
        if r >= max.0 {
            tr += 1;
            r -= max.0;
        }
        if c < 0 {
            tc -= 1;
            c += max.1;
        }
        if c >= max.1 {
            tc += 1;
            c -= max.1;
        }
        if !(0 <= r && r < max.0 && 0 <= c && c < max.1 && grid[r as usize][c as usize] != '#') {
            continue;
        }
        if reachable.contains_key(&(tr, tc, r, c)) {
            continue;
        }
        if tr.abs() > 4 || tc.abs() > 4 {
            continue;
        }
        reachable.insert((tr, tc, r, c), steps);
        for (dr, dc) in MOVES {
            queue.push_back((tr, tc, r + dr, c + dc, steps + 1))
        }
    }

    reachable
}

fn solve_main(input: &str, max_steps: usize, part1: bool) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut start = (0, 0);

    for (idr, row) in grid.iter().enumerate() {
        for (idc, c) in row.iter().enumerate() {
            if *c == 'S' {
                start = (idr as i32, idc as i32);
            }
        }
    }

    let reachable = find_reachable(start.0, start.1, (rows as i32, cols as i32), &grid);
    let mut ans = 0;

    assert_eq!(rows, cols);
    let opt: Vec<i32> = (-3..=3).collect();
    let minmax = [opt.first().unwrap(), opt.last().unwrap()];

    for r in 0..rows as i32 {
        for c in 0..cols as i32 {
            if reachable.contains_key(&(0, 0, r, c)) {
                for tr in &opt {
                    for tc in &opt {
                        if part1 && (*tr != 0 || *tc != 0) {
                            continue;
                        }
                        let steps = reachable.get(&(*tr, *tc, r, c)).unwrap();
                        if steps % 2 == max_steps % 2 && *steps <= max_steps {
                            ans += 1;
                        }
                        if minmax.contains(&tr)
                            && minmax.contains(&tc)
                        {
                            ans += solve(*steps, 2, max_steps, rows);
                        } else if minmax.contains(&tr)
                            || minmax.contains(&tc)
                        {
                            ans += solve(*steps, 1, max_steps, rows);
                        }
                    }
                }
            }
        }
    }

    ans
}

pub fn part1(input: &str) {
    let ans = solve_main(input, 64, true);

    println!("{ans}")
}

pub fn part2(input: &str) {
    let ans = solve_main(input, 26501365, false);

    println!("{ans}")
}

#[test]
fn test_solve_main() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(solve_main(input, 1, true), 2);
    assert_eq!(solve_main(input, 2, true), 4);
    assert_eq!(solve_main(input, 3, true), 6);
    assert_eq!(solve_main(input, 6, true), 16);

    assert_eq!(solve_main(input, 6, false), 16);
    assert_eq!(solve_main(input, 10, false), 50);
    assert_eq!(solve_main(input, 50, false), 1594);
    assert_eq!(solve_main(input, 100, false), 6536);
    assert_eq!(solve_main(input, 500, false), 167004);
    assert_eq!(solve_main(input, 1000, false), 668697);
    assert_eq!(solve_main(input, 5000, false), 16733044);
}
