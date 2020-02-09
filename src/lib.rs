use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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
        #[allow(clippy::suspicious_arithmetic_impl)]
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
        let mut a = self.0;
        let mut b = rhs.0;

        let mut p = 0;

        // Implementation details from https://en.wikipedia.org/wiki/Finite_field_arithmetic
        // Run the following loop eight times (once per bit).
        for _ in 0..8 {
            // If the rightmost bit of b is set, exclusive OR the product p by the value of a.
            // This is polynomial addition.
            p ^= ((((b & 1) as i8) << 7).wrapping_shr(7) as u8) & a;

            // Shift b one bit to the right, discarding the rightmost bit, and making the leftmost
            // bit have a value of zero. This divides the polynomial by x, discarding the x0 term.
            b >>= 1;

            // Keep track of whether the leftmost bit of a is set to one and call this value carry.
            let carry = (a >> 7) & 1;

            // Shift a one bit to the left, discarding the leftmost bit, and making the new
            // rightmost bit zero. This multiplies the polynomial by x, but we still need to take
            // account of carry which represented the coefficient of x7.
            a <<= 1;

            // If carry had a value of one, exclusive or a with the hexadecimal
            // number 0x1b (00011011 in binary). 0x1b corresponds to the irreducible polynomial with
            // the high term eliminated. Conceptually, the high term of the irreducible polynomial
            // and carry add modulo 2 to 0.
            a ^= ((((carry & 1) as i8) << 7).wrapping_shr(7) as u8) & 0x1b;
        }

        // p now has the product
        GF(p)
    }
}

impl MulAssign for GF {
    fn mul_assign(&mut self, rhs: Self) {
        *self = self.mul(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_behaviour() {
        let mut x: i8 = 1;
        x <<= 7;
        assert_eq!(x as u8, 0b1000_0000 as u8);
        x = x.wrapping_shr(7);
        assert_eq!(x as u8, 0b1111_1111);

        let mut x: i8 = 0;
        x <<= 7;
        assert_eq!(x as u8, 0b0000_0000 as u8);
        x = x.wrapping_shr(7);
        assert_eq!(x as u8, 0b0000_0000 as u8);
    }

    #[test]
    fn multiplication_example_wikipedia() {
        let mut x = GF(0x53);
        x *= GF(0xCA);
        assert_eq!(x, GF(0x01))
    }

    #[test]
    fn multiplication_example_fips197_4_2() {
        let mut x = GF(0x57);
        x *= GF(0x83);
        assert_eq!(x, GF(0xc1))
    }

    #[test]
    fn multiplication_example_fips197_4_2_1() {
        assert_eq!(GF(0x57) * GF(0x01), GF(0x57));
        assert_eq!(GF(0x57) * GF(0x02), GF(0xae));
        assert_eq!(GF(0x57) * GF(0x04), GF(0x47));
        assert_eq!(GF(0x57) * GF(0x08), GF(0x8e));
        assert_eq!(GF(0x57) * GF(0x10), GF(0x07));

        let mut x = GF(0x57);
        x *= GF(0x13);
        assert_eq!(x, GF(0xfe))
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
