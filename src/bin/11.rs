use aoc::intcode::*;
use euclid;
use itertools::Itertools;
use std::collections::HashMap;

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Hull = HashMap<Point, Number>;

const BLACK: Number = 0;
const WHITE: Number = 1;

fn paint(input: &str, hull: &mut Hull) {
    let mut interrupt = Program::parse(input).run();
    let mut pos = Point::new(0, 0);
    let mut dir = 0;
    while !interrupt.is_halted() {
        interrupt = interrupt.give_input(*hull.get(&pos).unwrap_or(&BLACK)).run();
        let (paint_color, program) = interrupt.take_output();
        hull.insert(pos, paint_color);
        let (rotation, program) = program.run().take_output();
        interrupt = program.run();
        dir = match rotation {
            0 => (dir + 4 - 1) % 4,
            1 => (dir + 1) % 4,
            _ => panic!("Invalid rotation direction {}", rotation),
        };
        match dir {
            0 => pos.y -= 1,
            1 => pos.x += 1,
            2 => pos.y += 1,
            3 => pos.x -= 1,
            _ => panic!("Invalid rotation {}", dir),
        }
    }
}

fn hull_to_string(hull: &Hull) -> String {
    let (x_min, x_max) = hull.keys().map(|p| p.x).minmax().into_option().unwrap();
    let (y_min, y_max) = hull.keys().map(|p| p.y).minmax().into_option().unwrap();
    let mut out = String::with_capacity(((x_max - x_min + 1 + 1) * (y_max - y_min + 1)) as usize);
    for y in y_min ..= y_max {
        out.push('\n');
        for x in x_min ..= x_max {
            let color = *hull.get(&Point::new(x, y)).unwrap_or(&BLACK);
            out.push(match color {
                BLACK => '░',
                WHITE => '█',
                _ => panic!("Unknown hull color value {}", color),
            });
        }
    }
    out
}

fn part1(input: &str) -> usize {
    let mut hull = HashMap::<Point, Number>::new();
    paint(input, &mut hull);
    // println!("{}", hull_to_string(&hull));
    hull.len()
}

fn part2(input: &str) -> String {
    let mut hull = HashMap::<Point, Number>::new();
    hull.insert(Point::new(0, 0), WHITE);
    paint(input, &mut hull);
    hull_to_string(&hull)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 2428, part2, "
░███░░░░██░█░░░░████░███░░█░░█░░██░░█░░█░░░
░█░░█░░░░█░█░░░░█░░░░█░░█░█░░█░█░░█░█░░█░░░
░█░░█░░░░█░█░░░░███░░███░░█░░█░█░░░░█░░█░░░
░███░░░░░█░█░░░░█░░░░█░░█░█░░█░█░░░░█░░█░░░
░█░█░░█░░█░█░░░░█░░░░█░░█░█░░█░█░░█░█░░█░░░
░█░░█░░██░░████░█░░░░███░░░██░░░██░░░██░░░░".to_string());
}
