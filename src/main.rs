use xorslp_ec::for_benchmark;
use xorslp_ec::reorder::Pebble;
use xorslp_ec::rsv_bitmatrix;
use xorslp_ec::run;
use xorslp_ec::slp;
use xorslp_ec::vandermonde;
use xorslp_ec::Parameter;

use std::time::Instant;

extern crate itertools;
use itertools::Itertools;

extern crate structopt;
use structopt::StructOpt;

extern crate clap;
use clap::arg_enum;
arg_enum! {
    #[derive(Debug, Copy, Clone)]
    enum OptimizeLevel {
        Nooptim,
        Fusion,
        FusionSchedule,
    }
}

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    data_block: Option<usize>,

    #[structopt(long)]
    parity_block: Option<usize>,

    #[structopt(long)]
    loop_iter: Option<usize>,

    #[structopt(long)]
    stat_enc: bool,

    #[structopt(long)]
    stat_dec: Option<Vec<usize>>,

    #[structopt(long)]
    all_stat: bool,

    #[structopt(long)]
    enc_dec: Option<Vec<usize>>, // parameter for removing blocks

    #[structopt(long)]
    no_compress: bool,

    #[structopt(long,
                possible_values = &OptimizeLevel::variants(),
                case_insensitive = true, default_value="FusionSchedule")]
    optimize_level: OptimizeLevel, // defualt full optimization

    #[structopt(long)]
    cache_estimate: bool,

    #[structopt(long)]
    compare_compress: bool,

    #[structopt(long)]
    stat_sec75: bool,
}

fn mean(vs: &[f64]) -> f64 {
    let sum: f64 = vs.iter().sum();
    sum / vs.len() as f64
}

fn sd(v: &[f64], mean: f64) -> f64 {
    let mut sd: f64 = 0.0;
    for e in v {
        sd += (*e - mean) * (*e - mean);
    }
    sd /= v.len() as f64;
    sd.sqrt()
}

fn avg_throughput(prefix: &str, times: &[f64], data_size: usize) {
    let throughputs: Vec<_> = times.iter().map(|e| (data_size as f64) / e).collect();
    let m = mean(&throughputs);
    let sd = sd(&throughputs, m);
    println!("{}: avg = {} MB/s, sd = {}", prefix, m, sd);
}

fn ceilup(data_size: usize, unit: usize) -> usize {
    if data_size % unit == 0 {
        data_size
    } else {
        ((data_size + unit) / unit) * unit
    }
}

fn optimize_program(
    slp: &slp::SLP,
    compress: bool,
    level: OptimizeLevel,
) -> for_benchmark::PebbleProgram {
    let graph = if compress {
        for_benchmark::xor_repair(&slp)
    } else {
        slp.to_trivial_graph()
    };

    match level {
        OptimizeLevel::Nooptim => {
            let (_, program) = for_benchmark::graph_analyze(&slp, &graph);
            program
        }
        OptimizeLevel::Fusion => {
            let (_, program) = for_benchmark::bench_fusion(&slp, &graph);
            program
        }
        OptimizeLevel::FusionSchedule => {
            let (_, _, _, _, program) = for_benchmark::bench_pebble(&slp, &graph);
            program
        }
    }
}

fn main() {
    let opt = Opt::from_args();

    // dbg!(&opt);
    println!("Block size = {}", xorslp_ec::BLOCK_SIZE_PER_ITER);

    let loop_iter = opt.loop_iter.unwrap_or(1000);
    let nr_data_block = opt.data_block.unwrap_or(10);
    let nr_parity_block = opt.parity_block.unwrap_or(4);
    let rs_parameter = Parameter {
        nr_data_block,
        nr_parity_block,
    };

    // let enc = vandermonde::rsv(nr_data_block, nr_parity_block);
    let enc = vandermonde::isa_rsv(nr_data_block, nr_parity_block);
    /*
    for i in 0..enc.height() {
        for j in 0..enc.width() {
            use xorslp_ec::fin_field::FiniteField;
            print!("{} ", enc[i][j].to_string());
        }
        println!("");
    }
     */

    let bitmatrix_enc = rsv_bitmatrix::matrix_to_bitmatrix(&enc);
    let enc_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_enc);

    // enc_slp.pprint();

    if opt.stat_sec75 {
        println!("Statistics for Encoding");
        xorslp_ec::comparison::sec75_stat(&enc_slp);

        println!("Statistics for Decoding");
        let mut inv = enc;
        inv.drop_rows([2, 4, 5, 6].to_vec());
        let inv = inv.inverse().unwrap();
        let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
        let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);
        xorslp_ec::comparison::sec75_stat(&inv_slp);

        return;
    }

    if opt.stat_enc {
        println!("Statistics for Encoding");

        xorslp_ec::comparison::all_stat(&enc_slp, false); // without compression
        xorslp_ec::comparison::all_stat(&enc_slp, true); // with compression

        return;
    }

    if let Some(remove) = opt.stat_dec {
        if nr_parity_block != remove.len() {
            println!(
                "[Error] Please Pass {} blocks (now passed {} blocks)",
                nr_parity_block,
                remove.len()
            );
            return;
        }

        println!("Statistics for Decoding: {:?}", remove);
        let mut inv = enc;
        inv.drop_rows(remove);
        let inv = inv.inverse().unwrap();
        let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
        let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);

        xorslp_ec::comparison::all_stat(&inv_slp, false); // without compression
        xorslp_ec::comparison::all_stat(&inv_slp, true); // with compression
        return;
    }

    if opt.compare_compress {
        println!("Dump All Statistics about Compression for Encoding and Decoding Programs");

        println!("Enc: ");
        xorslp_ec::comparison::compress_stat(&enc_slp);

        for it in (0..(nr_data_block + nr_parity_block)).combinations(nr_parity_block) {
            let remove: Vec<usize> = it.to_vec();
            let mut tmp = enc.clone();
            tmp.drop_rows(remove.clone());
            let inv = tmp.inverse().unwrap();
            let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
            let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);

            println!("Dec {:?}:", remove);
            xorslp_ec::comparison::compress_stat(&inv_slp);
        }
        return;
    }

    if opt.all_stat {
        println!("Dump All Statistics for Encoding and Decoding Programs");

        println!("Enc: ");
        xorslp_ec::comparison::all_stat(&enc_slp, false); // without compression
        xorslp_ec::comparison::all_stat(&enc_slp, true); // with compression

        for it in (0..(nr_data_block + nr_parity_block)).combinations(nr_parity_block) {
            let remove: Vec<usize> = it.to_vec();
            let mut tmp = enc.clone();
            tmp.drop_rows(remove.clone());
            let inv = tmp.inverse().unwrap();
            let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
            let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);

            println!("Dec {:?}:", remove);
            xorslp_ec::comparison::all_stat(&inv_slp, false);
            xorslp_ec::comparison::all_stat(&inv_slp, true);
        }
        return;
    }

    let remove = opt.enc_dec.unwrap();
    let remove = if !remove.is_empty() {
        remove
    } else if nr_parity_block <= 4 {
        let mut tmp = vec![2, 4, 5, 6];
        tmp.truncate(nr_parity_block);
        tmp
    } else {
        unreachable!("Please pass blocks to be erased")
    };

    println!("Benchmarking of Encoding & Decoding (with {:?})", remove);

    let mut inv = enc;
    inv.drop_rows(remove.clone());
    let inv = inv.inverse().unwrap();

    let bitmatrix_inv = rsv_bitmatrix::matrix_to_bitmatrix(&inv);
    let inv_slp = slp::SLP::build_from_bitmatrix_not_depending_variables(&bitmatrix_inv);

    let compress = !opt.no_compress; // default compress

    let level = opt.optimize_level;

    let enc_shrinked = for_benchmark::shrink(&enc_slp);
    let enc_program = optimize_program(&enc_shrinked, compress, level);

    let dec_shrinked = for_benchmark::shrink(&inv_slp);
    let dec_shrinked = dec_shrinked;
    let dec_program = optimize_program(&dec_shrinked, compress, level);

    {
        let enc_program: Vec<(Pebble, &[Pebble])> = enc_program
            .iter()
            .map(|(a, b)| (a.clone(), &b[..]))
            .collect();

        let dec_program: Vec<(Pebble, &[Pebble])> = dec_program
            .iter()
            .map(|(a, b)| (a.clone(), &b[..]))
            .collect();

        let data_size = ceilup(
            10_000_000,
            // 9830400,
            // xorslp_ec::BLOCK_SIZE_PER_ITER * (nr_data_block * 8)
            4096 * (nr_data_block * 8),
        );
        let data_size = if cfg!(feature = "4096_align") {
            data_size
        } else {
            data_size + xorslp_ec::BLOCK_SIZE_PER_ITER * (nr_data_block * 8)
        };

        // println!("data size = {}", data_size);

        let mut enc_durations = Vec::new();
        let mut dec_durations = Vec::new();

        let mut fixed_array = run::PageAlignedArray::new(data_size).unwrap();
        xorslp_ec::fill_by_random(fixed_array.as_mut_slice());

        let mut inputs = Vec::new();
        for i in 0..nr_data_block {
            inputs.push(fixed_array.split(nr_data_block)[i].to_vec());
        }

        let input = fixed_array.split(nr_data_block * 8);
        let width = input[0].len();

        let to_store = run::PageAlignedArray::new(width * nr_parity_block * 8).unwrap();
        let output = to_store.split(nr_parity_block * 8);

        let required_pebbles = std::cmp::max(
            run::required_pebbles(&enc_program),
            run::required_pebbles(&dec_program),
        );
        let tmp_pebbles = required_pebbles - nr_parity_block * 8;

        let for_tmp =
            run::PageAlignedArray::new(xorslp_ec::BLOCK_SIZE_PER_ITER * tmp_pebbles).unwrap();
        let tmp = for_tmp.split(tmp_pebbles);

        let for_decode = run::PageAlignedArray::new(width * nr_parity_block * 8).unwrap();
        let decode = for_decode.split(nr_parity_block * 8);
        let decoded = for_decode.split(nr_parity_block);

        let (enc_program, dec_program) = if !opt.cache_estimate {
            (
                run::compile(rs_parameter, &enc_program),
                run::compile(rs_parameter, &dec_program),
            )
        } else {
            (
                run::estimate_compile(&enc_program),
                run::estimate_compile(&dec_program),
            )
        };

        for _ in 0..loop_iter {
            let now = Instant::now();
            run::run_program(
                &run::combine_constant_target_tmp(&input, &output, &tmp),
                width / xorslp_ec::BLOCK_SIZE_PER_ITER,
                &enc_program,
            );
            enc_durations.push(now.elapsed().as_micros() as f64);

            let original = fixed_array.split(nr_data_block);

            if !opt.cache_estimate {
                for i in 0..nr_data_block {
                    assert!(inputs[i] == original[i]);
                }
            }

            // remove and add parities
            let mut decode_input = xorslp_ec::drop8(input.clone(), &remove);
            decode_input.append(&mut output.clone());

            let now = Instant::now();
            run::run_program(
                &run::combine_constant_target_tmp(&decode_input, &decode, &tmp),
                width / xorslp_ec::BLOCK_SIZE_PER_ITER,
                &dec_program,
            );
            dec_durations.push(now.elapsed().as_micros() as f64);

            if !opt.cache_estimate {
                for i in 0..nr_parity_block {
                    assert!(inputs[remove[i]] == original[remove[i]]);
                    assert!(decoded[i] == original[remove[i]]);
                }
            }
        }

        avg_throughput("Encode", &enc_durations, data_size);
        avg_throughput("Decode", &dec_durations, data_size);
    }
}
