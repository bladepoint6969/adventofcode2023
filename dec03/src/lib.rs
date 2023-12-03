#[derive(Debug, Clone, Copy)]
struct SchematicCell {
    value: char,
    visited: bool,
}

impl SchematicCell {
    fn new(value: char) -> Self {
        Self {
            value,
            visited: false,
        }
    }

    fn is_symbol(&self) -> bool {
        !self.value.is_ascii_digit() && self.value != '.'
    }

    fn is_unvisited_digit(&self) -> bool {
        self.value.is_ascii_digit() && !self.visited
    }
}

fn get_num(row: &mut Vec<SchematicCell>, col_num: usize) -> Option<String> {
    let cell = row.get_mut(col_num).unwrap();
    if cell.is_unvisited_digit() {
        cell.visited = true;
        let mut num = cell.value.to_string();
        if col_num > 0 {
            if let Some(val) = get_num(row, col_num - 1) {
                num = val + &num;
            }
        }
        if col_num + 1 < row.len() {
            if let Some(val) = get_num(row, col_num + 1) {
                num += &val;
            }
        }

        Some(num)
    } else {
        None
    }
}

pub fn part1(input: &str) {
    let mut grid: Vec<Vec<SchematicCell>> = input
        .lines()
        .map(|line| line.chars().map(SchematicCell::new).collect())
        .collect();

    let mut nums: Vec<i32> = vec![];

    for row_num in 0..grid.len() {
        let mut row_nums = vec![];
        for col_num in 0..grid[row_num].len() {
            if grid[row_num][col_num].is_symbol() {
                // Row Above
                if let Some(prev) = grid.get_mut(row_num - 1) {
                    if let Some(num) = get_num(prev, col_num - 1) {
                        // Top L
                        row_nums.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(prev, col_num) {
                        // Top
                        row_nums.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(prev, col_num + 1) {
                        // Top R
                        row_nums.push(num.parse().unwrap());
                    }
                }
                // This Row
                {
                    let row = &mut grid[row_num];
                    if let Some(num) = get_num(row, col_num - 1) {
                        // L
                        row_nums.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(row, col_num + 1) {
                        // R
                        row_nums.push(num.parse().unwrap());
                    }
                }
                // Row Below
                if let Some(next) = grid.get_mut(row_num + 1) {
                    if let Some(num) = get_num(next, col_num - 1) {
                        // Bottom L
                        row_nums.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(next, col_num) {
                        // Bottom
                        row_nums.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(next, col_num + 1) {
                        // Bottom R
                        row_nums.push(num.parse().unwrap());
                    }
                }
            }
        }
        nums.append(&mut row_nums);
    }

    let sum: i32 = nums.iter().sum();
    println!("{sum}")
}

pub fn part2(input: &str) {
    let mut grid: Vec<Vec<SchematicCell>> = input
        .lines()
        .map(|line| line.chars().map(SchematicCell::new).collect())
        .collect();

    let mut nums: Vec<i32> = vec![];

    for row_num in 0..grid.len() {
        for col_num in 0..grid[row_num].len() {
            if grid[row_num][col_num].value == '*' {
                let mut gears = vec![];
                // Row Above
                if let Some(prev) = grid.get_mut(row_num - 1) {
                    if let Some(num) = get_num(prev, col_num - 1) {
                        // Top L
                        gears.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(prev, col_num) {
                        // Top
                        gears.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(prev, col_num + 1) {
                        // Top R
                        gears.push(num.parse().unwrap());
                    }
                }
                // This Row
                {
                    let row = &mut grid[row_num];
                    if let Some(num) = get_num(row, col_num - 1) {
                        // L
                        gears.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(row, col_num + 1) {
                        // R
                        gears.push(num.parse().unwrap());
                    }
                }
                // Row Below
                if let Some(next) = grid.get_mut(row_num + 1) {
                    if let Some(num) = get_num(next, col_num - 1) {
                        // Bottom L
                        gears.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(next, col_num) {
                        // Bottom
                        gears.push(num.parse().unwrap());
                    }
                    if let Some(num) = get_num(next, col_num + 1) {
                        // Bottom R
                        gears.push(num.parse().unwrap());
                    }
                }
                if gears.len() == 2 {
                    nums.push(gears.iter().product())
                }
            }
        }
    }
    let sum: i32 = nums.iter().sum();
    println!("{sum}")
}
