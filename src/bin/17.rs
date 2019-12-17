use aoc::intcode::*;
use euclid;
use itertools::Itertools;

struct Grid;
type Point = euclid::Point2D<i64, Grid>;
type Vector = euclid::Vector2D<i64, Grid>;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

use Direction::*;

impl Direction {
    fn parse(c: u8) -> Option<Direction> {
        match c {
            b'^' => Some(North),
            b'>' => Some(East),
            b'v' => Some(South),
            b'<' => Some(West),
            _ => None
        }
    }

    fn to_vector(&self) -> Vector {
        match self {
            North => Vector::new(0, -1),
            East => Vector::new(1, 0),
            South => Vector::new(0, 1),
            West => Vector::new(-1, 0),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            North => '^',
            East => '>',
            South => 'v',
            West => '<',
        })
    }
}

type Instruction = u8;

const LEFT: Instruction = b'L';
const RIGHT: Instruction = b'R';
const FORWARD: Instruction = b'F';

impl std::ops::Add<Instruction> for Direction {
    type Output = Direction;
    fn add(self, instr: Instruction) -> Direction {
        match (self, instr) {
            (East, LEFT) | (West, RIGHT) => North,
            (South, LEFT) | (North, RIGHT) => East,
            (West, LEFT) | (East, RIGHT) => South,
            (North, LEFT) | (South, RIGHT) => West,
            _ => panic!("Cannot add {} to {}", instr, self),
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    is_paths: Vec<Vec<bool>>,
    pos: Point,
    dir: Direction,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for y in 0..self.ny() {
            for x in 0..self.nx() {
                let pos = Point::new(x, y);
                if pos == self.pos {
                    write!(f, "{}", self.dir)?;
                } else {
                    write!(f, "{}", if self.is_path(pos) { '#' } else { '.' })?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl State {
    fn parse(ascii: &str) -> Self {
        let mut pos = Point::default();
        let mut dir = North;
        let is_paths = ascii.trim().lines()
            .enumerate()
            .map(|(y, line)| {
                line.trim().as_bytes().iter()
                    .enumerate()
                    .map(|(x, chr)| {
                        match chr {
                            b'.' => false,
                            b'#' => true,
                            &dir_char => {
                                pos = Point::new(x as i64, y as i64);
                                dir = Direction::parse(dir_char).expect("Unknown character");
                                true
                            }
                        }
                    })
                    .collect()
            })
            .collect::<Vec<_>>();
        State { is_paths, pos, dir }
    }

    fn is_path(&self, pos: Point) -> bool {
        pos.x >= 0 && pos.x < self.nx() && pos.y >= 0 && pos.y < self.ny() && self.is_paths[pos.y as usize][pos.x as usize]
    }

    fn nx(&self) -> i64 {
        self.is_paths[0].len() as i64
    }

    fn ny(&self) -> i64 {
        self.is_paths.len() as i64
    }
}

fn alignment_parameters_sum(state: &State) -> i64 {
    let mut sum = 0;
    for y in 1..(state.ny() - 1) {
        for x in 1..(state.nx() - 1) {
            let pos = Point::new(x, y);
            if state.is_path(pos) &&
                state.is_path(pos + North.to_vector()) &&
                state.is_path(pos + East.to_vector()) &&
                state.is_path(pos + South.to_vector()) &&
                state.is_path(pos + West.to_vector())
            {
                sum += y * x;
            }
        }
    }
    sum
}

#[test]
fn test_alignment_parameters_sum() {
    assert_eq!(
        alignment_parameters_sum(&State::parse(
            "..#..........
             ..#..........
             #######...###
             #.#...#...#.#
             #############
             ..#...#...#..
             ..#####...^..")),
        76);
}

fn part1(input: &str) -> i64 {
    let output = Program::parse(input).run_with_io(vec![]).output;
    let ascii = String::from_utf8(output.iter().map(|&val| val as u8).collect()).unwrap();
    let state = State::parse(&ascii);
    alignment_parameters_sum(&state)
}

fn trace_path(state: &State) -> Vec<Instruction> {
    let mut path = Vec::new();
    let mut pos = state.pos;
    let mut dir = *[North, East, South, West].iter()
        .find(|dir| state.is_path(pos + dir.to_vector()))
        .expect("No start direction found");
    match (state.dir, dir) {
        (North, East) | (East, South) | (South, West) | (West, North) => path.extend(&[RIGHT]),
        (North, South) | (East, West) | (South, North) | (West, East) => path.extend(&[RIGHT, RIGHT]),
        (North, West) | (East, North) | (South, East) | (West, South) => path.extend(&[LEFT]),
        (_, _) => {},
    };
    loop {
        if state.is_path(pos + dir.to_vector()) {
            path.push(FORWARD);
            pos += dir.to_vector();
        } else {
            let turn_instr = [LEFT, RIGHT].iter()
                .find(|&&instr| state.is_path(pos + (dir + instr).to_vector()))
                .copied();
            if let Some(turn_instr) = turn_instr {
                path.push(turn_instr);
                path.push(FORWARD);
                dir = dir + turn_instr;
                pos += dir.to_vector();
            } else {
                break;
            }
        }
    }
    path
}

#[test]
fn test_trace_path() {
    assert_eq!(
        encode_path(&trace_path(&State::parse(
            "#######...#####
             #.....#...#...#
             #.....#...#...#
             ......#...#...#
             ......#...###.#
             ......#.....#.#
             ^########...#.#
             ......#.#...#.#
             ......#########
             ........#...#..
             ....#########..
             ....#...#......
             ....#...#......
             ....#...#......
             ....#####......"))),
        "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2".to_string());
}

fn encode_path(path: &[Instruction]) -> String {
    let mut out = String::new();
    let mut i = 0;
    let n = path.len();
    while i < n {
        if i > 0 {
            out += ",";
        }
        match path[i] {
            FORWARD => {
                let mut steps = 0;
                while i < n && path[i] == FORWARD {
                    i += 1;
                    steps += 1;
                }
                out += &steps.to_string();
            },
            LEFT | RIGHT => {
                out.push(path[i] as char);
                i += 1;
            },
            _ => panic!("Invalid instruction {} at index {}", path[i], i),
        }
    }
    out
}

fn path_to_functions(path: &[Instruction]) -> String {
    let mut main = vec![];
    let num_funcs = 3;
    for max_func_len in 0..path.len() {
        let mut func_lens = vec![1; num_funcs];
        while func_lens[func_lens.len() - 1] <= max_func_len {
            // If none is max_func_len, we already checked this combination in the previous
            // iteration.
            // TODO: iterate smarter, not harder (maybe just store in vec, then sort?)
            if func_lens.iter().any(|&len| len == max_func_len) {
                let mut funcs: Vec<&[u8]> = Vec::with_capacity(num_funcs);
                main.clear();
                let mut remaining = path;
                while !remaining.is_empty() {
                    let mut matched = false;
                    // Find a func that matches the path prefix.
                    for (i, func) in funcs.iter().enumerate() {
                        if remaining.starts_with(func) {
                            if !main.is_empty() {
                                main.push(b',');
                            }
                            main.push(b'A' + i as u8);
                            remaining = &remaining[func.len()..];
                            matched = true;
                            break;
                        }
                    }
                    if !matched {
                        // No matching func found. Can we use a new one?
                        let next_func_index = funcs.len();
                        if next_func_index < num_funcs && func_lens[next_func_index] <= remaining.len() {
                            // Start the next func here with the prescribed length.
                            funcs.push(&remaining[0..func_lens[next_func_index]]);
                        } else {
                            // Hopeless. Abort.
                            break;
                        }
                    }
                }
                if remaining.is_empty() {
                    // Found one!
                    let main_str = String::from_utf8(main.clone()).unwrap();
                    let func_strs = funcs.iter().map(|f| encode_path(f)).collect::<Vec<_>>();
                    if main_str.len() < 20 && func_strs.iter().all(|f| f.len() <= 20) {
                        return main_str + "\n" + &func_strs.iter().join("\n") + "\n";
                    }
                }
            }

            func_lens[0] += 1;
            for i in 0..num_funcs - 1 {
                if func_lens[i] > max_func_len {
                    func_lens[i] = 1;
                    func_lens[i + 1] += 1;
                }
            }
        }
    }
    panic!("No suitable path found")
}

#[test]
fn test_path_to_functions() {
    assert_eq!(
        path_to_functions("RFFFFFFFFRFFFFFFFFRFFFFRFFFFRFFFFFFFFLFFFFFFLFFRFFFFRFFFFRFFFFFFFFRFFFFFFFFRFFFFFFFFLFFFFFFLFF".as_bytes()),
        // Solution from the example:
        // "A,B,C,B,A,C\nR,8,R,8\nR,4,R,4,R,8\nL,6,L,2\n"
        // Solution that we find (checked by hand to be valid):
        "A,A,B,A,C,B,A,A,A,C\nR,8\nR,4,R,4\nL,6,L,2\n".to_string());
}

fn part2(input: &str) -> Number {
    let mut program = Program::parse(input);
    let output = program.clone().run_with_io(vec![]).output;
    let ascii = String::from_utf8(output.iter().map(|&val| val as u8).collect()).unwrap();
    let state = State::parse(&ascii);
    let path = trace_path(&state);

    let functions = path_to_functions(&path);
    program.mem[0] = 2;
    let input = (functions + "n\n").as_bytes().iter().map(|&c| c as Number).collect();
    let output = program.run_with_io(input).output;
    *output.last().unwrap()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 4044, part2, 893283);
}
