use crate::field::*;
use crate::fin_field::*;
use crate::matrix::*;
use std::convert::TryInto;

/*
 * vandermonde(size={height: m, width: n}, v=[a, b, c, ..., x]) |v| = m
 * is
 * (1 a a^2 a^3 a^4 ... a^n)
 * (1 b b^2 b^3 b^4 ... b^n)
 * (1 c c^2 c^3 c^4 ... c^n)
 *           ...
 * (1 x x^n x^3 x^4 ... x^n)
 *
 * Note: n == m is not needed
 */
pub fn vandermonde<F: Field>(size: MatrixSize, v: &[F]) -> Option<Matrix<F>> {
    let mut m = Matrix::new(size);

    if size.height != v.len() {
        return None;
    }

    for i in 0..v.len() {
        if v[i] == F::ONE {
            return None;
        }
        for j in (i + 1)..v.len() {
            if v[i] == v[j] {
                return None;
            }
        }
    }

    for i in 0..size.height {
        for j in 0..size.width {
            let e: u32 = j.try_into().unwrap();
            m[i][j] = v[i].exp(e);
        }
    }

    Some(m)
}

pub fn systematic_vandermonde<F: Field>(size: MatrixSize, v: &[F]) -> Option<Matrix<F>> {
    let m = vandermonde(size, v);

    if let Some(m) = m {
        let mut sub = m.clone();
        sub.drop_rows((size.width..size.height).collect());
        let inv = sub.inverse().unwrap();
        Some(&m * &inv)
    } else {
        None
    }
}

// systematic & the topmomst parity is 111...1
pub fn modified_systematic_vandermonde<F: Field>(size: MatrixSize, v: &[F]) -> Option<Matrix<F>> {
    let m = vandermonde(size, v);

    if let Some(m) = m {
        let mut sub = m.clone();
        sub.drop_rows((size.width..size.height).collect());
        let inv = sub.inverse().unwrap();
        let mut m = &m * &inv;

        for i in 0..size.width {
            let f = m[size.width][i];
            if f != F::ONE {
                for j in size.width..size.height {
                    m[j][i] = m[j][i] * F::mul_inv(&f);
                }
            }
        }

        for i in size.width + 1..size.height {
            let f = m[i][0];
            if f != F::ONE {
                for j in 0..size.width {
                    m[i][j] = m[i][j] * F::mul_inv(&f);
                }
            }
        }

        Some(m)
    } else {
        None
    }
}

pub fn rsv(data_fragments: usize, parity_fragments: usize) -> Matrix<GF_2_8> {
    let height = data_fragments + parity_fragments;

    let velems: Vec<GF_2_8> = (1..=height)
        .map(|i| GF_2_8::PRIMITIVE_ELEMENT.exp(i as u32))
        .collect();

    let m: Matrix<GF_2_8> = modified_systematic_vandermonde(
        MatrixSize {
            height,
            width: data_fragments,
        },
        &velems,
    )
    .unwrap();

    m
}

pub fn nonsystematic_rsv(data_fragments: usize, parity_fragments: usize) -> Matrix<GF_2_8> {
    let height = data_fragments + parity_fragments;

    let velems: Vec<GF_2_8> = (1..=height)
        .map(|i| GF_2_8::PRIMITIVE_ELEMENT.exp(i as u32))
        .collect();

    let m: Matrix<GF_2_8> = vandermonde(
        MatrixSize {
            height,
            width: data_fragments,
        },
        &velems,
    )
    .unwrap();

    m
}

pub fn isa_rsv(data: usize, parity: usize) -> Matrix<GF_2_8> {
    let m = data + parity;
    let k = data;

    let mut a = Matrix::new(MatrixSize {
        height: m,
        width: k,
    });

    let mut gen = GF_2_8::ONE;

    for i in 0..k {
        a[i][i] = GF_2_8::ONE;
    }

    for i in k..m {
        let mut p = GF_2_8::ONE;
        for j in 0..k {
            a[i][j] = p;
            p = p * gen;
        }
        gen = gen * GF_2_8::from(2);
    }

    a
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;
    use rand::prelude::*;

    #[test]
    fn test_rsv1() {
        let m = rsv(10, 4);

        let remove_pattern = (0..14).into_iter().combinations(4);

        for remove in remove_pattern.into_iter() {
            let mut tmp = m.clone();

            tmp.drop_rows(remove);

            let mut inv = tmp.clone();
            let inv = inv.inverse().unwrap();

            assert!(Matrix::identity(10) == &tmp * &inv);
        }
    }

    #[test]
    fn test_isa_rsv1() {
        let m = isa_rsv(10, 4);

        let remove_pattern = (0..14).into_iter().combinations(4);

        for remove in remove_pattern.into_iter() {
            let mut tmp = m.clone();

            tmp.drop_rows(remove);

            let mut inv = tmp.clone();
            let inv = inv.inverse().unwrap();

            assert!(Matrix::identity(10) == &tmp * &inv);
        }
    }

    #[test]
    fn test_isa_rsv2() {
        let mut m = isa_rsv(10, 4);

        m.drop_rows(vec![2, 4, 5, 9]); // (10, 10)

        let mut a = Matrix::new(MatrixSize {
            height: 10,
            width: 20,
        });
        for i in 0..a.height() {
            for j in 0..a.width() {
                a[i][j] = GF_2_8::from(rand::random::<u8>());
            }
        }

        let mut result = &m * &a; // (10, 20)
        result.drop_rows(vec![9]); // (9, 20)

        let mut inv = m.clone(); // (10, 10);
        let mut inv = inv.inverse().unwrap(); // (9, 9)
        {
            inv.drop_rows(vec![9]); // (9, 10);
            println!("{}", inv.dump());
            inv.drop_col(9);
            println!("{}", inv.dump());
        }

        let b = &inv * &result; // (9, 20)

        println!("{}", a.dump());

        println!("{}", b.dump());
    }

    #[test]
    fn test_inverse_vandermonde() {
        let r = GF_2_8::PRIMITIVE_ELEMENT;

        let v1 = vandermonde(
            MatrixSize {
                height: 4,
                width: 4,
            },
            &vec![r.exp(1), r.exp(2), r.exp(3), r.exp(4)],
        )
        .unwrap();

        let v1_inv = v1.clone().inverse().unwrap();
        assert_eq!(&v1 * &v1_inv, Matrix::identity(4));

        let v2 = vandermonde(
            MatrixSize {
                height: 4,
                width: 4,
            },
            &vec![r.exp(2), r.exp(2), r.exp(3), r.exp(4)],
        );

        assert!(v2.is_none());

        let v3 = vandermonde(
            MatrixSize {
                height: 5,
                width: 4,
            },
            &vec![r.exp(1), r.exp(2), r.exp(3), r.exp(4), r.exp(5)],
        )
        .unwrap();

        for i in 0..5 {
            let mut v = v3.clone();
            v.drop_rows(vec![i]);

            let v_inv = v.clone().inverse().unwrap();
            assert_eq!(&v * &v_inv, Matrix::identity(4));
        }
    }

    #[test]
    fn systematic_vandermonde_test() {
        let r = GF_2_8::PRIMITIVE_ELEMENT;

        let mut sv = systematic_vandermonde(
            MatrixSize {
                height: 5,
                width: 4,
            },
            &vec![r.exp(1), r.exp(2), r.exp(3), r.exp(4), r.exp(5)],
        )
        .unwrap();

        sv.drop_rows(vec![4]);

        assert_eq!(sv, Matrix::identity(4));
    }

    #[test]
    fn modified_systematic_vandermonde_test() {
        let r = GF_2_8::PRIMITIVE_ELEMENT;

        let mut sv = modified_systematic_vandermonde(
            MatrixSize {
                height: 5,
                width: 4,
            },
            &vec![r.exp(1), r.exp(2), r.exp(3), r.exp(4), r.exp(5)],
        )
        .unwrap();

        sv.drop_rows(vec![4]);

        assert_eq!(sv, Matrix::identity(4));
    }
}
