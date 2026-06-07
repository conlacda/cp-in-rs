// ANCHOR: main
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::sync::OnceLock;

const PRECALCULATE_LEN: usize = 1_000_005;
struct Precalc {
    inv_range: Vec<u32>,
    fact: Vec<u32>,
    finv: Vec<u32>,
}
static PRECALC: OnceLock<Precalc> = OnceLock::new();

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Mint<const MOD: u32 = 1000000007> {
    pub val: u32,
}

impl<const MOD: u32> Mint<MOD> {
    fn build_precalc() -> Precalc {
        let mut inv_range = vec![1u32; PRECALCULATE_LEN];
        for i in 2..PRECALCULATE_LEN {
            inv_range[i] = (MOD
                - (((MOD / i as u32) as u64) * (inv_range[(MOD % i as u32) as usize] as u64)
                    % MOD as u64) as u32)
                % MOD;
        }

        let mut fact = vec![1u32; PRECALCULATE_LEN];
        for i in 1..PRECALCULATE_LEN {
            fact[i] = ((fact[i - 1] as u64 * i as u64) % MOD as u64) as u32;
        }

        let mut finv = vec![1u32; PRECALCULATE_LEN];
        for i in 1..PRECALCULATE_LEN {
            finv[i] = ((finv[i - 1] as u64 * inv_range[i] as u64) % MOD as u64) as u32;
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

    #[inline]
    fn normalize(v: i64) -> u32 {
        let mut val: i64 = v % MOD as i64;
        if val < 0 {
            val += MOD as i64;
        }
        val.try_into().unwrap()
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
            val = (val as u64 * (MOD - z) as u64 % MOD as u64) as u32;
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
impl<const MOD: u32> From<i64> for Mint<MOD> {
    fn from(v: i64) -> Self {
        let val = Mint::<MOD>::normalize(v);
        Self { val }
    }
}
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
        Self::from(self.val as i64 * rhs.val as i64)
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
        self * rhs.inv()
    }
}

impl<const MOD: u32> DivAssign for Mint<MOD> {
    fn div_assign(&mut self, rhs: Self) {
        self.val = (*self / rhs).val;
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
            let expected =
                ((((a.val as i64 - b.val as i64) % MOD as i64) + MOD as i64) % MOD as i64) as u32;
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
