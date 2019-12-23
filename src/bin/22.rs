use aoc::math::*;

#[derive(Debug, Copy, Clone)]
enum Operation {
    DealIntoNewStack(usize),
    Cut(usize, usize),
    DealWithIncrement(usize, usize),
}

use Operation::*;

impl Operation {
    fn deal_into_new_stack(num_cards: usize) -> Self {
        DealIntoNewStack(num_cards)
    }

    fn cut(num_cut: i64, num_cards: usize) -> Self {
        let abs_num_cut = if num_cut >= 0 { num_cut } else { num_cards as i64 + num_cut } as usize;
        Cut(abs_num_cut, num_cards)
    }

    fn deal_with_increment(increment: usize, num_cards: usize) -> Self {
        DealWithIncrement(increment, num_cards)
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

    fn forward(self, pos: usize) -> usize {
        match self {
            DealIntoNewStack(num_cards) => {
                num_cards - pos - 1
            },
            Cut(num_cut, num_cards) => {
                (pos + num_cards - num_cut) % num_cards
            },
            DealWithIncrement(increment, num_cards) => {
                (pos * increment) % num_cards
            },
        }
    }

    #[cfg(test)]
    fn reverse(self, pos: usize) -> usize {
        self.reverse_as_mul_add().apply(pos, self.num_cards())
    }

    #[cfg(test)]
    fn num_cards(self) -> usize {
        match self {
            DealIntoNewStack(num_cards) => num_cards,
            Cut(_, num_cards) => num_cards,
            DealWithIncrement(_, num_cards) => num_cards,
        }
    }

    fn reverse_as_mul_add(self) -> MulAdd {
        match self {
            DealIntoNewStack(num_cards) => MulAdd { mul: num_cards as u128 - 1, add: num_cards as u128 - 1 },
            Cut(num_cut, _) => MulAdd { mul: 1, add: num_cut as u128 },
            DealWithIncrement(increment, num_cards) => MulAdd { mul: inverse_mod_n(increment, num_cards) as u128, add: 0 },
        }
    }
}

#[test]
fn test_forward() {
    let n = 10;
    assert_eq!(Operation::deal_into_new_stack(n).forward(0), 9);
    assert_eq!(Operation::deal_into_new_stack(n).forward(9), 0);
    assert_eq!(Operation::cut(3, n).forward(0), 7);
    assert_eq!(Operation::cut(3, n).forward(2), 9);
    assert_eq!(Operation::cut(3, n).forward(3), 0);
    assert_eq!(Operation::cut(3, n).forward(9), 6);
    assert_eq!(Operation::cut(-4, n).forward(0), 4);
    assert_eq!(Operation::cut(-4, n).forward(5), 9);
    assert_eq!(Operation::cut(-4, n).forward(6), 0);
    assert_eq!(Operation::cut(-4, n).forward(9), 3);
    assert_eq!(Operation::deal_with_increment(3, n).forward(0), 0);
    assert_eq!(Operation::deal_with_increment(3, n).forward(1), 3);
    assert_eq!(Operation::deal_with_increment(3, n).forward(8), 4);
    assert_eq!(Operation::deal_with_increment(3, n).forward(9), 7);
}

#[test]
fn test_reverse() {
    let n = 10;
    assert_eq!(Operation::deal_into_new_stack(n).reverse(0), 9);
    assert_eq!(Operation::deal_into_new_stack(n).reverse(9), 0);
    assert_eq!(Operation::cut(3, n).reverse(7), 0);
    assert_eq!(Operation::cut(3, n).reverse(9), 2);
    assert_eq!(Operation::cut(3, n).reverse(0), 3);
    assert_eq!(Operation::cut(3, n).reverse(6), 9);
    assert_eq!(Operation::cut(-4, n).reverse(4), 0);
    assert_eq!(Operation::cut(-4, n).reverse(9), 5);
    assert_eq!(Operation::cut(-4, n).reverse(0), 6);
    assert_eq!(Operation::cut(-4, n).reverse(3), 9);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(0), 0);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(1), 7);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(2), 4);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(3), 1);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(4), 8);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(5), 5);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(6), 2);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(7), 9);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(8), 6);
    assert_eq!(Operation::deal_with_increment(3, n).reverse(9), 3);
}

fn part1(input: &str) -> usize {
    let num_cards = 10007;
    let ops = input.lines().map(|line| Operation::from_str(line.trim(), num_cards)).collect::<Vec<_>>();
    ops.iter().fold(2019, |pos, op| { op.forward(pos) })
}

#[derive(Debug, Copy, Clone)]
struct MulAdd {
    mul: u128,
    add: u128,
}

impl Default for MulAdd {
    fn default() -> Self {
        MulAdd { mul: 1, add: 0 }
    }
}

impl MulAdd {
    fn apply(&self, pos: usize, num_cards: usize) -> usize {
        ((pos as u128 * self.mul + self.add) % num_cards as u128) as usize
    }

    fn then(&self, other: MulAdd, num_cards: usize) -> MulAdd {
        MulAdd {
            mul: (self.mul * other.mul) % num_cards as u128,
            add: (self.mul * other.add + self.add) % num_cards as u128,
        }
    }
}

fn card_in_position(final_pos: usize, num_cards: usize, mut num_iterations: usize, ops: &Vec<Operation>) -> usize {
    let single_iter_mul_add = ops.iter().rev().fold(MulAdd::default(), |acc, op| {
        acc.then(op.reverse_as_mul_add(), num_cards)
    });

    let mut power_mul_add = single_iter_mul_add;
    let mut total_mul_add = MulAdd::default();
    while num_iterations > 0 {
        if num_iterations % 2 != 0 {
            total_mul_add = total_mul_add.then(power_mul_add, num_cards);
        }
        power_mul_add = power_mul_add.then(power_mul_add, num_cards);
        num_iterations /= 2;
    }
    total_mul_add.apply(final_pos, num_cards)
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

    let reverse_pos = card_in_position(final_pos, num_cards, 1, &ops);
    let forward_reverse_pos = ops.iter().fold(reverse_pos, |pos, op| op.forward(pos));
    assert_eq!(forward_reverse_pos, final_pos);

    card_in_position(final_pos, num_cards, num_iterations, &ops)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 5540, part2, "TODO".to_string());
}
