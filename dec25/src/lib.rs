use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn component_size(graph: &HashMap<&str, HashSet<&str>>, a: &str) -> usize {
    let (mut seen, mut s) = (HashSet::new(), vec![a]);
    while let Some(x) = s.pop() {
        if !seen.insert(x) {
            continue;
        }
        s.extend(&graph[x]);
    }
    seen.len()
}

fn solve_part1(input: &str, remove_edges: [(&str, &str); 3], a: &str) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut edges = HashSet::new();

    for l in input.split('\n') {
        let (a, rest) = l.split_once(": ").unwrap();
        for b in rest.split(' ') {
            graph.entry(a).or_default().insert(b);
            graph.entry(b).or_default().insert(a);
            edges.insert(if a < b { (a, b) } else { (b, a) });
        }
    }

    let mut dot = String::from("graph {\n");
    for (a, b) in edges.iter().sorted() {
        dot += &format!("  {} -- {};\n", a, b);
    }
    dot += "}";
    // Run the following to visualize the graph:
    //   dot -Tsvg -Kneato out.dot > out.svg
    // Manually find the three edges.
    std::fs::write("out.dot", dot).unwrap();

    for (a, b) in remove_edges {
        graph.get_mut(a).unwrap().remove(b);
        graph.get_mut(b).unwrap().remove(a);
    }
    let size = component_size(&graph, a);
    let other_size = graph.len() - size;

    let product = size * other_size;
    println!("{product}");
    product
}

pub fn part1(input: &str) -> usize {
    solve_part1(input, [("ssd", "xqh"), ("nrs", "khn"), ("qlc", "mqb")], "mqb")
}

pub fn part2(_input: &str) {
    todo!();
}

#[test]
fn test_part1() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(solve_part1(input, [("cmg", "bvb"), ("nvd", "jqt"), ("pzl", "hfx")], "hfx"), 54);
}
