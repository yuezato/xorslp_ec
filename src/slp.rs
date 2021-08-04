use crate::bitmatrix::*;
use crate::*;
use std::fs::File;
use std::ops::{Index, IndexMut};

pub fn var_size(var: &[bool]) -> usize {
    let count = popcount(var);
    if count <= 2 {
        1
    } else {
        count - 1
    }
}
pub fn num_of_xor(var: &[bool]) -> usize {
    let count = popcount(var);
    assert!(count >= 1);
    count - 1
}

#[derive(Eq, PartialEq, Clone, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub struct SLP {
    pub repr: BitMatrix,
    pub num_of_variables: usize, // num of original variables
    pub num_of_constants: usize, // num of original constants
}

impl Index<usize> for SLP {
    type Output = Vec<bool>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.repr[index]
    }
}

impl IndexMut<usize> for SLP {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.repr[index]
    }
}

impl SLP {
    pub fn new(repr: BitMatrix, num_of_constants: usize, num_of_variables: usize) -> Self {
        SLP {
            repr,
            num_of_variables,
            num_of_constants,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.repr.is_empty()
    }

    pub fn index_to_term(&self, idx: usize) -> Term {
        if idx < self.num_of_original_constants() {
            Term::Cst(idx)
        } else {
            Term::Var(idx - self.num_of_original_constants())
        }
    }

    pub fn term_to_index(&self, term: &Term) -> usize {
        match term {
            Term::Cst(c) => *c,
            Term::Var(v) => *v + self.num_of_original_constants(),
        }
    }

    pub fn roots(&self) -> Vec<usize> {
        let mut v = Vec::new();
        for i in 0..self.num_of_variables() {
            let idx = i + self.num_of_original_constants();
            if self.out_degree(idx) == 0 {
                v.push(i);
            }
        }
        v
    }

    pub fn out_degree(&self, idx: usize) -> usize {
        let mut count = 0;
        for i in 0..self.num_of_variables() {
            if self[i][idx] {
                count += 1;
            }
        }
        count
    }
    pub fn out_degree_of_var(&self, var: &Term) -> Option<usize> {
        match var {
            Term::Cst(_) => None,
            Term::Var(v) => {
                let idx = self.num_of_constants + *v;
                let mut count = 0;
                for i in 0..self.num_of_variables() {
                    if self[i][idx] {
                        count += 1;
                    }
                }
                Some(count)
            }
        }
    }

    pub fn num_of_variables(&self) -> usize {
        self.repr.height()
    }

    pub fn whole_size(&self) -> usize {
        let mut count = 0;
        for i in 0..self.num_of_variables() {
            count += var_size(&self[i]);
        }
        count
    }
    pub fn whole_xor(&self) -> usize {
        let mut count = 0;
        for i in 0..self.num_of_variables() {
            count += num_of_xor(&self[i]);
        }
        count
    }
    pub fn num_of_memaccess_for_xor(&self) -> usize {
        // a <- b + c = 3
        // a <- b + c + d = tmp <- b + c; a <- tmp + c = 6
        // and so on
        self.whole_xor() * 3
    }

    pub fn dump(&self) {
        self.repr.dump();
    }

    fn short_repr_var(&self, i: usize) -> Vec<Term> {
        let v: &Vec<bool> = &self[i];
        let mut tv: Vec<Term> = Vec::new();

        for (idx, e) in v.iter().enumerate() {
            if *e {
                tv.push(self.index_to_term(idx));
            }
        }

        tv
    }

    pub fn pprint(&self) {
        let mut total = 0;
        for i in 0..self.num_of_variables() {
            let l = self.short_repr_var(i).len();
            total += l;
            println!("v_{} = [{}] {:?}", i, l, self.short_repr_var(i));
        }
        dbg!(total);
    }

    pub fn build_from_bitmatrix_not_depending_variables(repr: &BitMatrix) -> Self {
        let repr = repr.clone();

        let num_of_constants = repr.width();
        let num_of_variables = repr.height();

        SLP {
            repr,
            num_of_variables,
            num_of_constants,
        }
    }

    pub fn build_from_file(file: File) -> Option<Self> {
        let repr = BitMatrix::build_from_file(file);
        let repr = repr.as_ref()?;

        Some(SLP::build_from_bitmatrix_not_depending_variables(&repr))
    }

    pub fn num_of_original_constants(&self) -> usize {
        self.num_of_constants
    }

    pub fn is_binary_form(&self) -> bool {
        let m = &self.repr;
        for i in 0..m.height() {
            if m.pop_cnt_column(i) > 2 {
                return false;
            }
        }
        true
    }

    pub fn width(&self) -> usize {
        self.repr.width()
    }

    pub fn height(&self) -> usize {
        self.repr.height()
    }

    // add column(|)-vector
    pub fn add_column(&mut self) {
        self.repr.add_column()
    }

    // add row(-)-vector
    pub fn add_row(&mut self) {
        self.repr.add_row()
    }
    pub fn remove_row(&mut self, idx: usize) {
        self.repr.remove_row(idx);
    }

    pub fn add_var(&mut self, val: Vec<bool>) {
        self.repr.inner.push(val);
    }

    pub fn remove_trivials(&mut self) {
        let m = &mut self.repr;
        for i in (0..m.height()).rev() {
            if popcount(&m[i]) <= 1 {
                m.remove_row(i);
            }
        }
    }

    pub fn to_trivial_graph(&self) -> Graph {
        let mut graph = Graph::new();

        for i in 0..self.height() {
            let deps: &Vec<bool> = &self[i];

            let positions: Vec<usize> = deps
                .iter()
                .enumerate()
                .filter_map(|(idx, b)| if *b { Some(idx) } else { None })
                .collect();

            if positions.len() == 1 {
                unreachable!("unexpected");
            } else {
                graph.push((
                    Term::Var(i),
                    Term::Cst(positions[0]),
                    Term::Cst(positions[1]),
                ));
                for idx in &positions[2..] {
                    graph.push((Term::Var(i), Term::Var(i), Term::Cst(*idx)));
                }
            }
        }

        graph
    }
}
