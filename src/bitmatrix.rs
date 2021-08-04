use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BitMatrix {
    // inner[i] denotes a i-th row-vector
    pub inner: Vec<Vec<bool>>,
}

impl Index<usize> for BitMatrix {
    type Output = Vec<bool>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[index]
    }
}

impl IndexMut<usize> for BitMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.inner[index]
    }
}

pub fn popcount(v: &[bool]) -> usize {
    let mut count = 0;
    for b in v {
        if *b {
            count += 1;
        }
    }
    count
}

impl BitMatrix {
    pub fn new(height: usize, width: usize) -> Self {
        let mut inner = Vec::new();

        for _ in 0..height {
            inner.push(vec![false; width]);
        }

        BitMatrix { inner }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn drop_rows(&mut self, mut ixs: Vec<usize>) {
        ixs.sort_unstable();

        for ix in ixs.iter().rev() {
            self.inner.remove(*ix);
        }
    }

    pub fn pop_cnt_column(&self, i: usize) -> usize {
        popcount(&self[i])
    }

    pub fn from_nested_vecs(inner: Vec<Vec<bool>>) -> Self {
        Self { inner }
    }

    pub fn width(&self) -> usize {
        self.inner[0].len()
    }
    pub fn height(&self) -> usize {
        self.inner.len()
    }

    // add column(|)-vector
    pub fn add_column(&mut self) {
        for i in 0..self.height() {
            self[i].push(false);
        }
    }

    // add row(-)-vector
    pub fn add_row(&mut self) {
        let v = vec![false; self.width()];
        self.inner.push(v);
    }

    pub fn remove_row(&mut self, idx: usize) {
        self.inner.remove(idx);
    }

    pub fn col(&self, idx: usize) -> Vec<bool> {
        let mut c = vec![false; self.height()];
        for i in 0..self.height() {
            c[i] = self[i][idx];
        }
        c
    }

    pub fn dump(&self) {
        for v in &self.inner {
            // Vec<bool> -> String
            let mut l = String::new();

            for b in v {
                let c = if *b { '1' } else { '0' };
                l.push(c);
            }

            println!("{}", l);
        }
    }

    pub fn build_from_file(file: File) -> Option<Self> {
        let mut inner: Vec<Vec<bool>> = Vec::new();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            // read one line
            let line: String = line.expect("there is an invalid line in the input file");

            // a line which starts with # is a comment and skipped
            if line.as_str().starts_with('#') {
                continue;
            }

            // string -> vec<bool>
            // For example,
            // 0101 -> false true false true
            let mut bitvec: Vec<bool> = vec![false; line.len()];
            for (i, c) in line.chars().enumerate() {
                if c == '0' {
                    bitvec[i] = false;
                } else if c == '1' {
                    bitvec[i] = true;
                } else {
                    println!("there is an invalid character `{}` in the input file", c);
                    return None;
                }
            }

            inner.push(bitvec);
        }

        // check the input file forms an bitmatrix
        let l = inner[0].len();
        for v in &inner[1..] {
            assert_eq!(l, v.len());
        }

        Some(Self { inner })
    }

    pub fn mul(&self, right: &BitMatrix) -> BitMatrix {
        debug_assert!(self.width() == right.height());

        let mut bm = BitMatrix::new(self.height(), right.width());

        for i in 0..self.height() {
            for j in 0..right.width() {
                let vi = &self[i];
                let vj = right.col(j);

                let mut val = false;
                for k in 0..vi.len() {
                    val ^= vi[k] & vj[k];
                }
                bm[i][j] = val;
            }
        }

        bm
    }
}

pub fn u8_to_colvec(u: u8) -> BitMatrix {
    let mut bm = BitMatrix::new(8, 1);

    for i in 0..8 {
        bm[i][0] = (u >> (7 - i)) & 1 == 1;
    }

    bm
}

pub fn colvec_to_u8(m: &BitMatrix) -> u8 {
    let mut u = 0u8;

    for i in 0..8 {
        if m[i][0] {
            u |= 1 << (7 - i);
        }
    }

    u
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_colvec_test1() {
        for u in 0..255u8 {
            assert_eq!(colvec_to_u8(&u8_to_colvec(u)), u);
        }
    }
}
