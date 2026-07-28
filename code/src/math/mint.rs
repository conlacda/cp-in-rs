// ANCHOR: main
use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::OnceLock;

const PRECALCULATE_LEN: usize = 1_000_005;
struct Precalc {
    inv_range: Vec<u32>,
    fact: Vec<u32>,
    finv: Vec<u32>,
}
static PRECALC: OnceLock<Precalc> = OnceLock::new();

#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Mint<const MOD: u32 = 1000000007> {
    pub val: u32,
}

impl<const MOD: u32> Mint<MOD> {
    #[inline]
    fn mul_mod(a: u32, b: u32) -> u32 {
        (u64::from(a) * u64::from(b) % u64::from(MOD)) as u32
    }

    fn build_precalc() -> Precalc {
        let mut inv_range = vec![1u32; PRECALCULATE_LEN];
        for i in 2..PRECALCULATE_LEN {
            inv_range[i] =
                (MOD - Self::mul_mod(MOD / i as u32, inv_range[(MOD % i as u32) as usize])) % MOD;
        }

        let mut fact = vec![1u32; PRECALCULATE_LEN];
        for i in 1..PRECALCULATE_LEN {
            fact[i] = Self::mul_mod(fact[i - 1], i as u32);
        }

        let mut finv = vec![1u32; PRECALCULATE_LEN];
        for i in 1..PRECALCULATE_LEN {
            finv[i] = Self::mul_mod(finv[i - 1], inv_range[i]);
        }

        Precalc {
            inv_range,
            fact,
            finv,
        }
    }

    #[inline]
    fn precalc() -> &'static Precalc {
        PRECALC.get_or_init(Self::build_precalc)
    }

    pub fn inv(&self) -> Self {
        assert!(self.val != 0);
        let i = self.val as usize;
        if i < PRECALCULATE_LEN {
            return Self {
                val: Self::precalc().inv_range[i],
            };
        }

        let mut t: u32 = self.val;
        let mut val = 1u32;
        while t != 1 {
            let z = MOD / t;
            val = Self::mul_mod(val, MOD - z);
            t = MOD - t * z;
        }
        Self { val }
    }

    pub fn factor(&self) -> Self {
        assert!((self.val as usize) < PRECALCULATE_LEN);
        Self {
            val: Self::precalc().fact[self.val as usize],
        }
    }

    /// nCr
    pub fn ncr(&self, r: Self) -> Self {
        assert!((self.val as usize) < PRECALCULATE_LEN);
        assert!((r.val as usize) < PRECALCULATE_LEN);
        if self.val < r.val {
            return Self { val: 0 };
        }
        let p = Self::precalc();
        Self {
            val: p.fact[self.val as usize],
        } * Self {
            val: p.finv[r.val as usize],
        } * Self {
            val: p.finv[(self.val - r.val) as usize],
        }
    }

    /// nPr
    pub fn npr(&self, r: Self) -> Self {
        assert!((self.val as usize) < PRECALCULATE_LEN);
        assert!((r.val as usize) < PRECALCULATE_LEN);
        if self.val < r.val {
            return Self { val: 0 };
        }

        let p = Self::precalc();
        Self {
            val: p.fact[self.val as usize],
        } * Self {
            val: p.finv[(self.val - r.val) as usize],
        }
    }

    pub fn pow(&self, mut exp: u32) -> Self {
        let mut res: Self = 1.into();
        let mut cur: Self = *self;
        while exp != 0 {
            if (exp & 1) != 0 {
                res *= cur;
            }
            cur *= cur;
            exp >>= 1;
        }
        res
    }
}

macro_rules! impl_from_integer {
    ($($type:ty),* $(,)?) => {
        $(
            impl<const MOD: u32> From<$type> for Mint<MOD> {
                fn from(v: $type) -> Self {
                    Self {
                        val: (v as i128).rem_euclid(MOD as i128) as u32,
                    }
                }
            }
        )*
    };
}

impl_from_integer!(i32, u32, i64, u64);

impl<const MOD: u32> Add for Mint<MOD> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from(self.val as i64 + rhs.val as i64)
    }
}
impl<const MOD: u32> AddAssign for Mint<MOD> {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (*self + rhs).val;
    }
}

impl<const MOD: u32> Sub for Mint<MOD> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from(self.val as i64 - rhs.val as i64)
    }
}
impl<const MOD: u32> SubAssign for Mint<MOD> {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (*self - rhs).val;
    }
}

impl<const MOD: u32> Mul for Mint<MOD> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            val: Self::mul_mod(self.val, rhs.val),
        }
    }
}

impl<const MOD: u32> MulAssign for Mint<MOD> {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (*self * rhs).val;
    }
}

impl<const MOD: u32> Div for Mint<MOD> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.mul(rhs.inv())
    }
}

impl<const MOD: u32> DivAssign for Mint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        *self = self.div(rhs);
    }
}

macro_rules! impl_ops_with_integer {
    ($($type:ty),* $(,)?) => {
        $(
            impl<const MOD: u32> Add<$type> for Mint<MOD> {
                type Output = Self;
                fn add(self, rhs: $type) -> Self::Output {
                    self + Self::from(rhs)
                }
            }

            impl<const MOD: u32> AddAssign<$type> for Mint<MOD> {
                fn add_assign(&mut self, rhs: $type) {
                    *self += Self::from(rhs);
                }
            }

            impl<const MOD: u32> Sub<$type> for Mint<MOD> {
                type Output = Self;
                fn sub(self, rhs: $type) -> Self::Output {
                    self - Self::from(rhs)
                }
            }

            impl<const MOD: u32> SubAssign<$type> for Mint<MOD> {
                fn sub_assign(&mut self, rhs: $type) {
                    *self -= Self::from(rhs);
                }
            }

            impl<const MOD: u32> Mul<$type> for Mint<MOD> {
                type Output = Self;
                fn mul(self, rhs: $type) -> Self::Output {
                    self * Self::from(rhs)
                }
            }

            impl<const MOD: u32> MulAssign<$type> for Mint<MOD> {
                fn mul_assign(&mut self, rhs: $type) {
                    *self *= Self::from(rhs);
                }
            }

            impl<const MOD: u32> Div<$type> for Mint<MOD> {
                type Output = Self;
                fn div(self, rhs: $type) -> Self::Output {
                    self / Self::from(rhs)
                }
            }

            impl<const MOD: u32> DivAssign<$type> for Mint<MOD> {
                fn div_assign(&mut self, rhs: $type) {
                    *self /= Self::from(rhs);
                }
            }
        )*
    };
}

impl_ops_with_integer!(i32, i64, u32, u64);

impl<const MOD: u32> Display for Mint<MOD> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.val)
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    const MOD: u32 = 1000000007;
    #[test]
    fn test_inv() {
        let mut r = Random::new();
        for _ in 1..PRECALCULATE_LEN * 2 {
            let v: Mint<MOD> = r.num(1..MOD as i64).into();
            assert!(v * v.inv() == 1.into());
        }
    }
    #[test]
    #[should_panic]
    fn inv_not_zero() {
        let m = Mint::<MOD>::from(0);
        m.inv();
    }

    #[test]
    fn test_add_sub_mul_div() {
        let mut r = Random::new();
        for _ in 0..100 {
            let a: Mint<MOD> = r.num(1..MOD as i64).into();
            let b: Mint<MOD> = r.num(1..MOD as i64).into();
            assert!((a + b).val == (a.val + b.val) % MOD);
            let expected = (a.val as i64 - b.val as i64).rem_euclid(MOD as i64) as u32;
            assert!((a - b).val == expected);
            let mut x = a;
            x += b;
            assert!(x == a + b);
            x -= b;
            assert!(x == a);

            assert!((a * b).val == ((a.val as i64 * b.val as i64) % MOD as i64) as u32);
            let mut x = a;
            x /= b;
            assert!(x == a / b);
            assert!(((x.val as i64 * b.val as i64) % MOD as i64) as u32 == a.val);
            x *= b;
            assert!(x == a);
        }
        assert!(Mint::<MOD>::from(MOD as i64) == Mint::<MOD>::from(0));
    }

    #[test]
    fn test_ops_with_integers() {
        let mut m: Mint<MOD> = 3.into();
        m += 3;
        m -= 2_i64;
        m *= 3_u32;
        m /= 3_u64;
        assert_eq!(m, 4.into());

        assert_eq!(m + 2_i32, 6.into());
        assert_eq!(m - 5_i64, (-1).into());
        assert_eq!(m * 3_u32, 12.into());
        assert_eq!(m / 2_u64, 2.into());

        assert_eq!(Mint::<MOD>::from(-1_i32), (MOD as i64 - 1).into());
        assert_eq!(
            Mint::<MOD>::from(u64::MAX).val,
            (u64::MAX % u64::from(MOD)) as u32
        );
    }

    fn max_power(x: i64) -> u32 {
        assert!(x > 0);

        if x == 1 {
            return u32::MAX; // 1^y = 1 always fits
        }

        let mut y: u32 = 0;
        let mut cur: i128 = 1;

        while let Some(next) = cur.checked_mul(x.into()) {
            cur = next;
            y += 1;
        }

        y
    }

    #[test]
    fn test_power() {
        let mut r = Random::new();
        let x: i64 = r.num(2..1000);
        let m: Mint<MOD> = x.into();
        let exp: u32 = r.num(1..=max_power(x).min(64));
        assert!(m.pow(exp).val == ((x as i128).pow(exp) % MOD as i128) as u32);
    }

    #[test]
    fn compare() {
        assert!(Mint::<MOD>::from(3) == Mint::<MOD>::from(3));
        assert!(Mint::<MOD>::from(3) != Mint::<MOD>::from(2));
        assert!(Mint::<MOD>::from(2) < Mint::<MOD>::from(3));
        assert!(Mint::<MOD>::from(2) <= Mint::<MOD>::from(3));
        assert!(Mint::<MOD>::from(3) > Mint::<MOD>::from(2));
        assert!(Mint::<MOD>::from(3) >= Mint::<MOD>::from(2));
    }

    #[test]
    fn test_ncr() {
        assert!(Mint::<MOD>::from(5).ncr(Mint::<MOD>::from(3)) == 10.into());
        assert!(Mint::<MOD>::from(50).ncr(Mint::<MOD>::from(20)) == 211914057.into());
        assert!(Mint::<MOD>::from(50).ncr(Mint::<MOD>::from(60)) == 0.into());
    }

    #[test]
    fn test_npr() {
        assert!(Mint::<MOD>::from(7).npr(Mint::<MOD>::from(3)) == 210.into());
        assert!(Mint::<MOD>::from(10).npr(Mint::<MOD>::from(0)) == 1.into());
        assert!(Mint::<MOD>::from(5).npr(Mint::<MOD>::from(7)) == 0.into());
    }

    #[test]
    fn test_another_mod() {
        assert!(Mint::<5>::from(11).val == 1);
    }
}
