// ANCHOR: main
use std::collections::HashMap;
use std::hash::Hash;

/// # Example
/// ```rust
/// use rs_space::techniques::coordinate_compress::Compress;
/// let c = Compress::new(&[20, 40, 10, 30]); // compress = [1, 3, 0, 2];
/// // `up(x)` returns the compressed index of the smallest value >= x.
/// assert!(c.up(21) == Some(2));
/// // `down(x)` returns the compressed index of the largest value <= x.
/// assert!(c.down(21) == Some(1));
/// // c.by_index(index) = compress of a[index]
/// assert!(c.by_index(0) == 1);
/// // Convert a compressed index back to the original value.
/// assert!(c.original_val(3) == 40);
/// ```
pub struct Compress<T> {
    values: Vec<T>,
    compressed: Vec<usize>,
}

impl<T> Compress<T>
where
    T: Ord + Clone + Hash + Eq,
{
    pub fn new(a: &[T]) -> Self {
        let mut values = a.to_vec();
        values.sort();
        values.dedup();

        let mut m = HashMap::new();
        for (i, x) in values.iter().enumerate() {
            m.insert(x.clone(), i);
        }

        let compressed = a.iter().map(|x| m[x]).collect();

        Self { values, compressed }
    }

    pub fn up(&self, val: T) -> Option<usize> {
        let i = self.values.partition_point(|x| x < &val);

        if i < self.values.len() { Some(i) } else { None }
    }

    pub fn down(&self, val: T) -> Option<usize> {
        let i = self.values.partition_point(|x| x <= &val);

        if i > 0 { Some(i - 1) } else { None }
    }

    pub fn original_val(&self, compressed: usize) -> T {
        self.values[compressed].clone()
    }

    pub fn by_index(&self, index: usize) -> usize {
        self.compressed[index]
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test() {
        let mut r = Random::new();
        let a = r.vector(100, 0..1_000_000_000);
        let c = Compress::new(&a);

        let mut v = a.clone();
        v.sort();
        v.dedup();

        // test up
        for (index, value) in v.iter().enumerate() {
            assert!(c.up(*value - 1).unwrap() == index);
            assert!(c.up(*value).unwrap() == index);
        }
        assert!(c.up(v.last().unwrap() + 1) == None);

        // test down
        for (index, value) in v.iter().enumerate() {
            assert!(c.down(*value + 1).unwrap() == index);
            assert!(c.down(*value).unwrap() == index);
        }
        assert!(c.down(v.first().unwrap() - 1) == None);

        // original_value
        for (index, value) in v.iter().enumerate() {
            assert!(c.original_val(index) == *value);
        }

        // by_index
        for index in 0..100 {
            assert!(c.by_index(index) == c.up(a[index]).unwrap());
        }
    }
}
