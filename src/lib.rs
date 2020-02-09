/*
 *  Copyright (c) 2020 Thijs Raymakers
 *
 *  Permission is hereby granted, free of charge, to any person obtaining a copy
 *  of this software and associated documentation files (the "Software"), to deal
 *  in the Software without restriction, including without limitation the rights
 *  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 *  copies of the Software, and to permit persons to whom the Software is
 *  furnished to do so, subject to the following conditions:
 *
 *  The above copyright notice and this permission notice shall be included in all
 *  copies or substantial portions of the Software.
 *
 *  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 *  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 *  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 *  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 *  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 *  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 *  SOFTWARE.
 */
#![no_std]
#![deny(missing_docs)]

//! This crate implements
//! [finite field arithmetic](https://en.wikipedia.org/wiki/Finite_field_arithmetic)
//! on finite fields with 2<sup>8</sup> elements, often denoted as GF(2<sup>8</sup>),
//! in an [isochronous](https://en.wikipedia.org/wiki/Isochronous) manner. This means that it will always
//! run in the same amount of time, no matter the input.
//!
//! The implementation isochronous, because it:
//! * is branch free
//! * runs in constant time
//! * doesn't do table lookups
//!
//! This crate uses the irreducible polynomial
//! <i>x</i><sup>8</sup> + <i>x</i><sup>4</sup> + <i>x</i><sup>3</sup> + <i>x</i> + 1
//! for multiplication, as
//! standardized for the AES algorithm in
//! [FIPS 197](https://csrc.nist.gov/csrc/media/publications/fips/197/final/documents/fips-197.pdf).
//!
//! # Example
//! ```
//! # use isochronous_finite_fields::GF;
//! // Add two elements of the Galois field GF(2^8) together.
//! assert_eq!(GF(5) + GF(12), GF(9));
//!
//! // Subtract two elements of the Galois field GF(2^8).
//! assert_eq!(GF(32) - GF(219), GF(251));
//!
//! // Multiply two elements of the Galois field GF(2^8) together.
//! assert_eq!(GF(175) * GF(47),  GF(83));
//!
//! // Calculate the multiplicative inverse of GF(110) in the Galois field GF(2^8).
//! assert_eq!(GF(110).multiplicative_inverse(), GF(33));
//! assert_eq!(GF(110) * GF(33), GF(1));
//! ```

use core::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

/// Galois field wrapper struct.
///
/// It is wrapped around an `u8` type, to guarantee at compile time that
/// all elements are in the finite field GF(2<sup>8</sup>).
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Hash)]
pub struct GF(pub u8);

impl GF {
    /// Calculates the multiplicative inverse. The multiplicative inverse is the element in the
    /// Galois field that results in a product of 1.
    ///
    /// # Example
    /// ```
    /// # use isochronous_finite_fields::GF;
    /// let element = GF(148);
    /// let inverse = element.multiplicative_inverse();
    ///
    /// assert_eq!(element * inverse, GF(1));
    /// ```
    pub fn multiplicative_inverse(self) -> Self {
        let mut p = 0;

        for x in 0u8..=255u8 {
            // If zero, the multiplication is results in GF(1)
            // If non-zero, the multiplication ends with something different.
            let y = (self * GF(x)).0 ^ 1;

            // OR all bits together in the rightmost bit. If y is zero, that means that the
            // result of ORing all bits together will also be zero. Otherwise, it will be 1.
            let or = y | y >> 1 | y >> 2 | y >> 3 | y >> 4 | y >> 5 | y >> 6 | y >> 7;

            // Extend the bits to the full byte and negate it. This means that the AND will
            // be zero if the multiplication in y was 1.
            p ^= !extend_bit(or) & x;
        }

        GF(p)
    }
}

#[inline(always)]
/// Extend the right most bit to all the other bits in the byte.
fn extend_bit(input: u8) -> u8 {
    (((input) as i8) << 7).wrapping_shr(7) as u8
}

impl From<u8> for GF {
    fn from(x: u8) -> Self {
        GF(x)
    }
}

/// Adding two elements in the Galois field GF(2<sup>8</sup>) is equal to doing an exclusive
/// or (XOR) between the two elements.
/// It is also equal to subtracting two elements.
impl Add for GF {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        Self(self.0 ^ rhs.0)
    }
}

impl AddAssign for GF {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        *self = self.add(rhs)
    }
}

/// Subtracting two elements in the Galois field GF(2<sup>8</sup>) is equal to doing an exclusive
/// or (XOR) between the two elements.
/// It is also equal to adding two elements.
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

/// Multiplication in this finite field is multiplication modulo AES standardized irreducible
/// polynomial
/// <i>x</i><sup>8</sup> + <i>x</i><sup>4</sup> + <i>x</i><sup>3</sup> + <i>x</i> + 1
/// (or `0b1_0001_1011`).
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
            p ^= extend_bit(b & 1) & a;

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
            a ^= extend_bit(carry & 1) & 0x1b;
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
    fn test_multiplicative_inverse() {
        assert_eq!(GF(0x01).multiplicative_inverse(), GF(0x01));
        assert_eq!(GF(0x02).multiplicative_inverse(), GF(0x8d));
        assert_eq!(GF(0x03).multiplicative_inverse(), GF(0xf6));
        assert_eq!(GF(0x04).multiplicative_inverse(), GF(0xcb));
        assert_eq!(GF(0x05).multiplicative_inverse(), GF(0x52));
        assert_eq!(GF(0x06).multiplicative_inverse(), GF(0x7b));
        assert_eq!(GF(0xff).multiplicative_inverse(), GF(0x1c));
    }

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
    fn test_extend_bit() {
        assert_eq!(extend_bit(1), 0xff);
        assert_eq!(extend_bit(0), 0x00);
        assert_eq!(extend_bit(0b0000_0001), 0xff);
        assert_eq!(extend_bit(0b0000_0000), 0x00);
        assert_eq!(extend_bit(0b1000_0100), 0x00);
        assert_eq!(extend_bit(0b0100_0100), 0x00);
        assert_eq!(extend_bit(0b1100_0101), 0xff);
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
