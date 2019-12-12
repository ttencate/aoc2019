use aoc::math::lcm;
use euclid;
use packed_simd::i32x4;
use regex::Regex;

struct Grid;
type Position = euclid::Point3D<i32, Grid>;
type Velocity = euclid::Vector3D<i32, Grid>;

fn parse_input(input: &str) -> Vec<Position> {
    // <x=-6, y=2, z=-9>
    let pos_re = Regex::new(r"<x=(.*), y=(.*), z=(.*)>").unwrap();
    input.lines()
        .map(|line| {
            let cap = pos_re.captures(line).unwrap();
            Position::new(
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
                cap[3].parse::<i32>().unwrap())
        })
        .collect()
}

fn signum(v: Velocity) -> Velocity {
    Velocity::new(v.x.signum(), v.y.signum(), v.z.signum())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct State {
    pos: Vec<Position>,
    vel: Vec<Velocity>,
}

impl State {
    fn parse(input: &str) -> Self {
        let pos = parse_input(input);
        let vel = vec![Velocity::default(); pos.len()];
        State { pos, vel }
    }

    fn simulate(&mut self, timesteps: usize) -> &Self {
        let n = self.pos.len();
        for _ in 0..timesteps {
            for i in 0..n {
                for j in (i + 1)..n {
                    let acc = signum(self.pos[j] - self.pos[i]);
                    self.vel[i] += acc;
                    self.vel[j] -= acc;
                }
            }
            for i in 0..n {
                self.pos[i] += self.vel[i];
            }
        }
        self
    }

    fn axis_loop_length(start_pos: Vec<i32>) -> usize {
        assert_eq!(start_pos.len(), 4);
        let start_pos = i32x4::from_slice_unaligned(&start_pos);
        let start_vel = i32x4::from_slice_unaligned(&vec![0; 4]);
        let mut pos = start_pos;
        let mut vel = start_vel;
        let mut time = 0;
        loop {
            let acc = i32x4::new(
                (pos.extract(1) - pos.extract(0)).signum() +
                (pos.extract(2) - pos.extract(0)).signum() +
                (pos.extract(3) - pos.extract(0)).signum(),
                (pos.extract(0) - pos.extract(1)).signum() +
                (pos.extract(2) - pos.extract(1)).signum() +
                (pos.extract(3) - pos.extract(1)).signum(),
                (pos.extract(0) - pos.extract(2)).signum() +
                (pos.extract(1) - pos.extract(2)).signum() +
                (pos.extract(3) - pos.extract(2)).signum(),
                (pos.extract(0) - pos.extract(3)).signum() +
                (pos.extract(1) - pos.extract(3)).signum() +
                (pos.extract(2) - pos.extract(3)).signum());
            vel += acc;
            pos += vel;

            time += 1;
            if pos == start_pos && vel == start_vel {
                return time;
            }
        }
    }

    fn loop_length(&self) -> usize {
        let x_length = Self::axis_loop_length(self.pos.iter().map(|p| p.x).collect());
        let y_length = Self::axis_loop_length(self.pos.iter().map(|p| p.y).collect());
        let z_length = Self::axis_loop_length(self.pos.iter().map(|p| p.z).collect());
        lcm(lcm(x_length, y_length), z_length)
    }

    fn total_energy(&self) -> i32 {
        self.pos.iter().zip(&self.vel)
            .map(|(p, v)| {
                (p.x.abs() + p.y.abs() + p.z.abs()) * (v.x.abs() + v.y.abs() + v.z.abs())
            })
            .sum()
    }
}

#[test]
fn test_simulate() {
    assert_eq!(
        State::parse("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>")
            .simulate(10)
            .total_energy(),
        179);
    assert_eq!(State::parse("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")
            .simulate(100)
            .total_energy(),
        1940);
}

// #[test]
// fn test_axis_loop_length() {
//     assert_eq!(State::axis_loop_length(vec![-1, 1]), 6);
// }

#[test]
fn test_loop_length() {
    assert_eq!(
        State::parse("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>")
            .loop_length(),
        2772);
    assert_eq!(State::parse("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")
            .loop_length(),
        4686774924);
}

fn part1(input: &str) -> i32 {
    State::parse(input)
        .simulate(1000)
        .total_energy()
}

fn part2(input: &str) -> usize {
    State::parse(input)
        .loop_length()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 14907, part2, 467081194429464);
}
