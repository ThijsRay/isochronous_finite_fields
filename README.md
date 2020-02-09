# Isochronous finite fields

This crate implements
[finite field arithmetic](https://en.wikipedia.org/wiki/Finite_field_arithmetic)
on finite fields with 2<sup>8</sup> elements, often denoted as GF(2<sup>8</sup>),
in an [isochronous](https://en.wikipedia.org/wiki/Isochronous) manner. This means that it will always
run in the same amount of time, no matter the input.

The implementation isochronous, because it:
* is branch free
* runs in constant time
* doesn't do table lookups

This crate uses the irreducible polynomial
<i>x</i><sup>8</sup> + <i>x</i><sup>4</sup> + <i>x</i><sup>3</sup> + <i>x</i> + 1
for multiplication, as
standardized for the AES algorithm in
[FIPS 197](https://csrc.nist.gov/csrc/media/publications/fips/197/final/documents/fips-197.pdf).

# Example
```rust
// Add two elements of the Galois field GF(2^8) together.
assert_eq!(GF(5) + GF(12), GF(9));

// Subtract two elements of the Galois field GF(2^8).
assert_eq!(GF(32) - GF(219), GF(251));

// Multiply two elements of the Galois field GF(2^8) together.
assert_eq!(GF(175) * GF(47),  GF(83));

// Calculate the multiplicative inverse of GF(110) in the Galois field GF(2^8).
assert_eq!(GF(110).multiplicative_inverse(), GF(33));
assert_eq!(GF(110) * GF(33), GF(1));
```

# License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.