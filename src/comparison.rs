use crate::for_benchmark;
use crate::slp;
use crate::slp::SLP;
use itertools::Itertools;

/*
#[derive(Debug, Clone, Copy)]
pub enum XorDirection {
    None,
    Forward,
    Backward,
}
    do_xor(&original_slp, XorDirection::None, SortOrder::LexSmall);
    do_xor(&original_slp, XorDirection::None, SortOrder::LexLarge);
    do_xor(&original_slp, XorDirection::Forward, SortOrder::LexSmall);
    do_xor(&original_slp, XorDirection::Backward, SortOrder::LexSmall);
    do_xor(&original_slp, XorDirection::Forward, SortOrder::LexLarge);
    do_xor(&original_slp, XorDirection::Backward, SortOrder::LexLarge);
*/

pub fn repair_comparison(slp: &SLP) {
    use crate::fast_repair::{self, SortOrder};
    use crate::xor_repair;

    let shrinked_slp = for_benchmark::shrink(slp);

    if slp.is_empty() {
        return;
    }

    let nr_original = shrinked_slp.to_trivial_graph().len();

    let nr_xor1 = fast_repair::run_repair2(&shrinked_slp, SortOrder::LexSmall).len();
    let nr_xor2 = fast_repair::run_repair2(&shrinked_slp, SortOrder::LexLarge).len();

    let nr_xor3 = xor_repair::run_xor_repair_forward(&shrinked_slp, SortOrder::LexSmall).len();
    let nr_xor4 = xor_repair::run_xor_repair_reverse(&shrinked_slp, SortOrder::LexSmall).len();

    let nr_xor5 = xor_repair::run_xor_repair_forward(&shrinked_slp, SortOrder::LexLarge).len();
    let nr_xor6 = xor_repair::run_xor_repair_reverse(&shrinked_slp, SortOrder::LexLarge).len();

    println!("Original = {} => R(<) = {}, R(>) = {}, X(<, <) = {}, X(<, >) = {}, X(>, <) = {}, X(>, >) = {}",
             nr_original, nr_xor1, nr_xor2, nr_xor3, nr_xor4, nr_xor5, nr_xor6);
}

pub fn compression_stat() {
    use crate::{rsv_bitmatrix, vandermonde};

    let enc = vandermonde::rsv(10, 4);
    let bitmatrix_enc = rsv_bitmatrix::matrix_to_bitmatrix(&enc);
    let enc_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_enc);

    print!("Enc: ");
    repair_comparison(&enc_slp);

    for it in (0..14).combinations(4) {
        let remove: Vec<usize> = it.to_vec();
        let mut tmp = enc.clone();
        tmp.drop_rows(remove.clone());
        let inv = tmp.inverse().unwrap();
        let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
        let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);

        print!("Dec {:?}:", remove);
        repair_comparison(&inv_slp);
    }
}

pub fn compress_stat(original_slp: &slp::SLP) {
    let shrinked_slp = for_benchmark::shrink(original_slp);

    if shrinked_slp.is_empty() {
        println!("This is a trivial case: We need no computation, and there is no statistics");
        return;
    }

    let slp = shrinked_slp.to_trivial_graph();
    let (stat, _) = for_benchmark::graph_analyze(&shrinked_slp, &slp);
    let slp_xor_num = stat.nr_xors;

    let repaired_slp = for_benchmark::repair(&shrinked_slp);
    let (stat, _) = for_benchmark::graph_analyze(&shrinked_slp, &repaired_slp);
    let repaired_xor_num = stat.nr_xors;

    let xor_repaired_slp = for_benchmark::xor_repair(&shrinked_slp);
    let (stat, _) = for_benchmark::graph_analyze(&shrinked_slp, &xor_repaired_slp);
    let xor_repaired_xor_num = stat.nr_xors;

    println!(
        "  [NoComp] #XOR = {}, [RePair] #XOR = {}, [XorRePair] #XOR = {}",
        slp_xor_num, repaired_xor_num, xor_repaired_xor_num
    );
}

pub fn all_stat(original_slp: &slp::SLP, compress: bool) {
    let shrinked_slp = for_benchmark::shrink(original_slp);

    if shrinked_slp.is_empty() {
        println!("This is a trivial case: We need no computation, and there is no statistics");
        return;
    }

    let slp = if compress {
        for_benchmark::xor_repair(&shrinked_slp)
    } else {
        shrinked_slp.to_trivial_graph()
    };

    let (orig_stat, _) = for_benchmark::graph_analyze(&shrinked_slp, &slp);
    let xor_num = orig_stat.nr_xors;
    let base_mem_num = orig_stat.nr_memacc;
    let naive_page = orig_stat.nr_page_transfer;

    let (fusion_stat, _) = for_benchmark::bench_fusion(&shrinked_slp, &slp);
    let mem_num = fusion_stat.nr_memacc;
    let fusioned_page = fusion_stat.nr_page_transfer;

    let (_, _, _, pebble_stat4, pebble_program) = for_benchmark::bench_pebble(&shrinked_slp, &slp);
    let scheduled_page = pebble_stat4.nr_page_transfer;

    assert!(xor_num == fusion_stat.nr_xors);
    assert!(xor_num == pebble_stat4.nr_xors);
    assert!(mem_num == pebble_stat4.nr_memacc);

    let comp_or_not = if compress {
        "With comp.".to_owned()
    } else {
        "WithOUT comp.".to_owned()
    };

    println!(
        "[{}] #XOR = {}, #MemAcc = {}, #[Fusioned]MemAcc = {},
  #[NoFusion]CacheTrans = {}, #[Fusioned]CacheTrans = {}, #[Fusioned&Scheduled]CacheTrans = {},
  #[NoFusion]Variables = {}, #[Fusioned]Variables = {}, #[Fusioned&Scheduled]Variables = {},
  #[NoFusion]Capacity = {}, #[Fusioned]Capacity = {}, #[Fusioned&Scheduled]Capacity = {},
  #Statements = {}",
        comp_or_not,
        xor_num,
        base_mem_num,
        mem_num,
        naive_page,
        fusioned_page,
        scheduled_page,
        orig_stat.nr_variables,
        fusion_stat.nr_variables,
        pebble_stat4.nr_variables,
        orig_stat.required_cache_capacity,
        fusion_stat.required_cache_capacity,
        pebble_stat4.required_cache_capacity,
        pebble_program.len()
    );
}

pub fn sec75_stat(original_slp: &slp::SLP) {
    let shrinked_slp = for_benchmark::shrink(original_slp);

    if shrinked_slp.is_empty() {
        println!("This is a trivial case: We need no computation, and there is no statistics");
        return;
    }

    // no compression
    let slp = shrinked_slp.to_trivial_graph();
    let (orig, _) = for_benchmark::graph_analyze(&shrinked_slp, &slp);

    let compressed = for_benchmark::xor_repair(&shrinked_slp);
    let (comp, _) = for_benchmark::graph_analyze(&shrinked_slp, &compressed);

    let (fusion, _) = for_benchmark::bench_fusion(&shrinked_slp, &compressed);

    let (_, _, _, sched, _) = for_benchmark::bench_pebble(&shrinked_slp, &compressed);

    println!("        P   Co(P)   Fu(Co(P))   Dfs(Fu(Co(P)))");
    println!(
        "#XOR {:4} {:7} {:11} {:16}",
        orig.nr_xors, comp.nr_xors, fusion.nr_xors, sched.nr_xors
    );
    println!(
        "#MEM {:4} {:7} {:11} {:16}",
        orig.nr_memacc, comp.nr_memacc, fusion.nr_memacc, sched.nr_memacc
    );
    println!(
        "NVar {:4} {:7} {:11} {:16}",
        orig.nr_variables, comp.nr_variables, fusion.nr_variables, sched.nr_variables
    );
    println!(
        "CCap {:4} {:7} {:11} {:16}",
        orig.required_cache_capacity,
        comp.required_cache_capacity,
        fusion.required_cache_capacity,
        sched.required_cache_capacity
    );
}
