use packed_simd::Simd;

const LANES: usize = 16;
type V = Simd<[i32; LANES]>;

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn str_to_vec(input: &str) -> Vec<i32> {
    input.trim().as_bytes().iter().map(|b| (b - b'0') as i32).collect()
}

fn vec_to_string(v: &[i32]) -> String {
    String::from_utf8(v.iter().map(|&x| b'0' + x as u8).collect()).unwrap()
}

#[derive(Debug, Clone)]
struct SimdVec {
    main: Vec<V>,
    rest: Vec<i32>,
}

impl std::iter::FromIterator<i32> for SimdVec {
    fn from_iter<I: IntoIterator<Item=i32>>(input: I) -> Self {
        let v = input.into_iter().collect::<Vec<_>>();
        let mut main = Vec::with_capacity(v.len() / LANES);
        let mut iter = v.chunks_exact(LANES);
        while let Some(chunk) = iter.next() {
            main.push(V::from_slice_unaligned(chunk));
        }
        let rest = iter.remainder().to_vec();
        SimdVec {
            main,
            rest,
        }
    }
}

impl From<SimdVec> for Vec<i32> {
    fn from(v: SimdVec) -> Self {
        let mut out = Vec::with_capacity(v.main.len() * LANES + v.rest.len());
        for s in v.main {
            let mut slice = [0; LANES];
            s.write_to_slice_unaligned(&mut slice);
            out.extend(&slice);
        }
        out.extend(v.rest);
        out
    }
}

impl SimdVec {
    fn last_digit_of_dot_product(&self, other: &SimdVec) -> i32 {
        assert!(self.main.len() == other.main.len());
        assert!(self.rest.len() == other.rest.len());
        let mut out = 0;
        for i in 0..self.main.len() {
            // TODO this can also be simd'd
            out += (self.main[i] * other.main[i]).wrapping_sum();
        }
        for i in 0..self.rest.len() {
            out += self.rest[i] * other.rest[i];
        }
        (out % 10).abs()
    }
}

fn fft(input: &[i32], num_phases: usize) -> Vec<i32> {
    let n = input.len();
    let matrix: Vec<SimdVec> = (0..n)
        .map(|row| {
            (0..n).map(|col| PATTERN[((col + 1) / (row + 1)) % 4]).collect::<SimdVec>()
        })
        .collect();
    let mut v: SimdVec = input.iter().map(|&x| x as i32).collect();
    for _ in 0..num_phases {
        v = (0..n).map(|i| {
            matrix[i].last_digit_of_dot_product(&v)
        }).collect()
    }
    Vec::from(v).iter().map(|&x| x as i32).collect()
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
        // TODO simd this
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
