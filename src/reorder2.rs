use crate::reorder::*;
use crate::*;
use std::cmp::Ordering;

fn term_cmp(t1: &Term, t2: &Term) -> Ordering {
    use crate::Term::*;

    match (t1, t2) {
        (Var(t1), Var(t2)) => t1.cmp(t2),
        (Cst(t1), Cst(t2)) => t1.cmp(t2),
        (Var(_), Cst(_)) => Ordering::Less,
        (Cst(_), Var(_)) => Ordering::Greater,
    }
}

fn calc_candidates(dag: &DAG, alloc: &Alloc) -> Vec<(Term, bool, usize, usize)> {
    // (term, ready?, #hot, #children)
    let mut summary = Vec::new();

    for (t, children) in dag {
        let mut ready = true;
        let mut hot = 0;
        for c in children {
            if c.is_const() || alloc.get(c).is_some() {
                if alloc.is_hot(c) {
                    hot += 1;
                }
            } else {
                ready = false;
            }
        }
        if !ready {
            summary.push((t.clone(), false, 0, 0));
        } else {
            summary.push((t.clone(), true, hot, children.len()));
        }
    }

    summary
}

pub fn deal_multislp2(
    slp: &MultiSLP,
    num_of_constants: usize,
    targets: Vec<Term>,
    strategy: Strategy,
) -> Vec<(Pebble, Vec<Pebble>)> {
    // dbg!(PEBBLE_NUM);

    let mut pebble_computation: Vec<(Pebble, Vec<Pebble>)> = Vec::new();
    let mut dag = multislp_to_dag(slp);
    let original_len = dag.len();

    let mut alloc = Alloc::new(num_of_constants, targets, strategy);

    let mut outdegs = make_outdegrees(&dag);

    loop {
        if dag.is_empty() {
            break;
        }

        let mut reduced: Vec<(Term, bool, usize, usize)> = calc_candidates(&dag, &alloc)
            .into_iter()
            .filter(|(_, ready, _, _)| *ready)
            .collect();

        reduced.sort_by(|(_, _, hot1, children1), (_, _, hot2, children2)| {
            let ratio1 = (*hot1 as f64) / (*children1 as f64);
            let ratio2 = (*hot2 as f64) / (*children2 as f64);
            ratio2.partial_cmp(&ratio1).unwrap()
        });

        let (target, _, _, _) = &reduced[0];
        let children: &BTreeSet<Term> = dag.get(&target).unwrap();

        /*
         * Compute a visiting order of `children` by
         *  firstly sorting them using LRU-ordering (we mostly prefer the LRU element)
         * and then sorting them using TermConstOrdering.
         */
        let mut sorted: Vec<&Term> = children.iter().collect();
        sorted.sort_by(|a, b| {
            let x = alloc.index(a).unwrap_or(0xffffffff);
            let y = alloc.index(b).unwrap_or(0xffffffff);
            x.cmp(&y).then(term_cmp(a, b))
        });

        let mut pebbles = Vec::new();

        for c in sorted {
            pebbles.push(alloc.get(c).unwrap());
            alloc.access(&c);
            let mut_ref = outdegs.get_mut(c).unwrap();
            *mut_ref -= 1;
            if *mut_ref == 0 {
                alloc.try_release(c);
            }
        }
        let pebble = alloc.assign(&target);
        pebble_computation.push((pebble, pebbles));
        dag.remove(&target);
    }

    assert!(pebble_computation.len() == original_len);

    pebble_computation
}
