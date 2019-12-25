use aoc::intcode::*;

fn part1(input: &str) -> i64 {
    let mut program = Program::parse(input);
    program.mem[1] = 12;
    program.mem[2] = 2;
    program.run_without_io();
    program.mem[0]
}

fn part2(input: &str) -> i64 {
    let program = Program::parse(input);
    let mut max = 1;
    loop {
        for noun in 0..max {
            for verb in 0..max {
                let mut p = program.clone();
                p.mem[1] = noun;
                p.mem[2] = verb;
                p.run_without_io();
                if p.mem[0] == 19690720 {
                    return 100 * noun + verb;
                }
            }
        }
        max *= 10;
    }
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 4570637, part2, 5485);
}
