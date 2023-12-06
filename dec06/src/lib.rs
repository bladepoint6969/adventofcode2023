fn get_nums(input: &str) -> Vec<u64> {
    input.split(' ').filter_map(|s| s.parse().ok()).collect()
}

fn get_big_num(input: &str) -> u64 {
    input
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap()
}

fn possible_wins(time: u64, distance: u64) -> usize {
    (0..=time)
        .map(|speed| speed * (time - speed))
        .filter(|&travelled| travelled > distance)
        .count()
}

pub fn part1(input: &str) {
    let mut lines = input.lines();
    let times = get_nums(lines.next().unwrap());
    let distances = get_nums(lines.next().unwrap());

    let result: usize = times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| possible_wins(time, distance))
        .product();

    println!("{result}");
}

pub fn part2(input: &str) {
    let mut lines = input.lines();
    let time = get_big_num(lines.next().unwrap());
    let distance = get_big_num(lines.next().unwrap());

    let result = possible_wins(time, distance);

    println!("{result}")
}
