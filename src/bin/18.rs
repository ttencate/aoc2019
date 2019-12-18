use euclid;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Vector = euclid::Vector2D<i32, Grid>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct KeySet(usize);

impl KeySet {
    fn opens_door(self, c: u8) -> bool {
        assert!(c.is_ascii_uppercase());
        self.0 & (1 << ((c - b'A') as usize)) != 0
    }
}

impl From<u8> for KeySet {
    fn from(c: u8) -> Self {
        if c.is_ascii_lowercase() {
            KeySet(1 << ((c - b'a') as usize))
        } else {
            KeySet::default()
        }
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
    all_keys: KeySet,
}

impl Map {
    fn parse(input: &str) -> Self {
        let cells = input.lines().map(|line| line.trim().as_bytes().to_vec()).collect::<Vec<_>>();
        let mut all_keys = KeySet::default();
        for row in cells.iter() {
            for &c in row.iter() {
                all_keys = all_keys + KeySet::from(c);
            }
        }
        Map { cells, all_keys }
    }

    fn nx(&self) -> i32 {
        self.cells[0].len() as i32
    }

    fn ny(&self) -> i32 {
        self.cells.len() as i32
    }
}

impl std::ops::Index<Point> for Map {
    type Output = u8;
    fn index(&self, pos: Point) -> &u8 {
        &self.cells[pos.y as usize][pos.x as usize]
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
    Start,
    Key(u8),
    Door(u8),
}

impl From<u8> for Node {
    fn from(c: u8) -> Self {
        match c {
            b'@' => Node::Start,
            c if c.is_ascii_lowercase() => Node::Key(c),
            c if c.is_ascii_uppercase() => Node::Door(c),
            _ => panic!("Unknown node character {}", c as char),
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            Node::Start => b'@',
            Node::Key(key) => *key,
            Node::Door(door) => *door,
        } as char)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    keys: KeySet,
    steps: usize,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        other.steps.cmp(&self.steps)
    }
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);

    let mut distances: HashMap<Node, HashMap<Node, usize>> = HashMap::new();
    for y in 0..map.ny() {
        for x in 0..map.nx() {
            let start_point = Point::new(x, y);
            let start_c = map[start_point];
            if start_c == b'@' || start_c.is_ascii_alphabetic() {
                let mut queue: VecDeque<(Point, usize)> = VecDeque::new();
                queue.push_back((start_point, 0));
                let mut visited = HashSet::new();
                let mut dists: HashMap<Node, usize> = HashMap::new();
                while let Some((point, dist)) = queue.pop_front() {
                    if !visited.insert(point) {
                        continue;
                    }
                    let c = map[point];
                    if point != start_point && c.is_ascii_alphabetic() {
                        dists.insert(Node::from(c), dist);
                    } else {
                        for &step in &[
                            Vector::new(-1, 0),
                            Vector::new(1, 0),
                            Vector::new(0, -1),
                            Vector::new(0, 1),
                        ] {
                            let next = point + step;
                            if map[next] != b'#' {
                                queue.push_back((next, dist + 1));
                            }
                        }
                    }
                }
                distances.insert(Node::from(start_c), dists);
            }
        }
    }

    let mut queue = BinaryHeap::new();
    queue.push(State {
        node: Node::Start,
        keys: KeySet::default(),
        steps: 0,
    });
    let mut visited = HashSet::new();
    while let Some(cur) = queue.pop() {
        if !visited.insert((cur.node, cur.keys)) {
            continue;
        }
        if cur.keys == map.all_keys {
            return cur.steps;
        }
        let dists = distances.get(&cur.node).unwrap();
        for (&next_node, steps) in dists {
            let mut next_state = State {
                node: next_node,
                keys: cur.keys,
                steps: cur.steps + steps,
            };
            match next_node {
                Node::Start => panic!("Start node should only have out edges"),
                Node::Key(key) => {
                    next_state.keys = next_state.keys + KeySet::from(key);
                },
                Node::Door(c) => {
                    if !cur.keys.opens_door(c) {
                        continue;
                    }
                },
            }
            queue.push(next_state);
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
    // aoc::test(part1, 4270, part2, "TODO".to_string());
}
