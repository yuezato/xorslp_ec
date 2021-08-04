use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Field:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Neg<Output = Self>
    + Div<Output = Self>
    + Copy
    + Clone
    + Debug
    + Eq
    + PartialEq
    + Sized
{
    /*
     * the element 0 s.t.
     * 0 + x = x + 0 = 0,
     * 0 * x = x * 0 = 0.
     */
    const ZERO: Self;

    /*
     * the element 1 s.t.
     * 1 * x = x * 1 = x.
     */
    const ONE: Self;

    // x.mul_inv() denotes the *-inverse element of x
    fn mul_inv(&self) -> Self;

    // x.exp(N) denotes x^N
    fn exp(&self, exponent: u32) -> Self {
        if exponent == 0 {
            return Self::ONE;
        }

        let v = self.exp(exponent / 2);
        if exponent % 2 == 0 {
            // x^{2n} = x^n * x^n
            v * v
        } else {
            // x^{2n+1} = x^n * x^n * x
            *self * v * v
        }
    }
}
