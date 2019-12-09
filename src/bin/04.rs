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

fn make_increasing(mut cur: Vec<u8>) -> Vec<u8> {
    for i in 1..cur.len() {
        if cur[i] < cur[i - 1] {
            cur[i] = cur[i - 1];
        }
    }
    cur
}

#[test]
fn test_make_increasing() {
    assert_eq!(make_increasing("123456".as_bytes().to_vec()), "123456".as_bytes().to_vec());
    assert_eq!(make_increasing("127456".as_bytes().to_vec()), "127777".as_bytes().to_vec());
    assert_eq!(make_increasing("923256".as_bytes().to_vec()), "999999".as_bytes().to_vec());
}

fn increment(mut cur: Vec<u8>) -> Option<Vec<u8>> {
    for i in (0..cur.len()).rev() {
        if cur[i] < b'9' {
            cur[i] += 1;
            for j in i + 1..cur.len() {
                cur[j] = cur[i];
            }
            return Some(cur);
        }
    }
    None
}

#[test]
fn test_increment() {
    assert_eq!(increment("123456".as_bytes().to_vec()), Some("123457".as_bytes().to_vec()));
    assert_eq!(increment("123459".as_bytes().to_vec()), Some("123466".as_bytes().to_vec()));
    assert_eq!(increment("127777".as_bytes().to_vec()), Some("127778".as_bytes().to_vec()));
    assert_eq!(increment("999999".as_bytes().to_vec()), None);
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<u8>) {
    let from_to = input.trim().split('-').collect::<Vec<_>>();
    let from = from_to[0].as_bytes().to_vec();
    let to = from_to[1].as_bytes().to_vec();
    assert_eq!(from.len(), to.len());
    (from, to)
}

fn part1(input: &str) -> usize {
    let (from, to) = parse_input(input);
    let mut cur = make_increasing(from.to_vec());

    let mut count = 0;
    while cur < to {
        if has_adjacent_digits(&cur) {
            count += 1;
        }
        cur = increment(cur).unwrap();
    }
    count
}

fn part2(input: &str) -> usize {
    let (from, to) = parse_input(input);
    let mut cur = make_increasing(from.to_vec());

    let mut count = 0;
    while cur < to {
        if has_two_adjacent_digits(&cur) {
            count += 1;
        }
        cur = increment(cur).unwrap();
    }
    count
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 979, part2, 635);
}
