use aoc::intcode::*;

fn part1(input: &str) -> usize {
    let program = Program::parse(input);
    let mut count = 0;
    for y in 0..50 {
        for x in 0..50 {
            let (attracted, _) = program.clone()
                .run().give_input(x)
                .run().give_input(y)
                .run().take_output();
            if attracted != 0 {
                count += 1;
            }
        }
    }
    count
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 131, part2, "TODO".to_string());
}
