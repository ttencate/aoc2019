use std::collections::HashMap;

type Chemical = String;

#[derive(Debug)]
struct Reaction {
    inputs: HashMap<Chemical, usize>,
    output_chemical: Chemical,
    output_quantity: usize,
}

#[derive(Debug, Default)]
struct Reactor {
    have: HashMap<Chemical, usize>,
    mined_ore: usize,
}

impl Reactor {
    fn run_reaction(&mut self, reaction: &Reaction, num_runs: usize, reactions: &HashMap<Chemical, Reaction>) {
        assert!(num_runs > 0);
        for (input_chemical, input_quantity) in &reaction.inputs {
            let need_quantity = input_quantity * num_runs;
            self.ensure_have(input_chemical.to_string(), need_quantity, reactions);
            assert!(*self.have.entry(input_chemical.to_string()).or_default() >= need_quantity);
            *self.have.entry(input_chemical.to_string()).or_default() -= need_quantity;
        }
        *self.have.entry(reaction.output_chemical.to_string()).or_default() += reaction.output_quantity * num_runs;
    }

    fn ensure_have(&mut self, chemical: Chemical, quantity: usize, reactions: &HashMap<Chemical, Reaction>) {
        if chemical == "ORE" {
            self.mine_ore(quantity);
            *self.have.entry("ORE".to_string()).or_default() += quantity;
        } else {
            let have_quantity = *self.have.get(&chemical).unwrap_or(&0);
            if have_quantity < quantity {
                let reaction = reactions.get(&chemical).unwrap().clone();
                let num_runs = (quantity - have_quantity + reaction.output_quantity - 1) / reaction.output_quantity;
                self.run_reaction(reaction, num_runs, reactions);
            }
            assert!(*self.have.entry(chemical).or_default() >= quantity);
        }
    }

    fn mine_ore(&mut self, quantity: usize) {
        self.mined_ore += quantity;
    }

    fn produce_max(&mut self, chemical: Chemical, reactions: &HashMap<Chemical, Reaction>) -> usize {
        *self.have.get(chemical).unwrap()
    }
}

fn parse_chemical_quantity(input: &str) -> (Chemical, usize) {
    let mut parts = input.trim().split(' ');
    let quantity = parts.next().unwrap().parse::<usize>().unwrap();
    let chemical = parts.next().unwrap().to_string();
    (chemical, quantity)
}

fn parse_input(input: &str) -> HashMap<Chemical, Reaction> {
    input.lines()
        .map(|line| {
            let mut split = line.split("=>");
            let inputs = split.next().unwrap().split(",").map(parse_chemical_quantity).collect();
            let (output_chemical, output_quantity) = parse_chemical_quantity(split.next().unwrap());
            let reaction = Reaction { inputs, output_chemical, output_quantity };
            (reaction.output_chemical.to_string(), reaction)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let reactions = parse_input(input);
    let mut reactor = Reactor::default();
    reactor.ensure_have("FUEL".to_string(), 1, &reactions);
    reactor.mined_ore
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

fn part2(input: &str) -> usize {
    let reactions = parse_input(input);
    let mut reactor = Reactor::default();
    reactor.have.insert("ORE".to_string(), 1000000000000);
    reactor.produce_max("FUEL".to_string(), &reactions)
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
    // aoc::test(part1, "TODO".to_string(), part2, "TODO".to_string());
}
