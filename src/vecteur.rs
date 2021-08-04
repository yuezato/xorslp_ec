use crate::field::*;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

/*
 * Vector space over a field K.
 */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vecteur<F: Field> {
    inner: Vec<F>,
}

impl<F: Field> Index<usize> for Vecteur<F> {
    type Output = F;

    fn index(&self, idx: usize) -> &F {
        &self.inner[idx]
    }
}

impl<F: Field> IndexMut<usize> for Vecteur<F> {
    fn index_mut(&mut self, idx: usize) -> &mut F {
        &mut self.inner[idx]
    }
}

#[allow(clippy::len_without_is_empty)]
impl<F: Field> Vecteur<F> {
    pub fn remove(&mut self, idx: usize) {
        self.inner.remove(idx);
    }

    /// Make the zero vector with the size `len`.
    pub fn new(len: usize) -> Vecteur<F> {
        Self {
            inner: vec![F::ZERO; len],
        }
    }

    ///
    /// # Safety
    ///
    /// get the reference of the element without index checking
    pub unsafe fn get_unchecked(&self, idx: usize) -> &F {
        self.inner.get_unchecked(idx)
    }

    ///
    /// # Safety
    ///
    /// get the mutable reference of the element without index checking
    pub unsafe fn get_unchecked_mut(&mut self, idx: usize) -> &mut F {
        self.inner.get_unchecked_mut(idx)
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &F> {
        self.inner.iter()
    }

    pub fn from_vec(v: Vec<F>) -> Vecteur<F> {
        Vecteur { inner: v }
    }

    pub fn as_vec(&self) -> &Vec<F> {
        &self.inner
    }
}

/*
 (a1 a2 ... aN) + (b1 b2 ... bN) = (a1+b1 a2+b2 ... aN+bN)
*/
impl<F: Field> Add for &Vecteur<F> {
    type Output = Vecteur<F>;

    fn add(self, rhs: &Vecteur<F>) -> Vecteur<F> {
        assert!(self.len() == rhs.len());

        let elems = self.len();
        let mut v = vec![F::ZERO; elems];
        for (i, item) in v.iter_mut().enumerate() {
            *item = self[i] + rhs[i];
        }

        Vecteur { inner: v }
    }
}
impl<F: Field> Add for Vecteur<F> {
    type Output = Vecteur<F>;

    fn add(self, rhs: Vecteur<F>) -> Vecteur<F> {
        &self + &rhs
    }
}

/*
 v - w = v + (-1 w)
*/
impl<F: Field> Sub for &Vecteur<F> {
    type Output = Vecteur<F>;

    fn sub(self, rhs: &Vecteur<F>) -> Vecteur<F> {
        self + &(rhs * -F::ONE)
    }
}
impl<F: Field> Sub for Vecteur<F> {
    type Output = Vecteur<F>;

    fn sub(self, rhs: Vecteur<F>) -> Vecteur<F> {
        self + rhs * (-F::ONE)
    }
}

/*
(a1 a2 ... aN) * k = (k*a1 k*a2 ... k*aN)
*/
impl<F: Field> Mul<F> for &Vecteur<F> {
    type Output = Vecteur<F>;

    fn mul(self, rhs: F) -> Vecteur<F> {
        let v: Vec<F> = self.iter().map(|e| rhs * *e).collect();
        Vecteur { inner: v }
    }
}
impl<F: Field> Mul<F> for Vecteur<F> {
    type Output = Vecteur<F>;

    fn mul(self, rhs: F) -> Vecteur<F> {
        &self * rhs
    }
}

/*
 * Dot product.
 * (a1 a2 ... aN) * (b1 b2 ... bN) = (a1 b1) + (a2 b2) + ... + (aN bN)
 */
impl<F: Field> Mul for &Vecteur<F> {
    type Output = F;

    fn mul(self, rhs: &Vecteur<F>) -> F {
        assert!(self.len() == rhs.len());
        let mut v = F::ZERO;
        for i in 0..self.len() {
            v = v + (self[i] * rhs[i]);
        }
        v
    }
}
impl<F: Field> Mul for Vecteur<F> {
    type Output = F;

    fn mul(self, rhs: Vecteur<F>) -> F {
        &self * &rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::fin_field::*;

    type GV = Vecteur<GF_2>;

    #[test]
    fn add_test() {
        let z = GV::new(3);
        let v1 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ONE]);
        let v2 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ZERO]);
        let v3 = GV::from_vec(vec![GF_2::ONE, GF_2::ONE, GF_2::ONE]);

        let targets = vec![
            (&z, &v1, &v1),
            (&v1, &z, &v1),
            (&z, &v2, &v2),
            (&v2, &z, &v2),
            (&z, &v3, &v3),
            (&v3, &z, &v3),
            (&v1, &v1, &z),
            (&v2, &v2, &z),
            (&v3, &v3, &z),
        ];

        for (a, b, c) in targets {
            assert_eq!(a + b, *c);
        }
    }

    #[test]
    fn sub_test() {
        let z = GV::new(3);
        let v1 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ONE]);
        let v2 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ZERO]);
        let v3 = GV::from_vec(vec![GF_2::ONE, GF_2::ONE, GF_2::ONE]);

        let targets = vec![
            (&z, &v1, &v1),
            (&v1, &z, &v1),
            (&z, &v2, &v2),
            (&v2, &z, &v2),
            (&z, &v3, &v3),
            (&v3, &z, &v3),
            (&v1, &v1, &z),
            (&v2, &v2, &z),
            (&v3, &v3, &z),
        ];

        for (a, b, c) in targets {
            assert_eq!(a - b, *c);
        }
    }

    #[test]
    fn mul_with_field_test() {
        let z = GV::new(3);
        let v1 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ONE]);
        let v2 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ZERO]);
        let v3 = GV::from_vec(vec![GF_2::ONE, GF_2::ONE, GF_2::ONE]);

        let targets = vec![&z, &v1, &v2, &v3];

        for e in targets {
            assert_eq!(e * GF_2::ONE, *e);
            assert_eq!(e * GF_2::ZERO, z);
        }
    }

    #[test]
    fn mul_test() {
        let z = GV::new(3);
        let v1 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ONE]);
        let v2 = GV::from_vec(vec![GF_2::ONE, GF_2::ZERO, GF_2::ZERO]);
        let v3 = GV::from_vec(vec![GF_2::ONE, GF_2::ONE, GF_2::ONE]);

        let targets = vec![
            (&z, &v1, GF_2::ZERO),
            (&v1, &z, GF_2::ZERO),
            (&z, &v2, GF_2::ZERO),
            (&v2, &z, GF_2::ZERO),
            (&z, &v3, GF_2::ZERO),
            (&v3, &z, GF_2::ZERO),
            (&v1, &v2, GF_2::ONE),
            (&v2, &v1, GF_2::ONE),
            (&v1, &v3, GF_2::ZERO),
            (&v3, &v1, GF_2::ZERO),
            (&v2, &v3, GF_2::ONE),
            (&v3, &v2, GF_2::ONE),
            (&v1, &v1, GF_2::ZERO),
            (&v2, &v2, GF_2::ONE),
            (&v3, &v3, GF_2::ONE),
        ];

        for (a, b, c) in targets {
            assert_eq!(a * b, c, "{:?}", (a, b));
        }
    }
}
