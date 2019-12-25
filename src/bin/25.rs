use aoc::intcode::*;
use std::io::{BufRead, Write};

fn part1(input: &str) -> String {
    let mut program = Program::parse(input);
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    loop {
        match program.run() {
            mut interrupt @ Interrupt::Reading(_) => {
                let mut line = String::new();
                stdin.read_line(&mut line).unwrap();
                for &c in line.trim().as_bytes() {
                    interrupt = interrupt.give_input(c as Number).run();
                }
                program = interrupt.give_input('\n' as Number);
            },
            Interrupt::Writing(val, next) => {
                write!(stdout, "{}", val as u8 as char).unwrap();
                program = next();
            },
            Interrupt::Halted(_) => {
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
