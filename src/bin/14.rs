use std::collections::{HashMap, HashSet};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Chemical {
    Ore,
    Fuel,
    Other(u8),
}

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<Chemical, usize>,
    output_chemical: Chemical,
    output_quantity: usize,
}

fn parse_input(input: &str) -> HashMap<Chemical, Reaction> {
    let mut string_to_chemical = HashMap::<String, Chemical>::new();
    string_to_chemical.insert("ORE".to_string(), Chemical::Ore);
    string_to_chemical.insert("FUEL".to_string(), Chemical::Fuel);
    let mut next_chemical_id = 0;
    let mut parse_chemical_quantity = |input: &str| -> (Chemical, usize) {
        let mut parts = input.trim().split(' ');
        let quantity = parts.next().unwrap().parse::<usize>().unwrap();
        let chemical_name = parts.next().unwrap();
        let chemical = *string_to_chemical.entry(chemical_name.to_string()).or_insert_with(|| {
            next_chemical_id += 1;
            Chemical::Other(next_chemical_id)
        });
        (chemical, quantity)
    };
    input.lines()
        .map(|line| {
            let mut split = line.split("=>");
            let inputs = split.next().unwrap().split(",").map(&mut parse_chemical_quantity).collect();
            let (output_chemical, output_quantity) = parse_chemical_quantity(split.next().unwrap());
            let reaction = Reaction { inputs, output_chemical, output_quantity };
            (reaction.output_chemical, reaction)
        })
        .collect()
}

fn dfs(reactions: &HashMap<Chemical, Reaction>, node: Chemical, sorted: &mut Vec<Chemical>, visited: &mut HashSet<Chemical>) {
    if let Some(reaction) = reactions.get(&node) {
        for input_chemical in reaction.inputs.keys() {
            dfs(reactions, *input_chemical, sorted, visited);
        }
    }
    if visited.insert(node) {
        sorted.push(node);
    }
}

fn topological_sort(mut reactions: HashMap<Chemical, Reaction>) -> Vec<Reaction> {
    let mut sorted = Vec::new();
    let mut visited = HashSet::new();
    dfs(&reactions, Chemical::Fuel, &mut sorted, &mut visited);
    assert_eq!(*sorted.first().unwrap(), Chemical::Ore);
    assert_eq!(*sorted.last().unwrap(), Chemical::Fuel);
    sorted.into_iter().skip(1).map(|chemical| reactions.remove(&chemical).unwrap()).collect()
}

fn ore_needed_for_fuel(fuel_quantity: usize, ordered_reactions: &Vec<Reaction>) -> usize {
    let mut needed = HashMap::<Chemical, usize>::new();
    needed.insert(Chemical::Fuel, fuel_quantity);
    for reaction in ordered_reactions.iter().rev() {
        let num_runs = (*needed.get(&reaction.output_chemical).unwrap_or(&0) + reaction.output_quantity - 1) / reaction.output_quantity;
        for (input_chemical, input_quantity) in &reaction.inputs {
            *needed.entry(*input_chemical).or_default() += num_runs * input_quantity;
        }
    }
    *needed.get(&Chemical::Ore).unwrap()
}

#[test]
fn test_ore_needed_for_fuel() {
    let a = topological_sort(parse_input(
        "10 ORE => 10 A
         1 A => 1 FUEL"));
    assert_eq!(ore_needed_for_fuel(1, &a), 10);
    assert_eq!(ore_needed_for_fuel(9, &a), 10);
    assert_eq!(ore_needed_for_fuel(10, &a), 10);
    assert_eq!(ore_needed_for_fuel(20, &a), 20);

    let ab = topological_sort(parse_input(
        "10 ORE => 10 A
         1 ORE => 1 B
         1 A, 1 B => 1 FUEL"));
    assert_eq!(ore_needed_for_fuel(1, &ab), 11);
    assert_eq!(ore_needed_for_fuel(2, &ab), 12);
    assert_eq!(ore_needed_for_fuel(10, &ab), 20);
    assert_eq!(ore_needed_for_fuel(11, &ab), 31);
    assert_eq!(ore_needed_for_fuel(19, &ab), 39);
    assert_eq!(ore_needed_for_fuel(20, &ab), 40);
    assert_eq!(ore_needed_for_fuel(21, &ab), 51);
}

fn part1(input: &str) -> usize {
    let reactions = topological_sort(parse_input(input));
    ore_needed_for_fuel(1, &reactions)
}

#[test]
fn test_part1() {
    assert_eq!(
        part1("10 ORE => 10 A
               1 ORE => 1 B
               7 A, 1 B => 1 C
               7 A, 1 C => 1 D
               7 A, 1 D => 1 E
               7 A, 1 E => 1 FUEL"),
        31);
    assert_eq!(
        part1("9 ORE => 2 A
               8 ORE => 3 B
               7 ORE => 5 C
               3 A, 4 B => 1 AB
               5 B, 7 C => 1 BC
               4 C, 1 A => 1 CA
               2 AB, 3 BC, 4 CA => 1 FUEL"),
        165);
    assert_eq!(
        part1("157 ORE => 5 NZVS
               165 ORE => 6 DCFZ
               44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
               12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
               179 ORE => 7 PSHF
               177 ORE => 5 HKGWZ
               7 DCFZ, 7 PSHF => 2 XJWVT
               165 ORE => 2 GPVTF
               3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),
        13312);
    assert_eq!(
        part1("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
               17 NVRVD, 3 JNWZP => 8 VPVL
               53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
               22 VJHF, 37 MNCFX => 5 FWMGM
               139 ORE => 4 NVRVD
               144 ORE => 7 JNWZP
               5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
               5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
               145 ORE => 6 MNCFX
               1 NVRVD => 8 CXFTF
               1 VJHF, 6 MNCFX => 4 RFSQX
               176 ORE => 6 VJHF"),
        180697);
    assert_eq!(
        part1("171 ORE => 8 CNZTR
               7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
               114 ORE => 4 BHXH
               14 VRPVC => 6 BMBT
               6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
               6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
               15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
               13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
               5 BMBT => 4 WPTQ
               189 ORE => 9 KTJDG
               1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
               12 VRPVC, 27 CNZTR => 2 XDBXC
               15 KTJDG, 12 BHXH => 5 XCVML
               3 BHXH, 2 VRPVC => 7 MZWV
               121 ORE => 7 VRPVC
               7 XCVML => 6 RJRHP
               5 BHXH, 4 VRPVC => 5 LTCX"),
        2210736);
}

fn max_fuel_from_ore(input_ore: usize, ordered_reactions: &Vec<Reaction>) -> usize {
    let mut lower_fuel = input_ore / ore_needed_for_fuel(1, ordered_reactions);
    let mut upper_fuel = lower_fuel * 2;
    if upper_fuel == 0 {
        upper_fuel = 1;
    }
    while ore_needed_for_fuel(upper_fuel, ordered_reactions) <= input_ore {
        upper_fuel *= 2;
    }
    while lower_fuel + 1 < upper_fuel {
        let mid_fuel = (lower_fuel + upper_fuel) / 2;
        if ore_needed_for_fuel(mid_fuel, ordered_reactions) <= input_ore {
            lower_fuel = mid_fuel;
        } else {
            upper_fuel = mid_fuel;
        }
    }
    lower_fuel
}

#[test]
fn test_max_fuel_from_ore() {
    let a = &topological_sort(parse_input(
        "10 ORE => 10 A
         1 A => 1 FUEL"));
    assert_eq!(max_fuel_from_ore(1, &a), 0);
    assert_eq!(max_fuel_from_ore(9, &a), 0);
    assert_eq!(max_fuel_from_ore(10, &a), 10);
    assert_eq!(max_fuel_from_ore(11, &a), 10);

    let ab = &topological_sort(parse_input(
        "10 ORE => 10 A
         1 ORE => 1 B
         1 A, 1 B => 1 FUEL"));
    assert_eq!(max_fuel_from_ore(1, &ab), 0);
    assert_eq!(max_fuel_from_ore(10, &ab), 0);
    assert_eq!(max_fuel_from_ore(11, &ab), 1);
    assert_eq!(max_fuel_from_ore(12, &ab), 2);
    assert_eq!(max_fuel_from_ore(20, &ab), 10);
    assert_eq!(max_fuel_from_ore(21, &ab), 10);
}

fn part2(input: &str) -> usize {
    let reactions = topological_sort(parse_input(input));
    max_fuel_from_ore(1_000_000_000_000, &reactions)
}

#[test]
fn test_part2() {
    assert_eq!(
        part2("157 ORE => 5 NZVS
               165 ORE => 6 DCFZ
               44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
               12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
               179 ORE => 7 PSHF
               177 ORE => 5 HKGWZ
               7 DCFZ, 7 PSHF => 2 XJWVT
               165 ORE => 2 GPVTF
               3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"),
        82892753);
    assert_eq!(
        part2("2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
               17 NVRVD, 3 JNWZP => 8 VPVL
               53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
               22 VJHF, 37 MNCFX => 5 FWMGM
               139 ORE => 4 NVRVD
               144 ORE => 7 JNWZP
               5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
               5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
               145 ORE => 6 MNCFX
               1 NVRVD => 8 CXFTF
               1 VJHF, 6 MNCFX => 4 RFSQX
               176 ORE => 6 VJHF"),
        5586022);
    assert_eq!(
        part2("171 ORE => 8 CNZTR
               7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
               114 ORE => 4 BHXH
               14 VRPVC => 6 BMBT
               6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
               6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
               15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
               13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
               5 BMBT => 4 WPTQ
               189 ORE => 9 KTJDG
               1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
               12 VRPVC, 27 CNZTR => 2 XDBXC
               15 KTJDG, 12 BHXH => 5 XCVML
               3 BHXH, 2 VRPVC => 7 MZWV
               121 ORE => 7 VRPVC
               7 XCVML => 6 RJRHP
               5 BHXH, 4 VRPVC => 5 LTCX"),
        460664);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 579797, part2, 2521844);
}
