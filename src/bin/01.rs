fn fuel_mass(payload_mass: u64) -> u64 {
    (payload_mass / 3).saturating_sub(2)
}

#[test]
fn test_fuel_mass() {
    assert_eq!(fuel_mass(12), 2);
    assert_eq!(fuel_mass(14), 2);
    assert_eq!(fuel_mass(1969), 654);
    assert_eq!(fuel_mass(100756), 33583);
}

fn total_fuel_mass(payload_mass: u64) -> u64 {
    let mut current_mass = payload_mass;
    let mut total_fuel_mass = 0;
    loop {
        let fuel_mass = fuel_mass(current_mass);
        if fuel_mass == 0 {
            return total_fuel_mass;
        }
        total_fuel_mass += fuel_mass;
        current_mass = fuel_mass;
    }
}

#[test]
fn test_total_fuel_mass() {
    assert_eq!(total_fuel_mass(12), 2);
    assert_eq!(total_fuel_mass(1969), 966);
    assert_eq!(total_fuel_mass(100756), 50346);
}

fn part1(input: &str) -> u64 {
    input.lines()
        .map(|line| fuel_mass(line.trim().parse::<u64>().unwrap()))
        .sum()
}

fn part2(input: &str) -> u64 {
    input.lines()
        .map(|line| total_fuel_mass(line.trim().parse::<u64>().unwrap()))
        .sum()
}

fn main() {
    aoc::main(part1, part2);
}
