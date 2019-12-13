use aoc::intcode::*;
use euclid;
use itertools::Itertools;
use num_derive::FromPrimitive;
use num_traits::cast::FromPrimitive;
use std::collections::HashMap;

struct Grid;
type Point = euclid::Point2D<i64, Grid>;
type Screen = HashMap::<Point, Tile>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
enum Tile {
    Empty = 0,
    Wall = 1,
    Block = 2,
    HorizontalPaddle = 3,
    Ball = 4,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result<> {
        use Tile::*;
        write!(f, "{}", match self {
            Empty => ' ',
            Wall => '█',
            Block => '▒', // '▩',
            HorizontalPaddle => '▬', // '═',
            Ball => '●',
        })
    }
}

fn screen_to_string(screen: &Screen) -> String {
    let (x_min, x_max) = screen.keys().map(|p| p.x).minmax().into_option().unwrap();
    let (y_min, y_max) = screen.keys().map(|p| p.y).minmax().into_option().unwrap();
    let mut out = String::with_capacity(((x_max - x_min + 1 + 1) * (y_max - y_min + 1)) as usize);
    for y in y_min ..= y_max {
        out.push_str("\n  ");
        for x in x_min ..= x_max {
            let cell = *screen.get(&Point::new(x, y)).unwrap_or(&Tile::Empty);
            out.push_str(&cell.to_string());
        }
    }
    out
}

fn part1(input: &str) -> usize {
    let mut state = Program::parse(input).run();
    let mut screen = Screen::new();
    while !state.is_halted() {
        let mut pos = Point::default();
        state = state.take_output(|val| pos.x = val);
        state = state.take_output(|val| pos.y = val);
        state = state.take_output(|val| {
            screen.insert(pos, Tile::from_i64(val).expect("Invalid tile value"));
        });
    }
    // println!("{}", screen_to_string(&screen));
    screen.values().filter(|&&cell| cell == Tile::Block).count()
}

#[allow(unused_assignments)]
fn part2(input: &str) -> Number {
    let mut program = Program::parse(input);
    program.mem[0] = 2;
    let mut screen = Screen::new();
    let mut state = program.run();
    let mut paddle_pos = Point::default();
    let mut ball_pos = Point::default();
    let mut score = 0;
    let mut first_render = true;
    loop {
        let mut render = false;
        match state {
            State::Reading(next) => {
                render = true;
                state = next((ball_pos.x - paddle_pos.x).signum());
            },
            State::Writing(_, _) => {
                let mut pos = Point::default();
                state = state.take_output(|val| pos.x = val);
                state = state.take_output(|val| pos.y = val);
                state = state.take_output(|val| {
                    if pos == Point::new(-1, 0) {
                        score = val;
                    } else {
                        let tile = Tile::from_i64(val).expect("Invalid tile value");
                        screen.insert(pos, tile);
                        if tile == Tile::HorizontalPaddle {
                            paddle_pos = pos;
                        } else if tile == Tile::Ball {
                            ball_pos = pos;
                        }
                    }
                });
            },
            State::Halted(_) => {
                render = true;
                break;
            }
        }
        if cfg!(feature = "render") && render {
            print!("{}[2J", 27 as char);
            println!("{}\n", screen_to_string(&screen));
            println!("  Score: {}\n", score);
            std::thread::sleep(std::time::Duration::from_millis(33));
            if first_render {
                std::thread::sleep(std::time::Duration::from_millis(10000));
                first_render = false;
            }
        }
    }
    score
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 290, part2, "TODO".to_string());
}
