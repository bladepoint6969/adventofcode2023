struct Card {
    id: i32,
    winning_numbers: Vec<i32>,
    my_numbers: Vec<i32>,
}

impl Card {
    fn new(input: &str) -> Self {
        let mut nums = input.split(':');

        let id_str = nums.next().unwrap();
        let id: i32 = id_str.split(' ').last().unwrap().parse().unwrap();

        let mut nums = nums.next().unwrap().split('|');
        let winning_numbers: Vec<i32> = nums
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|e| match e.parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            })
            .collect();
        let my_numbers: Vec<i32> = nums
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|e| match e.parse() {
                Ok(num) => Some(num),
                Err(_) => None,
            })
            .collect();
        Self {
            id,
            winning_numbers,
            my_numbers,
        }
    }

    fn matches(&self) -> usize {
        self.my_numbers
            .iter()
            .filter(|&num| self.winning_numbers.contains(num))
            .count()
    }

    fn value(&self) -> i32 {
        let my_wins = self.matches();

        if my_wins == 0 {
            0
        } else {
            2_i32.pow(my_wins as u32 - 1)
        }
    }
}

pub fn part1(input: &str) {
    let sum: i32 = input.lines().map(Card::new).map(|card| card.value()).sum();
    println!("{sum}");
}

pub fn part2(input: &str) {
    let cards: Vec<Card> = input.lines().map(Card::new).collect();
    let mut copies: Vec<usize> = vec![1; cards.len()];

    for i in 0..cards.len() {
        let card = cards.get(i).unwrap();
        let card_copies = copies.get(i).unwrap().to_owned();
        println!("Card {}", card.id);
        let mut matches = card.matches();
        while matches > 0 {
            copies[i + matches] += card_copies;
            matches -= 1;
        }
    }
    let total: usize = copies.iter().sum();

    println!("{total}")
}
