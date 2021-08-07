# Notice

* This is a repository for the research paper https://arxiv.org/abs/2108.02692
* The paper is author's version of a paper accepted at SC'21 https://sc21.supercomputing.org/
    * After it will be published, we will put a link to publisher's site here
* There is an anonymized repository https://github.com/sc2021anonym/slp-ec of this repository for the AD/AE phase of SC'21
    * that repository is prepared for submitting the paper because SC'21 adopted the double-blind review system.
    * Please refer this author's repository for the latest information (the anonymized repository was frozen).

All codes are very proof-of-concept.
I'm currently working to prepare a solid erasure coding library based on the paper and also prepare a (API) docmentation.

# For benchmarking ISA-L
Please see the note: [HOWTO_BENCHMARK_ISAL.md](HOWTO_BENCHMARK_ISAL.md)

# For benchmarking Our Library

We need Rust https://www.rust-lang.org/.

If you have not installed Rust, please see the official instruction: https://www.rust-lang.org/tools/install

## Build
```
$ git clone https://github.com/yuezato/xorslp_ec
$ cd xorslp_ec;
$ cargo build --release
$ ./target/release/xorslp_ec --help
xorslp_ec 0.1.0

USAGE:
    xorslp_ec [FLAGS] [OPTIONS]

FLAGS:
        --all-stat
    -h, --help           Prints help information
        --no-compress
        --stat-enc
    -V, --version        Prints version information

OPTIONS:
        --data-block <data-block>
        --enc-dec <enc-dec>...
        --loop-iter <loop-iter>
        --optimize-level <optimize-level>     [default: FusionSchedule]  [possible values: Nooptim, Fusion,
                                             FusionSchedule]
        --parity-block <parity-block>
        --stat-dec <stat-dec>...
```

## Benchmarking Encoding and Decoding
For **RS(10, 4)**, we only pass `--enc-dec`
```
$ ./target/release/xorslp_ec --enc-dec
[src/main.rs:119] &opt = Opt {
    data_block: None,
    parity_block: None,
    loop_iter: None,
    stat_enc: false,
    stat_dec: None,
    all_stat: false,
    enc_dec: Some(
        [],
    ),
    no_compress: false,
    optimize_level: FusionSchedule,
}
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
data size = 10158080
Encode: avg = 7188.129360261446 MB/s, sd = 405.67342012547016
Decode: avg = 5776.016738162249 MB/s, sd = 318.8590603497914
```

Using the `--parity-block` option, we can test **RS(10, 3)** as follows:
```
$ ./target/release/xorslp_ec --enc-dec --parity-block 3
[src/main.rs:119] &opt = Opt {
    data_block: None,
    parity_block: Some(
        3,
    ),
    loop_iter: None,
    stat_enc: false,
    stat_dec: None,
    all_stat: false,
    enc_dec: Some(
        [],
    ),
    no_compress: false,
    optimize_level: FusionSchedule,
}
Benchmarking of Encoding & Decoding (with [2, 4, 5])
data size = 10158080
Encode: avg = 9672.005039520893 MB/s, sd = 469.68358550063635
Decode: avg = 7773.8606985381575 MB/s, sd = 396.7454904127163
```

Using the `--data-block` option, we can test **RS(9, 3)** as follows:
```
$ ./target/release/xorslp_ec --enc-dec --data-block 9
[src/main.rs:119] &opt = Opt {
    data_block: Some(
        9,
    ),
    parity_block: None,
    loop_iter: None,
    stat_enc: false,
    stat_dec: None,
    all_stat: false,
    enc_dec: Some(
        [],
    ),
    no_compress: false,
    optimize_level: FusionSchedule,
}
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
data size = 10027008
Encode: avg = 7040.0763688192665 MB/s, sd = 355.0680770006292
Decode: avg = 5839.390800319102 MB/s, sd = 303.60659053528127
```

We can `--data-block` and `--parity-block` at the same time.
For example, we can test **RS(8, 2)** as follows:
```
./target/release/xorslp_ec --enc-dec --data-block 8 --parity-block 2
[src/main.rs:122] &opt = Opt {
    data_block: Some(
        8,
    ),
    parity_block: Some(
        2,
    ),
    loop_iter: None,
    stat_enc: false,
    stat_dec: None,
    all_stat: false,
    enc_dec: Some(
        [],
    ),
    no_compress: false,
    optimize_level: FusionSchedule,
    cache_estimate: false,
}
block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4])
data size = 10354688
[src/main.rs:285] tmp_pebbles = 20
Encode: avg = 18992.380568566114 MB/s, sd = 1411.7205078800941
Decode: avg = 14975.864223554834 MB/s, sd = 992.8091275687873
```

## Obtain statistis for compressing, fusioning, scheduling
For the encoding SLP,
```
$ ./target/release/xorslp_ec --stat-enc
[src/main.rs:119] &opt = Opt {
    data_block: None,
    parity_block: None,
    loop_iter: None,
    stat_enc: true,
    stat_dec: None,
    all_stat: false,
    enc_dec: None,
    no_compress: false,
    optimize_level: FusionSchedule,
}
Statistics for Encoding
[WithOUT comp.] XOR_NUM = 890, BASE_MEM_NUM = 2670, FUSIONED_MEM_NUM = 954, BASE_TRANSFER = 1868, FUSIONED_TRANSFER = 1868, SCHEDULED_TRANSFER = 1698
[With comp.] XOR_NUM = 418, BASE_MEM_NUM = 1254, FUSIONED_MEM_NUM = 766, BASE_TRANSFER = 1782, FUSIONED_TRANSFER = 1300, SCHEDULED_TRANSFER = 1010
```

For the decoding SLPs,
```
./target/release/xorslp_ec --stat-dec 2 4 5 6
[src/main.rs:119] &opt = Opt {
   data_block: None,
   parity_block: None,
   loop_iter: None,
   stat_enc: false,
   stat_dec: Some(
       [
           2,
           4,
           5,
           6,
       ],
   ),
   all_stat: false,
   enc_dec: None,
   no_compress: false,
   optimize_level: FusionSchedule,
}
Statistics for Decoding: [2, 4, 5, 6]
[WithOUT comp.] XOR_NUM = 1368, BASE_MEM_NUM = 4104, FUSIONED_MEM_NUM = 1432, BASE_TRANSFER = 2824, FUSIONED_TRANSFER = 2824, SCHEDULED_TRANSFER = 2620
[With comp.] XOR_NUM = 519, BASE_MEM_NUM = 1557, FUSIONED_MEM_NUM = 965, BASE_TRANSFER = 2223, FUSIONED_TRANSFER = 1659, SCHEDULED_TRANSFER = 1247
```

We can obtain all the statistics by the one command
```
$ ./target/release/xorslp_ec --all-stat
[src/main.rs:119] &opt = Opt {
    data_block: None,
    parity_block: None,
    loop_iter: None,
    stat_enc: false,
    stat_dec: None,
    all_stat: true,
    enc_dec: None,
    no_compress: false,
    optimize_level: FusionSchedule,
}
Dump All Statistics for Encoding and Decoding Programs
Enc: [WithOUT comp.] XOR_NUM = 890, BASE_MEM_NUM = 2670, FUSIONED_MEM_NUM = 954, BASE_TRANSFER = 1868, FUSIONED_TRANSFER = 1868, SCHEDULED_TRANSFER = 1698
[With comp.] XOR_NUM = 418, BASE_MEM_NUM = 1254, FUSIONED_MEM_NUM = 766, BASE_TRANSFER = 1782, FUSIONED_TRANSFER = 1300, SCHEDULED_TRANSFER = 1010
Dec [0, 1, 2, 3]:[WithOUT comp.] XOR_NUM = 1164, BASE_MEM_NUM = 3492, FUSIONED_MEM_NUM = 1228, BASE_TRANSFER = 2416, FUSIONED_TRANSFER = 2416, SCHEDULED_TRANSFER = 2218
[With comp.] XOR_NUM = 503, BASE_MEM_NUM = 1509, FUSIONED_MEM_NUM = 915, BASE_TRANSFER = 2165, FUSIONED_TRANSFER = 1588, SCHEDULED_TRANSFER = 1204
Dec [0, 1, 2, 4]:[WithOUT comp.] XOR_NUM = 1196, BASE_MEM_NUM = 3588, FUSIONED_MEM_NUM = 1260, BASE_TRANSFER = 2480, FUSIONED_TRANSFER = 2480, SCHEDULED_TRANSFER = 2274
[With comp.] XOR_NUM = 511, BASE_MEM_NUM = 1533, FUSIONED_MEM_NUM = 935, BASE_TRANSFER = 2209, FUSIONED_TRANSFER = 1612, SCHEDULED_TRANSFER = 1243
Dec [0, 1, 2, 5]:[WithOUT comp.] XOR_NUM = 1186, BASE_MEM_NUM = 3558, FUSIONED_MEM_NUM = 1250, BASE_TRANSFER = 2460, FUSIONED_TRANSFER = 2460, SCHEDULED_TRANSFER = 2260
...
```
