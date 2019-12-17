use aoc::intcode::*;
use euclid;

struct Grid;
type Point = euclid::Point2D<usize, Grid>;

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn parse(c: u8) -> Option<Direction> {
        use Direction::*;
        match c {
            b'^' => Some(North),
            b'>' => Some(East),
            b'v' => Some(South),
            b'<' => Some(West),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    cells: Vec<Vec<u8>>,
    pos: Point,
    dir: Direction,
}

impl State {
    fn parse(ascii: &str) -> Self {
        let mut pos = Point::default();
        let mut dir = Direction::North;
        let cells = ascii.trim().lines()
            .enumerate()
            .map(|(row, chars)| {
                let mut chars = chars.trim().as_bytes().to_vec();
                for col in 0..chars.len() {
                    if let Some(d) = Direction::parse(chars[col]) {
                        pos = Point::new(col, row);
                        dir = d;
                        chars[col] = b'#';
                    }
                }
                chars
            })
            .collect::<Vec<_>>();
        State { cells, pos, dir }
    }

    fn cell(&self, row: usize, col: usize) -> u8 {
        self.cells[row][col]
    }

    fn num_rows(&self) -> usize {
        self.cells.len()
    }

    fn num_cols(&self) -> usize {
        self.cells[0].len()
    }
}

fn alignment_parameters_sum(ascii: &str) -> usize {
    let state = State::parse(ascii);

    let mut sum = 0;
    for row in 1..(state.num_rows() - 1) {
        for col in 1..(state.num_cols() - 1) {
            if state.cell(row, col) == b'#' &&
                state.cell(row - 1, col) == b'#' &&
                state.cell(row + 1, col) == b'#' &&
                state.cell(row, col - 1) == b'#' &&
                state.cell(row, col + 1) == b'#' {
                sum += row * col;
            }
        }
    }
    sum
}

#[test]
fn test_alignment_parameters_sum() {
    assert_eq!(
        alignment_parameters_sum("..#..........
                                  ..#..........
                                  #######...###
                                  #.#...#...#.#
                                  #############
                                  ..#...#...#..
                                  ..#####...^.."),
        76);
}

fn part1(input: &str) -> usize {
    let output = Program::parse(input).run_with_io(vec![]).output;
    let ascii = String::from_utf8(output.iter().map(|&val| val as u8).collect()).unwrap();
    alignment_parameters_sum(&ascii)
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
