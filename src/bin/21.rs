use aoc::intcode::*;

fn part1(input: &str) -> Number {
    let mut program = Program::parse(input);
    let springcode = "NOT A J
NOT C T
AND D T
OR T J
WALK
".as_bytes().iter().map(|&c| c as Number).collect::<Vec<_>>();
    let output = program.run_with_io(springcode);
    if *output.last().unwrap() >= 128 {
        return *output.last().unwrap()
    } else {
        panic!("Fell into a hole:\n{}", output.iter().map(|&val| val as u8 as char).collect::<String>());
    }
}

#[cfg(test)]
const SPRINGCODE_PREFER_WALK: &str = "OR F T
OR I T
AND E T
OR T J
OR H T
AND D T
OR G T
AND C T
OR F T
AND B T
OR T J
AND A J
NOT J J
RUN
";

#[cfg(test)]
const SPRINGCODE_PREFER_JUMP: &str = "OR F J
OR I J
AND E J
OR H J
AND D J
RUN
";

const SPRINGCODE_AD_HOC: &str = "OR B T
AND C T
NOT T J
AND E T
AND I T
OR H T
AND T J
AND D J
NOT A T
OR T J
RUN
";

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
enum Jump {
    Yes,
    No,
    Either,
}

#[cfg(test)]
fn should_jump(holes: usize) -> Option<Jump> {
    if holes == 0 {
        Some(Jump::Either)
    } else if holes & 1 != 0 {
        None
    } else {
        let ok_to_walk = should_jump(holes >> 1).is_some();
        let ok_to_jump = should_jump(holes >> 4).is_some();
        match (ok_to_walk, ok_to_jump) {
            (false, false) => None,
            (false, true) => Some(Jump::Yes),
            (true, false) => Some(Jump::No),
            (true, true) => Some(Jump::Either),
        }
    }
}

#[cfg(test)]
fn run_springcode(springcode: &str, holes: usize) -> bool {
    let mut t = false;
    let mut j = false;
    let eval = |s: &str, t: bool, j: bool| {
        match s.chars().next().unwrap() {
            'T' => t,
            'J' => j,
            c => (holes & (1 << (c as usize - b'A' as usize))) == 0,
        }
    };
    let write = |val: bool, s: &str, t: &mut bool, j: &mut bool| {
        match s.chars().next().unwrap() {
            'T' => *t = val,
            'J' => *j = val,
            c => panic!("Cannot write to register {}", c),
        }
    };
    for line in springcode.lines() {
        let parts = line.split(' ').collect::<Vec<_>>();
        if parts.len() == 3 {
            let a = eval(parts[1], t, j);
            let b = eval(parts[2], t, j);
            match parts[0] {
                "AND" => {
                    write(a && b, parts[2], &mut t, &mut j);
                },
                "OR" => {
                    write(a || b, parts[2], &mut t, &mut j);
                },
                "NOT" => {
                    write(!a, parts[2], &mut t, &mut j);
                },
                s => panic!("Unknown command {}", s),
            }
        }
    }
    j
}

#[cfg(test)]
fn verify_springcode(springcode: &str) -> bool {
    let mut total_cases = 0;
    let mut correct_cases = 0;
    println!(" ABCDEFGHI   J   O");
    for holes in 0..512 {
        if let Some(jump) = should_jump(holes << 1) {
            if jump == Jump::Yes || jump == Jump::No {
                let jump = jump == Jump::Yes;
                total_cases += 1;
                let answer = run_springcode(springcode, holes);
                if answer != jump {
                    println!("@");
                    print!("#");
                    for bit in 0..9 {
                        print!("{}", if holes & (1 << bit) == 0 { '#' } else { '.' });
                    }
                    print!("   {}", jump as usize);
                    print!("   {}", answer as usize);
                    println!();
                } else {
                    correct_cases += 1;
                }
            }
        }
    }
    correct_cases == total_cases
}

#[test]
fn test_springcode() {
    assert!(verify_springcode(SPRINGCODE_PREFER_WALK));
    assert!(verify_springcode(SPRINGCODE_PREFER_JUMP));
    assert!(verify_springcode(SPRINGCODE_AD_HOC));
}

fn part2(input: &str) -> Number {
    let mut program = Program::parse(input);
    let output = program.run_with_io(SPRINGCODE_AD_HOC.as_bytes().iter().map(|&c| c as Number).collect::<Vec<_>>());
    if *output.last().unwrap() >= 128 {
        return *output.last().unwrap()
    } else {
        panic!("{}", output.iter().map(|&val| val as u8 as char).collect::<String>());
    }
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 19361850, part2, 1138943788);
}
