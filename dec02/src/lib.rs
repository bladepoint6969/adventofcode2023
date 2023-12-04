const MAX: &Round = &Round {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug)]
pub struct Game {
    id: i32,
    rounds: Vec<Round>,
}

impl Game {
    fn new(input: &str) -> Self {
        let mut record = input.split(':');

        let id: i32 = record
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let rounds: Vec<Round> = record
            .next()
            .unwrap()
            .split(';')
            .map(str::trim)
            .map(Round::new)
            .collect();

        Self { id, rounds }
    }

    fn is_possible(&self, max: &Round) -> bool {
        self.rounds
            .iter()
            .filter(|&round| round.is_possible(max))
            .collect::<Vec<_>>()
            .len()
            == self.rounds.len()
    }

    fn get_power(&self) -> i32 {
        let mut red = self.rounds[0].red;
        let mut green = self.rounds[0].green;
        let mut blue = self.rounds[0].blue;

        for round in &self.rounds {
            if round.red > red {
                red = round.red;
            }
            if round.green > green {
                green = round.green;
            }
            if round.blue > blue {
                blue = round.blue;
            }
        }
        red * green * blue
    }
}

#[derive(Debug)]
pub struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

impl Round {
    fn new(input: &str) -> Self {
        let (mut red, mut green, mut blue) = (0, 0, 0);
        input
            .split(',')
            .map(str::trim)
            .map(|count| {
                let mut record = count.split(' ');
                let count: i32 = record.next().unwrap().parse().unwrap();
                let color = record.next().unwrap();
                (color, count)
            })
            .for_each(|cubes| match cubes {
                ("red", count) => red += count,
                ("green", count) => green += count,
                ("blue", count) => blue += count,
                _ => unimplemented!(),
            });

        Self { red, green, blue }
    }

    fn is_possible(&self, max: &Round) -> bool {
        self.red <= max.red && self.green <= max.green && self.blue <= max.blue
    }
}

pub fn part1(input: &str) {
    let games = get_games(input);
    let sum: i32 = games
        .iter()
        .filter(|game| game.is_possible(MAX))
        .map(|game| game.id)
        .sum();

    println!("{sum}")
}

pub fn part2(input: &str) {
    let games = get_games(input);

    let power_sum: i32 = games.iter().map(Game::get_power).sum();
    println!("{power_sum}")
}

fn get_games(input: &str) -> Vec<Game> {
    input.lines().map(Game::new).collect()
}
