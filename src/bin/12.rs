use euclid;
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

    fn loop_length(&self) -> usize {
        let mut state = self.clone();
        state.simulate(1);
        let mut time = 1;
        while &state != self {
            state.simulate(1);
            time += 1;
        }
        time
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

#[test]
fn test_loop_length() {
    assert_eq!(
        State::parse("<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>")
            .loop_length(),
        2772);
    //assert_eq!(State::parse("<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>")
    //        .loop_length(),
    //    4686774924);
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
    // aoc::test(part1, "TODO".to_string(), part2, "TODO".to_string());
}
