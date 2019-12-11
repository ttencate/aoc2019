use aoc::intcode::*;

fn part1(input: &str) -> Number {
    let output = Program::parse(input).run_with_io(vec![1]).output;
    assert_eq!(output.len(), 1);
    output[0]
}

fn part2(input: &str) -> Number {
    let output = Program::parse(input).run_with_io(vec![2]).output;
    assert_eq!(output.len(), 1);
    output[0]
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 3497884671, part2, 46470);
}
