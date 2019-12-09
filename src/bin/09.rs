use aoc::intcode::*;

fn part1(input: &str) -> Number {
    let output = Program::new(Memory::parse(input)).with_input(vec![1]).run().output;
    assert_eq!(output.len(), 1);
    output[0]
}

fn part2(input: &str) -> Number {
    let output = Program::new(Memory::parse(input)).with_input(vec![2]).run().output;
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
