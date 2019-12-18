use euclid;
use std::collections::{HashSet, VecDeque};

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Vector = euclid::Vector2D<i32, Grid>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct KeySet(usize);

impl KeySet {
    fn from_char(c: u8) -> Self {
        if c.is_ascii_lowercase() {
            KeySet(1 << ((c - b'a') as usize))
        } else {
            KeySet::default()
        }
    }

    fn opens_door(self, c: u8) -> bool {
        assert!(c.is_ascii_uppercase());
        self.0 & (1 << ((c - b'A') as usize)) != 0
    }
}

impl std::ops::Add<KeySet> for KeySet {
    type Output = KeySet;
    fn add(self, other: KeySet) -> KeySet {
        KeySet(self.0 | other.0)
    }
}

struct Map {
    cells: Vec<Vec<u8>>,
    start_pos: Point,
    all_keys: KeySet,
}

impl Map {
    fn parse(input: &str) -> Self {
        let cells = input.lines().map(|line| line.trim().as_bytes().to_vec()).collect::<Vec<_>>();
        let mut start_pos = Point::default();
        let mut all_keys = KeySet::default();
        for (y, row) in cells.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                match c {
                    b'@' => { start_pos = Point::new(x as i32, y as i32); },
                    &c if c.is_ascii_lowercase() => { all_keys = all_keys + KeySet::from_char(c); },
                    _ => {},
                }
            }
        }
        Map { cells, start_pos, all_keys }
    }

    fn is_passable(&self, state: &State) -> bool {
        match self[state.pos] {
            b'.' | b'@' => true,
            c if c.is_ascii_lowercase() => true,
            b'#' => false,
            c if c.is_ascii_uppercase() => state.keys.opens_door(c),
            c => panic!("Unknown map character {}", c),
        }
    }

    fn key_at(&self, pos: Point) -> KeySet {
        KeySet::from_char(self[pos])
    }
}

impl std::ops::Index<Point> for Map {
    type Output = u8;
    fn index(&self, pos: Point) -> &u8 {
        &self.cells[pos.y as usize][pos.x as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Point,
    keys: KeySet,
    steps: usize,
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let mut queue = VecDeque::new();
    queue.push_back(State {
        pos: map.start_pos,
        keys: KeySet::default(),
        steps: 0,
    });
    let mut visited = HashSet::new();
    while let Some(cur) = queue.pop_front() {
        if !visited.insert(cur) {
            continue;
        }
        if cur.keys == map.all_keys {
            return cur.steps;
        }
        for &step in &[
            Vector::new(-1, 0),
            Vector::new(1, 0),
            Vector::new(0, -1),
            Vector::new(0, 1),
        ] {
            let next_pos = cur.pos + step;
            let next = State {
                pos: next_pos,
                keys: cur.keys + map.key_at(next_pos),
                steps: cur.steps + 1,
            };
            if map.is_passable(&next) {
                queue.push_back(next);
            }
        }
    }
    panic!("No route found that collects all keys")
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("#########
               #b.A.@.a#
               #########"),
        8);
    assert_eq!(
        part1("########################
               #f.D.E.e.C.b.A.@.a.B.c.#
               ######################.#
               #d.....................#
               ########################"),
        86);
    assert_eq!(
        part1("########################
               #...............b.C.D.f#
               #.######################
               #.....@.a.B.c.d.A.e.F.g#
               ########################"),
        132);
    assert_eq!(
        part1("#################
               #i.G..c...e..H.p#
               ########.########
               #j.A..b...f..D.o#
               ########@########
               #k.E..a...g..B.n#
               ########.########
               #l.F..d...h..C.m#
               #################"),
        136);
    assert_eq!(
        part1("########################
               #@..............ac.GI.b#
               ###d#e#f################
               ###A#B#C################
               ###g#h#i################
               ########################"),
        81);
}

fn part2(_input: &str) -> String {
    "TODO".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    // aoc::test(part1, "TODO".to_string(), part2, "TODO".to_string());
}
