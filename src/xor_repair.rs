use crate::bitmatrix::*;
use crate::fast_repair::SortOrder;
use crate::repair::{bitvec_distance, bitvec_xor};
use crate::slp::*;
use crate::*;

fn replace_by(m: &mut SLP, a: usize, b: usize, v: usize) {
    for i in 0..m.height() {
        if m[i][a] && m[i][b] {
            m[i][a] = false;
            m[i][b] = false;
            m[i][v] = true;
        }
    }
}

fn count_occurences2(i: usize, j: usize, slp: &SLP) -> usize {
    let mut count = 0;
    for def in 0..slp.height() {
        if slp[def][i] && slp[def][j] {
            count += 1
        }
    }
    count
}

fn gen_forward_iter(start: usize, end: usize) -> Vec<usize> {
    (start..end).collect()
}
fn gen_rev_iter(start: usize, end: usize) -> Vec<usize> {
    (start..end).rev().collect()
}

fn build_syntax(
    valuation: &SLP, // mapping: added variables to constants
    goal: &[bool],   // depends to constants
    gen_iter: fn(usize, usize) -> Vec<usize>,
) -> (Vec<bool>, Vec<bool>) {
    let mut depends: Vec<bool> =
        vec![false; valuation.height() + valuation.num_of_original_constants()];

    let mut rest = goal.to_vec();

    // loop until we cannot make a good replace
    loop {
        let mut which_var = None;
        let mut current_min = popcount(&rest);

        for var in gen_iter(0, valuation.height()) {
            let value = &valuation[var];
            let count = bitvec_distance(&value, &rest);

            // first found is preferred
            if count < current_min {
                which_var = Some(var);
                current_min = count;
            }
        }

        if let Some(var) = which_var {
            depends[valuation.num_of_original_constants() + var] = true;
            rest = bitvec_xor(&rest, &valuation[var]);
        } else {
            for idx in 0..rest.len() {
                if rest[idx] {
                    depends[idx] = true;
                }
            }
            break;
        }
    }

    (depends, rest)
}

fn xor_pair_finder(
    valuation: &SLP,
    slp: &SLP,         // goal
    program: &mut SLP, // syntax repr for the given slp
    gen_iter: fn(usize, usize) -> Vec<usize>,
    order: &SortOrder,
) -> Option<(Term, Term)> {
    use std::cmp::Ordering;

    let mut count_max = 0;
    let mut candidates = Vec::new();

    for i in 0..slp.num_of_variables() {
        // this is one of remained goal
        let goal = &slp[i];
        let (depends, _) = build_syntax(valuation, &goal, gen_iter);

        if popcount(&depends) < popcount(&program[i]) {
            program[i] = depends;
        }
    }

    for i in 0..program.width() {
        for j in i + 1..program.width() {
            let count = count_occurences2(i, j, &program);

            match count_max.cmp(&count) {
                Ordering::Equal => {
                    // count_max == count
                    candidates.push((i, j));
                }
                Ordering::Less => {
                    // count_max < count
                    candidates = vec![(i, j)];
                    count_max = count;
                }
                _ => {}
            }
        }
    }

    match order {
        SortOrder::LexSmall => {
            candidates.sort_by(|(a1, a2), (b1, b2)| a1.cmp(b1).then(a2.cmp(b2)));
        }
        SortOrder::LexLarge => {
            candidates.sort_by(|(a1, a2), (b1, b2)| a1.cmp(b1).then(a2.cmp(b2)).reverse());
        }
    }

    if let Some((i, j)) = candidates.pop() {
        Some((valuation.index_to_term(i), valuation.index_to_term(j)))
    } else {
        None
    }
}

fn get_valuation(valuation: &SLP, left: &Term, right: &Term) -> Vec<bool> {
    let mut val: Vec<bool> = vec![false; valuation.num_of_original_constants()];

    match left {
        Term::Cst(c) => {
            val[*c] ^= true;
        }
        Term::Var(v) => {
            val = valuation[*v].clone();
        }
    }

    match right {
        Term::Cst(c) => {
            val[*c] ^= true;
        }
        Term::Var(v) => {
            val = bitvec_xor(&val, &valuation[*v]);
        }
    }

    val
}

pub fn run_xor_repair_forward(goal: &SLP, order: SortOrder) -> Vec<(Term, Term, Term)> {
    run_xor_repair(goal, gen_forward_iter, order)
}
pub fn run_xor_repair_reverse(goal: &SLP, order: SortOrder) -> Vec<(Term, Term, Term)> {
    run_xor_repair(goal, gen_rev_iter, order)
}

fn run_xor_repair(
    goal: &SLP,
    gen_iter: fn(usize, usize) -> Vec<usize>,
    order: SortOrder,
) -> Vec<(Term, Term, Term)> {
    let mut defs = Vec::new();
    let mut slp = goal.clone();

    let mut program = slp.clone();
    let mut valuation = SLP::new(
        BitMatrix::new(0, goal.num_of_original_constants()),
        goal.num_of_original_constants(),
        0,
    );

    loop {
        let candidate = xor_pair_finder(&valuation, &slp, &mut program, gen_iter, &order);

        if let Some((left, right)) = candidate {
            let fresh = defs.len();
            let new_var = Term::Var(fresh);
            let new_val = get_valuation(&valuation, &left, &right);

            program.add_column(); // make a space for the new variable
            {
                let left = slp.term_to_index(&left);
                let right = slp.term_to_index(&right);
                let new = slp.term_to_index(&new_var);
                replace_by(&mut program, left, right, new);
                valuation.add_var(new_val);
            }
            defs.push((new_var, left, right));
        } else {
            panic!("bug!");
        }

        remove_achieved_goal(&valuation, &mut program, &mut slp);
        if slp.height() == 0 {
            return defs;
        }
    }
}

fn remove_achieved_goal(valuation: &SLP, program: &mut SLP, slp: &mut SLP) {
    let num_of_variables = valuation.height();
    let latest_var = &valuation[num_of_variables - 1];

    for i in (0..slp.height()).rev() {
        let val = &slp[i];

        if val == latest_var {
            program.remove_row(i);
            slp.remove_row(i);
            break;
        }
    }
}
