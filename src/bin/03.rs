use std::collections::{HashMap, HashSet};

use euclid::{Point2D, Vector2D};

struct Grid;
type Point = Point2D<i32, Grid>;
type Vector = Vector2D<i32, Grid>;
type Wire = HashMap<Point, i32>;

fn parse_wire(line: &str) -> Wire {
    let mut wire = Wire::new();
    let mut pos = Point::zero();
    let mut total_dist = 0;
    for instr in line.split(',') {
        let dir = instr.chars().nth(0).unwrap();
        let dist = instr[1..].parse::<i32>().unwrap();
        let step = match dir {
            'U' => Vector::new(0, -1),
            'R' => Vector::new(1, 0),
            'D' => Vector::new(0, 1),
            'L' => Vector::new(-1, 0),
            _ => panic!("Invalid direction {}", dir)
        };
        for _ in 0..dist {
            pos += step;
            total_dist += 1;
            wire.insert(pos, total_dist);
        }
    }
    wire
}

fn manhattan_length(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

fn part1(input: &str) -> i32 {
    let wires = input.lines().map(parse_wire).collect::<Vec<_>>();
    let wire1 = wires[0].keys().collect::<HashSet<_>>();
    let wire2 = wires[1].keys().collect::<HashSet<_>>();
    let closest_crossing = wire1
        .intersection(&wire2)
        .min_by_key(|p| manhattan_length(p))
        .unwrap();
    manhattan_length(closest_crossing)
}

#[test]
fn test_part1() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    assert_eq!(part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 159);
    assert_eq!(part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
}

fn part2(input: &str) -> i32 {
    let wires = input.lines().map(parse_wire).collect::<Vec<_>>();
    wires[0].iter()
        .filter_map(|(p, d1)| {
            wires[1].get(p).map(|d2| d1 + d2)
        })
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    assert_eq!(part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 610);
    assert_eq!(part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
}

fn main() {
    aoc::main(part1, part2);
}
