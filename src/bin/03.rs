use euclid::Point2D;
use itertools::iproduct;

struct Grid;
type Point = Point2D<i32, Grid>;
type Wire = Vec<Segment>;
#[derive(Debug)]
enum Segment {
    Horizontal { x_start: i32, x_end: i32, y: i32, start_dist: i32 },
    Vertical { x: i32, y_start: i32, y_end: i32, start_dist: i32 },
}

// Start is always exclusive because we don't want to count intersections at (0, 0).
fn between(start: i32, x: i32, end: i32) -> bool {
    if start < end {
        start < x && x <= end
    } else {
        end <= x && x < start
    }
}

impl Segment {
    fn intersect(&self, other: &Segment) -> Option<Point> {
        use Segment::*;
        match (self, other) {
            (Horizontal { x_start, x_end, y, .. }, Vertical { x, y_start, y_end, .. }) => {
                if between(*x_start, *x, *x_end) && between(*y_start, *y, *y_end) {
                    return Some(Point::new(*x, *y));
                }
            },
            (Vertical { x, y_start, y_end, .. }, Horizontal { x_start, x_end, y, .. }) => {
                if between(*x_start, *x, *x_end) && between(*y_start, *y, *y_end) {
                    return Some(Point::new(*x, *y));
                }
            },
            _ => {
                // Let's hope we don't get lines that overlap on the same axis.
                return None;
            },
        }
        None
    }

    fn start(&self) -> Point {
        match self {
            Segment::Horizontal { x_start, y, .. } => Point::new(*x_start, *y),
            Segment::Vertical { x, y_start, .. } => Point::new(*x, *y_start),
        }
    }

    fn start_dist(&self) -> i32 {
        match self {
            Segment::Horizontal { start_dist, .. } => *start_dist,
            Segment::Vertical { start_dist, .. } => *start_dist,
        }
    }
}

fn parse_wire(line: &str) -> Wire {
    let mut wire = Wire::new();
    let mut pos = Point::zero();
    let mut start_dist = 0;
    for instr in line.split(',') {
        let dir = instr.chars().nth(0).unwrap();
        let dist = instr[1..].parse::<i32>().unwrap();
        let mut end = pos;
        use Segment::*;
        let segment = match dir {
            'U' => {
                end.y -= dist;
                Vertical { x: pos.x, y_start: pos.y, y_end: end.y, start_dist }
            }
            'R' => {
                end.x += dist;
                Horizontal { x_start: pos.x, x_end: end.x, y: pos.y, start_dist }
            }
            'D' => {
                end.y += dist;
                Vertical { x: pos.x, y_start: pos.y, y_end: end.y, start_dist }
            }
            'L' => {
                end.x -= dist;
                Horizontal { x_start: pos.x, x_end: end.x, y: pos.y, start_dist }
            }
            _ => panic!("Invalid direction {}", dir)
        };
        wire.push(segment);
        pos = end;
        start_dist += dist;
    }
    wire
}

fn manhattan_length(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

fn manhattan_dist(a: &Point, b: &Point) -> i32 {
    let diff = *b - *a;
    diff.x.abs() + diff.y.abs()
}

fn part1(input: &str) -> i32 {
    let wires = input.lines().map(parse_wire).collect::<Vec<_>>();
    let closest_crossing = iproduct!(wires[0].iter(), wires[1].iter())
        .filter_map(|(segment1, segment2)| {
            segment1.intersect(segment2)
        })
        .min_by_key(|p| manhattan_length(p))
        .unwrap();
    manhattan_length(&closest_crossing)
}

#[test]
fn test_part1() {
    assert_eq!(part1("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    assert_eq!(part1("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 159);
    assert_eq!(part1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
}

fn part2(input: &str) -> i32 {
    let wires = input.lines().map(parse_wire).collect::<Vec<_>>();
    iproduct!(wires[0].iter(), wires[1].iter())
        .filter_map(|(segment1, segment2)| {
            segment1.intersect(segment2).map(|intersection| {
                segment1.start_dist() + manhattan_dist(&segment1.start(), &intersection) +
                    segment2.start_dist() + manhattan_dist(&segment2.start(), &intersection)
            })
        })
        .min()
        .unwrap()
}

#[test]
fn test_part2() {
    assert_eq!(part2("R8,U5,L5,D3\nU7,R6,D4,L4"), 30);
    assert_eq!(part2("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"), 610);
    assert_eq!(part2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 410);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 3229, part2, 32132);
}
