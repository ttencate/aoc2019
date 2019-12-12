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
