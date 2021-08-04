#![allow(clippy::ptr_arg)]

#[macro_use]
extern crate lazy_static;
extern crate itertools;
extern crate libc;

pub mod bitmatrix;
pub mod comparison;
pub mod fast_repair;
pub mod field;
pub mod fin_field;
pub mod for_benchmark;
pub mod fusion;
pub mod matrix;
pub mod optimize_slp;
pub mod renaming;
pub mod reorder;
pub mod reorder2;
pub mod repair;
pub mod rsv_bitmatrix;
pub mod run;
pub mod slp;
pub mod stat;
pub mod univariate_polynomial;
pub mod validation;
pub mod vandermonde;
pub mod vecteur;
pub mod xor;
pub mod xor64;
pub mod xor_repair;

pub const BLOCK_SIZE_PER_ITER: usize = if cfg!(feature = "4096block") {
    4096
} else if cfg!(feature = "3072block") {
    3072
} else if cfg!(feature = "2048block") {
    2048
} else if cfg!(feature = "1024block") {
    1024
} else if cfg!(feature = "512block") {
    512
} else if cfg!(feature = "256block") {
    256
} else if cfg!(feature = "128block") {
    128
} else if cfg!(feature = "64block") {
    64
} else if cfg!(feature = "8192block") {
    8192
} else {
    // default
    2048
};
pub const PEBBLE_NUM: usize = (32 * 1024) / BLOCK_SIZE_PER_ITER;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub struct Parameter {
    pub nr_data_block: usize,
    pub nr_parity_block: usize,
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub enum Term {
    Cst(usize),
    Var(usize),
}

impl std::fmt::Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Term::Var(v) => write!(f, "V{}", v),
            Term::Cst(v) => write!(f, "C{}", v),
        }
    }
}

impl Term {
    pub fn is_const(&self) -> bool {
        match self {
            Term::Cst(_) => true,
            Term::Var(_) => false,
        }
    }

    pub fn is_var(&self) -> bool {
        match self {
            Term::Cst(_) => false,
            Term::Var(_) => true,
        }
    }

    pub fn var_to_usize(&self) -> Option<usize> {
        match self {
            Term::Cst(_) => None,
            Term::Var(var) => Some(*var),
        }
    }

    pub fn cst_to_usize(&self) -> Option<usize> {
        match self {
            Term::Cst(cst) => Some(*cst),
            Term::Var(_) => None,
        }
    }
}

pub type Graph = Vec<(Term, Term, Term)>;
pub type MultiSLP = Vec<(Term, BTreeSet<Term>)>;

use std::collections::{BTreeMap, BTreeSet};
pub type Valuation = BTreeMap<Term, BTreeSet<Term>>;

#[allow(clippy::upper_case_acronyms)]
pub type DAG = Valuation; // SSA-form probram can be seen as DAG

pub fn dump_valuation(valuation: &Valuation) {
    for (t, val) in valuation {
        let mut s: String = String::new();
        let mut iter = val.iter();
        s.push_str(&format!("{}", iter.next().unwrap().cst_to_usize().unwrap()));
        while let Some(v) = iter.next() {
            s.push_str(&format!(" + {}", v.cst_to_usize().unwrap()));
        }
        println!("{} = {}", t.var_to_usize().unwrap(), s);
    }
}

pub fn graph_to_multislp(g: &Graph) -> MultiSLP {
    use std::iter::FromIterator;

    let mut slp = MultiSLP::new();

    for (t, l, r) in g {
        slp.push((t.clone(), BTreeSet::from_iter(vec![l.clone(), r.clone()])));
    }

    slp
}

pub fn graph_to_multiterm_slp(g: &Graph) -> Vec<(Term, Vec<Term>)> {
    let mut slp = Vec::new();

    for (t, l, r) in g {
        slp.push((t.clone(), vec![l.clone(), r.clone()]));
    }

    slp
}

pub fn multislp_to_multiterm_slp(g: &MultiSLP) -> Vec<(Term, Vec<Term>)> {
    let mut slp = Vec::new();

    for (t, children) in g {
        slp.push((t.clone(), children.iter().cloned().collect()));
    }

    slp
}

pub fn gen_data(len: usize) -> Vec<u8> {
    let mut v: Vec<u8> = loop {
        let v = vec![0u8; len];
        if v.as_ptr() as usize % 32 == 0 {
            break v;
        }
    };

    for ptr in v.iter_mut() {
        *ptr = rand::random::<u8>();
    }

    v
}

pub fn fill_by_random(ary: &mut [u8]) {
    for a in ary.iter_mut() {
        *a = rand::random::<u8>();
    }
}

pub fn split(n: usize, data: Vec<u8>) -> Vec<Vec<u8>> {
    assert!(data.len() % n == 0);

    let width = data.len() / n;
    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for i in 0..n {
        matrix.push(data[width * i..width * (i + 1)].to_vec());
    }

    matrix
}

pub fn drop<T>(vv: Vec<T>, remove: &[usize]) -> Vec<T> {
    let mut vv = vv;
    let mut remove = remove.to_vec();
    remove.sort_by(|a, b| b.cmp(a)); // v[0] > v[1] > ...

    for r in remove {
        vv.remove(r);
    }

    vv
}

pub fn drop8<T>(vv: Vec<T>, remove: &[usize]) -> Vec<T> {
    let mut vv = vv;
    let mut remove = remove.to_vec();
    remove.sort_by(|a, b| b.cmp(a)); // v[0] > v[1] > ...

    for r in remove {
        for _ in 0..8 {
            vv.remove(r * 8);
        }
    }

    vv
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    pub fn split_test() {
        let v1 = (0..16).collect::<Vec<_>>();
        assert_eq!(
            split(2, v1),
            vec![(0..8).collect::<Vec<_>>(), (8..16).collect::<Vec<_>>()]
        );
    }

    #[test]
    pub fn drop_test1() {
        let v = vec![0, 1, 2, 3, 4, 5];

        assert_eq!(drop(v, &vec![3, 1]), vec![0, 2, 4, 5]);
    }

    #[test]
    pub fn drop8_test1() {
        let v = vec![
            01, 02, 03, 04, 05, 06, 07, 08, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26,
            27, 28,
        ];

        assert_eq!(
            drop8(v, &vec![1]),
            vec![01, 02, 03, 04, 05, 06, 07, 08, 21, 22, 23, 24, 25, 26, 27, 28]
        );
    }

    #[test]
    pub fn drop8_test2() {
        let v = vec![
            01, 02, 03, 04, 05, 06, 07, 08, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26,
            27, 28, 31, 32, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 45, 46, 47, 48,
        ];

        assert_eq!(
            drop8(v, &vec![1, 3]),
            vec![
                01, 02, 03, 04, 05, 06, 07, 08, 21, 22, 23, 24, 25, 26, 27, 28, 41, 42, 43, 44, 45,
                46, 47, 48
            ]
        );
    }
}
