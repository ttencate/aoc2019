use aoc::intcode::*;
use std::io::{BufRead, Write};

fn part1(input: &str) -> String {
    let mut program = Program::parse(input);
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    loop {
        match program.run_until_interrupt() {
            Interrupt::Reading => {
                let mut line = String::new();
                stdin.read_line(&mut line).unwrap();
                for &c in line.as_bytes() {
                    program.give_input(c as Number);
                }
            },
            Interrupt::Writing => {
                let output = program.take_output();
                write!(stdout, "{}", output as u8 as char).unwrap();
            },
            Interrupt::Halted => {
                return "TODO".to_string();
            },
        }
    }
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, "TODO".to_string(), part2, "TODO".to_string());
}
