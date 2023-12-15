fn hash(c: char, mut collector: u32) -> u32 {
    let code = c as u32;
    collector += code;
    collector *= 17;
    collector % 256
}

fn hash_string(input: &str) -> u32 {
    input.chars().fold(0, |collector, c| hash(c, collector))
}

pub fn part1(input: &str) -> u32 {
    let total = input.split(',').map(hash_string).sum();
    println!("{total}");
    total
}

pub fn part2(_input: &str) {
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::{hash_string, part1};

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
}
