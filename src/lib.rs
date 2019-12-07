use std::time::Instant;
use std::env;
use std::fmt::{Debug, Display};

mod input;
pub mod intcode;
pub mod permute;

const YEAR: u32 = 2019;

fn get_day() -> u32 {
    env::current_exe().unwrap()
        .file_stem().unwrap()
        .to_str().unwrap()
        .get(0..2).unwrap()
        .parse::<u32>().unwrap()
}

pub fn main<P1, P2, R1, R2>(part1: P1, part2: P2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Display, R2: Display
{
    let day = get_day();
    let input = input::get_input(YEAR, day);
    run(day, 1, part1, &input);
    run(day, 2, part2, &input);
}

fn run<P, R>(day: u32, part: u32, func: P, input: &str)
    where P: Fn(&str) -> R, R: Display
{
    let start = Instant::now();
    let output = func(input);
    let duration = start.elapsed();

    println!("Answer to day {}, part {} ({}.{:03} s): {}", day, part, duration.as_secs(), duration.subsec_millis(), output);
}

pub fn test<P1, P2, R1, R2>(part1: P1, answer1: R1, part2: P2, answer2: R2)
    where P1: Fn(&str) -> R1, P2: Fn(&str) -> R2, R1: Debug + PartialEq, R2: Debug + PartialEq
{
    let day = get_day();
    let input = input::get_input(YEAR, day);
    assert_eq!(part1(&input), answer1);
    assert_eq!(part2(&input), answer2);
}
