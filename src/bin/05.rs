use aoc::intcode::*;

fn part1(input: &str) -> Number {
    *Program::parse(input).run_with_io(vec![1]).last().unwrap()
}

fn part2(input: &str) -> Number {
    *Program::parse(input).run_with_io(vec![5]).last().unwrap()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 13978427, part2, 11189491);
}
