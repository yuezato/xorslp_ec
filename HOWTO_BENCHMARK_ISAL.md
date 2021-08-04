# 1. Getting ISA-L
```
mkdir wks; cd wks
git clone https://github.com/intel/isa-l
cd isa-l
cp ../../bench_isal.c .
```

# 2. Compiling Libraries

To make, ISA-L requires the following package:
```
libtool, automake, autoconf, nasm
```
Please install them in advance.

```
./autogen.sh
./configure
make
```
To do benchmarks, we do not need to install ISA-L by `make install`.

# 3. Benchmark
To run a benchmark set provided by ISA-L,
```
make perf
./erasure_code/erasure_code_perf
```

To run our benchmark program `bench_isal.c`, which is based on Intel's benchmark code and used in the paper,
```
[Mac] gcc -O3 -DDATA_BLOCK=10 -DPARITY_BLOCK=4 -DDATA_SIZE=10000000 bench_isal.c -Iinclude -lisal -o bench_isal
[Linux] gcc -O3 -DDATA_BLOCK=10 -DPARITY_BLOCK=4 -DDATA_SIZE=10000000 bench_isal.c -Iinclude -L.libs -lisal -lm -Wl,-rpath .libs  -o bench_isal
./bench_isal
```
By changing the parameters -DDATA_BLOCK=d and -DPARITY_BLOCK=p, we can run **RS(d, p)**.
