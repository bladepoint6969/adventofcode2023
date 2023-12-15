use std::fmt::Display;

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Display for Lens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {}]", self.label, self.focal_length)
    }
}

fn hash(c: char, mut collector: usize) -> usize {
    let code = c as usize;
    collector += code;
    collector *= 17;
    collector % 256
}

fn hash_string(input: &str) -> usize {
    input.chars().fold(0, |collector, c| hash(c, collector))
}

fn process_step(step: &str, boxes: &mut [Vec<Lens>]) {
    if let Some(stripped) = step.strip_suffix('-') {
        let hash = hash_string(stripped);
        boxes[hash].retain(|lens| lens.label != stripped);
    } else {
        let mut split = step.split('=');
        let label = split.next().unwrap().to_string();
        let focal_length = split.next().unwrap().parse().unwrap();
        let hash = hash_string(&label);
        if let Some(lens) = boxes[hash].iter_mut().find(|lens| lens.label == label) {
            lens.focal_length = focal_length;
        } else {
            boxes[hash].push(Lens {
                label,
                focal_length,
            });
        }
    }
}

pub fn part1(input: &str) -> usize {
    let total = input.split(',').map(hash_string).sum();
    println!("{total}");
    total
}

pub fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    for step in input.split(',') {
        process_step(step, &mut boxes);
    }

    let total = boxes
        .iter()
        .enumerate()
        .map(|(box_num, lens_box)| {
            lens_box
                .iter()
                .enumerate()
                .map(|(lens_slot, lens)| (box_num + 1) * (lens_slot + 1) * lens.focal_length)
                .sum::<usize>()
        })
        .sum();

    println!("{total}");
    total
}

#[cfg(test)]
mod tests {
    use crate::{hash_string, part1, part2};

    #[test]
    fn test_hash() {
        let result = hash_string("HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part1() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part1(input), 1320)
    }

    #[test]
    fn test_part2() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(part2(input), 145)
    }
}
