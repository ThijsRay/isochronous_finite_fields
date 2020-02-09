use std::ops::{Add, AddAssign, BitXor, Sub, SubAssign, Mul, MulAssign};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
// The reason to use a fixed u8 type instead of generics is to guarantee at compile time that all
// elements fit in the finite field GF(2^8).
pub struct GF(pub u8);

impl GF {
    pub fn multiplicative_inverse(self) -> Self {
        unimplemented!()
    }
}

impl Add for GF {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl AddAssign for GF {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

impl Sub for GF {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}

impl SubAssign for GF {
    fn sub_assign(&mut self, rhs: Self) {
        self.add_assign(rhs)
    }
}

// Uses the AES standardized irreducible polynomial x^8 + x^4 + x^3 + x + 1 or 0b1_0001_1011 as
// the modulus of the multiplication operation.
impl Mul for GF {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl MulAssign for GF {
    fn mul_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_addition() {
        assert_eq!(GF(0x53) + GF(0xCA), GF(0x99));
    }

    #[test]
    fn simple_subtraction() {
        assert_eq!(GF(0x53) - GF(0xCA), GF(0x99));
    }

    #[test]
    fn zero_addition() {
        assert_eq!(GF(0x53) + GF(0x0), GF(0x53));
    }

    #[test]
    fn zero_subtraction() {
        assert_eq!(GF(0x53) - GF(0x0), GF(0x53));
    }

    #[test]
    fn simple_addition_assign() {
        let mut x = GF(0x22);
        x += GF(0x81);
        assert_eq!(x, GF(0xa3))
    }

    #[test]
    fn simple_subtraction_assign() {
        let mut x = GF(0x93);
        x -= GF(0x5b);
        assert_eq!(x, GF(0xc8))
    }

}
