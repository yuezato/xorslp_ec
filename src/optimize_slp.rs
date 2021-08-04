use crate::bitmatrix::{popcount, BitMatrix};
use crate::slp::*;

// (a, b) \in v1, (b, c) \in v2 => (a, c) \in new_vec
pub fn compose<T: PartialEq + Copy>(v1: &[(T, T)], v2: &[(T, T)]) -> Vec<(T, T)> {
    let mut new = Vec::new();

    for (a, b) in v1 {
        let (_, c) = v2.iter().find(|(b_, _)| b == b_).unwrap();
        new.push((*a, *c));
    }

    new
}

pub fn step1(slp: &SLP) -> SLP {
    // renaming vec
    // (shrinked_pos, original_pos) \in v
    let mut v: Vec<(usize, usize)> = Vec::new();

    // non-trivial rows
    let mut bitmatrix: Vec<Vec<bool>> = Vec::new();

    for i in 0..slp.num_of_variables() {
        let def = &slp[i];
        if popcount(&def) > 1 {
            let current = bitmatrix.len();
            v.push((current, i));
            bitmatrix.push(def.clone());
        }
    }

    let num_of_variables = bitmatrix.len();
    let num_of_constants = if bitmatrix.is_empty() {
        0
    } else {
        bitmatrix[0].len()
    };
    let bitmatrix = BitMatrix::from_nested_vecs(bitmatrix);
    SLP {
        repr: bitmatrix,
        num_of_variables,
        num_of_constants,
    }
}

// sub \subseteq? sup
#[allow(non_snake_case)]
pub fn check_sub_SLP(sub: &SLP, sup: &SLP) -> bool {
    if sub.num_of_variables() > sup.num_of_variables() {
        return false;
    }

    for var in 0..sub.num_of_variables() {
        if sub[var] != sup[var] {
            return false;
        }
    }

    true
}
