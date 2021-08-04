use crate::field::*;
use crate::univariate_polynomial::*;
use std::convert::{From, Into};
use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::string::ToString;

pub trait FiniteField: Field {
    // Since this is a finite field, we can enumerate all the elements.
    fn enumerate() -> Vec<Self>;

    const CARDINALITY: usize;

    // isomorphisms betwee byte array
    fn from_bytes(v: &[u8]) -> Self;
    fn to_byte(&self, idx: usize) -> u8;
}

pub trait HasPrimitiveElement: Copy {
    const PRIMITIVE_ELEMENT: Self;
}

/*
 * The section of GF(2)
 */

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(non_camel_case_types)]
pub struct GF_2 {
    // true = 1, false = 0
    value: bool,
}

/*
 *  + 0 1   * 0 1
 *  0 0 1   0 0 0
 *  1 1 0   1 0 1
 *
 *  + == bit XOR
 *  * == bit AND
 */
impl GF_2 {
    const ZERO: GF_2 = GF_2 { value: false };
    const ONE: GF_2 = GF_2 { value: true };

    pub fn mul_inv(&self) -> GF_2 {
        GF_2::ONE
    }

    pub fn to_u8(&self) -> u8 {
        if self.value {
            1
        } else {
            0
        }
    }
}

impl Add for GF_2 {
    type Output = GF_2;

    // XOR
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: GF_2) -> GF_2 {
        let value = self.value ^ rhs.value;
        GF_2 { value }
    }
}

impl Neg for GF_2 {
    type Output = GF_2;

    fn neg(self) -> GF_2 {
        self
    }
}

impl Sub for GF_2 {
    type Output = GF_2;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: GF_2) -> GF_2 {
        self + (-rhs)
    }
}

impl Mul for GF_2 {
    type Output = GF_2;

    // AND
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: GF_2) -> GF_2 {
        let value = self.value & rhs.value;
        GF_2 { value }
    }
}

impl Div for GF_2 {
    type Output = GF_2;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: GF_2) -> GF_2 {
        self * rhs.mul_inv()
    }
}

impl Field for GF_2 {
    const ZERO: GF_2 = GF_2::ZERO;
    const ONE: GF_2 = GF_2::ONE;

    fn mul_inv(&self) -> Self {
        self.mul_inv()
    }
}

impl FiniteField for GF_2 {
    fn enumerate() -> Vec<Self> {
        vec![GF_2::ZERO, GF_2::ONE]
    }

    const CARDINALITY: usize = 2;

    fn from_bytes(v: &[u8]) -> GF_2 {
        debug_assert!(v.len() == 1);

        if v[0] == 1 {
            GF_2::ONE
        } else if v[0] == 0 {
            GF_2::ZERO
        } else {
            panic!("invalid value");
        }
    }

    fn to_byte(&self, idx: usize) -> u8 {
        debug_assert!(idx == 0);

        if !self.value {
            0
        } else {
            1
        }
    }
}

/*
 * The section of GF(2^8)
 */

lazy_static! {
    // use x^8 + x^4 + x^3 + x^2 + 1.
    pub static ref GF_2_8_IMPL: GF_2_8_impl = GF_2_8_impl::new(
        Poly::from_vec( vec![
            (8, GF_2::ONE), (4, GF_2::ONE), (3, GF_2::ONE), (2, GF_2::ONE), (0, GF_2::ONE)
        ])
    );
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct GF_2_8(u8);

impl ToString for GF_2_8 {
    fn to_string(&self) -> String {
        format!("{:#04x}", self.0)
    }
}

impl GF_2_8 {
    pub fn to_poly(&self) -> Poly<GF_2> {
        let v = self.0;

        let mut m = Vec::new();

        for deg in 0..8 {
            if (v >> deg) & 1 == 0 {
                m.push((deg, GF_2::ZERO));
            } else {
                m.push((deg, GF_2::ONE));
            }
        }

        Poly::from_vec(m)
    }

    pub fn coef(&self, degree: usize) -> u8 {
        debug_assert!(degree < 8);

        (self.0 >> degree) & 1
    }
}

impl HasPrimitiveElement for GF_2_8 {
    const PRIMITIVE_ELEMENT: GF_2_8 = GF_2_8(0b10);
}

impl Field for GF_2_8 {
    // 0
    const ZERO: GF_2_8 = GF_2_8(0);

    // \alpha^0
    const ONE: GF_2_8 = GF_2_8(1);

    fn mul_inv(&self) -> GF_2_8 {
        GF_2_8_IMPL.mul_inv(*self)
    }
}

impl FiniteField for GF_2_8 {
    fn enumerate() -> Vec<Self> {
        (0u8..=0xff).map(|v| v.into()).collect()
    }

    const CARDINALITY: usize = 0x100;

    fn from_bytes(v: &[u8]) -> Self {
        debug_assert!(v.len() == 1);

        GF_2_8(v[0])
    }

    fn to_byte(&self, idx: usize) -> u8 {
        debug_assert!(idx == 0);

        self.0
    }
}

impl From<u8> for GF_2_8 {
    fn from(v: u8) -> Self {
        GF_2_8(v)
    }
}

impl From<GF_2_8> for u8 {
    fn from(v: GF_2_8) -> Self {
        v.0
    }
}

impl From<Poly<GF_2>> for GF_2_8 {
    fn from(p: Poly<GF_2>) -> Self {
        let mut v: u8 = 0;

        for (deg, coef) in p.iter() {
            if *coef == GF_2::ONE {
                v |= 1 << (*deg)
            }
        }

        v.into()
    }
}

impl Mul<GF_2_8> for GF_2_8 {
    type Output = GF_2_8;

    fn mul(self, rhs: GF_2_8) -> GF_2_8 {
        GF_2_8_IMPL.mul(self, rhs)
    }
}

impl Add<GF_2_8> for GF_2_8 {
    type Output = GF_2_8;

    fn add(self, rhs: GF_2_8) -> GF_2_8 {
        GF_2_8_IMPL.add(self, rhs)
    }
}

impl Neg for GF_2_8 {
    type Output = GF_2_8;

    fn neg(self) -> GF_2_8 {
        GF_2_8_IMPL.add_inv(self)
    }
}

impl Sub<GF_2_8> for GF_2_8 {
    type Output = GF_2_8;

    fn sub(self, rhs: GF_2_8) -> GF_2_8 {
        self + (-rhs)
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Div<GF_2_8> for GF_2_8 {
    type Output = GF_2_8;

    fn div(self, rhs: GF_2_8) -> GF_2_8 {
        self * GF_2_8_IMPL.mul_inv(rhs)
    }
}

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub struct GF_2_8_impl {
    // primitive polynomial
    ppoly: Poly<GF_2>,
    psi: Vec<GF_2_8>,
    phi: Vec<u8>,
}

impl GF_2_8_impl {
    const MAX_EXP: u8 = 0xff - 1;
    const MODULO: u8 = 0xff;
    const ORDER: u16 = 0x100;

    pub fn zero() -> GF_2_8 {
        0.into()
    }

    pub fn one() -> GF_2_8 {
        1.into()
    }

    // build multiplication table
    pub fn new(ppoly: Poly<GF_2>) -> GF_2_8_impl {
        debug_assert!(ppoly.degree() == Some(8));

        let mut psi: Vec<GF_2_8> = vec![0.into(); (GF_2_8_impl::MAX_EXP + 1) as usize];
        let mut phi: Vec<u8> = vec![0; GF_2_8_impl::ORDER as usize];
        let mut p = None;

        for i in 0u8..=GF_2_8_impl::MAX_EXP {
            if let Some(p_) = p {
                p = Some(p_ * Poly::<GF_2>::from_mono(1, GF_2::ONE));
            } else {
                p = Some(Poly::<GF_2>::from_mono(0, GF_2::ONE))
            }

            let reduced = &(p.unwrap()) % &ppoly;
            p = Some(reduced.clone());

            let rep: GF_2_8 = reduced.into();
            let bin_rep: u8 = rep.into();

            psi[i as usize] = rep;
            phi[bin_rep as usize] = i;
        }

        GF_2_8_impl { ppoly, psi, phi }
    }

    pub fn ppoly(&self) -> &Poly<GF_2> {
        &self.ppoly
    }

    pub fn mul(&self, p: GF_2_8, q: GF_2_8) -> GF_2_8 {
        let p = u8::from(p);
        let q = u8::from(q);

        if p == 0 || q == 0 {
            return 0.into();
        }

        let i: u16 = self.phi[p as usize].into();
        let j: u16 = self.phi[q as usize].into();

        let i_j = (i + j) % (GF_2_8_impl::MODULO as u16);
        self.psi[i_j as usize]
    }

    pub fn mul_inv(&self, p: GF_2_8) -> GF_2_8 {
        let p = u8::from(p);

        debug_assert!(p != 0);

        let i = self.phi[p as usize];
        let inv = (GF_2_8_impl::MODULO - i) % GF_2_8_impl::MODULO;
        self.psi[inv as usize]
    }

    pub fn add(&self, p: GF_2_8, q: GF_2_8) -> GF_2_8 {
        (u8::from(p) ^ u8::from(q)).into()
    }

    pub fn add_inv(&self, p: GF_2_8) -> GF_2_8 {
        p
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_inv() {
        check_add_inv::<GF_2>();
        check_add_inv::<GF_2_8>();
    }

    #[test]
    fn test_mul_inv() {
        check_mul_inv::<GF_2>();
        check_mul_inv::<GF_2_8>();
    }

    #[test]
    fn test_distributive_law() {
        check_distributive_law::<GF_2>();
        check_distributive_law::<GF_2_8>();
    }

    fn check_add_inv<F: FiniteField>() {
        for e in F::enumerate() {
            assert_eq!(e + (-e), F::ZERO);
        }
    }

    fn check_mul_inv<F: FiniteField>() {
        for e in F::enumerate() {
            if e != F::ZERO {
                let inv = (&e).mul_inv();
                assert_eq!(e * inv, F::ONE);
            }
        }
    }

    fn check_distributive_law<F: FiniteField>() {
        for e1 in F::enumerate() {
            for e2 in F::enumerate() {
                for e3 in F::enumerate() {
                    assert_eq!(e1 * (e2 + e3), e1 * e2 + e1 * e3);
                }
            }
        }
    }
}
