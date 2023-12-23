use std::collections::HashMap;

use itertools::Itertools;

const NEIGHBORS: &[(isize, isize)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];

type NodeDistance = (usize, usize, usize);

fn dfs(
    graph: &HashMap<(usize, usize), Vec<NodeDistance>>,
    seen: &mut Vec<Vec<bool>>,
    (r, c): (usize, usize),
) -> Option<usize> {
    if r == seen.len() - 1 {
        return Some(0);
    }
    let mut max_dist = None;
    for &(rr, cc, d) in &graph[&(r, c)] {
        if !seen[rr][cc] {
            seen[rr][cc] = true;
            if let Some(dist) = dfs(graph, seen, (rr, cc)) {
                max_dist = Some(max_dist.unwrap_or(0).max(d + dist))
            }
            seen[rr][cc] = false;
        }
    }
    max_dist
}

fn solve(input: &str, part2: bool) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut graph = HashMap::new();

    for (row, column) in (0..grid.len()).cartesian_product(0..grid[0].len()) {
        let neighbors = match grid[row][column] {
            '#' => continue,
            _ if part2 => NEIGHBORS,
            '.' => NEIGHBORS,
            '^' => &NEIGHBORS[0..][..1],
            '>' => &NEIGHBORS[1..][..1],
            'v' => &NEIGHBORS[2..][..1],
            '<' => &NEIGHBORS[3..][..1],
            _ => unreachable!(),
        };
        let entry: &mut Vec<_> = graph.entry((row, column)).or_default();
        for (dr, dc) in neighbors {
            let neighbor_row = (row as isize + dr) as usize;
            let neighbor_col = (column as isize + dc) as usize;
            let Some(&tile) = grid.get(neighbor_row).and_then(|row| row.get(neighbor_col)) else {
                continue;
            };
            if tile != '#' {
                entry.push((neighbor_row, neighbor_col, 1));
            }
        }
    }
    while let Some((&(row, col), _)) = graph.iter().find(|(_, n)| n.len() == 2) {
        let neighbors = graph.remove(&(row, col)).unwrap();
        let (r1, c1, d1) = neighbors[0];
        let (r2, c2, d2) = neighbors[1];
        let neighbor_1 = graph.get_mut(&(r1, c1)).unwrap();
        if let Some(i) = neighbor_1
            .iter()
            .position(|&(rr, cc, _)| (rr, cc) == (row, col))
        {
            neighbor_1[i] = (r2, c2, d1 + d2);
        }
        let neighbor_2 = graph.get_mut(&(r2, c2)).unwrap();
        if let Some(i) = neighbor_2
            .iter()
            .position(|&(rr, cc, _)| (rr, cc) == (row, col))
        {
            neighbor_2[i] = (r1, c1, d1 + d2);
        }
    }

    println!("Graph size: {}", graph.len());

    dfs(
        &graph,
        &mut vec![vec![false; grid[0].len()]; grid.len()],
        (0, 1),
    )
    .unwrap()
}

pub fn part1(input: &str) -> usize{
    let distance = solve(input, false);

    println!("{distance}");
    distance
}

pub fn part2(input: &str)  -> usize{
    let distance = solve(input, true);

    println!("{distance}");
    distance
}

#[test]
fn test_part1() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(part1(input), 94);
}

#[test]
fn test_part2() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(part2(input), 154);
}