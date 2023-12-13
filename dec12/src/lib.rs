use itertools::Itertools;

fn analyze_str(record: &str) -> Vec<usize> {
    record
        .split('.')
        .filter(|s| !s.is_empty())
        .map(|s| s.len())
        .collect()
}

fn construct_strings(prefix: &str, remaining: &str, collector: &mut Vec<String>) {
    let mut iter = remaining.chars();
    match iter.next() {
        None => collector.push(prefix.to_string()),
        Some(first) => {
            let remaining: String = iter.collect();
            match first {
                '?' => {
                    construct_strings(&format!("{prefix}."), &remaining, collector);
                    construct_strings(&format!("{prefix}#"), &remaining, collector);
                }
                c => construct_strings(&format!("{prefix}{c}"), &remaining, collector),
            }
        }
    }
}

fn count_matches(record: &str, pattern: &[usize]) -> usize {
    let mut collector = vec![];
    construct_strings("", record, &mut collector);

    collector
        .iter()
        .filter(|&record| analyze_str(record) == pattern)
        .count()
}

pub fn part1(input: &str) -> usize {
    let counts: usize = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let record = split.next().unwrap();
            let pattern: Vec<usize> = split
                .next()
                .unwrap()
                .split(',')
                .map(|num| num.parse().unwrap())
                .collect();

            count_matches(record, &pattern)
        })
        .sum();

    println!("{counts}");
    counts
}

fn solve(spring: &str, counts: impl Iterator<Item = usize>) -> usize {
    let counts = counts.collect_vec();

    let spring = format!(".{}", spring.trim_end_matches('.'));
    let spring = spring.chars().collect_vec();

    let mut dp = vec![0; spring.len() + 1];
    dp[0] = 1;

    for (i, _) in spring.iter().take_while(|&&c| c != '#').enumerate() {
        dp[i + 1] = 1;
    }

    for count in counts {
        let mut n_dp = vec![0; spring.len() + 1];
        let mut chunk = 0;

        for (i, &c) in spring.iter().enumerate() {
            if c != '.' {
                chunk += 1;
            } else {
                chunk = 0;
            }

            if c != '#' {
                n_dp[i + 1] += n_dp[i];
            }

            if chunk >= count && spring[i - count] != '#' {
                n_dp[i + 1] += dp[i - count];
            }
        }

        dp = n_dp;
    }

    *dp.last().unwrap()
}

pub fn part2(input: &str) -> usize {
    let count = input
        .lines()
        .map(|line| {
            let (spring, counts) = line.split_once(' ').unwrap();

            let spring = std::iter::once(spring).cycle().take(5).join("?");

            let counts = counts
                .split(',')
                .map(|number| number.parse::<usize>().unwrap())
                .collect_vec();
            let n = counts.len();

            solve(&spring, counts.into_iter().cycle().take(5 * n))
        })
        .sum::<usize>();
    println!("{count}");
    count
}

#[cfg(test)]
mod tests {
    use crate::{analyze_str, construct_strings, count_matches, part1, part2};

    #[test]
    fn test_analyze_str() {
        assert_eq!(analyze_str("#.#.###"), [1, 1, 3]);
        assert_eq!(analyze_str(".#...#....###."), [1, 1, 3]);
        assert_eq!(analyze_str(".###.##....#"), [3, 2, 1]);
    }

    #[test]
    fn test_construct_strings() {
        let mut collector = vec![];
        construct_strings("", ".#", &mut collector);
        assert_eq!(collector, [".#"]);

        let mut collector = vec![];
        construct_strings("", ".#?", &mut collector);
        assert_eq!(collector, [".#.", ".##"]);
    }

    #[test]
    fn test_count_matches() {
        assert_eq!(count_matches("???.###", &[1, 1, 3]), 1);
        assert_eq!(count_matches(".??..??...?##.", &[1, 1, 3]), 4);
        assert_eq!(count_matches("?###????????", &[3, 2, 1]), 10);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 21);
    }
    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 525152);
    }
}
