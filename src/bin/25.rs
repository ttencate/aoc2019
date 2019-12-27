use aoc::intcode::*;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
enum State {
    NoState,
    Doors,
    Items,
}

use State::*;

#[derive(Debug, Clone)]
struct Adventure {
    program: Program,
    print: bool,
    location: String,
    doors_here: Vec<String>,
    items_here: Vec<String>,
    inventory: Vec<String>,
    code: Option<String>,
}

impl Adventure {
    fn new(program: Program, print: bool) -> Self {
        let mut adventure = Adventure {
            program,
            print,
            location: "".to_string(),
            doors_here: Vec::new(),
            items_here: Vec::new(),
            inventory: Vec::new(),
            code: None,
        };
        adventure.parse_output();
        adventure
    }

    fn parse_output(&mut self) {
        let output = self.program.take_output_ascii();
        if self.print {
            print!("{}", output);
        }
        let mut state = NoState;
        for line in output.lines() {
            if line.starts_with("==") {
                self.location = line.trim_start_matches("== ").trim_end_matches(" ==").to_string();
            } else if line == "Doors here lead:" {
                state = Doors;
                self.doors_here.clear();
            } else if line == "Items here:" {
                state = Items;
                self.items_here.clear();
            } else if line.starts_with("- ") {
                match state {
                    NoState => {},
                    Doors => {
                        let door = line.trim_start_matches("- ").to_string();
                        self.doors_here.push(door);
                    },
                    Items => {
                        let item = line.trim_start_matches("- ").to_string();
                        self.items_here.push(item);
                    },
                }
            } else if line.starts_with("You take ") {
                let taken = line.trim_start_matches("You take the ").trim_end_matches(".");
                self.items_here.retain(|i| i != taken);
                self.inventory.push(taken.to_string());
            } else if line.starts_with("You drop ") {
                let dropped = line.trim_start_matches("You drop the ").trim_end_matches(".");
                self.inventory.retain(|i| i != dropped);
                self.items_here.push(dropped.to_string());
            } else if line.starts_with("\"Oh, hello!") {
                self.code = Some(line.trim_start_matches("\"Oh, hello! You should be able to get in by typing ").trim_end_matches(" on the keypad at the main airlock.\"").to_string());
            }
        }
    }

    fn run_command(&mut self, command: &str) {
        let input = command.to_string() + "\n";
        if self.print {
            print!("{}", input);
        }
        self.program.give_input_ascii(&input);
        self.parse_output();
    }
}

fn collect_items(adventure: &mut Adventure, visited: &mut HashSet<String>, stack: &mut Vec<String>, path_to_security: &mut Vec<String>) {
    let current_location = adventure.location.clone();
    if !visited.insert(current_location.clone()) {
        return;
    }

    let items = adventure.items_here.clone();
    for item in items {
        if item != "infinite loop" && item != "molten lava" && item != "photons" && item != "escape pod" && item != "giant electromagnet" {
            adventure.run_command(&format!("take {}", item));
        }
    }

    let doors = adventure.doors_here.clone();
    for door in doors {
        adventure.run_command(&door);
        stack.push(door.clone());
        if adventure.location == current_location {
            if adventure.location == "Security Checkpoint" {
                *path_to_security = stack.clone();
                path_to_security.push(door.clone());
            }
            continue;
        }

        collect_items(adventure, visited, stack, path_to_security);

        stack.pop();
        let door_back = match door.as_ref() {
            "north" => "south",
            "east" => "west",
            "south" => "north",
            "west" => "east",
            _ => panic!("Uninvertible door {}", door),
        };
        adventure.run_command(door_back);
    }
}

fn part1(input: &str) -> String {
    let mut adventure = Adventure::new(Program::parse(input), false);

    let mut visited = HashSet::new();
    let mut stack = Vec::new();
    let mut path_to_security = Vec::new();
    collect_items(&mut adventure, &mut visited, &mut stack, &mut path_to_security);

    for door in path_to_security.iter().take(path_to_security.len() - 1) {
        adventure.run_command(&door);
    }
    let door_to_pressure = path_to_security.last().unwrap();

    adventure.run_command("inv");

    let items = adventure.inventory.clone();
    for item in items.iter() {
        adventure.run_command(&format!("drop {}", item));
    }
    for bits in 0..(1 << items.len()) {
        let mut adv = adventure.clone();
        for i in 0..items.len() {
            if bits & (1 << i) != 0 {
                adv.run_command(&format!("take {}", items[i]));
            }
        }
        adv.run_command(door_to_pressure);
        if let Some(code) = adv.code {
            return code;
        }
    }
    panic!("Code not found");
}

fn part2(_input: &str) -> String {
    "victory".to_string()
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, "1073874948".to_string(), part2, "victory".to_string());
}
