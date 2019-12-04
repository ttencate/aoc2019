fn has_adjacent_digits(s: &[u8]) -> bool {
    s.iter().zip(s.iter().skip(1)).any(|(a, b)| a == b)
}

#[test]
fn test_has_adjacent_digits() {
    assert_eq!(has_adjacent_digits(b"111111"), true);
    assert_eq!(has_adjacent_digits(b"223450"), true);
    assert_eq!(has_adjacent_digits(b"123789"), false);
}

fn has_two_adjacent_digits(s: &[u8]) -> bool {
    let n = s.len();
    (0..(n - 1)).any(|i|
                     s[i] == s[i + 1] &&
                     (i == 0 || s[i - 1] != s[i]) &&
                     (i + 1 == n - 1 || s[i + 2] != s[i]))
}

#[test]
fn test_has_two_adjacent_digits() {
    assert_eq!(has_two_adjacent_digits(b"111111"), false);
    assert_eq!(has_two_adjacent_digits(b"223450"), true);
    assert_eq!(has_two_adjacent_digits(b"123789"), false);
    assert_eq!(has_two_adjacent_digits(b"112233"), true);
    assert_eq!(has_two_adjacent_digits(b"123444"), false);
    assert_eq!(has_two_adjacent_digits(b"111122"), true);
}

fn has_decreasing_digits(s: &[u8]) -> bool {
    s.iter().zip(s.iter().skip(1)).any(|(a, b)| a > b)
}

#[test]
fn test_has_decreasing_digits() {
    assert_eq!(has_decreasing_digits(b"111111"), false);
    assert_eq!(has_decreasing_digits(b"223450"), true);
    assert_eq!(has_decreasing_digits(b"123789"), false);
}

fn part1(input: &str) -> usize {
    let from_to = input.trim().split('-').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    (from_to[0]..from_to[1])
        .filter(|n| {
            let s = n.to_string();
            s.len() == 6 && has_adjacent_digits(s.as_bytes()) && !has_decreasing_digits(s.as_bytes())
        })
        .count()
}

fn part2(input: &str) -> usize {
    let from_to = input.trim().split('-').map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
    (from_to[0]..from_to[1])
        .filter(|n| {
            let s = n.to_string();
            s.len() == 6 && has_two_adjacent_digits(s.as_bytes()) && !has_decreasing_digits(s.as_bytes())
        })
        .count()
}

fn main() {
    aoc::main(part1, part2);
}
