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

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    LexSmall,
    LexLarge,
}

fn pair_finder2(
    slp: &SLP,         // goal
    program: &SLP,     // syntax repr for the given slp
    order: &SortOrder, // 0 = <lex, 1 = >lex
) -> Option<(Term, Term)> {
    use std::cmp::Ordering;

    let mut count_max = 0;
    let mut candidates = Vec::new();

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
        Some((slp.index_to_term(i), slp.index_to_term(j)))
    } else {
        None
    }
}

pub fn run_repair2(goal: &SLP, sort_order: SortOrder) -> Vec<(Term, Term, Term)> {
    let mut defs = Vec::new();
    let slp = goal.clone();

    let mut program = slp.clone();

    loop {
        let pair = pair_finder2(&slp, &program, &sort_order);

        if let Some((left, right)) = pair {
            let fresh = defs.len();
            let new_var = Term::Var(fresh);
            // println!("add {:?} <- {:?} + {:?}", new_v, left, right);
            program.add_column();
            {
                let left = slp.term_to_index(&left);
                let right = slp.term_to_index(&right);
                let new = slp.term_to_index(&new_var);
                replace_by(&mut program, left, right, new);
            }
            defs.push((new_var, left, right));
        } else {
            panic!("bug!");
        }

        program.remove_trivials();
        if program.height() == 0 {
            return defs;
        }
        // println!("[{}] #rest = {}", iter, slp.num_of_variables());
    }
}
