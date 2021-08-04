use crate::reorder::Pebble;
use crate::slp::SLP;
use crate::*;
use std::iter::FromIterator;

fn get_val(v: &Valuation, term: &Term) -> BTreeSet<Term> {
    if term.is_const() {
        BTreeSet::from_iter(vec![term.clone()])
    } else {
        v.get(term).unwrap().clone()
    }
}

pub fn xor_set(b1: &BTreeSet<Term>, b2: &BTreeSet<Term>) -> BTreeSet<Term> {
    b1.symmetric_difference(b2).cloned().collect()
}

pub fn graph_to_valuations(g: &Graph) -> Valuation {
    let mut valuation = Valuation::new();

    for (t, t1, t2) in g {
        let v1 = get_val(&valuation, t1);
        let v2 = get_val(&valuation, t2);
        valuation.insert(t.clone(), xor_set(&v1, &v2));
    }

    valuation
}

pub fn multislp_to_valuation(slp: &MultiSLP) -> Valuation {
    let mut valuation = Valuation::new();

    for (t, set) in slp {
        let mut val = BTreeSet::new();
        for e in set {
            let v = get_val(&valuation, e);
            val = xor_set(&val, &v);
        }
        valuation.insert(t.clone(), val);
    }

    valuation
}

pub fn pebble_computation_to_valuation(computation: &[(Pebble, Vec<Pebble>)]) -> Valuation {
    let mut valuation = Valuation::new();

    for (t, children) in computation {
        let mut val = BTreeSet::new();
        for c in children {
            let v = get_val(&valuation, &c.to_term());
            val = xor_set(&val, &v);
        }
        valuation.insert(t.to_term(), val);
    }

    valuation
}

pub fn term_computation_to_valuation(computation: &[(Term, Vec<Term>)]) -> Valuation {
    let mut valuation = Valuation::new();

    for (t, children) in computation {
        let mut val = BTreeSet::new();
        for c in children {
            let v = get_val(&valuation, &c);
            val = xor_set(&val, &v);
        }
        valuation.insert(t.clone(), val);
    }

    valuation
}

fn consts_to_val(v: &[bool]) -> BTreeSet<Term> {
    let mut set = BTreeSet::new();

    for (idx, val) in v.iter().enumerate() {
        if *val {
            set.insert(Term::Cst(idx));
        }
    }

    set
}

pub fn slp_to_valuation(slp: &SLP) -> Valuation {
    let mut valuation = Valuation::new();

    for v in 0..slp.num_of_variables() {
        let val = consts_to_val(&slp[v]);

        valuation.insert(Term::Var(v), val);
    }

    valuation
}

// v1 \sqsubseteq v2
// (a, b) \in subvaluation <=> v1[b] = v1[a]
pub fn is_subvaluation(v1: &Valuation, v2: &Valuation) -> Option<Vec<(Term, Term)>> {
    let mut v2_to_v1 = Vec::new();
    for (b, val) in v1 {
        if let Some((a, _)) = v2.iter().find(|(_, v)| v == &val) {
            v2_to_v1.push((a.clone(), b.clone()));
        } else {
            return None;
        }
    }
    Some(v2_to_v1)
}

pub fn is_strict_subvaluation(v1: &Valuation, v2: &Valuation) -> bool {
    for (var, val) in v1 {
        if val != v2.get(var).unwrap() {
            return false;
        }
    }
    true
}
