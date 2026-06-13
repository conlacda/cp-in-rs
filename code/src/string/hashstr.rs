// ANCHOR: hash-function
use std::sync::OnceLock;
const FACTOR: u32 = 263; // prime number >= 255 which is length of Acsii
const MOD: u32 = 1_000_000_007;

static FACTORIAL: OnceLock<Vec<u32>> = OnceLock::new();
static FACTORIAL_INV: OnceLock<Vec<u32>> = OnceLock::new();

/// Polynomial rolling hash for sequences.
///
/// # Example
/// ```rust
/// use rs_space::string::hashstr::Hash;
/// Hash::init(250000);
/// let v = vec![1, 2, 3, 4, 5];
/// let h = Hash::new(&v);
/// let str_hash = Hash::new(String::from("aabaa").as_bytes());
/// assert!(str_hash.is_substr_palindrome(/* start */0, /* end */ 4));
/// assert!(str_hash.common_prefix(0, 3) == 2);
///
/// assert_eq!(h.substr(1, 3), Hash::once(&[2, 3, 4]));
/// assert_eq!(h.rolling(3, 1), Hash::once(&[4, 5, 1, 2]));
/// assert!(Hash::is_palindrome(Hash::once("abcba".as_bytes())));
/// dbg!(Hash::reversed(Hash::once(&[1, 2, 3])));
/// assert_eq!(Hash::merge(Hash::once(&[1, 2]), 2, Hash::once(&[3, 4, 5]), 3), Hash::once(&[1, 2, 3, 4, 5]));
/// ```
///
/// # Notes
/// - All indices are **inclusive**
/// - Call **init()** before using
pub struct Hash<T> {
    pub s: Vec<T>,
    prefix_hash: Vec<u32>,
    suffix_hash: Vec<u32>,
}

impl Hash<()> {
    pub fn init(len: usize) {
        FACTORIAL.get_or_init(|| -> Vec<u32> {
            let mut p = 1;
            let mut pc: Vec<u32> = vec![0; len];
            let mut i = 0;
            while i < len {
                pc[i] = p;
                p = ((p as u64 * FACTOR as u64) % MOD as u64) as u32;
                i += 1;
            }
            pc
        });
        FACTORIAL_INV.get_or_init(|| -> Vec<u32> {
            let mut inv: Vec<u32> = vec![0; len];
            let mut i = 0;
            while i < len {
                inv[i] = Hash::mod_inv(FACTORIAL.get().unwrap()[i]);
                i += 1;
            }
            inv
        });
    }

    /// check if a hash_value belongs to a palindrome string
    pub fn is_palindrome(hash_value: u64) -> bool {
        ((hash_value >> 32) & 0xFFFFFFFF) == (hash_value & 0xFFFFFFFF)
    }

    pub fn reversed(hash_value: u64) -> u64 {
        let first = (hash_value >> 32) & 0xFFFFFFFF;
        let last = hash_value & 0xFFFFFFFF;
        (last << 32) | first
    }

    const fn mod_inv(mut x: u32) -> u32 {
        assert!(x != 0);
        let mut val = 1u32;
        while x != 1 {
            let z = MOD / x;
            val = (val as u64 * (MOD - z) as u64 % MOD as u64) as u32;
            x = MOD - x * z;
        }
        val
    }

    /// Concatenates two hashed sequences.
    ///
    /// If:
    /// - `h1` is the hash of sequence `A` with length `len1`
    /// - `h2` is the hash of sequence `B` with length `len2`
    ///
    /// then `merge(h1, len1, h2, len2)` returns the hash of `A + B`.
    pub fn merge(h1: u64, len1: usize, h2: u64, len2: usize) -> u64 {
        let fwh1: u64 = (h1 >> 32) & 0xFFFFFFFF;
        let bwh1: u64 = h1 & 0xFFFFFFFF;
        let fwh2: u64 = (h2 >> 32) & 0xFFFFFFFF;
        let bwh2: u64 = h2 & 0xFFFFFFFF;
        let first = (fwh1 + fwh2 * FACTORIAL.get().unwrap()[len1] as u64) % MOD as u64;
        let last = (bwh2 + bwh1 * FACTORIAL.get().unwrap()[len2] as u64) % MOD as u64;
        (first << 32) | last
    }
}

impl<T> Hash<T>
where
    T: Into<i64> + Copy,
{
    pub fn new(s: &[T]) -> Self {
        assert!(
            FACTORIAL.get().is_some(),
            "ERROR: run Hash::init(max_length) first"
        );
        let n = s.len();
        let mut prefix_hash: Vec<u32> = [0].to_vec();
        let mut suffix_hash: Vec<u32> = [0].to_vec();
        let mut pref: u32 = 0;
        let mut suf: u32 = 0;
        for i in 0..n {
            pref = ((((pref as i64 + s[i].into() * FACTORIAL.get().unwrap()[i] as i64)
                % MOD as i64)
                + MOD as i64)
                % MOD as i64) as u32;
            suf = ((((suf as i64 + s[n - i - 1].into() * FACTORIAL.get().unwrap()[i] as i64)
                % MOD as i64)
                + MOD as i64)
                % MOD as i64) as u32;
            prefix_hash.push(pref);
            suffix_hash.push(suf);
        }
        Self {
            s: s.to_vec(),
            prefix_hash,
            suffix_hash,
        }
    }

    pub fn once(s: &[T]) -> u64 {
        assert!(
            FACTORIAL.get().is_some(),
            "ERROR: run Hash::init(max_length) first"
        );
        let one_way_hash = |s: &[T]| -> u32 {
            let mut hash_value: i64 = 0;
            for i in 0..s.len() {
                hash_value =
                    (hash_value + s[i].into() * FACTORIAL.get().unwrap()[i] as i64) % MOD as i64;
            }
            if hash_value < 0 {
                hash_value += MOD as i64;
            }
            hash_value as u32
        };
        let forward = one_way_hash(s);
        let reversed_s: Vec<T> = s.iter().rev().copied().collect();
        let backward = one_way_hash(&reversed_s);
        (forward as u64) << 32 | (backward as u64)
    }

    fn len(&self) -> usize {
        self.s.len()
    }

    /// Returns hash of `s[start..=end]`
    ///
    /// # Example
    /// ```rust
    /// use rs_space::string::hashstr::Hash;
    /// let v = vec![10, 20, 30, 40];
    /// Hash::init(250000);
    /// let h = Hash::new(&v);
    /// assert_eq!(h.substr(1, 2), Hash::once(&[20, 30]));
    /// ```
    pub fn substr(&self, start: usize, end: usize) -> u64 {
        let n = self.s.len();
        let one_way_hash = |start: usize, end: usize, ps_hash: &[u32]| -> u32 {
            assert!(end < n);
            let a = ps_hash[end + 1] as u64;
            let b = ps_hash[start] as u64;
            let res = (a + MOD as u64 - b) % MOD as u64;
            (res * (FACTORIAL_INV.get().unwrap()[start] as u64) % MOD as u64) as u32
        };
        (one_way_hash(start, end, &self.prefix_hash) as u64) << 32
            | one_way_hash(n - 1 - end, n - 1 - start, &self.suffix_hash) as u64
    }

    /// Returns the hash of a segment on a circular array.
    ///
    /// If `start <= end`, this is the same as `substr(start, end)`.
    ///
    /// If `start > end`, this means:
    /// `s[start..=n-1] + s[0..=end]`.
    ///
    /// # Example
    /// For `s = [a, b, c, d, e]`,
    /// `rolling(3, 1)` hashes `[d, e, a, b]`.
    pub fn rolling(&self, start: usize, end: usize) -> u64 {
        let n = self.s.len();
        if start <= end {
            self.substr(start, end)
        } else {
            Hash::merge(
                self.substr(start, n - 1),
                n - start,
                self.substr(0, end),
                end + 1,
            )
        }
    }

    pub fn is_substr_palindrome(&self, start: usize, end: usize) -> bool {
        Hash::is_palindrome(self.substr(start, end))
    }

    /// Return the longest length of common prefix of s[start1..] & s[start2..]
    pub fn common_prefix(&self, start1: usize, start2: usize) -> usize {
        let mut l = 0;
        let mut r = (self.len() - start1).min(self.len() - start2);
        while l < r {
            let mut mid = (l + r) / 2;
            if mid * 2 < l + r {
                mid += 1;
            }
            if self.substr(start1, start1 + mid - 1) == self.substr(start2, start2 + mid - 1) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        if l != 0 {
            assert!(self.substr(start1, start1 + l - 1) == self.substr(start2, start2 + l - 1));
        }
        l
    }

    // pub fn compare_substrs(start1, len1, start2, len2)
}
// ANCHOR_END: hash-function

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_once_substr() {
        let mut r = Random::new();
        Hash::init(250000);
        const N: usize = 1000;
        let v = r.vector(N, -1_000_000_000..1_000_000_000);
        let hash = Hash::new(&v);
        dbg!(Hash::once(&v));
        dbg!(hash.substr(0, N - 1));
        assert_eq!(Hash::once(&v), hash.substr(0, N - 1));
    }

    #[test]
    fn test_merge_hash() {
        let mut r = Random::new();
        Hash::init(250000);
        const N: usize = 1000;
        let v = r.vector(N, -1_000_000_000..1_000_000_000);
        let hash = Hash::new(&v);
        let start: usize = r.num(0..=200);
        let mid: usize = r.num(start + 1..=500);
        let end: usize = r.num(mid + 1..N);
        let sm = hash.substr(start, mid);
        let me = hash.substr(mid + 1, end);
        assert_eq!(
            hash.substr(start, end),
            Hash::merge(sm, mid - start + 1, me, end - mid)
        );
    }

    #[test]
    fn test_rolling() {
        let mut r = Random::new();
        Hash::init(250000);
        const N: usize = 1000;
        let v = r.vector(N, -1_000_000_000..1_000_000_000);
        let hash = Hash::new(&v);
        let start = r.num(501..N);
        let end = r.num(0..=500);
        let s: Vec<i32> = v[start..=N - 1]
            .iter()
            .chain(v[0..=end].iter())
            .cloned()
            .collect();
        assert_eq!(hash.rolling(start, end), Hash::once(&s));
    }

    #[test]
    fn test_palindrome() {
        let mut r = Random::new();
        Hash::init(250000);
        let len = r.num(1..10000);
        let s = r.string(len);
        let rs: String = s.chars().rev().collect();
        let palindrome = s.clone() + &rs.clone();
        let hash_value = Hash::once(palindrome.as_bytes());
        assert!(Hash::is_palindrome(hash_value));
        let h = Hash::new(palindrome.as_bytes());
        assert!(h.is_substr_palindrome(0, palindrome.len() - 1));
    }

    #[test]
    fn test_reversed() {
        let mut r = Random::new();
        Hash::init(250000);
        let len = r.num(1..10000);
        let s = r.string(len);
        let hash_value = Hash::once(&s.as_bytes());
        assert_eq!(hash_value, Hash::reversed(Hash::reversed(hash_value)));
    }

    #[test]
    fn test_common_prefix() {
        let mut r = Random::new();
        Hash::init(250000);
        let len = 10000;
        let s = r.string(len);
        let h = Hash::new(s.as_bytes());
        for _ in 0..100 {
            let start1 = r.num(0..5000);
            let start2 = r.num(5000..10000);
            let common_prefix = h.common_prefix(start1, start2);
            assert_eq!(
                &s[start1..start1 + common_prefix],
                &s[start2..start2 + common_prefix]
            );
            if start2 + common_prefix < len {
                assert_ne!(
                    &s[start1..start1 + common_prefix + 1],
                    &s[start2..start2 + common_prefix + 1]
                );
            }
        }
    }
}
