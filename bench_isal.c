#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "erasure_code.h"

#include <stdint.h>
#include <inttypes.h>
#include <time.h>
#include <math.h>

double microDiff(struct timespec *start, struct timespec *end)
{
  return
    ((  end->tv_sec * 1000000) + (  end->tv_nsec * 0.001)) -
    ((start->tv_sec * 1000000) + (start->tv_nsec * 0.001));
}

#define TEST_SOURCES 32
#define TEST_LEN(arg) (DATA_SIZE / arg)

#define MMAX TEST_SOURCES
#define KMAX TEST_SOURCES

#define BAD_MATRIX -1

typedef unsigned char u8;

typedef struct pair {
  double v1;
  double v2;
} pair;

pair mean_and_sd(double *data, int len) {
  double sum = 0.0, mean, SD = 0.0;
  for (int i = 0; i < len; ++i)
        sum += data[i];

  mean = sum / len;

  for (int i = 0; i < len; ++i)
    SD += pow(data[i] - mean, 2);

  pair p = { mean, sqrt(SD / len) };
  return p;
}

double enc_one_step(int m /* 14 */, int k /* 10 */, int nerrs /* 4 */,
		    u8 *a, u8 *g_tbls, u8 **buffs)
{
  struct timespec ts, te;      
  
  clock_gettime(CLOCK_MONOTONIC, &ts);
  // actual encoding
  ec_encode_data(TEST_LEN(k), k, m - k, g_tbls, buffs, &buffs[k]);
  clock_gettime(CLOCK_MONOTONIC, &te);

  return microDiff(&ts, &te);
}

double dec_one_step(int m /* 14 */, int k /* 10 */, int nerrs /* 4 */,
		    u8 *a, u8 *g_tbls, u8 **recov, u8 **temp_buffs) 
{
  struct timespec ts, te;      
  
  clock_gettime(CLOCK_MONOTONIC, &ts);
  // actual decoding
  ec_encode_data(TEST_LEN(k), k, nerrs, g_tbls, recov, temp_buffs);
  clock_gettime(CLOCK_MONOTONIC, &te);

  return microDiff(&ts, &te);
}
		      

int main(int argc, char *argv[])
{
  int m, k, nerrs;
  u8 a[MMAX * KMAX];
  u8 g_tbls[KMAX * TEST_SOURCES * 32], src_in_err[TEST_SOURCES];
  u8 src_err_list[TEST_SOURCES];

  // Pick test parameters
  k = DATA_BLOCK;
  nerrs = PARITY_BLOCK;
  m = k + nerrs;
  const u8 err_list[] = { 2, 4, 5, 6 };

  printf("erasure_code_perf: data size = %dx%d %d\n", k, TEST_LEN(k), nerrs);

  if (m > MMAX || k > KMAX || nerrs > (m - k)) {
    printf(" Input test parameter error\n");
    return -1;
  }

  memcpy(src_err_list, err_list, nerrs);
  memset(src_in_err, 0, TEST_SOURCES);
  for (int i = 0; i < nerrs; i++)
    src_in_err[src_err_list[i]] = 1;

  gf_gen_rs_matrix(a, m, k);

  const uint64_t size = k * TEST_LEN(k);
  const int count = 1000;

  double enc[count];
  double dec[count];


  void *buf;
  u8 *buffs[TEST_SOURCES], *temp_buffs[TEST_SOURCES];
  // Setup for encoding
  {
    for (int i = 0; i < m; ++i) {
      if (posix_memalign(&buf, 64, TEST_LEN(k))) {
	printf("alloc error: Fail\n");
	exit(-1);
      }
      buffs[i] = buf;
    }
    for (int i = 0; i < (m - k); ++i) {
      if (posix_memalign(&buf, 64, TEST_LEN(k))) {
	printf("alloc error: Fail\n");
	exit(-1);
      }
      temp_buffs[i] = buf;
    }
    
    // Make random data at each time
    for (int i = 0; i < k; i++)
      for (int j = 0; j < TEST_LEN(k); j++)
	buffs[i][j] = rand();
    
    ec_init_tables(k, m - k, &a[k * k], g_tbls);
  }
  // Benchmark encoding
  for(int iter = 0; iter < count; ++iter) {
    enc[iter] = enc_one_step(m, k, nerrs, a, g_tbls, buffs);
  }
  
  u8 *recov[TEST_SOURCES];
  {
    u8 b[MMAX * KMAX], c[MMAX * KMAX], d[MMAX * KMAX];
    
    // Construct b by removing error rows
    for (int i = 0, r = 0; i < k; i++, r++) {
      while (src_in_err[r])
	r++;
      recov[i] = buffs[r];
      for (int j = 0; j < k; j++)
	b[k * i + j] = a[k * r + j];
    }
    
    if (gf_invert_matrix(b, d, k) < 0) {
      puts("error");
      exit(-1);
    }
    
    for (int i = 0; i < nerrs; i++)
      for (int j = 0; j < k; j++)
	c[k * i + j] = d[k * src_err_list[i] + j];
    
    // setup for decoding
    ec_init_tables(k, nerrs, c, g_tbls);
  }
  // Benchmark decoding
  for(int iter = 0; iter < count; ++iter) {
    dec[iter] = dec_one_step(m, k, nerrs, a, g_tbls, recov, temp_buffs);
  }

  // Check the consitency of encoding and decoding
  for (int i = 0; i < nerrs; i++) {
    if (0 != memcmp(temp_buffs[i], buffs[src_err_list[i]], TEST_LEN(k))) {
      printf("Fail error recovery (%d, %d, %d) - ", m, k, nerrs);
      exit(-1);
    }
  }

  for (int i = 0; i < m; i++)
    if (buffs[i] != NULL)
      free(buffs[i]);
  
  for (int i = 0; i < (m - k); i++)
    if (temp_buffs[i] != NULL)
      free(temp_buffs[i]);


  for (int i = 0; i < count; ++i) {
      enc[i] = size / enc[i]; // BYTE/microsec = MBYTE/sec
      dec[i] = size / dec[i];
  }
  pair enc_mean_sd = mean_and_sd(enc, count);
  pair dec_mean_sd = mean_and_sd(dec, count);
  
  printf("data size = %" PRId64 ", iter = %d\n", size, count);
  printf("ENC throughput = %lf MB/sec, SD = %lf\n", enc_mean_sd.v1, enc_mean_sd.v2);
  printf("DEC throughput = %lf MB/sec, SD = %lf\n", dec_mean_sd.v1, dec_mean_sd.v2);

  return 0;
}
