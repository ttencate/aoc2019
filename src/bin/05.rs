use aoc::intcode::*;

fn part1(input: &str) -> Number {
    Program::new(Memory::parse(input))
        .with_input(vec![1]).run()
        .output.pop_back().unwrap()
}

fn part2(input: &str) -> Number {
    Program::new(Memory::parse(input))
        .with_input(vec![5]).run()
        .output.pop_back().unwrap()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 13978427, part2, 11189491);
}
