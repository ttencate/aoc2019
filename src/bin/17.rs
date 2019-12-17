use aoc::intcode::*;
use euclid;

struct Grid;
type Point = euclid::Point2D<i64, Grid>;
type Vector = euclid::Vector2D<i64, Grid>;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct State {
    cells: Vec<Vec<u8>>,
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
                    write!(f, "{}", self.cell(pos) as char)?;
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
        let cells = ascii.trim().lines()
            .enumerate()
            .map(|(y, line)| {
                let mut row = line.trim().as_bytes().to_vec();
                for x in 0..row.len() {
                    if let Some(d) = Direction::parse(row[x]) {
                        pos = Point::new(x as i64, y as i64);
                        dir = d;
                        row[x] = b'#';
                    }
                }
                row
            })
            .collect::<Vec<_>>();
        State { cells, pos, dir }
    }

    fn cell(&self, pos: Point) -> u8 {
        if pos.x < 0 || pos.x >= self.nx() || pos.y < 0 || pos.y >= self.ny() {
            b'.'
        } else {
            self.cells[pos.y as usize][pos.x as usize]
        }
    }

    fn nx(&self) -> i64 {
        self.cells[0].len() as i64
    }

    fn ny(&self) -> i64 {
        self.cells.len() as i64
    }
}

fn alignment_parameters_sum(state: &State) -> i64 {
    let mut sum = 0;
    for y in 1..(state.ny() - 1) {
        for x in 1..(state.nx() - 1) {
            let pos = Point::new(x, y);
            if state.cell(pos) == b'#' &&
                state.cell(pos + North.to_vector()) == b'#' &&
                state.cell(pos + East.to_vector()) == b'#' &&
                state.cell(pos + South.to_vector()) == b'#' &&
                state.cell(pos + West.to_vector()) == b'#'
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
    // println!("{}", state);
    alignment_parameters_sum(&state)
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 4044, part2, "TODO".to_string());
}
