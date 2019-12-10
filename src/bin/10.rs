use euclid::{Point2D, Vector2D};
use itertools::iproduct;
use std::cmp::Ordering;

struct Grid;
type Coord = Point2D<i32, Grid>;
type Vector = Vector2D<i32, Grid>;

const ASTEROID: u8 = b'#';
const SPACE: u8 = b'.';

struct Map(Vec<Vec<u8>>);

impl std::ops::Index<Coord> for Map {
    type Output = u8;
    fn index(&self, coord: Coord) -> &Self::Output {
        &self.0[coord.y as usize][coord.x as usize]
    }
}

impl std::ops::IndexMut<Coord> for Map {
    fn index_mut(&mut self, coord: Coord) -> &mut Self::Output {
        &mut self.0[coord.y as usize][coord.x as usize]
    }
}

fn parse_map(input: &str) -> Map {
    Map(input.lines()
        .map(|line| line.trim().as_bytes().to_vec())
        .collect())
}

fn find_asteroids(map: &Map) -> Vec<Coord> {
    iproduct!(0..map.0[0].len() as i32, 0..map.0.len() as i32)
        .filter_map(|xy| {
            let c = Coord::new(xy.0, xy.1);
            if map[c] == ASTEROID { Some(c) } else { None }
        })
        .collect()
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn have_line_of_sight(map: &Map, a: Coord, b: Coord) -> bool {
    let diff = b - a;
    let d = gcd(diff.x.abs(), diff.y.abs());
    let step = Vector::new(diff.x / d, diff.y / d);
    let mut cur = a + step;
    while cur != b {
        if map[cur] == ASTEROID {
            return false;
        }
        cur += step;
    }
    true
}

fn best_station_pos(map: &Map) -> (Coord, usize) {
    let asteroids = find_asteroids(&map);
    let n = asteroids.len();
    let mut counts = vec![0; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if have_line_of_sight(&map, asteroids[i], asteroids[j]) {
                counts[i] += 1;
                counts[j] += 1;
            }
        }
    }
    let idx = (0..n).max_by_key(|&i| counts[i]).unwrap();
    (asteroids[idx], counts[idx])
}

fn part1(input: &str) -> usize {
    let map = parse_map(input);
    best_station_pos(&map).1
}

#[test]
fn test_part1() {
    assert_eq!(part1(".#..#
                      .....
                      #####
                      ....#
                      ...##"), 8);
    assert_eq!(part1("......#.#.
                      #..#.#....
                      ..#######.
                      .#.#.###..
                      .#..#.....
                      ..#....#.#
                      #..#....#.
                      .##.#..###
                      ##...#..#.
                      .#....####"), 33);
    assert_eq!(part1("#.#...#.#.
                      .###....#.
                      .#....#...
                      ##.#.#.#.#
                      ....#.#.#.
                      .##..###.#
                      ..#...##..
                      ..##....##
                      ......#...
                      .####.###."), 35);
    assert_eq!(part1(".#..#..###
                      ####.###.#
                      ....###.#.
                      ..###.##.#
                      ##.##.#.#.
                      ....###..#
                      ..#.#..#.#
                      #..#.#.###
                      .##...##.#
                      .....#.#.."), 41);
    assert_eq!(part1(".#..##.###...#######
                      ##.############..##.
                      .#.######.########.#
                      .###.#######.####.#.
                      #####.##.#.##.###.##
                      ..#####..#.#########
                      ####################
                      #.####....###.#.#.##
                      ##.#################
                      #####.##.###..####..
                      ..######..##.#######
                      ####.##.####...##..#
                      .#####..#.######.###
                      ##...#.##########...
                      #.##########.#######
                      .####.#.###.###.#.##
                      ....##.##.###..#####
                      .#.#.###########.###
                      #.#.#.#####.####.###
                      ###.##.####.##.#..##"), 210);
}

fn quadrant(v: Vector) -> usize {
    if v.x >= 0 && v.y < 0 {
        0
    } else if v.x > 0 && v.y >= 0 {
        1
    } else if v.x <= 0 && v.y > 0 {
        2
    } else if v.x < 0 && v.y <= 0 {
        3
    } else {
        panic!("Cannot determine quadrant of {}", v)
    }
}

fn laser_order(a: Vector, b: Vector) -> Ordering {
    let a_quad = quadrant(a);
    let b_quad = quadrant(b);
    a_quad.cmp(&b_quad)
        .then_with(|| {
            match a_quad {
                0 | 2 => {
                    (a.x * -b.y).cmp(&(b.x * -a.y))
                },
                1 | 3 => {
                    (a.y * b.x).cmp(&(b.y * a.x))
                }
                _ => panic!("Nonexistent quadrant {}", a_quad),
            }
        })
        .then_with(|| {
            (a.x.abs() + a.y.abs()).cmp(&(b.x.abs() + b.y.abs()))
        })
}

#[test]
fn test_laser_order() {
    assert_eq!(laser_order(Vector::new(0, -1), Vector::new(1, -1)), Ordering::Less);
    assert_eq!(laser_order(Vector::new(0, -1), Vector::new(0, -2)), Ordering::Less);
    assert_eq!(laser_order(Vector::new(0, -1), Vector::new(1, 0)), Ordering::Less);
    assert_eq!(laser_order(Vector::new(0, -1), Vector::new(0, 1)), Ordering::Less);
    assert_eq!(laser_order(Vector::new(0, -1), Vector::new(-1, 0)), Ordering::Less);
}

fn collinear(a: Vector, b: Vector) -> bool {
    a.x * b.y == b.x * a.y && a.x.signum() == b.x.signum() && a.y.signum() == b.y.signum()
}

fn vaporize(mut map: Map) -> Vec<Coord> {
    let (station, _) = best_station_pos(&map);
    map[station] = SPACE;

    let mut asteroids = find_asteroids(&map);
    asteroids.sort_by(|&a, &b| laser_order(a - station, b - station));

    let mut out = Vec::with_capacity(asteroids.len());
    while !asteroids.is_empty() {
        let mut remaining = vec![];
        let mut prev_dist = None;
        for ast in asteroids {
            let dist = ast - station;
            if prev_dist.is_none() || !collinear(dist, prev_dist.unwrap()) {
                out.push(ast);
                prev_dist = Some(dist);
            } else {
                remaining.push(ast);
            }
        }
        asteroids = remaining;
    }
    out
}

#[test]
fn test_vaporization_order() {
    assert_eq!(
        vaporize(parse_map(".#....#####...#..
                            ##...##.#####..##
                            ##...#...#.#####.
                            ..#.....#...###..
                            ..#.#.....#....##")),
        vec![
            Coord::new(8, 1),
            Coord::new(9, 0),
            Coord::new(9, 1),
            Coord::new(10, 0),
            Coord::new(9, 2),
            Coord::new(11, 1),
            Coord::new(12, 1),
            Coord::new(11, 2),
            Coord::new(15, 1),

            Coord::new(12, 2),
            Coord::new(13, 2),
            Coord::new(14, 2),
            Coord::new(15, 2),
            Coord::new(12, 3),
            Coord::new(16, 4),
            Coord::new(15, 4),
            Coord::new(10, 4),
            Coord::new(4, 4),

            Coord::new(2, 4),
            Coord::new(2, 3),
            Coord::new(0, 2),
            Coord::new(1, 2),
            Coord::new(0, 1),
            Coord::new(1, 1),
            Coord::new(5, 2),
            Coord::new(1, 0),
            Coord::new(5, 1),

            Coord::new(6, 1),
            Coord::new(6, 0),
            Coord::new(7, 0),
            Coord::new(8, 0),
            Coord::new(10, 1),
            Coord::new(14, 0),
            Coord::new(16, 1),
            Coord::new(13, 3),
            Coord::new(14, 3),
        ]);
    assert_eq!(
        vaporize(parse_map(".#..##.###...#######
                            ##.############..##.
                            .#.######.########.#
                            .###.#######.####.#.
                            #####.##.#.##.###.##
                            ..#####..#.#########
                            ####################
                            #.####....###.#.#.##
                            ##.#################
                            #####.##.###..####..
                            ..######..##.#######
                            ####.##.####...##..#
                            .#####..#.######.###
                            ##...#.##########...
                            #.##########.#######
                            .####.#.###.###.#.##
                            ....##.##.###..#####
                            .#.#.###########.###
                            #.#.#.#####.####.###
                            ###.##.####.##.#..##"))
            .iter()
            .enumerate()
            .filter_map(|(i, coord)| match i + 1 {
                1 | 2 | 3 | 10 | 20 | 50 | 100 | 199 | 200 | 201 | 299 => Some(coord),
                _ => None,
            })
            .copied()
            .collect::<Vec<_>>(),
        vec![
            Coord::new(11, 12),
            Coord::new(12, 1),
            Coord::new(12, 2),
            Coord::new(12, 8),
            Coord::new(16, 0),
            Coord::new(16, 9),
            Coord::new(10, 16),
            Coord::new(9, 6),
            Coord::new(8, 2),
            Coord::new(10, 9),
            Coord::new(11, 1),
        ]);
}

fn part2(input: &str) -> i32 {
    let map = parse_map(input);
    let coord = vaporize(map)[199];
    coord.x * 100 + coord.y
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 292, part2, 317);
}
