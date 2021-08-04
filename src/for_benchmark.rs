use crate::fast_repair::SortOrder;
use crate::fusion;
use crate::reorder::{self, Pebble, Strategy};
use crate::reorder2;
use crate::repair;
use crate::slp::SLP;
use crate::stat::*;
use crate::validation;
use crate::xor_repair;
use crate::Term;
use crate::*;

#[derive(Debug, Clone, Copy)]
pub enum XorDirection {
    None,
    Forward,
    Backward,
}

pub type PebbleProgram = Vec<(Pebble, Vec<Pebble>)>;

fn rename(valuation: &Valuation, program: &[(Pebble, Vec<Pebble>)]) -> PebbleProgram {
    let mapping = validation::is_subvaluation(
        &valuation,
        &validation::pebble_computation_to_valuation(program),
    );

    let mapping: Vec<(usize, usize)> = mapping
        .unwrap()
        .iter()
        .map(|(a, b)| (a.var_to_usize().unwrap(), b.var_to_usize().unwrap()))
        .collect();

    let renaming = renaming::mapping_to_rewriting(&mapping);
    let renamed =
        renaming::rename_multislp_by(&renaming, &reorder::pebble_slp_to_term_slp(program));

    assert!(validation::is_strict_subvaluation(
        &valuation,
        &validation::term_computation_to_valuation(&renamed)
    ));

    reorder::term_slp_to_pebble_slp(&renamed)
}

pub fn shrink(original_slp: &SLP) -> SLP {
    optimize_slp::step1(original_slp)
}

pub fn to_ssa(shrinked_slp: &SLP) -> Graph {
    let shrinked_program = shrinked_slp.to_trivial_graph();

    fusion::slp_to_ssa(&shrinked_program)
}

pub fn repair(shrinked_slp: &SLP) -> Graph {
    fast_repair::run_repair2(&shrinked_slp, SortOrder::LexSmall)
}

pub fn xor_repair(shrinked_slp: &SLP) -> Graph {
    xor_repair::run_xor_repair_reverse(&shrinked_slp, SortOrder::LexSmall)
}

pub fn graph_analyze(shrinked_slp: &SLP, graph: &Graph) -> (Stat, PebbleProgram) {
    let program: Vec<(Term, Vec<Term>)> = graph_to_multiterm_slp(&graph);
    let program: Vec<(Pebble, Vec<Pebble>)> = reorder::term_slp_to_pebble_slp(&program);
    let stat = stat::analyze(&program);

    let shrinked_valuation = validation::slp_to_valuation(&shrinked_slp);
    let renamed = rename(&shrinked_valuation, &program);

    (stat, renamed)
}

pub fn bench_fusion(shrinked_slp: &SLP, graph: &Graph) -> (Stat, PebbleProgram) {
    let graph = if fusion::is_ssa(graph) {
        graph.clone()
    } else {
        fusion::slp_to_ssa(graph)
    };

    let evaluated = repair::evaluate_program(&graph);

    let targets: Vec<Term> = repair::realizes(&evaluated, &shrinked_slp)
        .unwrap()
        .iter()
        .map(|(a, _)| Term::Var(*a))
        .collect();

    let multislp = fusion::graph_to_multislp_by_fusion(graph.to_vec(), &targets);
    let multislp: Vec<(Term, Vec<Term>)> = multislp_to_multiterm_slp(&multislp);
    let multislp: Vec<(Pebble, Vec<Pebble>)> = reorder::term_slp_to_pebble_slp(&multislp);
    let multislp_stat = stat::analyze(&multislp);

    let shrinked_valuation = validation::slp_to_valuation(&shrinked_slp);
    let renamed = rename(&shrinked_valuation, &multislp);

    (multislp_stat, renamed)
}

pub fn bench_pebble(shrinked_slp: &SLP, graph: &Graph) -> (Stat, Stat, Stat, Stat, PebbleProgram) {
    let graph = if fusion::is_ssa(graph) {
        graph.clone()
    } else {
        fusion::slp_to_ssa(graph)
    };
    let evaluated = repair::evaluate_program(&graph);

    let targets: Vec<Term> = repair::realizes(&evaluated, &shrinked_slp)
        .unwrap()
        .iter()
        .map(|(a, _)| Term::Var(*a))
        .collect();

    let multislp = fusion::graph_to_multislp_by_fusion(graph.to_vec(), &targets);

    let nr_constants = shrinked_slp.num_of_original_constants();

    let scheduled1 =
        reorder::deal_multislp(&multislp, nr_constants, targets.clone(), Strategy::UseLRU);
    let scheduled2 =
        reorder::deal_multislp(&multislp, nr_constants, targets.clone(), Strategy::UseMRU);
    let scheduled3 =
        reorder2::deal_multislp2(&multislp, nr_constants, targets.clone(), Strategy::UseLRU);
    let scheduled4 = reorder2::deal_multislp2(&multislp, nr_constants, targets, Strategy::UseMRU);

    let schedule_stat1 = stat::analyze(&scheduled1);
    let schedule_stat2 = stat::analyze(&scheduled2);
    let schedule_stat3 = stat::analyze(&scheduled3);
    let schedule_stat4 = stat::analyze(&scheduled4);

    let shrinked_valuation = validation::slp_to_valuation(&shrinked_slp);
    let renamed = rename(
        &shrinked_valuation,
        if cfg!(feature = "dfs_sched") {
            // dbg!("dfs_sched");
            &scheduled2
        } else if cfg!(feature = "bottomup_sched") {
            // dbg!("bottomup sched");
            &scheduled4
        } else {
            &scheduled2
        },
    );

    (
        schedule_stat1,
        schedule_stat2,
        schedule_stat3,
        schedule_stat4,
        renamed,
    )
}
