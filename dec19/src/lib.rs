use std::{collections::HashMap, str::Lines};

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn new(x: u32, m: u32, a: u32, s: u32) -> Self {
        Self { x, m, a, s }
    }

    fn value(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }

    fn get_rating(&self, rating: &Rating) -> u32 {
        match rating {
            Rating::A => self.a,
            Rating::M => self.m,
            Rating::S => self.s,
            Rating::X => self.x,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rating {
    X,
    M,
    A,
    S,
}

impl Rating {
    fn new(val: char) -> Self {
        match val {
            'x' => Self::X,
            'm' => Self::M,
            'a' => Self::A,
            's' => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleOutcome {
    Accept,
    Reject,
    Workflow(String),
}

impl RuleOutcome {
    fn new(input: &str) -> Self {
        match input {
            "A" => Self::Accept,
            "R" => Self::Reject,
            s => Self::Workflow(s.to_string()),
        }
    }
    fn unwrap(self) -> String {
        match self {
            Self::Workflow(s) => s,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    rating: Option<Rating>,
    is_gt: bool,
    test_val: u32,
    outcome: RuleOutcome,
}

impl Rule {
    fn new(val: &str) -> Self {
        let mut split = val.split(':');
        let part_1 = split.next().unwrap();
        let Some(part_2) = split.next() else {
            return Self {
                rating: None,
                is_gt: false,
                test_val: 0,
                outcome: RuleOutcome::new(part_1),
            };
        };
        let mut condition = part_1.chars();
        let rating = Rating::new(condition.next().unwrap());
        let is_gt = condition.next().unwrap() == '>';
        let test_val = condition.collect::<String>().parse().unwrap();
        Self {
            rating: Some(rating),
            is_gt,
            test_val,
            outcome: RuleOutcome::new(part_2),
        }
    }

    fn evaluate(&self, part: &Part) -> Option<&RuleOutcome> {
        let Some(ref rating) = self.rating else {
            return Some(&self.outcome);
        };
        let part_val = part.get_rating(rating);
        let passed = match self.is_gt {
            true => part_val > self.test_val,
            false => part_val < self.test_val,
        };
        if passed {
            Some(&self.outcome)
        } else {
            None
        }
    }
}

fn apply_workflow(workflows: &HashMap<String, Vec<Rule>>, label: &str, part: &Part) -> RuleOutcome {
    let rules = workflows.get(label).unwrap();
    for rule in rules {
        match rule.evaluate(part) {
            Some(RuleOutcome::Workflow(next)) => return apply_workflow(workflows, next, part),
            Some(accept_or_reject) => return accept_or_reject.to_owned(),
            None => {}
        }
    }
    unreachable!("Always an unconditional rule");
}

fn build_workflows(lines: &mut Lines) -> HashMap<String, Vec<Rule>> {
    let mut workflows = HashMap::new();

    let workflow_re = Regex::new(r"(?<label>\w+)\{(?<rules>.+)\}").unwrap();

    for line in lines.by_ref() {
        if line.is_empty() {
            return workflows;
        }
        let captures = workflow_re.captures(line).unwrap();
        let label = captures["label"].to_string();
        let rules = captures["rules"].split(',').map(Rule::new).collect();
        workflows.insert(label, rules);
    }

    workflows
}

fn build_parts(lines: &mut Lines) -> Vec<Part> {
    let part_re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
    lines
        .map(|line| part_re.captures(line).unwrap())
        .map(|captures| {
            let x = captures["x"].parse().unwrap();
            let m = captures["m"].parse().unwrap();
            let a = captures["a"].parse().unwrap();
            let s = captures["s"].parse().unwrap();
            Part::new(x, m, a, s)
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let workflows = build_workflows(&mut lines);

    let parts: Vec<Part> = build_parts(&mut lines);
    let sum = parts
        .iter()
        .filter(|&part| apply_workflow(&workflows, "in", part) == RuleOutcome::Accept)
        .map(|part| part.value())
        .sum();

    println!("{sum}");
    sum
}

type RangeStack<'a> = Vec<(
    (u32, u32),
    (u32, u32),
    (u32, u32),
    (u32, u32),
    RuleOutcome,
    usize,
)>;
type RangeVec = Vec<((u32, u32), (u32, u32), (u32, u32), (u32, u32))>;

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();
    let workflows = build_workflows(&mut lines);
    let mut stack: RangeStack = vec![(
        (1, 4000),
        (1, 4000),
        (1, 4000),
        (1, 4000),
        RuleOutcome::Workflow("in".to_string()),
        0,
    )];
    let mut accepted: RangeVec = vec![];

    while let Some(range) = stack.pop() {
        let (x, m, a, s, outcome, rule_key) = range;
        if outcome == RuleOutcome::Accept {
            accepted.push((x, m, a, s));
            continue;
        }
        if outcome == RuleOutcome::Reject {
            continue;
        }

        // Invalid bounds check
        if x.0 > x.1 || m.0 > m.1 || a.0 > a.1 || s.0 > s.1 {
            continue;
        }

        let key = outcome.unwrap();
        let rules = workflows.get(&key).unwrap();
        let rule = &rules[rule_key];
        match rule {
            Rule {
                rating: None,
                outcome: RuleOutcome::Accept,
                ..
            } => {
                accepted.push((x, m, a, s));
                continue;
            }
            Rule {
                rating: None,
                outcome: RuleOutcome::Reject,
                ..
            } => {
                continue;
            }
            Rule {
                rating: None,
                outcome,
                ..
            } => {
                stack.push((x, m, a, s, outcome.clone(), 0));
                continue;
            }
            Rule {
                rating: Some(rating),
                is_gt,
                test_val,
                outcome,
            } => {
                if *is_gt {
                    match rating {
                        Rating::X => {
                            stack.push(((test_val + 1, x.1), m, a, s, outcome.clone(), 0));
                            stack.push((
                                (x.0, *test_val),
                                m,
                                a,
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::M => {
                            stack.push((x, (test_val + 1, m.1), a, s, outcome.clone(), 0));
                            stack.push((
                                x,
                                (m.0, *test_val),
                                a,
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::A => {
                            stack.push((x, m, (test_val + 1, a.1), s, outcome.clone(), 0));
                            stack.push((
                                x,
                                m,
                                (a.0, *test_val),
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::S => {
                            stack.push((x, m, a, (test_val + 1, s.1), outcome.clone(), 0));
                            stack.push((
                                x,
                                m,
                                a,
                                (s.0, *test_val),
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                    }
                } else {
                    match rating {
                        Rating::X => {
                            stack.push(((x.0, test_val - 1), m, a, s, outcome.clone(), 0));
                            stack.push((
                                (*test_val, x.1),
                                m,
                                a,
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::M => {
                            stack.push((x, (m.0, test_val - 1), a, s, outcome.clone(), 0));
                            stack.push((
                                x,
                                (*test_val, m.1),
                                a,
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::A => {
                            stack.push((x, m, (a.0, test_val - 1), s, outcome.clone(), 0));
                            stack.push((
                                x,
                                m,
                                (*test_val, a.1),
                                s,
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                        Rating::S => {
                            stack.push((x, m, a, (s.0, test_val - 1), outcome.clone(), 0));
                            stack.push((
                                x,
                                m,
                                a,
                                (*test_val, s.1),
                                RuleOutcome::Workflow(key),
                                rule_key + 1,
                            ));
                        }
                    }
                }
            }
        }
    }

    let count = accepted
        .iter()
        .map(|(x, m, a, s)| {
            (x.1 - x.0 + 1) as u64
                * (m.1 - m.0 + 1) as u64
                * (a.1 - a.0 + 1) as u64
                * (s.1 - s.0 + 1) as u64
        })
        .sum();

    println!("{count}");
    count
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 167409079868000);
    }
}
