use crate::*;
use std::collections::{BTreeMap, BTreeSet};

pub fn is_ssa(slp: &Graph) -> bool {
    let mut map: BTreeSet<Term> = BTreeSet::new();

    for (target, _, _) in slp {
        if map.contains(target) {
            // we try to define one already defined
            return false;
        }
        map.insert(target.clone());
    }
    true
}

pub fn slp_to_ssa(slp: &[(Term, Term, Term)]) -> Graph {
    let mut ssa = Graph::new();

    // map (`v` in slp) to (l that is the line `l` number defines `v`)
    let mut map: BTreeMap<Term, usize> = BTreeMap::new();

    let mut current_line = 1;
    for (target, left, right) in slp {
        let left = map.get_mut(&left).map_or(
            left.clone(),      // if not found
            |v| Term::Var(*v), // if found
        );

        let right = map.get_mut(&right).map_or(
            right.clone(),     // if not found
            |v| Term::Var(*v), // if found
        );

        ssa.push((Term::Var(current_line), left, right));

        map.insert(target.clone(), current_line);
        current_line += 1;
    }

    ssa
}

/*
 * (x, [e1, e2, ..., en]) in Forest
 *   <->
 * x = e1 `xor` e2 `xor` ... `xor` en
 */
type Forest = DAG;

pub fn number_of_access(forest: &Forest) -> usize {
    let mut number = 0;

    for value in forest.values() {
        number += 1 + value.len();
    }

    number
}

pub fn number_of_access2(slp: &MultiSLP) -> usize {
    let mut number = 0;

    for (_, value) in slp {
        number += 1 + value.len();
    }

    number
}

pub fn csv_dump(forest: &Forest) {
    for (key, value) in forest {
        let mut s = key.to_string();
        for v in value {
            s.push(',');
            s.push_str(&v.to_string());
        }
        println!("{}", s);
    }
}

pub fn graph_to_forest(g: Graph) -> Forest {
    let mut forest = BTreeMap::new();

    for (x, e1, e2) in g {
        forest.insert(x, [e1, e2].iter().cloned().collect());
    }

    forest
}

fn expandable<'a>(forest: &'a Forest, term: &'a Term) -> Option<Term> {
    let mut target = None;

    for (k, v) in forest {
        if v.contains(term) {
            if target.is_none() {
                target = Some(k.clone());
            } else {
                // `term` is not expandable
                // because it is used at more than once.
                return None;
            }
        }
    }

    target
}

pub fn fusion(forest: &mut Forest, targets: &Vec<Term>) -> bool {
    let terms: Vec<Term> = forest.keys().cloned().collect();

    for x in terms {
        // if target <- x + ... and
        // x is just used once,
        // we expand it as target <- defs(x) + ...

        // if target is goal, we do not nothing.
        if targets.contains(&x) {
            continue;
        }

        if let Some(target) = expandable(forest, &x) {
            let l1 = forest.get(&target).unwrap().len();
            let l2 = forest.get(&x).unwrap().len();
            if l1 - 1 + l2 >= 8 {
                // continue;
            }
            let mut v = forest.remove(&x).unwrap();
            let defs = forest.get_mut(&target).unwrap();
            defs.remove(&x);
            defs.append(&mut v);
            return true;
        }
    }
    false
}

pub fn fusion_iter(graph: Graph, targets: &Vec<Term>) -> Forest {
    let mut forest = graph_to_forest(graph);

    loop {
        let do_next = fusion(&mut forest, targets);
        if !do_next {
            return forest;
        }
    }
}

pub fn graph_to_multislp_by_fusion(graph: Graph, targets: &Vec<Term>) -> MultiSLP {
    let ordering: Vec<Term> = graph.iter().map(|(a, _, _)| a).cloned().collect();
    let forest = fusion_iter(graph, targets);

    let mut slp = MultiSLP::new();
    for v in &ordering {
        if let Some(def) = forest.get(v) {
            slp.push((v.clone(), def.clone()));
        }
    }

    slp
}
