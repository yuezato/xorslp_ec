use crate::Term;
use std::collections::VecDeque;

pub type Program = Vec<(Term, Term, Term)>;

pub fn rename(var_a: usize, var_b: usize, term: &Term) -> Option<Term> {
    match term {
        Term::Cst(_) => None,
        Term::Var(v) => {
            if *v == var_a {
                Some(Term::Var(var_b))
            } else {
                None
            }
        }
    }
}

pub fn mapping_to_rewriting(mapping: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut strong_components: Vec<VecDeque<usize>> = Vec::new();

    for (from, to) in mapping {
        let mut component = VecDeque::new();
        component.push_front(*from);
        component.push_back(*to);
        strong_components.push(component);
    }

    loop {
        let mut pair = None;
        for i in 0..strong_components.len() {
            for j in 0..strong_components.len() {
                if pair.is_some() {
                    break;
                }

                if i == j {
                    continue;
                }

                let component_i = &strong_components[i];
                let component_j = &strong_components[j];

                if component_i.back() == component_j.front() {
                    pair = Some((i, j));
                }
            }
        }

        if let Some((i, j)) = pair {
            let mut c_j = strong_components[j].clone();
            c_j.pop_front();
            strong_components[i].append(&mut c_j);
            strong_components.remove(j);
        } else {
            break;
        }
    }

    let mut rewriting = Vec::new();

    for component in strong_components {
        for i in 0..component.len() {
            let next = (i + 1) % component.len();
            rewriting.push((component[i], component[next]));
        }
    }

    rewriting
}

pub fn rename_by_rules(renaming: &[(usize, usize)], term: &Term) -> Term {
    for (a, b) in renaming {
        if let Some(term) = rename(*a, *b, term) {
            return term;
        }
    }

    term.clone()
}

pub fn rename_program_by(
    renaming: &[(usize, usize)],
    program: &[(Term, Term, Term)],
) -> Vec<(Term, Term, Term)> {
    let mut program = program.to_vec();

    for instr in &mut program {
        let (def, left, right) = instr;
        let def = rename_by_rules(renaming, def);
        let left = rename_by_rules(renaming, left);
        let right = rename_by_rules(renaming, right);
        *instr = (def, left, right);
    }

    program
}

pub fn rename_multislp_by(
    renaming: &[(usize, usize)],
    program: &[(Term, Vec<Term>)],
) -> Vec<(Term, Vec<Term>)> {
    let mut program = program.to_vec();

    for instr in &mut program {
        let (def, terms) = instr;
        let def = rename_by_rules(renaming, def);
        let terms: Vec<_> = terms.iter().map(|e| rename_by_rules(renaming, e)).collect();
        *instr = (def, terms);
    }

    program
}
