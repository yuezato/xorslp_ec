use crate::bitmatrix::*;
use crate::field::*;
use crate::fin_field::*;
use crate::matrix::*;
use crate::univariate_polynomial::*;

#[allow(non_camel_case_types)]
pub struct Bit_GF_2_8_impl {
    ppoly: Poly<GF_2>,
    table: Vec<[u8; 8]>,
    is_ready: bool,
}

lazy_static! {
    // use x^8 + x^4 + x^3 + x^2 + 1.
    pub static ref BIT_GF_2_8_IMPL: Bit_GF_2_8_impl = Bit_GF_2_8_impl::build(
        Poly::from_vec( vec![
            (8, GF_2::ONE), (4, GF_2::ONE), (3, GF_2::ONE), (2, GF_2::ONE), (0, GF_2::ONE)
        ])
    );
}

impl Bit_GF_2_8_impl {
    pub fn ppoly(&self) -> Poly<GF_2> {
        self.ppoly.clone()
    }

    pub fn new(ppoly: Poly<GF_2>) -> Self {
        Self {
            ppoly,
            table: Vec::new(),
            is_ready: false,
        }
    }

    pub fn to_nested_array(&self, u: u8) -> [u8; 8] {
        debug_assert!(self.is_ready);

        self.table[u as usize]
    }

    pub fn to_bitmatrix(&self, u: u8) -> BitMatrix {
        debug_assert!(self.is_ready);

        let v: [u8; 8] = self.table[u as usize];

        let mut inner: Vec<Vec<bool>> = Vec::new();

        for u in &v {
            inner.push(u8_to_bitvec(*u));
        }

        BitMatrix::from_nested_vecs(inner)
    }

    pub fn setup(&mut self) {
        for i in 0u8..=255u8 {
            let t = self.conv(i.into());
            self.table.push(t);
        }
        self.is_ready = true;
    }

    pub fn build(ppoly: Poly<GF_2>) -> Self {
        let mut imp = Self::new(ppoly);
        imp.setup();
        imp
    }

    pub fn mul(&self, p: GF_2_8, q: GF_2_8) -> GF_2_8 {
        debug_assert!(self.is_ready);

        let bm = self.to_bitmatrix(u8::from(p));
        let col_vec = u8_to_colvec(u8::from(q));

        let r = bm.mul(&col_vec);
        GF_2_8::from(colvec_to_u8(&r))
    }

    fn conv(&self, p: GF_2_8) -> [u8; 8] {
        let mut r: [u8; 8] = [7, 6, 5, 4, 3, 2, 1, 0];

        for i in 0..8 {
            let i: u32 = i as u32;
            let mut v: u8 = 0;
            for deg in 0..8 {
                let p_ = (p.to_poly() * Poly::from_mono(deg, GF_2::ONE)) % self.ppoly.clone();
                v |= p_.at(&i).to_u8() << deg;
            }
            r[(7 - i) as usize] = v;
        }

        r
    }
}

fn expand(v: &[[u8; 8]]) -> Vec<Vec<u8>> {
    let mut w = Vec::new();

    for j in 0..8 {
        let mut tmp = Vec::new();
        for ve in v {
            tmp.push(ve[j]);
        }
        w.push(tmp);
    }

    w
}

fn u8_to_bitvec(u: u8) -> Vec<bool> {
    let mut v = vec![false; 8];

    for (i, v) in v.iter_mut().enumerate() {
        if (u >> (7 - i)) & 1 == 1 {
            *v = true;
        }
    }

    v
}

fn u8vec_to_bitvec(v: &[u8]) -> Vec<bool> {
    let mut bitv = Vec::new();

    for u in v {
        bitv.append(&mut u8_to_bitvec(*u));
    }

    bitv
}

pub fn matrix_to_bitmatrix(m: &Matrix<GF_2_8>) -> BitMatrix {
    let mut inner: Vec<Vec<bool>> = Vec::new();

    for i in 0..m.height() {
        let row: Vec<u8> = m[i].as_vec().iter().map(|e| u8::from(*e)).collect();
        let v: Vec<[u8; 8]> = row
            .iter()
            .map(|e| BIT_GF_2_8_IMPL.to_nested_array(*e))
            .collect();

        let v: Vec<Vec<u8>> = expand(&v);
        debug_assert!(v.len() == 8);

        let mut bitvec: Vec<Vec<bool>> = v.iter().map(|u| u8vec_to_bitvec(u)).collect();
        inner.append(&mut bitvec);
    }

    BitMatrix::from_nested_vecs(inner)
}

pub fn rsv_bitmatrix(data_fragment: usize, parity_fragment: usize) -> BitMatrix {
    let rsvm = crate::vandermonde::rsv(data_fragment, parity_fragment);
    matrix_to_bitmatrix(&rsvm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vandermonde::*;
    use itertools::Itertools;

    #[test]
    fn test_m2bm1() {
        let rsvm = rsv(4, 4);
        matrix_to_bitmatrix(&rsvm);
    }

    #[test]
    fn test_bitmatrix_rsv1() {
        let m = rsv(10, 4);

        let remove_pattern = (0..14).into_iter().combinations(4);

        for remove in remove_pattern.into_iter() {
            let mut tmp = m.clone();

            tmp.drop_rows(remove);

            let mut inv = tmp.clone();
            let inv = inv.inverse().unwrap();

            let bitmatrix_tmp = matrix_to_bitmatrix(&tmp);
            let bitmatrix_inv = matrix_to_bitmatrix(&inv);
            let bitmatrix = bitmatrix_tmp.mul(&bitmatrix_inv);

            let identity = matrix_to_bitmatrix(&Matrix::identity(10));

            assert!(identity == bitmatrix);
        }
    }

    fn ppoly() -> Poly<GF_2> {
        Poly::from_vec(vec![
            (8, GF_2::ONE),
            (4, GF_2::ONE),
            (3, GF_2::ONE),
            (2, GF_2::ONE),
            (0, GF_2::ONE),
        ])
    }

    #[test]
    fn test_expand() {
        let v = vec![[0, 1, 2, 3, 4, 5, 6, 7], [10, 11, 12, 13, 14, 15, 16, 17]];

        let w = vec![
            vec![0, 10],
            vec![1, 11],
            vec![2, 12],
            vec![3, 13],
            vec![4, 14],
            vec![5, 15],
            vec![6, 16],
            vec![7, 17],
        ];

        assert!(expand(&v) == w);
    }

    #[test]
    fn conv_test() {
        let i2 = Bit_GF_2_8_impl::build(ppoly());

        for a in GF_2_8::enumerate() {
            let a_bitmatrix = i2.to_bitmatrix(a.into());
            for b in GF_2_8::enumerate() {
                let b_colmatrix = u8_to_colvec(b.into());
                let c = a_bitmatrix.mul(&b_colmatrix);
                let c = colvec_to_u8(&c).into();

                assert!(a * b == c);
            }
        }
    }

    #[test]
    // to_bitmatrix(a * b) = to_bitmatrix(a) * to_bitmatrix(b)
    fn hom_test() {
        let i1 = GF_2_8_impl::new(ppoly());
        let i2 = Bit_GF_2_8_impl::build(ppoly());

        for a in GF_2_8::enumerate() {
            let a_bitmatrix = i2.to_bitmatrix(a.into());
            for b in GF_2_8::enumerate() {
                let c: GF_2_8 = i1.mul(a, b);

                let b_bitmatrix = i2.to_bitmatrix(b.into());

                let ab_bitmatrix = a_bitmatrix.mul(&b_bitmatrix);
                let c_bitmatrix = i2.to_bitmatrix(c.into());

                assert!(ab_bitmatrix == c_bitmatrix);
            }
        }
    }

    #[test]
    fn mul_test() {
        let i1 = GF_2_8_impl::new(ppoly());
        let i2 = Bit_GF_2_8_impl::build(ppoly());

        for i in 0u8..=255u8 {
            for j in 0u8..=255u8 {
                let r1 = i1.mul(i.into(), j.into());
                let r2 = i2.mul(i.into(), j.into());

                assert!(r1 == r2, "[1] {} * {}", i, j);
            }
        }
    }
}
