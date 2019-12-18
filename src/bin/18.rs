use euclid;
use generic_array::{arr, ArrayLength, GenericArray};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::convert::TryFrom;
use std::cmp::Reverse;

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Vector = euclid::Vector2D<i32, Grid>;

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct KeySet(usize);

impl KeySet {
    fn opens_door(self, c: u8) -> bool {
        assert!(c.is_ascii_uppercase());
        self.0 & (1 << ((c - b'A') as usize)) != 0
    }

    fn iter(self) -> KeySetIterator {
        KeySetIterator { keys: self.0, bit: 1 }
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

impl std::ops::Sub<KeySet> for KeySet {
    type Output = KeySet;
    fn sub(self, other: KeySet) -> KeySet {
        KeySet(self.0 & !other.0)
    }
}

impl std::iter::Sum for KeySet {
    fn sum<I: IntoIterator<Item = KeySet>>(iter: I) -> KeySet {
        iter.into_iter().fold(KeySet::default(), |a, b| a + b)
    }
}

struct KeySetIterator {
    keys: usize,
    bit: usize,
}

impl Iterator for KeySetIterator {
    type Item = KeySet;
    fn next(&mut self) -> Option<KeySet> {
        while self.bit <= self.keys && self.keys & self.bit == 0 {
            self.bit <<= 1;
        }
        if self.bit > self.keys {
            return None;
        }
        let next = KeySet(self.bit);
        self.bit <<= 1;
        Some(next)
    }
}

#[test]
fn test_keyset_iter() {
    assert_eq!(KeySet::default().iter().collect::<Vec<_>>(), vec![]);
    assert_eq!(KeySet::from(b'a').iter().collect::<Vec<_>>(), vec![KeySet::from(b'a')]);
    assert_eq!(KeySet::from(b'b').iter().collect::<Vec<_>>(), vec![KeySet::from(b'b')]);
    assert_eq!(KeySet::from(b'z').iter().collect::<Vec<_>>(), vec![KeySet::from(b'z')]);
    assert_eq!(KeySet(3).iter().collect::<Vec<_>>(), vec![KeySet::from(b'a'), KeySet::from(b'b')]);
    assert_eq!(KeySet(6).iter().collect::<Vec<_>>(), vec![KeySet::from(b'b'), KeySet::from(b'c')]);
    assert_eq!(KeySet(10).iter().collect::<Vec<_>>(), vec![KeySet::from(b'b'), KeySet::from(b'd')]);
}

impl std::fmt::Debug for KeySet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bit = 1;
        let mut c = b'a';
        write!(f, "[")?;
        while bit != 0 {
            if self.0 & bit != 0 {
                write!(f, "{}", c as char)?;
            }
            bit <<= 1;
            c += 1;
        }
        write!(f, "]")?;
        Ok(())
    }
}

struct Map(Vec<Vec<u8>>);

impl Map {
    fn parse(input: &str) -> Self {
        Map(input.lines().map(|line| line.trim().as_bytes().to_vec()).collect())
    }

    fn nx(&self) -> i32 {
        self.0[0].len() as i32
    }

    fn ny(&self) -> i32 {
        self.0.len() as i32
    }
}

impl std::ops::Index<Point> for Map {
    type Output = u8;
    fn index(&self, pos: Point) -> &u8 {
        &self.0[pos.y as usize][pos.x as usize]
    }
}

impl std::ops::IndexMut<Point> for Map {
    fn index_mut(&mut self, pos: Point) -> &mut u8 {
        &mut self.0[pos.y as usize][pos.x as usize]
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.0 {
            write!(f, "{}\n", String::from_utf8(row.to_vec()).unwrap())?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Node {
    Start(usize),
    Key(u8),
    Door(u8),
}

impl std::convert::TryFrom<u8> for Node {
    type Error = ();
    fn try_from(c: u8) -> Result<Self, Self::Error> {
        match c {
            b'@' => Ok(Node::Start(0)),
            c if c.is_ascii_digit() => Ok(Node::Start((c - b'0').into())),
            c if c.is_ascii_lowercase() => Ok(Node::Key(c)),
            c if c.is_ascii_uppercase() => Ok(Node::Door(c)),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Node::Start(r) => b'0' + r as u8,
            Node::Key(key) => key,
            Node::Door(door) => door,
        } as char)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State<N: ArrayLength<Node>> {
    nodes: GenericArray<Node, N>,
    keys: KeySet,
    steps: usize,
    heuristic: usize,
}

impl<N: ArrayLength<Node> + std::cmp::PartialEq + std::cmp::Eq> std::cmp::PartialOrd for State<N> {
    fn partial_cmp(&self, other: &State<N>) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<N: ArrayLength<Node> + std::cmp::Eq> std::cmp::Ord for State<N> {
    fn cmp(&self, other: &State<N>) -> std::cmp::Ordering {
        (other.steps + other.heuristic).cmp(&(self.steps + self.heuristic))
    }
}

fn compute_distances(map: &Map) -> HashMap<Node, HashMap<Node, usize>> {
    let mut distances: HashMap<Node, HashMap<Node, usize>> = HashMap::new();
    for y in 0..map.ny() {
        for x in 0..map.nx() {
            let start_point = Point::new(x, y);
            if let Ok(start_node) = Node::try_from(map[start_point]) {
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
                        dists.insert(Node::try_from(c).unwrap(), dist);
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
                distances.insert(start_node, dists);
            }
        }
    }
    distances
}

fn compute_distances_to_keys(distances: &HashMap<Node, HashMap<Node, usize>>) -> HashMap<Node, HashMap<KeySet, usize>> {
    distances.keys()
        .map(|&start_node| {
            let mut dists = HashMap::new();
            let mut visited = HashSet::new();
            let mut queue = BinaryHeap::new();
            queue.push(Reverse((0, start_node)));
            while let Some(Reverse((cur_steps, cur_node))) = queue.pop() {
                if !visited.insert(cur_node) {
                    continue;
                }
                if let Node::Key(c) = cur_node {
                    dists.insert(KeySet::from(c), cur_steps);
                }
                for (next_node, steps) in distances.get(&cur_node).unwrap() {
                    queue.push(Reverse((cur_steps + steps, *next_node)));
                }
            }
            (start_node, dists)
        })
        .collect()
}

struct AStarHeuristic {
    distances_to_keys: HashMap<Node, HashMap<KeySet, usize>>,
    all_keys: KeySet,
}

impl AStarHeuristic
{
    fn new(distances: &HashMap<Node, HashMap<Node, usize>>, all_keys: KeySet) -> Self {
        AStarHeuristic {
            distances_to_keys: compute_distances_to_keys(distances),
            all_keys,
        }
    }

    fn calc<N>(&self, nodes: &GenericArray<Node, N>, keys: KeySet) -> usize
        where N: ArrayLength<Node> + std::cmp::Eq
    {
        let missing_keys = self.all_keys - keys;
        nodes.iter()
            .map(|node| {
                let dists = &self.distances_to_keys.get(node).unwrap();
                missing_keys.iter()
                    .filter_map(|missing_key| dists.get(&missing_key))
                    .max()
                    .unwrap_or(&0)
            })
            .sum()
    }
}

fn find_path_steps<N>(distances: &HashMap<Node, HashMap<Node, usize>>, start_nodes: GenericArray<Node, N>) -> usize
    where N: ArrayLength<Node> + std::cmp::Eq + std::fmt::Debug
{
    let all_keys = distances.keys()
        .filter_map(|&node| {
            if let Node::Key(c) = node { Some(KeySet::from(c)) } else { None }
        })
        .sum();

    let heuristic = AStarHeuristic::new(distances, all_keys);

    let mut queue = BinaryHeap::new();
    let start_keys = KeySet::default();
    let start_heuristic = heuristic.calc(&start_nodes, start_keys);
    queue.push(State {
        nodes: start_nodes,
        keys: start_keys,
        steps: 0,
        heuristic: start_heuristic,
    });
    let mut visited = HashSet::new();
    while let Some(cur) = queue.pop() {
        if !visited.insert((cur.nodes.clone(), cur.keys)) {
            continue;
        }
        if cur.keys == all_keys {
            return cur.steps;
        }
        for i in 0..cur.nodes.len() {
            let dists = distances.get(&cur.nodes[i]).unwrap();
            for (&next_node, steps) in dists {
                if let Node::Door(c) = next_node {
                    if !cur.keys.opens_door(c) {
                        continue;
                    }
                }
                let mut next_nodes = cur.nodes.clone();
                next_nodes[i] = next_node;
                let mut next_keys = cur.keys;
                if let Node::Key(c) = next_node {
                    next_keys = next_keys + KeySet::from(c);
                }
                let next_heuristic = heuristic.calc(&next_nodes, next_keys);
                queue.push(State {
                    nodes: next_nodes,
                    keys: next_keys,
                    steps: cur.steps + steps,
                    heuristic: next_heuristic,
                });
            }
        }
    }
    panic!("No route found that collects all keys")
}

fn part1(input: &str) -> usize {
    let map = Map::parse(input);
    let distances = compute_distances(&map);
    find_path_steps(&distances, arr![Node; Node::Start(0)])
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

fn patch_map(map: &mut Map) {
    for y in 0..map.ny() {
        for x in 0..map.nx() {
            if map[Point::new(x as i32, y as i32)] == b'@' {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        map[Point::new(x as i32 + dx, y as i32 + dy)] = match (dx, dy) {
                            (-1, -1) => b'0',
                            (1, -1) => b'1',
                            (-1, 1) => b'2',
                            (1, 1) => b'3',
                            _ => b'#',
                        };
                    }
                }
                break;
            }
        }
    }
}

fn part2(input: &str) -> usize {
    let mut map = Map::parse(input);
    patch_map(&mut map);
    let distances = compute_distances(&map);
    find_path_steps(&distances, arr![Node; Node::Start(0), Node::Start(1), Node::Start(2), Node::Start(3)])
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("#######
               #a.#Cd#
               ##...##
               ##.@.##
               ##...##
               #cB#Ab#
               #######"),
        8);
    assert_eq!(
        part2("###############
               #d.ABC.#.....a#
               ######...######
               ######.@.######
               ######...######
               #b.....#.....c#
               ###############"),
        24);
    assert_eq!(
        part2("#############
               #DcBa.#.GhKl#
               #.###...#I###
               #e#d#.@.#j#k#
               ###C#...###J#
               #fEbA.#.FgHi#
               #############"),
        32);
    assert_eq!(
        part2("#############
               #g#f.D#..h#l#
               #F###e#E###.#
               #dCba...BcIJ#
               #####.@.#####
               #nK.L...G...#
               #M###N#H###.#
               #o#m..#i#jk.#
               #############"),
        72);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 4270, part2, 1982);
}
