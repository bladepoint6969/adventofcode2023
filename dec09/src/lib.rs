fn get_sequences(history: &[i32]) -> Vec<Vec<i32>> {
    let mut sequences = vec![];
    let mut current_sequence = history;
    while current_sequence.iter().filter(|&e| e != &0).count() > 0 {
        let next_sequence: Vec<i32> = current_sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();
        sequences.push(next_sequence);
        current_sequence = sequences.last().unwrap();
    }
    sequences
}

fn extrapolate_backwards(history: &[i32]) -> i32 {
    let sequences = get_sequences(history);

    let mut last_extrapolation = 0;
    sequences
        .iter()
        .rev()
        .for_each(|sequence| last_extrapolation = sequence.first().unwrap() - last_extrapolation);

    history.first().unwrap() - last_extrapolation
}

fn extrapolate(history: &[i32]) -> i32 {
    let sequences = get_sequences(history);

    let mut last_extrapolation = 0;
    sequences
        .iter()
        .rev()
        .for_each(|sequence| last_extrapolation += sequence.last().unwrap());

    history.last().unwrap() + last_extrapolation
}

pub fn part1(input: &str) -> i64 {
    let extrapolations: i64 = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|history| extrapolate(&history) as i64)
        .sum();

    println!("{extrapolations}");
    extrapolations
}

pub fn part2(input: &str) {
    let extrapolations: i64 = input
        .lines()
        .map(|line| {
            line.split(' ')
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|history| extrapolate_backwards(&history) as i64)
        .sum();

    println!("{extrapolations}");
}

#[cfg(test)]
mod tests {
    use crate::{extrapolate, part1, extrapolate_backwards};

    #[test]
    fn test_extrapolate() {
        let vec = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(extrapolate(&vec), 18);

        let vec = vec![1, 3, 6, 10, 15, 21];
        assert_eq!(extrapolate(&vec), 28);

        let vec = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate(&vec), 68);
    }

    #[test]
    fn test_part1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        assert_eq!(part1(input), 114)
    }

    #[test]
    fn test_extrapolate_backwards() {
        let vec = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_backwards(&vec), 5)
    }
}
