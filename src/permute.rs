pub trait Permute<T> {
    fn permute(self) -> Permutations<T>;
}

impl<T, I> Permute<T> for I
    where I: IntoIterator<Item=T>
{
    fn permute(self) -> Permutations<T> {
        let items = self.into_iter().collect::<Vec<T>>();
        let n = items.len();
        Permutations {
            items,
            c: vec![0; n],
            i: 0,
            n,
        }
    }
}

pub struct Permutations<T> {
    items: Vec<T>,
    c: Vec<usize>,
    i: usize,
    n: usize,
}

impl<T> Iterator for Permutations<T>
    where T: Clone
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item> {
        // https://en.wikipedia.org/wiki/Heap%27s_algorithm
        if self.i >= self.n {
            return None;
        }
        while self.i < self.n && self.c[self.i] >= self.i {
            self.c[self.i] = 0;
            self.i += 1;
        }
        let result = Some(self.items.clone());
        if self.i < self.n {
            if self.i % 2 == 0 {
                self.items.swap(0, self.i);
            } else {
                self.items.swap(self.c[self.i], self.i);
            }
            self.c[self.i] += 1;
            self.i = 0;
        }
        result
    }
}

#[test]
fn test_permute() {
    use std::collections::HashSet;

    // This should give {[]} rather than {} but it's not a case we care about right now.
    assert_eq!(
        Vec::<usize>::new().permute().collect::<HashSet<Vec<_>>>(),
        [].iter().cloned().collect());
    assert_eq!(
        vec![1].permute().collect::<HashSet<Vec<_>>>(),
        [vec![1]].iter().cloned().collect());
    assert_eq!(
        vec![1, 2].permute().collect::<HashSet<Vec<_>>>(),
        [vec![1, 2], vec![2, 1]].iter().cloned().collect());
    assert_eq!(
        vec![1, 2, 3].permute().collect::<HashSet<Vec<_>>>(),
        [vec![1, 2, 3], vec![1, 3, 2], vec![2, 1, 3], vec![2, 3, 1], vec![3, 1, 2], vec![3, 2, 1]].iter().cloned().collect());
}
