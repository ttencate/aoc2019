#[derive(Debug, Copy, Clone)]
enum Operation {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(usize),
}

use Operation::*;

impl Operation {
    fn apply_to_pos(self, pos: usize, num_cards: usize) -> usize {
        match self {
            DealIntoNewStack => num_cards - pos - 1,
            Cut(num_cut) => {
                let abs_num_cut = if num_cut >= 0 { num_cut } else { num_cards as i32 + num_cut } as usize;
                if pos < abs_num_cut {
                    pos + num_cards - abs_num_cut
                } else {
                    pos - abs_num_cut
                }
            },
            DealWithIncrement(increment) => {
                (pos * increment) % num_cards
            },
        }
    }
}

#[test]
fn test_apply_to_pos() {
    assert_eq!(DealIntoNewStack.apply_to_pos(0, 10), 9);
    assert_eq!(DealIntoNewStack.apply_to_pos(9, 10), 0);
    assert_eq!(Cut(3).apply_to_pos(0, 10), 7);
    assert_eq!(Cut(3).apply_to_pos(2, 10), 9);
    assert_eq!(Cut(3).apply_to_pos(3, 10), 0);
    assert_eq!(Cut(3).apply_to_pos(9, 10), 6);
    assert_eq!(Cut(-4).apply_to_pos(0, 10), 4);
    assert_eq!(Cut(-4).apply_to_pos(5, 10), 9);
    assert_eq!(Cut(-4).apply_to_pos(6, 10), 0);
    assert_eq!(Cut(-4).apply_to_pos(9, 10), 3);
    assert_eq!(DealWithIncrement(3).apply_to_pos(0, 10), 0);
    assert_eq!(DealWithIncrement(3).apply_to_pos(1, 10), 3);
    assert_eq!(DealWithIncrement(3).apply_to_pos(8, 10), 4);
    assert_eq!(DealWithIncrement(3).apply_to_pos(9, 10), 7);
}

impl std::str::FromStr for Operation {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "deal into new stack" {
            Ok(DealIntoNewStack)
        } else if s.starts_with("cut ") {
            Ok(Cut(s[4..].parse::<i32>().map_err(|err| format!("{}", err))?))
        } else if s.starts_with("deal with increment ") {
            Ok(DealWithIncrement(s[20..].parse::<usize>().map_err(|err| format!("{}", err))?))
        } else {
            Err(s.to_string())
        }
    }
}

fn part1(input: &str) -> usize {
    let ops = input.lines().map(|line| line.trim().parse::<Operation>().unwrap()).collect::<Vec<_>>();
    let num_cards = 10007;
    ops.iter().fold(2019, |pos, op| { op.apply_to_pos(pos, num_cards) })
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 5540, part2, "TODO".to_string());
}
