use crate::slp::SLP;
use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pebble {
    Const(usize),
    Var(usize),
}

impl std::fmt::Display for Pebble {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pebble::Var(v) => write!(f, "V{}", v),
            Pebble::Const(v) => write!(f, "C{}", v),
        }
    }
}

impl Pebble {
    pub fn is_var(&self) -> bool {
        match self {
            Pebble::Const(_) => false,
            Pebble::Var(_) => true,
        }
    }
    pub fn from_var(&self) -> Option<usize> {
        match self {
            Pebble::Const(_) => None,
            Pebble::Var(v) => Some(*v),
        }
    }
    pub fn from_const(&self) -> Option<usize> {
        match self {
            Pebble::Const(v) => Some(*v),
            Pebble::Var(_) => None,
        }
    }
    pub fn to_term(&self) -> Term {
        match self {
            Pebble::Var(v) => Term::Var(*v),
            Pebble::Const(v) => Term::Cst(*v),
        }
    }
    pub fn from_term(t: &Term) -> Pebble {
        match t {
            Term::Var(v) => Pebble::Var(*v),
            Term::Cst(v) => Pebble::Const(*v),
        }
    }
}

pub struct GenericRecentlyUse<T> {
    // inner[0] is least recently used
    // inner[inner.len()-1] is most recently used
    inner: Vec<T>,
}

impl<T> GenericRecentlyUse<T> {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl<T> Default for GenericRecentlyUse<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(clippy::len_without_is_empty)]
impl<T> GenericRecentlyUse<T>
where
    T: PartialEq + Clone,
{
    pub fn access(&mut self, p: T) {
        if let Some(idx) = self.inner.iter().position(|x| x == &p) {
            self.inner.remove(idx);
        }
        self.inner.push(p);
    }

    // self.get_pos() => 0 means most recently used
    // self.get_pos() => inner.len means least recently used
    pub fn get_pos(&self, p: &T) -> Option<usize> {
        self.inner
            .iter()
            .position(|x| x == p)
            .map(|pos| (self.inner.len() - 1) - pos)
    }

    // elements_within(...)[0] is more recent than [1]
    // [1] is more recen than [2]
    // ...
    pub fn elements_within(&self, within: usize) -> Vec<T> {
        let l = self.inner.len();
        let start = l.saturating_sub(within);
        self.inner[start..l].iter().rev().cloned().collect()
    }

    pub fn is_hot(&self, element: &T) -> bool {
        self.is_in(element, PEBBLE_NUM)
    }

    pub fn is_in(&self, element: &T, within: usize) -> bool {
        if let Some(pos) = self.get_pos(element) {
            pos < within
        } else {
            false
        }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

pub type RecentlyUse = GenericRecentlyUse<Pebble>;

pub enum Strategy {
    UseLRU,
    UseMRU,
}

pub struct Alloc {
    strategy: Strategy,
    ru: RecentlyUse,
    frees: BTreeSet<Pebble>,
    mapping: BTreeMap<Term, Pebble>,
    fresh: usize,
    targets: Vec<Term>,
}

impl Alloc {
    pub fn new(num_of_constants: usize, targets: Vec<Term>, strategy: Strategy) -> Self {
        let mut mapping = BTreeMap::new();
        for i in 0..num_of_constants {
            mapping.insert(Term::Cst(i), Pebble::Const(i));
        }

        Self {
            strategy,
            ru: RecentlyUse::new(),
            frees: BTreeSet::new(),
            mapping,
            fresh: 0,
            targets,
        }
    }

    pub fn try_release(&mut self, t: &Term) -> bool {
        if t.is_var() && !self.targets.contains(t) {
            let pebble = self.get(t).unwrap();
            self.frees.insert(pebble);
            self.mapping.remove(t);
            true
        } else {
            false
        }
    }

    pub fn access(&mut self, t: &Term) {
        self.ru.access(self.get(t).unwrap());
    }

    // return least recent within limit
    fn search_within(&self, limit: usize) -> Option<Pebble> {
        match self.strategy {
            Strategy::UseMRU => self
                .ru
                .elements_within(limit)
                .iter()
                .find(|p| self.frees.contains(p))
                .cloned(),
            Strategy::UseLRU => self
                .ru
                .elements_within(limit)
                .iter()
                .rev()
                .find(|p| self.frees.contains(p))
                .cloned(),
        }
    }

    pub fn is_hot(&self, t: &Term) -> bool {
        let pebble = self.get(t).unwrap();
        self.ru.is_hot(&pebble)
    }

    pub fn index(&self, t: &Term) -> Option<usize> {
        let pebble = self.get(t).unwrap();
        self.ru.get_pos(&pebble)
    }

    pub fn assign(&mut self, target: &Term) -> Pebble {
        if self.frees.is_empty() {
            // there is no free pebbles
            let pebble = Pebble::Var(self.fresh);
            self.mapping.insert(target.clone(), pebble.clone());
            self.fresh += 1;
            return pebble;
        }

        let pebble = if let Some(l1_pebble) = self.search_within(PEBBLE_NUM) {
            l1_pebble
        } else {
            self.search_within(self.ru.len()).unwrap()
        };
        self.frees.remove(&pebble);
        self.mapping.insert(target.clone(), pebble.clone());
        pebble
    }

    pub fn get(&self, t: &Term) -> Option<Pebble> {
        self.mapping.get(t).cloned()
    }
}

pub fn multi_slp_to_pebble_computation(slp: &MultiSLP) -> Vec<(Pebble, Vec<Pebble>)> {
    let mut v = Vec::new();

    for (t, seq) in slp {
        let seq: Vec<_> = seq.iter().map(|t| Pebble::from_term(t)).collect();
        v.push((Pebble::from_term(t), seq));
    }

    v
}

pub fn cache_misses_multislp_computation(slp: &MultiSLP) -> usize {
    cache_misses_pebble_computation(&multi_slp_to_pebble_computation(slp))
}

pub fn cache_misses_graph(graph: &Graph) -> usize {
    let slp = graph_to_multislp(graph);
    cache_misses_pebble_computation(&multi_slp_to_pebble_computation(&slp))
}

pub fn cache_misses_pebble_computation(computation: &Vec<(Pebble, Vec<Pebble>)>) -> usize {
    let mut ru = RecentlyUse::new();
    let mut cache_miss = 0;

    for (target, seq) in computation {
        for v in seq {
            if !ru.is_hot(v) {
                cache_miss += 1;
            }
            ru.access(v.clone());
        }

        if !ru.is_hot(target) {
            cache_miss += 1;
        }
        ru.access(target.clone());
    }

    cache_miss
}

fn assign(
    out_degrees: &mut BTreeMap<Term, usize>,
    computation: &Vec<(Term, Vec<Term>)>,
    alloc: &mut Alloc,
) -> Vec<(Pebble, Vec<Pebble>)> {
    let mut real_computation = Vec::new();

    for (target, seq) in computation {
        let mut pebbles = Vec::new();

        for v in seq {
            pebbles.push(alloc.get(v).unwrap());
            alloc.access(v);

            let deg = out_degrees.get_mut(v).unwrap();
            *deg -= 1;

            if *deg == 0 {
                alloc.try_release(v);
            }
        }

        let pebble = alloc.assign(&target);
        alloc.access(&target);

        real_computation.push((pebble, pebbles));
    }

    real_computation
}

pub fn make_outdegrees(dag: &DAG) -> BTreeMap<Term, usize> {
    let mut mapping = BTreeMap::new();

    for (t, v) in dag {
        mapping.insert(t.clone(), 0);

        for t in v {
            mapping.insert(t.clone(), 0);
        }
    }

    for v in dag.values() {
        for t in v {
            let r = mapping.get_mut(t).unwrap();
            *r += 1;
        }
    }

    mapping
}

pub fn make_indegrees(dag: &DAG) -> BTreeMap<Term, usize> {
    let mut mapping = BTreeMap::new();

    for (t, v) in dag {
        mapping.insert(t.clone(), 0);

        for t in v {
            mapping.insert(t.clone(), 0);
        }
    }

    for t in dag.keys() {
        let r = mapping.get_mut(t).unwrap();
        *r += 1;
    }

    mapping
}

pub fn dag_visit(dag: &DAG, cur: &Term, visited: &mut BTreeSet<Term>) -> Vec<(Term, Vec<Term>)> {
    if dag.get(cur).is_none() {
        return Vec::new();
    }

    let defs: Vec<_> = dag.get(cur).unwrap().iter().cloned().collect();

    let mut order = Vec::new();
    for t in &defs {
        if !visited.contains(t) {
            order.append(&mut dag_visit(dag, t, visited));
            visited.insert(t.clone());
        }
    }
    order.push((cur.clone(), defs));
    visited.insert(cur.clone());

    order
}

pub fn multislp_to_dag(slp: &MultiSLP) -> DAG {
    let mut dag = DAG::new();

    for (t, children) in slp {
        dag.insert(t.clone(), children.clone());
    }

    dag
}

pub fn deal_multislp(
    slp: &MultiSLP,
    num_of_constants: usize,
    targets: Vec<Term>,
    strategy: Strategy,
) -> Vec<(Pebble, Vec<Pebble>)> {
    let dag = multislp_to_dag(slp);

    let mut alloc = Alloc::new(num_of_constants, targets, strategy);
    let mut out_degrees: BTreeMap<Term, usize> = make_outdegrees(&dag);

    let root_nodes: Vec<Term> = dag
        .iter()
        .filter(|(t, _)| out_degrees.get(t).unwrap() == &0)
        .map(|(t, _)| t)
        .cloned()
        .collect();

    let mut visited = BTreeSet::new();
    let mut computations = Vec::new();

    for root in root_nodes {
        let arguments = dag_visit(&dag, &root, &mut visited);
        let mut computation = assign(&mut out_degrees, &arguments, &mut alloc);
        computations.append(&mut computation);
    }

    computations
}

pub fn pebble_slp_to_term_slp(slp: &[(Pebble, Vec<Pebble>)]) -> Vec<(Term, Vec<Term>)> {
    let mut new_slp = Vec::new();

    for (p, body) in slp {
        let body: Vec<Term> = body.iter().map(|p| p.to_term()).collect();
        new_slp.push((p.to_term(), body));
    }

    new_slp
}

pub fn term_slp_to_pebble_slp(slp: &[(Term, Vec<Term>)]) -> Vec<(Pebble, Vec<Pebble>)> {
    let mut new_slp = Vec::new();

    for (p, body) in slp {
        let body: Vec<Pebble> = body.iter().map(|p| Pebble::from_term(p)).collect();
        new_slp.push((Pebble::from_term(p), body));
    }

    new_slp
}

pub fn finalize(
    original_slp: &SLP,
    program: &[(Term, Vec<Term>)],
    mapping1: &[(usize, usize)], // shrinked -> original
    mapping2: &[(usize, usize)], // optimized -> shrinked
) -> Vec<(Term, Vec<Term>)> {
    let mapping = optimize_slp::compose(&mapping2, &mapping1);
    let renaming = renaming::mapping_to_rewriting(&mapping);

    let mut program = renaming::rename_multislp_by(&renaming, &program);

    let not_shrinked: Vec<usize> = mapping1.iter().map(|(_, b)| *b).collect();

    for v in 0..original_slp.num_of_variables() {
        if !not_shrinked.contains(&v) {
            let value = &original_slp[v];
            debug_assert!(bitmatrix::popcount(value) == 1);
            let c = value.iter().position(|b| *b).unwrap();
            program.push((Term::Var(v), vec![Term::Cst(c)]));
        }
    }

    program
}
