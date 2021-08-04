This is a note to reproduce the results in Section 7.3–7.6.

# Raw logs
We put logs obtained by executing following commands in the AMD environment described in the paper into the directory `rawlogs`.

# Requirement
## Branch
We use the branch `for_ae` instead of the master branch:
```
$ git clone -b for_ae https://github.com/sc2021anonym/slp-ec/
$ cd slp-ec
$ git branch
* for_ae
```

## Directory
We perform all the below commands in the directory `reproducing`:
```
$ cd reproducing
$ ls
README.md			full_optimized.sh		summarize_cache.rb		throughput_sec75.sh
all_stat.log			rawlogs				summarize_comp_compare.rb	throughput_sec76.sh
comp_compare.log		sec76_isal.sh			summarize_memacc.rb		uncompressed_fusioned.sh
```

## Ruby
We use scripts written in [Ruby](https://www.ruby-lang.org/en/).

If it is not installed, please install it following the official installation instruction: https://www.ruby-lang.org/en/documentation/installation/.

I used the following version of **Ruby 3**:
```
$ ruby --version
ruby 3.0.1p64 (2021-04-05 revision 0fb782ee38) [x86_64-darwin19]
```

# Section 7.3

## First Table
To reproduce the first table in the paper, we use the following commands:
```
cargo build --release
./target/release/xorslp_ec --compare-compress > comp_stat.log
 ```
Since the command `--compare-compress` takes a long time,
we put the result file `comp_compare.log` in this directory.

Then, we use a Ruby script `summarize_comp_compare.rb` to summarize the results as follows:

```
$ ruby summarize_comp_compare.rb comp_compare.log (<- or, pass your file name)
Repair(P)/P = 42.0499782581751 %
XorRepair(P)/P = 40.855934574708606 %
```

## Second Table
To reproduce the second table in the paper, we use the following commands:
```
cargo build --release
./target/release/xorslp_ec --all-stat > stat.log
 ```
Since the command `--all-stat` takes a long time,
we put the result file `all_stat.log` in this directory.

Then, we use a Ruby script `summarize_memacc.rb` to summarize the result:
```
$ ruby summarize_memacc.rb all_stat.log
Co(P)/P = 40.855934574708606 %
Fu(P)/P = 35.11680821157586 %
Fu(Co(P))/Co(P) = 59.29160709043092 %
Fu(Co(P))/P = 24.084185162285948 %
```

## Third Table
We use a Ruby script `summarize_cache.rb` to summarize `all_stat.log`:
```
$ ruby summarize_cache.rb all_stat.log
<NVar>
  Co(P)/P = 1552.7014652014652 %
  Fu(P)/P = 100.0 %
  Fu(Co(P))/Co(P) = 38.93741063564638 %
  Dfs(Fu(Co(P)))/Co(P) = 24.388399485290822 %
</NVar>

<CCap>
  Co(P)/P = 498.84493303824564 %
  Fu(P)/P = 98.94273577904123 %
  Fu(Co(P))/Co(P) = 51.27606659783468 %
  Dfs(Fu(Co(P)))/Co(P) = 40.00431827796581 %
</CCap>
```

# Section 7.4

## First Table
To obtain the first table, please use a shell-script `uncompressed_fusioned.sh`:
```
$ ./uncompressed_fusioned.sh

<64block>
   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 1.98s
Block size = 64
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 857.3209410483786 MB/s, sd = 27.492976559754535
Decode: avg = 557.2278477740666 MB/s, sd = 20.227456863094563
</64block>

<128block>
   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 1.88s
Block size = 128
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 1669.2170401811557 MB/s, sd = 58.441506919289985
Decode: avg = 1075.2696059689035 MB/s, sd = 35.739666470507395
</128block>

...
```

## Second Table
To obtain the second table, please use a shell-script `full_optimized.sh`:
```
$ ./full_optimized.sh

<GREEDY SCHEDULING>
<64block>
+ cargo build --release --features '64block bottomup_sched'
    Finished release [optimized] target(s) in 0.02s
Block size = 64
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 2205.652280780655 MB/s, sd = 64.80537317420949
Decode: avg = 1585.5501069999607 MB/s, sd = 46.990673655364986
</64block>

<128block>
+ cargo build --release --features '128block bottomup_sched'
   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 8.46s
Block size = 128
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 4107.097099018794 MB/s, sd = 240.31446814875807
Decode: avg = 2978.3766812284057 MB/s, sd = 141.32450938118478
</128block>

...

</GREEDY SCHEDULING>


<DFS SCHEDULING>
<64block>
+ cargo build --release --features '64block dfs_sched'
   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 8.85s
Block size = 64
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 2238.414346247583 MB/s, sd = 64.52953943412706
Decode: avg = 1593.1939012058338 MB/s, sd = 41.99721023876602
</64block>

<128block>
+ cargo build --release --features '128block dfs_sched'
   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 8.72s
Block size = 128
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 4096.210712190879 MB/s, sd = 152.14873724180617
Decode: avg = 2991.9212842683514 MB/s, sd = 111.57287213389486
</128block>

...
</DFS SCHEDULING>
```

# Section 7.5
To reproduce the statistical parts of the two tables of Section 7.5,
we can use the option `--stat-sec75`, which is introduced in this branch for reproducing:
```
$ cargo run --release -- --stat-sec75

   Compiling xorslp_ec v0.1.0
    Finished release [optimized] target(s) in 8.54s
     Running `slp-ec/target/release/xorslp_ec --stat-sec75`
Block size = 2048
Statistics for Encoding
        P   Co(P)   Fu(Co(P))   Dfs(Fu(Co(P)))
#XOR  755     385         385              385
#MEM 2265    1155         677              677
NVar   32     385         146               91
CCap   92     447         224              170
Statistics for Decoding
        P   Co(P)   Fu(Co(P))   Dfs(Fu(Co(P)))
#XOR 1368     511         511              511
#MEM 4104    1533         923              923
NVar   32     511         206              132
CCap   89     585         283              212
```

To evaluate throughputs of encoding and decoding,
we use a shell script `throughput_sec75.sh` as follows:
```
$ ./throughput_sec75.sh
    Finished release [optimized] target(s) in 0.02s
< P >
Block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 4825.195648146016 MB/s, sd = 337.7238437663774
Decode: avg = 2919.69425024661 MB/s, sd = 170.612412792345
</ P >

< Co(P) >
Block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 4436.429434784424 MB/s, sd = 188.07835300869473
Decode: avg = 3391.6802030427275 MB/s, sd = 139.48085669903026
</ Co(P) >

...
```

# Section 7.6
To evaluate throughputs of encoding and decoding for various RS-parameters, we use a shell script `throughput_sec76.sh` as follows:
```
$ ./throughput_sec76.sh
< RS(8, 4) >
Block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 8806.605652242857 MB/s, sd = 497.85961990022207
Decode: avg = 6781.160167463341 MB/s, sd = 408.0536843894189
</ RS(8, 4) >

< RS(9, 4) >
Block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 8795.183834045032 MB/s, sd = 456.7461757286176
Decode: avg = 6552.067040250238 MB/s, sd = 330.7330212968266
</ RS(9, 4) >

< RS(10, 4) >
Block size = 2048
Benchmarking of Encoding & Decoding (with [2, 4, 5, 6])
Encode: avg = 8943.55748732993 MB/s, sd = 446.3809230576777
Decode: avg = 6689.646258943815 MB/s, sd = 417.0992957465346
</ RS(10, 4) >
...
```

## For ISA-L
After compiling ISA-L on the basis of https://github.com/sc2021anonym/slp-ec/blob/main/HOWTO_BENCHMARK_ISAL.md,
please put a script `sec76_isal.sh` to your ISA-L directory.

Now your ISA-L directory has both `bench_isal.c` and `sec_isal.sh`.

We can use `sec_isal.sh` as follows:
```
$ ./sec76_isal.sh
< RS(8, 4) >
erasure_code_perf: data size = 8x1250000 4
data size = 10000000, iter = 1000
ENC throughput = 4568.191059 MB/sec, SD = 128.414259
DEC throughput = 4562.385426 MB/sec, SD = 132.687940
</ RS(8, 4) >

< RS(9, 4) >
erasure_code_perf: data size = 9x1111111 4
data size = 9999999, iter = 1000
ENC throughput = 4685.322337 MB/sec, SD = 61.155225
DEC throughput = 4627.249882 MB/sec, SD = 54.811222
</ RS(9, 4) >

...
```
