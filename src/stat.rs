use crate::reorder::{self, Pebble};
use std::collections::BTreeSet;

pub struct Stat {
    pub nr_xors: usize,
    pub nr_memacc: usize,
    pub nr_page_transfer: usize,
    pub required_cache_capacity: usize,
    pub nr_variables: usize,
}

// https://en.wikipedia.org/wiki/Iverson_bracket
fn iverson(b: bool) -> usize {
    if b {
        1
    } else {
        0
    }
}

pub fn analyze(program: &Vec<(Pebble, Vec<Pebble>)>) -> Stat {
    let mut stat = Stat {
        nr_xors: 0,
        nr_memacc: 0,
        nr_page_transfer: 0,
        // nr_variables: 0,
        required_cache_capacity: 0,
        nr_variables: 0,
    };

    // page queue
    let mut ru = reorder::RecentlyUse::new();

    let mut variables = BTreeSet::<Pebble>::new();

    for (t, vars) in program {
        // t <- XOR(vars)
        assert!(vars.len() > 1);

        // t <- XOR(a, b, c)
        // => 2 xor since (a + b) + c
        // => 4 memory accesses t, a, b, c
        stat.nr_xors += vars.len() - 1;
        stat.nr_memacc += vars.len() + 1;

        for v in vars {
            if v.is_var() {
                variables.insert(v.clone());
            }

            // read access
            if ru.is_hot(v) {
                // cache hit
            } else {
                // check evict
                stat.nr_page_transfer += iverson(ru.len() >= 8);

                // load
                stat.nr_page_transfer += 1;
            }
            ru.access(v.clone());
        }
        assert!(t.is_var());
        variables.insert(t.clone());

        // write access
        if !ru.is_hot(t) {
            // check evict for allocate
            stat.nr_page_transfer += iverson(ru.len() >= 8);
        }
        ru.access(t.clone());
    }
    stat.nr_variables = variables.len();

    let mut cap = 1;
    loop {
        if check_runnable(&program, cap) {
            stat.required_cache_capacity = cap;
            return stat;
        }
        cap += 1;
    }
}

fn check_runnable(program: &Vec<(Pebble, Vec<Pebble>)>, capacity: usize) -> bool {
    let mut visited = BTreeSet::<Pebble>::new();
    let mut ru = reorder::RecentlyUse::new();

    // println!("trying... {}", capacity);

    for (t, vars) in program {
        for v in vars {
            if visited.contains(v) && !ru.is_in(v, capacity) {
                return false;
            }
            ru.access(v.clone());
            visited.insert(v.clone());
        }
        ru.access(t.clone());
        visited.insert(t.clone());
    }

    true
}
