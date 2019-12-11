use aoc::intcode::*;
use aoc::permute::*;

fn part1(input: &str) -> Number {
    let program = Program::parse(input);
    (0..=4)
        .permute()
        .map(|p| {
            let mut output = 0;
            for i in 0..5 {
                output = program.clone().run_with_io(vec![p[i], output]).output[0];
            }
            output
        })
        .max()
        .unwrap()
}

#[test]
fn test_part1() {
    assert_eq!(part1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 43210);
    assert_eq!(part1("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 54321);
    assert_eq!(part1("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 65210);
}

fn part2(input: &str) -> Number {
    let program = Program::parse(input);
    (5..=9)
        .permute()
        .map(|p| {
            let mut states = p.iter().map(|&i| {
                Some(match program.clone().run() {
                    State::Reading(next) => next(i),
                    _ => panic!("Expected machine to read input first"),
                })
            }).collect::<Vec<_>>();

            let mut output = 0;
            let mut i = 0;
            loop {
                let state = states[i].take().unwrap();
                let state = match state {
                    State::Reading(next) => next(output),
                    State::Halted(_) => break,
                    _ => panic!("Expected machine to read input"),
                };
                let state = match state {
                    State::Writing(val, next) => {
                        output = val;
                        next()
                    },
                    State::Halted(_) => break,
                    _ => panic!("Expected machine to write output"),
                };
                states[i].replace(state);
                i = (i + 1) % 5;
            }
            output
        })
        .max()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"), 139629729);
    assert_eq!(part2("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"), 18216);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 18812, part2, 25534964);
}
