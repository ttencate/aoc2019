use aoc::intcode::*;
use std::io::{BufRead, Write};

fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let file_name = args.next().expect("Give file name of intcode program on the command line");
    let mut program = Program::parse(&std::fs::read_to_string(&file_name).expect("Could not read intcode program"));
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    loop {
        match program.run_until_interrupt() {
            Interrupt::Reading => {
                let mut line = String::new();
                stdin.read_line(&mut line).unwrap();
                program.give_input_ascii(&line);
            },
            Interrupt::Writing => {
                let output = program.take_output_ascii();
                write!(stdout, "{}", output).unwrap();
            },
            Interrupt::Halted => {
                return;
            },
        }
    }
}
