use crate::bitmatrix::*;
use crate::slp::*;
use crate::*;
use std::collections::HashMap;

pub fn bitvec_to_intvec(bv: &[bool]) -> Vec<usize> {
    let mut v = vec![0; bv.len()];

    for (idx, b) in bv.iter().enumerate() {
        if *b {
            v[idx] = 1;
        } else {
            v[idx] = 0;
        }
    }

    v
}

pub fn bitvec_distance(v1: &[bool], v2: &[bool]) -> usize {
    debug_assert_eq!(v1.len(), v2.len());

    let mut count = 0;

    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            count += 1;
        }
    }

    count
}

pub fn bitvec_xor(v1: &[bool], v2: &[bool]) -> Vec<bool> {
    debug_assert_eq!(v1.len(), v2.len());

    let mut v = vec![false; v1.len()];
    for i in 0..v1.len() {
        v[i] = v1[i] ^ v2[i];
    }
    v
}

fn build_syntax(valuation: &HashMap<usize, Vec<bool>>, goal: &[bool]) -> (Vec<Term>, Vec<bool>) {
    let mut syntax: Vec<Term> = Vec::new();

    let mut goal = goal.to_vec();

    loop {
        let mut which_var = None;
        // let mut current_min = var_size(&goal);
        let mut current_min = popcount(&goal);

        for var in (0..valuation.keys().count()).rev() {
            let value = &valuation[&var];
            let count = bitvec_distance(&value, &goal);

            if count < current_min {
                which_var = Some(var);
                current_min = count;
            }
        }

        if let Some(var) = which_var {
            syntax.push(Term::Var(var));
            goal = bitvec_xor(&goal, &valuation[&var]);
        } else {
            for (idx, dep) in goal.iter().enumerate() {
                if *dep {
                    syntax.push(Term::Cst(idx));
                }
            }
            break;
        }
    }

    (syntax, goal)
}

pub fn replace(component: &[Term], left: &Term, right: &Term, new: &Term) -> Vec<Term> {
    use std::cmp;

    // if component = ... left, right ... => ... new ...
    // if component = ... right, left ... => ... new ...
    // otherwie component => component

    let mut component = component.to_owned();

    let idx1 = component.iter().position(|x| x == left);
    let idx2 = component.iter().position(|x| x == right);

    if let Some(idx1) = idx1 {
        if let Some(idx2) = idx2 {
            component.remove(cmp::max(idx1, idx2));
            component.remove(cmp::min(idx1, idx2));
            component.push(new.clone());
        }
    }

    component
}

pub fn replace_program(program: &mut [Vec<Term>], left: &Term, right: &Term, new: &Term) {
    for def in program {
        *def = replace(def, left, right, new);
    }
}

pub fn count_occurences(left: &Term, right: &Term, values: &[Vec<Term>]) -> usize {
    let mut cur = 0;

    for value in values {
        if value.contains(left) && value.contains(right) {
            cur += 1;
        }
    }

    cur
}

fn heuristics1(_: &[Vec<Term>], pairs: &[(Term, Term)]) -> (Term, Term) {
    debug_assert!(!pairs.is_empty());

    pairs[0].clone()
}

fn heuristics2(program: &[Vec<Term>], pairs: &[(Term, Term)]) -> (Term, Term) {
    debug_assert!(!pairs.is_empty());

    let mut current = usize::MAX;
    let mut candidate = None;

    for (left, right) in pairs {
        let mut program: Vec<Vec<Term>> = program.to_vec();
        replace_program(&mut program, left, right, &Term::Var(usize::MAX));

        let mut distance = 0;
        for def in program {
            let l = def.len();
            distance += l * l;
        }

        if distance < current {
            current = distance;
            candidate = Some((left.clone(), right.clone()));
        }
    }

    candidate.unwrap()
}

fn heuristics3(program: &[Vec<Term>], pairs: &[(Term, Term)]) -> (Term, Term) {
    debug_assert!(!pairs.is_empty());

    let mut current = usize::MAX;
    let mut candidate = None;

    for (left, right) in pairs {
        let mut program = program.to_vec();
        replace_program(&mut program, left, right, &Term::Var(usize::MAX));

        let max = program.iter().map(|v| v.len()).max().unwrap();

        if max < current {
            current = max;
            candidate = Some((left.clone(), right.clone()));
        }
    }

    candidate.unwrap()
}

type TieBreaker = fn(&[Vec<Term>], &[(Term, Term)]) -> (Term, Term);

fn repair(
    program: &[Vec<Term>],
    num_of_variables: usize,
    slp: &SLP,
    tie_break: TieBreaker,
) -> Option<(Term, Term)> {
    use std::cmp::Ordering;

    let num_of_constants = slp.num_of_original_constants();

    let mut count_max = 0;
    let mut tie_keeping = Vec::new();

    for i in 0..(num_of_constants + num_of_variables) {
        for j in i + 1..(num_of_constants + num_of_variables) {
            let left = slp.index_to_term(i);
            let right = slp.index_to_term(j);

            let count = count_occurences(&left, &right, &program);

            let mut to_update = false;

            match count_max.cmp(&count) {
                Ordering::Less => {
                    // count_max < count
                    tie_keeping = Vec::new();
                    count_max = count;
                    to_update = true;
                }
                Ordering::Equal => {
                    to_update = true;
                }
                _ => {}
            };

            if to_update {
                tie_keeping.push((left, right));
            }
        }
    }

    if tie_keeping.is_empty() {
        None
    } else {
        Some(tie_break(program, &tie_keeping))
    }
}

pub fn repair_trivial(
    program: &[Vec<Term>],
    num_of_variables: usize,
    slp: &SLP,
) -> Option<(Term, Term)> {
    repair(program, num_of_variables, slp, heuristics1)
}

pub fn repair_distance(
    program: &[Vec<Term>],
    num_of_variables: usize,
    slp: &SLP,
) -> Option<(Term, Term)> {
    repair(program, num_of_variables, slp, heuristics2)
}

pub fn repair_long(
    program: &[Vec<Term>],
    num_of_variables: usize,
    slp: &SLP,
) -> Option<(Term, Term)> {
    repair(program, num_of_variables, slp, heuristics3)
}

fn remove_trivials(vec: &mut Vec<Vec<Term>>) {
    for i in (0..vec.len()).rev() {
        if vec[i].len() == 1 {
            vec.remove(i);
        }
    }
}

pub fn run_repair_trivial(goal: &SLP) -> Vec<(Term, Term, Term)> {
    run_repair(goal, repair_trivial)
}

pub fn run_repair_distance(goal: &SLP) -> Vec<(Term, Term, Term)> {
    run_repair(goal, repair_distance)
}

pub fn run_repair_long(goal: &SLP) -> Vec<(Term, Term, Term)> {
    run_repair(goal, repair_long)
}

type PairFinder = fn(&[Vec<Term>], usize, &SLP) -> Option<(Term, Term)>;

fn run_repair(goal: &SLP, pair_finder: PairFinder) -> Vec<(Term, Term, Term)> {
    let mut defs = Vec::new();
    let slp = goal.clone();

    let mut program = Vec::new();

    for i in 0..slp.num_of_variables() {
        let (def, _) = build_syntax(&HashMap::new(), &slp[i]);
        program.push(def);
    }

    loop {
        let pair = pair_finder(&program, defs.len(), &slp);

        if let Some((left, right)) = pair {
            let fresh = defs.len();
            let new_v = Term::Var(fresh);
            // println!("add {:?} <- {:?} + {:?}", new_v, left, right);
            replace_program(&mut program, &left, &right, &new_v);
            defs.push((new_v, left, right));
        } else {
            panic!("bug!");
        }

        remove_trivials(&mut program);
        if program.is_empty() {
            return defs;
        }
        // println!("[{}] #rest = {}", iter, slp.num_of_variables());
    }
}

pub fn programs_to_slp(program: &[(Term, Term, Term)]) -> SLP {
    let mut num_of_variables = 0;
    let mut num_of_constants = 0;

    for (var, left, right) in program {
        if let Term::Var(var) = var {
            num_of_variables = std::cmp::max(num_of_variables, *var);
        } else {
            panic!(";-|");
        }

        if let Term::Cst(left) = left {
            num_of_constants = std::cmp::max(num_of_constants, *left);
        }
        if let Term::Cst(right) = right {
            num_of_constants = std::cmp::max(num_of_constants, *right);
        }
    }

    num_of_variables += 1;
    num_of_constants += 1;

    let mut slp = BitMatrix::new(num_of_variables, num_of_constants + num_of_variables);

    for (var, left, right) in program {
        let left_index = match left {
            Term::Cst(left) => *left,
            Term::Var(left) => num_of_constants + *left,
        };

        let right_index = match right {
            Term::Cst(right) => *right,
            Term::Var(right) => num_of_constants + *right,
        };

        if let Term::Var(var) = var {
            slp[*var][left_index] = true;
            slp[*var][right_index] = true;
        } else {
            panic!(";-|");
        }
    }

    // new(repr, #original_constants, #original_variables)
    SLP::new(slp, num_of_constants, num_of_variables)
}

fn execute_slp_rec(slp: &SLP, var: usize, valuation: &mut HashMap<usize, Vec<bool>>) {
    if valuation.contains_key(&var) {
        // already visited
        return;
    }

    let num_constants = slp.num_of_original_constants();
    let mut val: Vec<bool> = vec![false; num_constants];

    let depends: &Vec<bool> = &slp[var];

    for (idx, b) in depends.iter().enumerate() {
        if *b {
            if idx < num_constants {
                val[idx] ^= true;
            } else {
                let depending_var = idx - num_constants;

                if !valuation.contains_key(&depending_var) {
                    execute_slp_rec(slp, depending_var, valuation);
                }

                // bitvec xor bitvec
                val = bitvec_xor(&val, &valuation[&depending_var]);
            }
        }
    }

    valuation.insert(var, val);
}

pub fn execute_slp(slp: &SLP) -> SLP {
    let mut valuation: HashMap<usize, Vec<bool>> = HashMap::new();
    let mut bitmatrix: Vec<Vec<bool>> = Vec::new();

    for i in 0..slp.num_of_variables() {
        execute_slp_rec(&slp, i, &mut valuation);
        bitmatrix.push(valuation[&i].clone());
    }

    let bitmatrix = BitMatrix::from_nested_vecs(bitmatrix);
    SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix)
}

pub fn evaluate_program(program: &[(Term, Term, Term)]) -> SLP {
    assert!(crate::fusion::is_ssa(&program.to_vec()));
    // let slp = programs_to_slp(&crate::fusion::slp_to_ssa(program));
    let slp = programs_to_slp(&program);
    execute_slp(&slp)
}

// Does slp_a \models slp_b ??
// return None if slp_a does not \models slp_b
// otherwise, if (x, y) in realize(...) then x of slp_a = y of slp_b
pub fn realizes(slp_a: &SLP, slp_b: &SLP) -> Option<Vec<(usize, usize)>> {
    let mut mapping = Vec::new();

    for var_b in 0..slp_b.num_of_variables() {
        let value_b = &slp_b[var_b];

        let mut found = false;
        for var_a in 0..slp_a.num_of_variables() {
            let value_a = &slp_a[var_a];

            if value_a == value_b {
                mapping.push((var_a, var_b));
                found = true;
                break;
            }
        }
        if !found {
            return None;
        }
    }

    Some(mapping)
}
