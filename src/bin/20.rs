use euclid;
use std::collections::{HashMap, HashSet, VecDeque};

struct Grid;
type Point = euclid::Point2D<i32, Grid>;
type Vector = euclid::Vector2D<i32, Grid>;

const LEFT: Vector = Vector::new(0, -1);
const RIGHT: Vector = Vector::new(0, 1);
const UP: Vector = Vector::new(-1, 0);
const DOWN: Vector = Vector::new(1, 0);

const SPACE: u8 = b' ';
const CORRIDOR: u8 = b'.';

enum PortalType {
    Outer,
    Inner,
}
use PortalType::*;

struct Maze {
    cells: Vec<Vec<u8>>,
    portals: HashMap<Point, (i32, Point)>,
    start: Point,
    end: Point,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut maze = Maze {
            cells: input.lines().map(|line| line.as_bytes().to_vec()).collect(),
            portals: HashMap::new(),
            start: Point::default(),
            end: Point::default(),
        };
        let mut unmatched_portals = HashMap::new();
        let nx = maze.nx();
        let ny = maze.ny();
        for y in 0..ny {
            for x in 0..nx {
                let p = Point::new(x, y);
                let portal_first_letter = maze[p];
                if portal_first_letter.is_ascii_uppercase() {
                    for &search_dir in &[RIGHT, DOWN] {
                        let portal_second_letter = maze[p + search_dir];
                        if portal_second_letter.is_ascii_uppercase() {
                            let portal_pos = *[p + search_dir + search_dir, p - search_dir]
                                .iter()
                                .find(|&&p| maze[p] == CORRIDOR)
                                .expect("No portal found near capitals");
                            let portal_type =
                                if (3..(nx - 3)).contains(&portal_pos.x) && (3..(ny - 3)).contains(&portal_pos.y) {
                                    Inner
                                } else {
                                    Outer
                                };
                            let portal_name = format!("{}{}", portal_first_letter as char, portal_second_letter as char);
                            match portal_name.as_str() {
                                "AA" => maze.start = portal_pos,
                                "ZZ" => maze.end = portal_pos,
                                _ => {
                                    if let Some(portal_dest) = unmatched_portals.remove(&portal_name) {
                                        maze.portals.insert(
                                            portal_pos,
                                            (match portal_type { Outer => -1, Inner => 1 }, portal_dest));
                                        maze.portals.insert(
                                            portal_dest,
                                            (match portal_type { Outer => 1, Inner => -1 }, portal_pos));
                                    } else {
                                        unmatched_portals.insert(portal_name, portal_pos);
                                    }
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
        assert!(unmatched_portals.len() == 0, "Unmatched portals remain: {:?}", unmatched_portals);
        maze
    }

    fn nx(&self) -> i32 {
        self.cells.iter().map(|row| row.len()).max().unwrap() as i32
    }

    fn ny(&self) -> i32 {
        self.cells.len() as i32
    }
}

impl std::ops::Index<Point> for Maze {
    type Output = u8;
    fn index(&self, p: Point) -> &u8 {
        self.cells.get(p.y as usize).map(|row| row.get(p.x as usize).unwrap_or(&SPACE)).unwrap_or(&SPACE)
    }
}

impl std::fmt::Debug for Maze {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ny = self.ny();
        let nx = self.nx();
        for y in 0..ny {
            for x in 0..nx {
                let p = Point::new(x, y);
                write!(f, "{}",
                    if let Some((level_step, _)) = self.portals.get(&p) {
                        match level_step { -1 => '^', 1 => 'v', _ => panic!("Non-unitary level step") }
                    } else if p == self.start {
                        '@'
                    } else if p == self.end {
                        '*'
                    } else {
                        self[p] as char
                    })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part1(input: &str) -> usize {
    let maze = Maze::parse(input);
    let mut queue = VecDeque::new();
    queue.push_back((maze.start, 0));
    let mut visited = HashSet::new();
    while let Some((cur, dist)) = queue.pop_front() {
        if !visited.insert(cur) {
            continue;
        }
        if cur == maze.end {
            return dist;
        }
        for &step in &[LEFT, RIGHT, UP, DOWN] {
            let next = cur + step;
            if maze[next] == CORRIDOR {
                queue.push_back((next, dist + 1));
            }
        }
        if let Some((_, next)) = maze.portals.get(&cur) {
            queue.push_back((*next, dist + 1));
        }
    }
    panic!("No path found");
}

#[test]
fn test_part1() {
    assert_eq!(part1(
"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z     "),
        23);
    assert_eq!(part1(
"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               "),
        58);
}

fn part2(input: &str) -> usize {
    let maze = Maze::parse(input);
    let mut queue = VecDeque::new();
    queue.push_back((0, maze.start, 0));
    let mut visited = HashSet::new();
    while let Some((level, cur, dist)) = queue.pop_front() {
        if !visited.insert((level, cur)) {
            continue;
        }
        if level == 0 && cur == maze.end {
            return dist;
        }
        for &step in &[LEFT, RIGHT, UP, DOWN] {
            let next = cur + step;
            if maze[next] == CORRIDOR {
                queue.push_back((level, next, dist + 1));
            }
        }
        if let Some((level_step, next)) = maze.portals.get(&cur) {
            let next_level = level + *level_step;
            if next_level >= 0 {
                queue.push_back((level + *level_step, *next, dist + 1));
            }
        }
    }
    panic!("No path found");
}

#[test]
fn test_part2() {
    assert_eq!(part2(
"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z     "),
        26);
    assert_eq!(part2(
"             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     "),
        396);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 410, part2, 5084);
}
