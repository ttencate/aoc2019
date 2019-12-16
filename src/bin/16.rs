use itertools::iproduct;

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn str_to_vec(input: &str) -> Vec<i32> {
    input.trim().as_bytes().iter().map(|b| (b - b'0') as i32).collect()
}

fn vec_to_string(v: &[i32]) -> String {
    String::from_utf8(v.iter().map(|&x| b'0' + x as u8).collect()).unwrap()
}

fn fft(input: &[i32], num_phases: usize) -> Vec<i32> {
    let n = input.len();
    let matrix = iproduct!(0..n, 0..n)
        .map(|(row, col)| PATTERN[((col + 1) / (row + 1)) % 4])
        .collect::<Vec<_>>();
    let mut v = input.to_vec();
    for _ in 0..num_phases {
        v = (0..n).map(|i| {
            (v.iter().zip(matrix[(i * n)..((i + 1) * n)].iter()).map(|(x, m)| x * m).sum::<i32>() % 10).abs()
        }).collect()
    }
    v
}

#[test]
fn test_fft() {
    assert_eq!(fft(&str_to_vec("12345678"), 1), str_to_vec("48226158"));
    assert_eq!(fft(&str_to_vec("12345678"), 2), str_to_vec("34040438"));
    assert_eq!(fft(&str_to_vec("12345678"), 3), str_to_vec("03415518"));
    assert_eq!(fft(&str_to_vec("12345678"), 4), str_to_vec("01029498"));
    assert_eq!(fft(&str_to_vec("80871224585914546619083218645595"), 100)[0..8].to_vec(), str_to_vec("24176176"));
    assert_eq!(fft(&str_to_vec("19617804207202209144916044189917"), 100)[0..8].to_vec(), str_to_vec("73745418"));
    assert_eq!(fft(&str_to_vec("69317163492948606335995924319873"), 100)[0..8].to_vec(), str_to_vec("52432133"));
}

fn part1(input: &str) -> String {
    vec_to_string(&fft(&str_to_vec(input), 100)[0..8])
}

fn fft_tail(input: &[i32], num_phases: usize, offset: usize) -> Vec<i32> {
    let n = input.len();
    assert!(offset >= n / 2);
    let mut cur = input[offset..].to_vec();
    for _ in 0..num_phases {
        let mut cum_sum = 0;
        for i in (0..(n - offset)).rev() {
            cum_sum += cur[i];
            cur[i] = (cum_sum % 10).abs();
        }
    }
    cur
}

#[test]
fn test_fft_tail() {
    assert_eq!(fft_tail(&str_to_vec("12345678"), 4, 4), str_to_vec("9498"));
    assert_eq!(fft_tail(&str_to_vec("12345678"), 4, 5), str_to_vec("498"));
    assert_eq!(fft_tail(&str_to_vec("12345678"), 4, 6), str_to_vec("98"));
    assert_eq!(fft_tail(&str_to_vec("12345678"), 4, 7), str_to_vec("8"));
}

fn real_signal(input: &[i32]) -> Vec<i32> {
    let offset = vec_to_string(&input[0..7]).parse::<usize>().unwrap();
    fft_tail(&input.repeat(10000), 100, offset)[0..8].to_vec()
}

#[test]
fn test_real_signal() {
    assert_eq!(real_signal(&str_to_vec("03036732577212944063491565474664")), str_to_vec("84462026"));
    assert_eq!(real_signal(&str_to_vec("02935109699940807407585447034323")), str_to_vec("78725270"));
    assert_eq!(real_signal(&str_to_vec("03081770884921959731165446850517")), str_to_vec("53553731"));
}

fn part2(input: &str) -> String {
    vec_to_string(&real_signal(&str_to_vec(input)))
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, "34694616".to_string(), part2, "17069048".to_string());
}
