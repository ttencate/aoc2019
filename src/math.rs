pub fn gcd<T>(mut a: T, mut b: T) -> T
    where T : std::cmp::PartialEq + std::ops::Rem<Output = T> + Copy + Default
{
    let zero = T::default();
    while b != zero {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

pub fn lcm<T>(a: T, b: T) -> T
    where T : std::cmp::PartialEq + std::ops::Mul<Output = T> + std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy + Default
{
    a * b / gcd(a, b)
}

pub fn inverse_mod_n(a: i128, n: i128) -> i128 {
    // https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Modular_integers
    let mut t = 0;
    let mut r = n;
    let mut new_t = 1;
    let mut new_r = a;
    while new_r != 0 {
        let q = r / new_r;
        let old_t = t;
        t = new_t;
        new_t = old_t - q * new_t;
        let old_r = r;
        r = new_r;
        new_r = old_r - q * new_r;
    }
    if r > 1 {
        panic!("{:?} is not invertible modulo {:?}", a, n);
    }
    if t < 0 {
        t += n;
    }
    t
}

#[test]
fn test_inverse_mod_n() {
    assert_eq!(inverse_mod_n(1, 2), 1);

    assert_eq!(inverse_mod_n(1, 3), 1);
    assert_eq!(inverse_mod_n(2, 3), 2);

    assert_eq!(inverse_mod_n(1, 5), 1);
    assert_eq!(inverse_mod_n(2, 5), 3);
    assert_eq!(inverse_mod_n(3, 5), 2);
    assert_eq!(inverse_mod_n(4, 5), 4);

    assert_eq!(inverse_mod_n(1, 10), 1);
    assert_eq!(inverse_mod_n(3, 10), 7);
    assert_eq!(inverse_mod_n(7, 10), 3);
    assert_eq!(inverse_mod_n(9, 10), 9);

    assert_eq!(inverse_mod_n(1, 11), 1);
    assert_eq!(inverse_mod_n(2, 11), 6);
    assert_eq!(inverse_mod_n(3, 11), 4);
    assert_eq!(inverse_mod_n(4, 11), 3);
    assert_eq!(inverse_mod_n(5, 11), 9);
    assert_eq!(inverse_mod_n(6, 11), 2);
    assert_eq!(inverse_mod_n(7, 11), 8);
    assert_eq!(inverse_mod_n(8, 11), 7);
    assert_eq!(inverse_mod_n(9, 11), 5);
    assert_eq!(inverse_mod_n(10, 11), 10);
}
