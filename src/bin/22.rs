use aoc::math::*;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Operation {
    DealIntoNewStack(usize),
    Cut(usize, usize),
    DealWithIncrement(usize, usize, usize),
}

use Operation::*;

impl Operation {
    fn deal_into_new_stack(num_cards: usize) -> Self {
        DealIntoNewStack(num_cards)
    }

    fn cut(num_cut: i32, num_cards: usize) -> Self {
        let abs_num_cut = if num_cut >= 0 { num_cut } else { num_cards as i32 + num_cut } as usize;
        let inverse_num_cut = num_cards - abs_num_cut;
        Cut(abs_num_cut, inverse_num_cut)
    }

    fn deal_with_increment(increment: usize, num_cards: usize) -> Self {
        let inverse_increment = inverse_mod_n(increment as i128, num_cards as i128) as usize;
        DealWithIncrement(increment, inverse_increment, num_cards)
    }

    fn from_str(s: &str, num_cards: usize) -> Self {
        if s == "deal into new stack" {
            Self::deal_into_new_stack(num_cards)
        } else if s.starts_with("cut ") {
            Self::cut(s[4..].parse::<i32>().unwrap(), num_cards)
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
            Cut(num_cut, inverse_cut) => {
                if pos < num_cut {
                    pos + inverse_cut
                } else {
                    pos - num_cut
                }
            },
            DealWithIncrement(increment, _, num_cards) => {
                ((pos as u128 * increment as u128) % num_cards as u128) as usize
            },
        }
    }

    fn reverse(self, pos: usize) -> usize {
        match self {
            DealIntoNewStack(num_cards) => {
                num_cards - pos - 1
            },
            Cut(num_cut, inverse_cut) => {
                if pos < inverse_cut {
                    pos + num_cut
                } else {
                    pos - inverse_cut
                }
            },
            DealWithIncrement(_, inverse_increment, num_cards) => {
                ((pos as u128 * inverse_increment as u128) % num_cards as u128) as usize
            },
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

fn reverse_trace_card(final_pos: usize, num_iterations: usize, ops: &Vec<Operation>) -> usize {
    let mut pos = final_pos;
    let mut i = 0;
    let mut visited = HashSet::new();
    let cycle_length = loop {
        if !visited.insert(pos) {
            break i;
        }
        pos = ops.iter().rev().fold(pos, |pos, op| { op.reverse(pos) });
        i += 1;
    };

    let mut pos = final_pos;
    for _ in 0..(num_iterations % cycle_length) {
        pos = ops.iter().rev().fold(pos, |pos, op| { op.reverse(pos) });
    }
    pos
}

#[test]
fn test_reverse_trace_card() {
    let n = 10;
    assert_eq!(reverse_trace_card(0, 0, &vec![]), 0);
    assert_eq!(reverse_trace_card(0, 1, &vec![]), 0);
    assert_eq!(reverse_trace_card(0, 1000000000000, &vec![]), 0);

    let ops = vec![Operation::deal_into_new_stack(n)];
    assert_eq!(reverse_trace_card(0, 0, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1, &ops), 9);
    assert_eq!(reverse_trace_card(0, 2, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1000000000000, &ops), 0);

    let ops = vec![Operation::cut(1, n)];
    assert_eq!(reverse_trace_card(0, 0, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1, &ops), 1);
    assert_eq!(reverse_trace_card(0, 10, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1000000000000, &ops), 0);

    let ops = vec![Operation::deal_with_increment(3, n)];
    assert_eq!(reverse_trace_card(0, 0, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1, &ops), 0);
    assert_eq!(reverse_trace_card(0, 1000000000000, &ops), 0);
    assert_eq!(reverse_trace_card(1, 0, &ops), 1);
    assert_eq!(reverse_trace_card(1, 1, &ops), 7);
    assert_eq!(reverse_trace_card(1, 2, &ops), 9);
    assert_eq!(reverse_trace_card(1, 3, &ops), 3);
    assert_eq!(reverse_trace_card(1, 4, &ops), 1);
    assert_eq!(reverse_trace_card(1, 5, &ops), 7);
    assert_eq!(reverse_trace_card(1, 6, &ops), 9);
    assert_eq!(reverse_trace_card(1, 7, &ops), 3);
    assert_eq!(reverse_trace_card(1, 8, &ops), 1);
    assert_eq!(reverse_trace_card(1, 1000000000000, &ops), 1);
    assert_eq!(reverse_trace_card(1, 1000000000001, &ops), 7);
    assert_eq!(reverse_trace_card(1, 1000000000002, &ops), 9);
    assert_eq!(reverse_trace_card(1, 1000000000003, &ops), 3);
}

fn part2(input: &str) -> usize {
    let num_cards = 119315717514047;
    let num_iterations = 101741582076661;
    let final_pos = 2020;
    let ops = input.lines().map(|line| Operation::from_str(line.trim(), num_cards)).collect::<Vec<_>>();
    reverse_trace_card(final_pos, num_iterations, &ops)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, 5540, part2, "TODO".to_string());
}
