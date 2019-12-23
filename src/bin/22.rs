use aoc::math::*;

#[derive(Debug)]
struct Operation {
    mul: i128,
    add: i128,
    num_cards: i128,
}

impl Operation {
    fn identity(num_cards: usize) -> Self {
        Operation {
            mul: 1,
            add: 0,
            num_cards: num_cards as i128,
        }
    }

    fn deal_into_new_stack(num_cards: usize) -> Self {
        Operation {
            mul: num_cards as i128 - 1,
            add: num_cards as i128 - 1,
            num_cards: num_cards as i128,
        }
    }

    fn cut(num_cut: i64, num_cards: usize) -> Self {
        Operation {
            mul: 1,
            add: if num_cut >= 0 { num_cards as i64 - num_cut } else { -num_cut } as i128,
            num_cards: num_cards as i128,
        }
    }

    fn deal_with_increment(increment: usize, num_cards: usize) -> Self {
        Operation {
            mul: increment as i128,
            add: 0,
            num_cards: num_cards as i128,
        }
    }

    fn from_str(s: &str, num_cards: usize) -> Self {
        if s == "deal into new stack" {
            Self::deal_into_new_stack(num_cards)
        } else if s.starts_with("cut ") {
            Self::cut(s[4..].parse::<i64>().unwrap(), num_cards)
        } else if s.starts_with("deal with increment ") {
            Self::deal_with_increment(s[20..].parse::<usize>().unwrap(), num_cards)
        } else {
            panic!("Cannot parse \"{}\"", s);
        }
    }

    fn apply(&self, pos: usize) -> usize {
        (((pos as i128 * self.mul) + self.add) % self.num_cards) as usize
    }

    fn inverse(&self) -> Operation {
        let inv = inverse_mod_n(self.mul, self.num_cards);
        Operation {
            mul: inv,
            add: (-inv * self.add).rem_euclid(self.num_cards),
            num_cards: self.num_cards,
        }
    }

    fn then(&self, next: &Operation) -> Operation {
        Operation {
            mul: (next.mul * self.mul) % self.num_cards,
            add: (next.mul * self.add + next.add) % self.num_cards,
            num_cards: self.num_cards,
        }
    }
}

#[test]
fn test_apply() {
    let n = 10;
    assert_eq!(Operation::deal_into_new_stack(n).apply(0), 9);
    assert_eq!(Operation::deal_into_new_stack(n).apply(9), 0);
    assert_eq!(Operation::cut(3, n).apply(0), 7);
    assert_eq!(Operation::cut(3, n).apply(2), 9);
    assert_eq!(Operation::cut(3, n).apply(3), 0);
    assert_eq!(Operation::cut(3, n).apply(9), 6);
    assert_eq!(Operation::cut(-4, n).apply(0), 4);
    assert_eq!(Operation::cut(-4, n).apply(5), 9);
    assert_eq!(Operation::cut(-4, n).apply(6), 0);
    assert_eq!(Operation::cut(-4, n).apply(9), 3);
    assert_eq!(Operation::deal_with_increment(3, n).apply(0), 0);
    assert_eq!(Operation::deal_with_increment(3, n).apply(1), 3);
    assert_eq!(Operation::deal_with_increment(3, n).apply(8), 4);
    assert_eq!(Operation::deal_with_increment(3, n).apply(9), 7);
}

#[test]
fn test_inverse() {
    let n = 10;
    assert_eq!(Operation::deal_into_new_stack(n).inverse().apply(0), 9);
    assert_eq!(Operation::deal_into_new_stack(n).inverse().apply(9), 0);
    assert_eq!(Operation::cut(3, n).inverse().apply(7), 0);
    assert_eq!(Operation::cut(3, n).inverse().apply(9), 2);
    assert_eq!(Operation::cut(3, n).inverse().apply(0), 3);
    assert_eq!(Operation::cut(3, n).inverse().apply(6), 9);
    assert_eq!(Operation::cut(-4, n).inverse().apply(4), 0);
    assert_eq!(Operation::cut(-4, n).inverse().apply(9), 5);
    assert_eq!(Operation::cut(-4, n).inverse().apply(0), 6);
    assert_eq!(Operation::cut(-4, n).inverse().apply(3), 9);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(0), 0);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(1), 7);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(2), 4);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(3), 1);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(4), 8);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(5), 5);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(6), 2);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(7), 9);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(8), 6);
    assert_eq!(Operation::deal_with_increment(3, n).inverse().apply(9), 3);
}

fn part1(input: &str) -> usize {
    let num_cards = 10007;
    let ops = input.lines().map(|line| Operation::from_str(line.trim(), num_cards)).collect::<Vec<_>>();
    ops.iter().fold(2019, |pos, op| { op.apply(pos) })
}

fn card_in_position(final_pos: usize, num_cards: usize, mut num_iterations: usize, ops: &Vec<Operation>) -> usize {
    let single_iter_op = ops.iter().fold(Operation::identity(num_cards), |acc, op| {
        acc.then(op)
    });

    let mut power_op = single_iter_op;
    let mut total_op = Operation::identity(num_cards);
    while num_iterations > 0 {
        if num_iterations % 2 != 0 {
            total_op = total_op.then(&power_op);
        }
        power_op = power_op.then(&power_op);
        num_iterations /= 2;
    }
    total_op.inverse().apply(final_pos)
}

#[test]
fn test_card_in_position_small() {
    let n = 10;
    assert_eq!(card_in_position(0, n, 0, &vec![]), 0);
    assert_eq!(card_in_position(0, n, 1, &vec![]), 0);
    assert_eq!(card_in_position(0, n, 1000000000000, &vec![]), 0);

    let ops = vec![Operation::deal_into_new_stack(n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(0, n, 1, &ops), 9);
    assert_eq!(card_in_position(0, n, 2, &ops), 0);
    assert_eq!(card_in_position(0, n, 1000000000000, &ops), 0);

    let ops = vec![Operation::cut(1, n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(0, n, 1, &ops), 1);
    assert_eq!(card_in_position(0, n, 10, &ops), 0);
    assert_eq!(card_in_position(0, n, 1000000000000, &ops), 0);

    let ops = vec![Operation::deal_with_increment(3, n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(0, n, 1, &ops), 0);
    assert_eq!(card_in_position(0, n, 1000000000000, &ops), 0);
    assert_eq!(card_in_position(1, n, 0, &ops), 1);
    assert_eq!(card_in_position(1, n, 1, &ops), 7);
    assert_eq!(card_in_position(1, n, 2, &ops), 9);
    assert_eq!(card_in_position(1, n, 3, &ops), 3);
    assert_eq!(card_in_position(1, n, 4, &ops), 1);
    assert_eq!(card_in_position(1, n, 5, &ops), 7);
    assert_eq!(card_in_position(1, n, 6, &ops), 9);
    assert_eq!(card_in_position(1, n, 7, &ops), 3);
    assert_eq!(card_in_position(1, n, 8, &ops), 1);
    assert_eq!(card_in_position(1, n, 1000000000000, &ops), 1);
    assert_eq!(card_in_position(1, n, 1000000000001, &ops), 7);
    assert_eq!(card_in_position(1, n, 1000000000002, &ops), 9);
    assert_eq!(card_in_position(1, n, 1000000000003, &ops), 3);

    let ops = vec![
        Operation::deal_with_increment(3, n),
        Operation::deal_into_new_stack(n),
        Operation::cut(3, n),
    ];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(0, n, 1, &ops), 2);
    assert_eq!(card_in_position(1, n, 1, &ops), 5);
    assert_eq!(card_in_position(8, n, 1, &ops), 6);
    assert_eq!(card_in_position(9, n, 1, &ops), 9);
    assert_eq!(card_in_position(0, n, 2, &ops), 8);
    assert_eq!(card_in_position(1, n, 2, &ops), 7);
    assert_eq!(card_in_position(8, n, 2, &ops), 0);
    assert_eq!(card_in_position(9, n, 2, &ops), 9);
}

#[test]
fn test_card_in_position_large() {
    let n = 119_315_717_514_047;

    let ops = vec![Operation::deal_into_new_stack(n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(n - 1, n, 0, &ops), n - 1);
    assert_eq!(card_in_position(0, n, 1, &ops), n - 1);
    assert_eq!(card_in_position(n - 1, n, 1, &ops), 0);
    assert_eq!(card_in_position(0, n, 1_000_000_000_000, &ops), 0);
    assert_eq!(card_in_position(n - 1, n, 1_000_000_000_000, &ops), n - 1);
    assert_eq!(card_in_position(0, n, 1_000_000_000_001, &ops), n - 1);
    assert_eq!(card_in_position(n - 1, n, 1_000_000_000_001, &ops), 0);

    let ops = vec![Operation::cut(100_000_000_000_000, n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(n - 1, n, 0, &ops), n - 1);
    assert_eq!(card_in_position(0, n, 1, &ops), 100_000_000_000_000);
    assert_eq!(card_in_position(n - 1, n, 1, &ops), 99_999_999_999_999);

    let ops = vec![Operation::deal_with_increment(3, n)];
    assert_eq!(card_in_position(0, n, 0, &ops), 0);
    assert_eq!(card_in_position(n - 1, n, 0, &ops), n - 1);
    assert_eq!(card_in_position(0, n, 1, &ops), 0);
    assert_eq!(card_in_position(1, n, 1, &ops), n / 3 + 1);
}

fn part2(input: &str) -> usize {
    let num_cards = 119315717514047;
    let num_iterations = 101741582076661;
    let final_pos = 2020;
    let ops = input.lines().map(|line| Operation::from_str(line.trim(), num_cards)).collect::<Vec<_>>();
    card_in_position(final_pos, num_cards, num_iterations, &ops)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 5540, part2, 6821410630991);
}
