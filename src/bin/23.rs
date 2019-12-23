use aoc::intcode::*;
use std::collections::VecDeque;

const N: usize = 50;

#[derive(Debug, Default, Clone)]
struct Packet {
    x: Number,
    y: Number,
}

type PacketQueue = VecDeque<Packet>;

fn tick_programs(programs: &mut Vec<Option<Program>>, input_queues: &mut Vec<PacketQueue>, nat: &mut Option<Packet>) -> bool {
    let mut progress = false;
    for i in 0..N {
        let program = programs[i].take().unwrap();
        programs[i] = Some(match program.run() {
            Interrupt::Reading(next) => {
                if let Some(packet) = input_queues[i].pop_front() {
                    progress = true;
                    next(packet.x).run().give_input(packet.y)
                } else {
                    next(-1)
                }
            },
            Interrupt::Writing(dest, next) => {
                progress = true;
                let (x, p) = next().run().take_output();
                let (y, p) = p.run().take_output();
                let packet = Packet { x, y };
                match dest {
                    dest if dest >= 0 && dest < N as Number => {
                        input_queues[dest as usize].push_back(packet);
                    },
                    255 => {
                        *nat = Some(packet);
                    },
                    _ => { panic!("Unexpected destination address {}", dest); },
                }
                p
            },
            Interrupt::Halted(_) => panic!("Program halted unexpectedly"),
        });
    }
    progress
}

fn part1(input: &str) -> Number {
    let program = Program::parse(input);
    let mut programs: Vec<Option<Program>> = (0..N).map(|i| Some(program.clone().run().give_input(i as Number))).collect();
    let mut input_queues: Vec<PacketQueue> = (0..N).map(|_| PacketQueue::new()).collect();
    let mut nat = None;
    loop {
        tick_programs(&mut programs, &mut input_queues, &mut nat);
        if let Some(packet) = nat {
            return packet.y;
        }
    }
}

fn part2(input: &str) -> Number {
    let program = Program::parse(input);
    let mut programs: Vec<Option<Program>> = (0..N).map(|i| Some(program.clone().run().give_input(i as Number))).collect();
    let mut input_queues: Vec<PacketQueue> = (0..N).map(|_| PacketQueue::new()).collect();
    let mut nat = None;
    let mut last_y_sent = None;
    loop {
        let progress = tick_programs(&mut programs, &mut input_queues, &mut nat);
        if !progress {
            if let Some(packet) = nat.as_ref() {
                if Some(packet.y) == last_y_sent {
                    return packet.y;
                }
                last_y_sent = Some(packet.y);
                input_queues[0].push_back((*packet).clone());
            }
        }
    }
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 24922, part2, 19478);
}
