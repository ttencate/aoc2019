use aoc::intcode::*;
use euclid;
use std::collections::{HashSet, VecDeque};

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Vector = euclid::Vector2D<i32, Grid>;

struct ExploreNode {
    program: Program,
    pos: Point,
    dist: usize,
}

struct FillNode {
    pos: Point,
    dist: usize,
}

const HIT_WALL: Number = 0;
const MOVED: Number = 1;
const FOUND_OXYGEN_SYSTEM: Number = 2;

fn part1(input: &str) -> usize {
    let mut program = Program::parse(input);
    program.run_until_interrupt();
    let start_node = ExploreNode {
        program: program,
        pos: Point::default(),
        dist: 0,
    };
    let mut queue = VecDeque::new();
    queue.push_back(start_node);
    let mut visited = HashSet::new();
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node.pos) {
            continue;
        }
        for (input, step) in &[
            (1, Vector::new(0, -1)),
            (2, Vector::new(0, 1)),
            (3, Vector::new(-1, 0)),
            (4, Vector::new(1, 0)),
        ] {
            let mut program = node.program.clone();
            program.give_input(*input);
            let output = program.take_output();
            program.run_until_interrupt();
            match output {
                HIT_WALL => {},
                MOVED => {
                    queue.push_back(ExploreNode {
                        program: program,
                        pos: node.pos + *step,
                        dist: node.dist + 1,
                    });
                },
                FOUND_OXYGEN_SYSTEM => {
                    return node.dist + 1;
                },
                _ => panic!("Invalid output {}", output),
            };
        }
    }
    panic!("Could not find the oxygen system");
}

fn part2(input: &str) -> usize {
    let mut program = Program::parse(input);
    program.run_until_interrupt();
    let start_node = ExploreNode {
        program: program,
        pos: Point::default(),
        dist: 0,
    };
    let mut queue = VecDeque::new();
    queue.push_back(start_node);
    let mut visited = HashSet::new();
    let mut oxygen_system_pos = None;
    while let Some(node) = queue.pop_front() {
        if !visited.insert(node.pos) {
            continue;
        }
        for (input, step) in &[
            (1, Vector::new(0, -1)),
            (2, Vector::new(0, 1)),
            (3, Vector::new(-1, 0)),
            (4, Vector::new(1, 0)),
        ] {
            let mut program = node.program.clone();
            program.give_input(*input);
            let output = program.take_output();
            program.run_until_interrupt();
            match output {
                HIT_WALL => {},
                MOVED | FOUND_OXYGEN_SYSTEM => {
                    let new_pos = node.pos + *step;
                    queue.push_back(ExploreNode {
                        program: program,
                        pos: node.pos + *step,
                        dist: node.dist + 1,
                    });
                    if output == FOUND_OXYGEN_SYSTEM {
                        oxygen_system_pos = Some(new_pos);
                    }
                },
                _ => panic!("Invalid output {}", output),
            };
        }
    }

    let mut queue = VecDeque::new();
    queue.push_back(FillNode {
        pos: oxygen_system_pos.expect("Could not find the oxygen system"),
        dist: 0,
    });
    let mut max_dist = 0;
    while let Some(node) = queue.pop_front() {
        if !visited.remove(&node.pos) {
            continue;
        }
        max_dist = max_dist.max(node.dist);
        for step in &[
            Vector::new(0, -1),
            Vector::new(0, 1),
            Vector::new(-1, 0),
            Vector::new(1, 0),
        ] {
            queue.push_back(FillNode {
                pos: node.pos + *step,
                dist: node.dist + 1,
            });
        }
    }
    max_dist
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 298, part2, 346);
}
