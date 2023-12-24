use std::str::FromStr;
use z3::{ast::Ast, *};

const MIN_POS: f64 = 200000000000000.;
const MAX_POS: f64 = 400000000000000.;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn as_floats(&self) -> (f64, f64, f64) {
        (self.x as f64, self.y as f64, self.z as f64)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Hailstone {
    x: i64,
    y: i64,
    z: i64,
    vx: i64,
    vy: i64,
    vz: i64,
}

impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('@');
        let p: Vec<i64> = split
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.trim())
            .map(|num| num.parse().unwrap())
            .collect();
        let v: Vec<i64> = split
            .next()
            .unwrap()
            .split(',')
            .map(|num| num.trim())
            .map(|num| num.parse().unwrap())
            .collect();
        Ok(Self::new((p[0], p[1], p[2]), (v[0], v[1], v[2])))
    }
}

impl Hailstone {
    fn new(p: (i64, i64, i64), v: (i64, i64, i64)) -> Self {
        Self {
            x: p.0,
            y: p.1,
            z: p.2,
            vx: v.0,
            vy: v.1,
            vz: v.2,
        }
    }

    fn two_points(&self) -> (Point, Point) {
        let p1 = Point {
            x: self.x,
            y: self.y,
            z: self.z,
        };

        let p2 = Point {
            x: self.x + self.vx,
            y: self.y + self.vy,
            z: self.z + self.vz,
        };

        (p1, p2)
    }

    fn slope_and_y_intercept(&self) -> (f64, f64) {
        let (a, b) = self.two_points();

        let (x1, y1, _) = a.as_floats();
        let (x2, y2, _) = b.as_floats();

        let slope = (y2 - y1) / (x2 - x1);
        let y_int = y1 - (slope * x1);

        (slope, y_int)
    }

    fn is_in_future(&self, x: f64, y: f64) -> bool {
        let our_x = self.x as f64;
        let our_y = self.y as f64;

        if self.vx > 0 && our_x > x {
            return false;
        }
        if self.vx < 0 && our_x < x {
            return false;
        }

        if self.vy > 0 && our_y > y {
            return false;
        }
        if self.vy < 0 && our_y < y {
            return false;
        }

        true
    }

    fn intersect2d(&self, other: &Self) -> Option<(f64, f64)> {
        let (a, c) = self.slope_and_y_intercept();
        let (b, d) = other.slope_and_y_intercept();

        if a == b {
            return None;
        }

        let x_intersect = (d - c) / (a - b);
        let y_intersect = (a * ((d - c) / (a - b))) + c;

        if !self.is_in_future(x_intersect, y_intersect)
            || !other.is_in_future(x_intersect, y_intersect)
        {
            None
        } else {
            Some((x_intersect, y_intersect))
        }
    }

    fn create_z3_rep<'a>(&self, ctx: &'a Context) -> Z3Hailstone<'a> {
        let px = ast::Real::from_int(&ast::Int::from_i64(ctx, self.x));
        let py = ast::Real::from_int(&ast::Int::from_i64(ctx, self.y));
        let pz = ast::Real::from_int(&ast::Int::from_i64(ctx, self.z));

        let vx = ast::Real::from_int(&ast::Int::from_i64(ctx, self.vx));
        let vy = ast::Real::from_int(&ast::Int::from_i64(ctx, self.vy));
        let vz = ast::Real::from_int(&ast::Int::from_i64(ctx, self.vz));

        Z3Hailstone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        }
    }
}

struct Z3Hailstone<'a> {
    px: ast::Real<'a>,
    py: ast::Real<'a>,
    pz: ast::Real<'a>,
    vx: ast::Real<'a>,
    vy: ast::Real<'a>,
    vz: ast::Real<'a>,
}

fn build_stones(input: &str) -> Vec<Hailstone> {
    input
        .lines()
        .map(Hailstone::from_str)
        .map(Result::unwrap)
        .collect()
}

fn solve_part1(input: &str, min: f64, max: f64) -> usize {
    let hailstones = build_stones(input);
    let mut intersections = 0;

    for a in 0..hailstones.len() {
        for b in (a + 1)..hailstones.len() {
            if let Some((x, y)) = hailstones[a].intersect2d(&hailstones[b]) {
                if x >= min && x <= max && y >= min && y <= max {
                    intersections += 1;
                }
            }
        }
    }

    intersections
}

pub fn part1(input: &str) -> usize {
    let count = solve_part1(input, MIN_POS, MAX_POS);

    println!("{count}");
    count
}

pub fn part2(input: &str) -> i64 {
    let hailstones = build_stones(input);

    let cfg = Config::new();
    let ctx = Context::new(&cfg);

    let pxr = ast::Real::new_const(&ctx, "pxr");
    let pyr = ast::Real::new_const(&ctx, "pyr");
    let pzr = ast::Real::new_const(&ctx, "pzr");
    let vxr = ast::Real::new_const(&ctx, "vxr");
    let vyr = ast::Real::new_const(&ctx, "vyr");
    let vzr = ast::Real::new_const(&ctx, "vzr");
    let solver = Solver::new(&ctx);

    for (k, h) in hailstones[..3].iter().enumerate() {
        let tk = ast::Real::new_const(&ctx, format!("t{k}").as_str());
        solver.assert(&tk.gt(&ast::Real::from_real(&ctx, 0, 1)));

        let h = h.create_z3_rep(&ctx);
        solver.assert(&(&pxr + &tk * &vxr)._eq(&(h.px + &tk * h.vx)));
        solver.assert(&(&pyr + &tk * &vyr)._eq(&(h.py + &tk * h.vy)));
        solver.assert(&(&pzr + &tk * &vzr)._eq(&(h.pz + &tk * h.vz)));
    }

    solver.check();

    let model = solver.get_model().unwrap();

    let pxr = model.get_const_interp(&pxr).unwrap().as_real().unwrap().0;
    let pyr = model.get_const_interp(&pyr).unwrap().as_real().unwrap().0;
    let pzr = model.get_const_interp(&pzr).unwrap().as_real().unwrap().0;

    let sum = pxr + pyr + pzr;

    println!("{sum}");
    sum
}

#[test]
fn test_parse() {
    let input = "0, 0, 0 @ 1, 1, 1";
    assert!(Hailstone::from_str(input).is_ok());
}

#[test]
fn test_part1() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(solve_part1(input, 7., 27.), 2)
}

#[test]
fn test_part2() {
    let input = include_str!("../input_simple.txt");
    assert_eq!(part2(input), 47);
}
