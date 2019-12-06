use std::collections::HashMap;

type Graph = HashMap<String, String>;

fn parse_graph(input: &str) -> Graph {
    let mut graph = Graph::new();
    for line in input.lines() {
        let parts = line.trim().split(')').collect::<Vec<_>>();
        graph.insert(parts[1].to_string(), parts[0].to_string());
    }
    graph
}

fn part1(input: &str) -> usize {
    let graph = parse_graph(input);
    graph.keys()
        .map(|leaf| {
            let mut count = 0;
            let mut cur = leaf;
            while cur != "COM" {
                count += 1;
                cur = &graph[cur];
            }
            count
        })
        .sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"), 42);
}

fn path_to_root<'a>(graph: &'a Graph, leaf: &'a str) -> Vec<&'a str> {
    let mut path = vec![];
    let mut cur = leaf;
    loop {
        path.push(cur);
        if cur == "COM" {
            break;
        }
        cur = &graph[cur];
    }
    path.reverse();
    path
}

fn part2(input: &str) -> usize {
    let graph = parse_graph(input);
    let you_path = path_to_root(&graph, "YOU");
    let san_path = path_to_root(&graph, "SAN");
    let mut i = 0;
    while you_path[i] == san_path[i] {
        i += 1;
    }
    you_path.len() - 1 - i + san_path.len() - 1 - i
}

#[test]
fn test_part2() {
    assert_eq!(part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"), 4);
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 249308, part2, 349);
}
