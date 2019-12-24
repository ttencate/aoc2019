use std::collections::HashSet;

type State = usize;

fn parse_input(input: &str) -> State {
    input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line
                .trim()
                .as_bytes()
                .iter()
                .enumerate()
                .map(move |(x, b)| if *b == b'#' { 1 << (5 * y + x) } else { 0 })
        })
        .flatten()
        .fold(0, |a, b| a | b)
}

fn next_bit(state: State, bit: State, num_neigh: u32) -> State {
    if state & bit != 0 {
        if num_neigh == 1 { bit } else { 0 }
    } else {
        if num_neigh == 1 || num_neigh == 2 { bit } else { 0 }
    }
}

const NEIGH_MASK: [State; 25] = [
    0b00000_00000_00000_00001_00010,
    0b00000_00000_00000_00010_00101,
    0b00000_00000_00000_00100_01010,
    0b00000_00000_00000_01000_10100,
    0b00000_00000_00000_10000_01000,

    0b00000_00000_00001_00010_00001,
    0b00000_00000_00010_00101_00010,
    0b00000_00000_00100_01010_00100,
    0b00000_00000_01000_10100_01000,
    0b00000_00000_10000_01000_10000,

    0b00000_00001_00010_00001_00000,
    0b00000_00010_00101_00010_00000,
    0b00000_00100_01010_00100_00000,
    0b00000_01000_10100_01000_00000,
    0b00000_10000_01000_10000_00000,

    0b00001_00010_00001_00000_00000,
    0b00010_00101_00010_00000_00000,
    0b00100_01010_00100_00000_00000,
    0b01000_10100_01000_00000_00000,
    0b10000_01000_10000_00000_00000,

    0b00010_00001_00000_00000_00000,
    0b00101_00010_00000_00000_00000,
    0b01010_00100_00000_00000_00000,
    0b10100_01000_00000_00000_00000,
    0b01000_10000_00000_00000_00000,
];

fn first_repeated_state(mut state: State) -> State {
    let mut visited = HashSet::new();
    loop {
        if !visited.insert(state) {
            break state;
        }
        state = (0..25)
            .map(|i| next_bit(state, 1 << i, (state & NEIGH_MASK[i]).count_ones()))
            .fold(0, |a, b| a | b);
    }
}

#[test]
fn test_first_repeated_state() {
    assert_eq!(
        first_repeated_state(parse_input(
            "....#
             #..#.
             #..##
             ..#..
             #....")),
        2129920);
}

fn part1(input: &str) -> usize {
    first_repeated_state(parse_input(input))
}

const RECURSIVE_NEIGH_MASK: [State; 25] = [
    // -----outer-----  ------------inner------------  -----------current-----------
    // S    E W    N
    0b_000_00010_00100__00000_00000_00000_00000_00000__00000_00000_00000_00001_00010,
    0b_000_00000_00100__00000_00000_00000_00000_00000__00000_00000_00000_00010_00101,
    0b_000_00000_00100__00000_00000_00000_00000_00000__00000_00000_00000_00100_01010,
    0b_000_00000_00100__00000_00000_00000_00000_00000__00000_00000_00000_01000_10100,
    0b_000_01000_00100__00000_00000_00000_00000_00000__00000_00000_00000_10000_01000,

    0b_000_00010_00000__00000_00000_00000_00000_00000__00000_00000_00001_00010_00001,
    0b_000_00000_00000__00000_00000_00000_00000_00000__00000_00000_00010_00101_00010,
    0b_000_00000_00000__00000_00000_00000_00000_11111__00000_00000_00000_01010_00100,
    0b_000_00000_00000__00000_00000_00000_00000_00000__00000_00000_01000_10100_01000,
    0b_000_01000_00000__00000_00000_00000_00000_00000__00000_00000_10000_01000_10000,

    0b_000_00010_00000__00000_00000_00000_00000_00000__00000_00001_00010_00001_00000,
    0b_000_00000_00000__00001_00001_00001_00001_00001__00000_00010_00001_00010_00000,
    0b_000_00000_00000__00000_00000_00000_00000_00000__00000_00000_00000_00000_00000,
    0b_000_00000_00000__10000_10000_10000_10000_10000__00000_01000_10000_01000_00000,
    0b_000_01000_00000__00000_00000_00000_00000_00000__00000_10000_01000_10000_00000,

    0b_000_00010_00000__00000_00000_00000_00000_00000__00001_00010_00001_00000_00000,
    0b_000_00000_00000__00000_00000_00000_00000_00000__00010_00101_00010_00000_00000,
    0b_000_00000_00000__11111_00000_00000_00000_00000__00100_01010_00000_00000_00000,
    0b_000_00000_00000__00000_00000_00000_00000_00000__01000_10100_01000_00000_00000,
    0b_000_01000_00000__00000_00000_00000_00000_00000__10000_01000_10000_00000_00000,

    0b_100_00010_00000__00000_00000_00000_00000_00000__00010_00001_00000_00000_00000,
    0b_100_00000_00000__00000_00000_00000_00000_00000__00101_00010_00000_00000_00000,
    0b_100_00000_00000__00000_00000_00000_00000_00000__01010_00100_00000_00000_00000,
    0b_100_00000_00000__00000_00000_00000_00000_00000__10100_01000_00000_00000_00000,
    0b_100_01000_00000__00000_00000_00000_00000_00000__01000_10000_00000_00000_00000,
];

fn num_bugs_after(initial_state: State, num_iterations: usize) -> usize {
    let max_level = 3 + num_iterations;
    let initial_level = max_level / 2;
    let mut state: Vec<State> = vec![0; max_level];
    state[initial_level] = initial_state;
    for iter in 0..num_iterations {
        state = (0..max_level)
            .map(|level| {
                let expanse = iter / 2 + 1;
                if level < initial_level - expanse || level > initial_level + expanse {
                    return 0;
                }
                let current_level = state[level];
                let inner_level = state[level + 1];
                let outer_level = state[level - 1];
                let outer_part = outer_level >> 5;
                let env =
                    current_level |
                    inner_level << 25 |
                    outer_part << 50;
                next_bit(env, 1 <<  0, (env & RECURSIVE_NEIGH_MASK[0]).count_ones()) |
                next_bit(env, 1 <<  1, (env & RECURSIVE_NEIGH_MASK[1]).count_ones()) |
                next_bit(env, 1 <<  2, (env & RECURSIVE_NEIGH_MASK[2]).count_ones()) |
                next_bit(env, 1 <<  3, (env & RECURSIVE_NEIGH_MASK[3]).count_ones()) |
                next_bit(env, 1 <<  4, (env & RECURSIVE_NEIGH_MASK[4]).count_ones()) |
                next_bit(env, 1 <<  5, (env & RECURSIVE_NEIGH_MASK[5]).count_ones()) |
                next_bit(env, 1 <<  6, (env & RECURSIVE_NEIGH_MASK[6]).count_ones()) |
                next_bit(env, 1 <<  7, (env & RECURSIVE_NEIGH_MASK[7]).count_ones()) |
                next_bit(env, 1 <<  8, (env & RECURSIVE_NEIGH_MASK[8]).count_ones()) |
                next_bit(env, 1 <<  9, (env & RECURSIVE_NEIGH_MASK[9]).count_ones()) |
                next_bit(env, 1 << 10, (env & RECURSIVE_NEIGH_MASK[10]).count_ones()) |
                next_bit(env, 1 << 11, (env & RECURSIVE_NEIGH_MASK[11]).count_ones()) |
                // next_bit(env, 1 << 12, (env & RECURSIVE_NEIGH_MASK[12]).count_ones()) |
                next_bit(env, 1 << 13, (env & RECURSIVE_NEIGH_MASK[13]).count_ones()) |
                next_bit(env, 1 << 14, (env & RECURSIVE_NEIGH_MASK[14]).count_ones()) |
                next_bit(env, 1 << 15, (env & RECURSIVE_NEIGH_MASK[15]).count_ones()) |
                next_bit(env, 1 << 16, (env & RECURSIVE_NEIGH_MASK[16]).count_ones()) |
                next_bit(env, 1 << 17, (env & RECURSIVE_NEIGH_MASK[17]).count_ones()) |
                next_bit(env, 1 << 18, (env & RECURSIVE_NEIGH_MASK[18]).count_ones()) |
                next_bit(env, 1 << 19, (env & RECURSIVE_NEIGH_MASK[19]).count_ones()) |
                next_bit(env, 1 << 20, (env & RECURSIVE_NEIGH_MASK[20]).count_ones()) |
                next_bit(env, 1 << 21, (env & RECURSIVE_NEIGH_MASK[21]).count_ones()) |
                next_bit(env, 1 << 22, (env & RECURSIVE_NEIGH_MASK[22]).count_ones()) |
                next_bit(env, 1 << 23, (env & RECURSIVE_NEIGH_MASK[23]).count_ones()) |
                next_bit(env, 1 << 24, (env & RECURSIVE_NEIGH_MASK[24]).count_ones())
            })
            .collect();
    }
    state.iter().map(|s| s.count_ones() as usize).sum()
}

#[test]
fn test_num_bugs_after() {
    assert_eq!(
        num_bugs_after(
            parse_input("....#
                         #..#.
                         #.?##
                         ..#..
                         #...."),
            10),
        99);
}

fn part2(input: &str) -> usize {
    num_bugs_after(parse_input(input), 200)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 27777901, part2, 2047);
}
