use aoc::intcode::*;
use std::ops::Range;

#[derive(Debug)]
struct BeamTracer {
    program: Program,
    last_slice: BeamSlice,
}

#[derive(Debug, Clone)]
struct BeamSlice {
    x: Number,
    ys: Range<Number>,
}

impl BeamTracer {
    fn new(program: Program) -> Self {
        BeamTracer { program, last_slice: BeamSlice { x: -1, ys: 0..0 } }
    }
}

impl Iterator for BeamTracer {
    type Item = BeamSlice;
    fn next(&mut self) -> Option<BeamSlice> {
        let slice = &mut self.last_slice;
        slice.x += 1;
        if slice.x < 12 {
            slice.ys.start = 0;
            while slice.ys.start < 12 &&
                self.program.clone().run_with_io(vec![slice.x, slice.ys.start]).output[0] == 0
            {
                slice.ys.start += 1;
            }
            slice.ys.end = slice.ys.start;
            while slice.ys.end < 12 &&
                self.program.clone().run_with_io(vec![slice.x, slice.ys.end]).output[0] == 1
            {
                slice.ys.end += 1;
            }
        } else {
            while self.program.clone().run_with_io(vec![slice.x, slice.ys.start]).output[0] == 0 {
                slice.ys.start += 1;
            }
            while self.program.clone().run_with_io(vec![slice.x, slice.ys.end]).output[0] == 1 {
                slice.ys.end += 1;
            }
        }
        Some(slice.clone())
    }
}

fn part1(input: &str) -> usize {
    let program = Program::parse(input);
    BeamTracer::new(program).take(50).map(|slice| (slice.ys.end - slice.ys.start) as usize).sum()
}

fn part2(input: &str) -> i64 {
    let program = Program::parse(input);
    let mut y_ends = Vec::new();
    for slice in BeamTracer::new(program) {
        y_ends.push(slice.ys.end);
        if slice.x >= 100 && y_ends[slice.x as usize - 99] >= slice.ys.start + 100 {
            return (slice.x - 99) * 10000 + slice.ys.start;
        }
    }
    panic!("Beam tracer ended unexpectedly")
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 131, part2, 15231022);
}
