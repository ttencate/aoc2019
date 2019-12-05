use aoc::intcode::*;

fn part1(input: &str) -> Number {
    *Program::new(parse_mem(input))
        .with_input(vec![1]).run()
        .output.last().unwrap()
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 13978427, part2, "TODO".to_string());
}
