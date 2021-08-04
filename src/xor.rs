#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]

use crate::BLOCK_SIZE_PER_ITER;
use std::arch::x86_64::*;

pub unsafe fn page_generic_slow(dst: *mut u8, vs: &[*const u8]) {
    // we take this implementation to deal the case dst in vs
    if vs.contains(&(dst as *const u8)) {
        for ptr in vs {
            if dst as *const u8 == *ptr {
                continue;
            } else {
                for i in 0..BLOCK_SIZE_PER_ITER {
                    *dst.add(i) ^= *ptr.add(i);
                }
            }
        }
    } else {
        std::ptr::copy_nonoverlapping(vs[0], dst, BLOCK_SIZE_PER_ITER);
        for ptr in vs.iter().skip(1) {
            for i in 0..BLOCK_SIZE_PER_ITER {
                *dst.add(i) ^= *ptr.add(i);
            }
        }
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_generic(dst: *mut u8, vs: &[*const u8]) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v0: *const __m256i = vs[0] as *const __m256i;

    for cur in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let mut reg0 = _mm256_load_si256(v0);
        let mut reg1 = _mm256_load_si256(v0.add(1));
        let mut reg2 = _mm256_load_si256(v0.add(2));
        let mut reg3 = _mm256_load_si256(v0.add(3));

        for ptr in vs.iter().skip(1) {
            let w: *const __m256i = *ptr as *const __m256i;
            let w = w.add(4 * cur);

            let reg4 = _mm256_load_si256(w);
            let reg5 = _mm256_load_si256(w.add(1));
            let reg6 = _mm256_load_si256(w.add(2));
            let reg7 = _mm256_load_si256(w.add(3));

            reg0 = _mm256_xor_si256(reg0, reg4);
            reg1 = _mm256_xor_si256(reg1, reg5);
            reg2 = _mm256_xor_si256(reg2, reg6);
            reg3 = _mm256_xor_si256(reg3, reg7);
        }
        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);

        v0 = v0.add(4);
        dst = dst.add(4)
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor2(dst: *mut u8, v1: *const u8, v2: *const u8) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let x0 = _mm256_xor_si256(reg0, reg4);
        let x1 = _mm256_xor_si256(reg1, reg5);
        let x2 = _mm256_xor_si256(reg2, reg6);
        let x3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, x0);
        _mm256_store_si256(dst.add(1), x1);
        _mm256_store_si256(dst.add(2), x2);
        _mm256_store_si256(dst.add(3), x3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor3(dst: *mut u8, v1: *const u8, v2: *const u8, v3: *const u8) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor4(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor5(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor6(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor7(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor8(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor9(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor10(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor11(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
    v11: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;
    let mut v11: *const __m256i = v11 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v11);
        let reg5 = _mm256_load_si256(v11.add(1));
        let reg6 = _mm256_load_si256(v11.add(2));
        let reg7 = _mm256_load_si256(v11.add(3));
        v11 = v11.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor12(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
    v11: *const u8,
    v12: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;
    let mut v11: *const __m256i = v11 as *const __m256i;
    let mut v12: *const __m256i = v12 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v11);
        let reg5 = _mm256_load_si256(v11.add(1));
        let reg6 = _mm256_load_si256(v11.add(2));
        let reg7 = _mm256_load_si256(v11.add(3));
        v11 = v11.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v12);
        let reg5 = _mm256_load_si256(v12.add(1));
        let reg6 = _mm256_load_si256(v12.add(2));
        let reg7 = _mm256_load_si256(v12.add(3));
        v12 = v12.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor13(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
    v11: *const u8,
    v12: *const u8,
    v13: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;
    let mut v11: *const __m256i = v11 as *const __m256i;
    let mut v12: *const __m256i = v12 as *const __m256i;
    let mut v13: *const __m256i = v13 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v11);
        let reg5 = _mm256_load_si256(v11.add(1));
        let reg6 = _mm256_load_si256(v11.add(2));
        let reg7 = _mm256_load_si256(v11.add(3));
        v11 = v11.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v12);
        let reg5 = _mm256_load_si256(v12.add(1));
        let reg6 = _mm256_load_si256(v12.add(2));
        let reg7 = _mm256_load_si256(v12.add(3));
        v12 = v12.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v13);
        let reg5 = _mm256_load_si256(v13.add(1));
        let reg6 = _mm256_load_si256(v13.add(2));
        let reg7 = _mm256_load_si256(v13.add(3));
        v13 = v13.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor14(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
    v11: *const u8,
    v12: *const u8,
    v13: *const u8,
    v14: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;
    let mut v11: *const __m256i = v11 as *const __m256i;
    let mut v12: *const __m256i = v12 as *const __m256i;
    let mut v13: *const __m256i = v13 as *const __m256i;
    let mut v14: *const __m256i = v14 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v11);
        let reg5 = _mm256_load_si256(v11.add(1));
        let reg6 = _mm256_load_si256(v11.add(2));
        let reg7 = _mm256_load_si256(v11.add(3));
        v11 = v11.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v12);
        let reg5 = _mm256_load_si256(v12.add(1));
        let reg6 = _mm256_load_si256(v12.add(2));
        let reg7 = _mm256_load_si256(v12.add(3));
        v12 = v12.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v13);
        let reg5 = _mm256_load_si256(v13.add(1));
        let reg6 = _mm256_load_si256(v13.add(2));
        let reg7 = _mm256_load_si256(v13.add(3));
        v13 = v13.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v14);
        let reg5 = _mm256_load_si256(v14.add(1));
        let reg6 = _mm256_load_si256(v14.add(2));
        let reg7 = _mm256_load_si256(v14.add(3));
        v14 = v14.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[target_feature(enable = "avx2")]
pub unsafe fn avx2_page_xor15(
    dst: *mut u8,
    v1: *const u8,
    v2: *const u8,
    v3: *const u8,
    v4: *const u8,
    v5: *const u8,
    v6: *const u8,
    v7: *const u8,
    v8: *const u8,
    v9: *const u8,
    v10: *const u8,
    v11: *const u8,
    v12: *const u8,
    v13: *const u8,
    v14: *const u8,
    v15: *const u8,
) {
    let mut dst: *mut __m256i = dst as *mut __m256i;
    let mut v1: *const __m256i = v1 as *const __m256i;
    let mut v2: *const __m256i = v2 as *const __m256i;
    let mut v3: *const __m256i = v3 as *const __m256i;
    let mut v4: *const __m256i = v4 as *const __m256i;
    let mut v5: *const __m256i = v5 as *const __m256i;
    let mut v6: *const __m256i = v6 as *const __m256i;
    let mut v7: *const __m256i = v7 as *const __m256i;
    let mut v8: *const __m256i = v8 as *const __m256i;
    let mut v9: *const __m256i = v9 as *const __m256i;
    let mut v10: *const __m256i = v10 as *const __m256i;
    let mut v11: *const __m256i = v11 as *const __m256i;
    let mut v12: *const __m256i = v12 as *const __m256i;
    let mut v13: *const __m256i = v13 as *const __m256i;
    let mut v14: *const __m256i = v14 as *const __m256i;
    let mut v15: *const __m256i = v15 as *const __m256i;

    for _ in 0..(BLOCK_SIZE_PER_ITER / 128) {
        let reg0 = _mm256_load_si256(v1);
        let reg1 = _mm256_load_si256(v1.add(1));
        let reg2 = _mm256_load_si256(v1.add(2));
        let reg3 = _mm256_load_si256(v1.add(3));
        v1 = v1.add(4);

        let reg4 = _mm256_load_si256(v2);
        let reg5 = _mm256_load_si256(v2.add(1));
        let reg6 = _mm256_load_si256(v2.add(2));
        let reg7 = _mm256_load_si256(v2.add(3));
        v2 = v2.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v3);
        let reg5 = _mm256_load_si256(v3.add(1));
        let reg6 = _mm256_load_si256(v3.add(2));
        let reg7 = _mm256_load_si256(v3.add(3));
        v3 = v3.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v4);
        let reg5 = _mm256_load_si256(v4.add(1));
        let reg6 = _mm256_load_si256(v4.add(2));
        let reg7 = _mm256_load_si256(v4.add(3));
        v4 = v4.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v5);
        let reg5 = _mm256_load_si256(v5.add(1));
        let reg6 = _mm256_load_si256(v5.add(2));
        let reg7 = _mm256_load_si256(v5.add(3));
        v5 = v5.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v6);
        let reg5 = _mm256_load_si256(v6.add(1));
        let reg6 = _mm256_load_si256(v6.add(2));
        let reg7 = _mm256_load_si256(v6.add(3));
        v6 = v6.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v7);
        let reg5 = _mm256_load_si256(v7.add(1));
        let reg6 = _mm256_load_si256(v7.add(2));
        let reg7 = _mm256_load_si256(v7.add(3));
        v7 = v7.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v8);
        let reg5 = _mm256_load_si256(v8.add(1));
        let reg6 = _mm256_load_si256(v8.add(2));
        let reg7 = _mm256_load_si256(v8.add(3));
        v8 = v8.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v9);
        let reg5 = _mm256_load_si256(v9.add(1));
        let reg6 = _mm256_load_si256(v9.add(2));
        let reg7 = _mm256_load_si256(v9.add(3));
        v9 = v9.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v10);
        let reg5 = _mm256_load_si256(v10.add(1));
        let reg6 = _mm256_load_si256(v10.add(2));
        let reg7 = _mm256_load_si256(v10.add(3));
        v10 = v10.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v11);
        let reg5 = _mm256_load_si256(v11.add(1));
        let reg6 = _mm256_load_si256(v11.add(2));
        let reg7 = _mm256_load_si256(v11.add(3));
        v11 = v11.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v12);
        let reg5 = _mm256_load_si256(v12.add(1));
        let reg6 = _mm256_load_si256(v12.add(2));
        let reg7 = _mm256_load_si256(v12.add(3));
        v12 = v12.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v13);
        let reg5 = _mm256_load_si256(v13.add(1));
        let reg6 = _mm256_load_si256(v13.add(2));
        let reg7 = _mm256_load_si256(v13.add(3));
        v13 = v13.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v14);
        let reg5 = _mm256_load_si256(v14.add(1));
        let reg6 = _mm256_load_si256(v14.add(2));
        let reg7 = _mm256_load_si256(v14.add(3));
        v14 = v14.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        let reg4 = _mm256_load_si256(v15);
        let reg5 = _mm256_load_si256(v15.add(1));
        let reg6 = _mm256_load_si256(v15.add(2));
        let reg7 = _mm256_load_si256(v15.add(3));
        v15 = v15.add(4);

        let reg0 = _mm256_xor_si256(reg0, reg4);
        let reg1 = _mm256_xor_si256(reg1, reg5);
        let reg2 = _mm256_xor_si256(reg2, reg6);
        let reg3 = _mm256_xor_si256(reg3, reg7);

        _mm256_store_si256(dst, reg0);
        _mm256_store_si256(dst.add(1), reg1);
        _mm256_store_si256(dst.add(2), reg2);
        _mm256_store_si256(dst.add(3), reg3);
        dst = dst.add(4);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::gen_data;

    fn naiive_xor(dst: &mut [u8], left: &[u8], right: &[u8]) {
        assert!(dst.len() == BLOCK_SIZE_PER_ITER);
        assert!(dst.len() == left.len());
        assert!(dst.len() == right.len());

        for i in 0..BLOCK_SIZE_PER_ITER {
            dst[i] = left[i] ^ right[i];
        }
    }

    fn gen_array(len: usize) -> Vec<u8> {
        loop {
            let v = vec![0u8; len];
            if v.as_ptr() as usize % 32 == 0 {
                return v;
            }
        }
    }

    #[test]
    fn avx2xor2_test1() {
        let left = gen_data(BLOCK_SIZE_PER_ITER);
        let right = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &left, &right);

        let mut dst2 = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor2(dst2.as_mut_ptr(), left.as_ptr(), right.as_ptr());
        }

        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        unsafe {
            page_generic_slow(dst3.as_mut_ptr(), &vec![left.as_ptr(), right.as_ptr()]);
        }
        assert!(dst1 == dst3);

        assert!(dst1 == dst2);
    }

    #[test]
    fn avx2xor3_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor3(dst.as_mut_ptr(), v1.as_ptr(), v2.as_ptr(), v3.as_ptr());
        }

        let mut dst_ = vec![0; BLOCK_SIZE_PER_ITER];
        unsafe {
            page_generic_slow(
                dst_.as_mut_ptr(),
                &vec![v1.as_ptr(), v2.as_ptr(), v3.as_ptr()],
            );
        }
        assert!(dst == dst_);

        assert!(dst2 == dst);
    }

    #[test]
    fn avx2xor4_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor4(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
            );
        }

        let mut dst_ = vec![0; BLOCK_SIZE_PER_ITER];
        unsafe {
            page_generic_slow(
                dst_.as_mut_ptr(),
                &vec![v1.as_ptr(), v2.as_ptr(), v3.as_ptr(), v4.as_ptr()],
            );
        }
        assert!(dst == dst_);

        assert!(dst3 == dst);
    }

    #[test]
    fn avx2xor5_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor5(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
            );
        }

        assert!(dst4 == dst);
    }

    #[test]
    fn avx2xor6_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor6(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
            );
        }

        assert!(dst5 == dst);
    }

    #[test]
    fn avx2xor7_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor7(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
            );
        }

        assert!(dst6 == dst);
    }

    #[test]
    fn avx2xor8_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor8(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
            );
        }

        let mut dst_ = vec![0; BLOCK_SIZE_PER_ITER];
        unsafe {
            page_generic_slow(
                dst_.as_mut_ptr(),
                &vec![
                    v1.as_ptr(),
                    v2.as_ptr(),
                    v3.as_ptr(),
                    v4.as_ptr(),
                    v5.as_ptr(),
                    v6.as_ptr(),
                    v7.as_ptr(),
                    v8.as_ptr(),
                ],
            );
        }
        assert!(dst_ == dst);

        assert!(dst7 == dst);
    }

    #[test]
    fn avx2xor9_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor9(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
            );
        }

        assert!(dst8 == dst);
    }

    #[test]
    fn avx2xor10_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor10(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
            );
        }

        assert!(dst9 == dst);
    }

    #[test]
    fn avx2xor11_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);
        let mut dst10 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst10, &dst9, &v11);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor11(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
            );
        }

        assert!(dst10 == dst);
    }

    #[test]
    fn avx2xor12_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);
        let mut dst10 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst10, &dst9, &v11);
        let mut dst11 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst11, &dst10, &v12);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor12(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
                v12.as_ptr(),
            );
        }

        assert!(dst11 == dst);
    }

    #[test]
    fn avx2xor13_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);
        let v13 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);
        let mut dst10 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst10, &dst9, &v11);
        let mut dst11 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst11, &dst10, &v12);
        let mut dst12 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst12, &dst11, &v13);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor13(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
                v12.as_ptr(),
                v13.as_ptr(),
            );
        }

        assert!(dst12 == dst);
    }

    #[test]
    fn avx2xor14_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);
        let v13 = gen_data(BLOCK_SIZE_PER_ITER);
        let v14 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);
        let mut dst10 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst10, &dst9, &v11);
        let mut dst11 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst11, &dst10, &v12);
        let mut dst12 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst12, &dst11, &v13);
        let mut dst13 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst13, &dst12, &v14);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor14(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
                v12.as_ptr(),
                v13.as_ptr(),
                v14.as_ptr(),
            );
        }

        assert!(dst13 == dst);
    }

    #[test]
    fn avx2xor15_test1() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);
        let v13 = gen_data(BLOCK_SIZE_PER_ITER);
        let v14 = gen_data(BLOCK_SIZE_PER_ITER);
        let v15 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst1, &v1, &v2);
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst2, &dst1, &v3);
        let mut dst3 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst3, &dst2, &v4);
        let mut dst4 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst4, &dst3, &v5);
        let mut dst5 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst5, &dst4, &v6);
        let mut dst6 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst6, &dst5, &v7);
        let mut dst7 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst7, &dst6, &v8);
        let mut dst8 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst8, &dst7, &v9);
        let mut dst9 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst9, &dst8, &v10);
        let mut dst10 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst10, &dst9, &v11);
        let mut dst11 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst11, &dst10, &v12);
        let mut dst12 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst12, &dst11, &v13);
        let mut dst13 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst13, &dst12, &v14);
        let mut dst14 = vec![0; BLOCK_SIZE_PER_ITER];
        naiive_xor(&mut dst14, &dst13, &v15);

        let mut dst = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor15(
                dst.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
                v12.as_ptr(),
                v13.as_ptr(),
                v14.as_ptr(),
                v15.as_ptr(),
            );
        }
    }

    #[test]
    fn avx2xor_generic_test() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);
        let v13 = gen_data(BLOCK_SIZE_PER_ITER);
        let v14 = gen_data(BLOCK_SIZE_PER_ITER);
        let v15 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_xor15(
                dst1.as_mut_ptr(),
                v1.as_ptr(),
                v2.as_ptr(),
                v3.as_ptr(),
                v4.as_ptr(),
                v5.as_ptr(),
                v6.as_ptr(),
                v7.as_ptr(),
                v8.as_ptr(),
                v9.as_ptr(),
                v10.as_ptr(),
                v11.as_ptr(),
                v12.as_ptr(),
                v13.as_ptr(),
                v14.as_ptr(),
                v15.as_ptr(),
            );
        }
        let mut dst2 = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_generic(
                dst2.as_mut_ptr(),
                &vec![
                    v1.as_ptr(),
                    v2.as_ptr(),
                    v3.as_ptr(),
                    v4.as_ptr(),
                    v5.as_ptr(),
                    v6.as_ptr(),
                    v7.as_ptr(),
                    v8.as_ptr(),
                    v9.as_ptr(),
                    v10.as_ptr(),
                    v11.as_ptr(),
                    v12.as_ptr(),
                    v13.as_ptr(),
                    v14.as_ptr(),
                    v15.as_ptr(),
                ],
            );
        }

        assert!(dst1 == dst2);
    }

    #[test]
    fn generic_slow_test() {
        let v1 = gen_data(BLOCK_SIZE_PER_ITER);
        let v2 = gen_data(BLOCK_SIZE_PER_ITER);
        let v3 = gen_data(BLOCK_SIZE_PER_ITER);
        let v4 = gen_data(BLOCK_SIZE_PER_ITER);
        let v5 = gen_data(BLOCK_SIZE_PER_ITER);
        let v6 = gen_data(BLOCK_SIZE_PER_ITER);
        let v7 = gen_data(BLOCK_SIZE_PER_ITER);
        let v8 = gen_data(BLOCK_SIZE_PER_ITER);
        let v9 = gen_data(BLOCK_SIZE_PER_ITER);
        let v10 = gen_data(BLOCK_SIZE_PER_ITER);
        let v11 = gen_data(BLOCK_SIZE_PER_ITER);
        let v12 = gen_data(BLOCK_SIZE_PER_ITER);
        let v13 = gen_data(BLOCK_SIZE_PER_ITER);
        let v14 = gen_data(BLOCK_SIZE_PER_ITER);
        let v15 = gen_data(BLOCK_SIZE_PER_ITER);
        let v16 = gen_data(BLOCK_SIZE_PER_ITER);
        let v17 = gen_data(BLOCK_SIZE_PER_ITER);

        let mut dst1 = gen_array(BLOCK_SIZE_PER_ITER);
        unsafe {
            avx2_page_generic(
                dst1.as_mut_ptr(),
                &vec![
                    v1.as_ptr(),
                    v2.as_ptr(),
                    v3.as_ptr(),
                    v4.as_ptr(),
                    v5.as_ptr(),
                    v6.as_ptr(),
                    v7.as_ptr(),
                    v8.as_ptr(),
                    v9.as_ptr(),
                    v10.as_ptr(),
                    v11.as_ptr(),
                    v12.as_ptr(),
                    v13.as_ptr(),
                    v14.as_ptr(),
                    v15.as_ptr(),
                    v16.as_ptr(),
                    v17.as_ptr(),
                ],
            );
        }
        let mut dst2 = vec![0; BLOCK_SIZE_PER_ITER];
        unsafe {
            page_generic_slow(
                dst2.as_mut_ptr(),
                &vec![
                    v1.as_ptr(),
                    v2.as_ptr(),
                    v3.as_ptr(),
                    v4.as_ptr(),
                    v5.as_ptr(),
                    v6.as_ptr(),
                    v7.as_ptr(),
                    v8.as_ptr(),
                    v9.as_ptr(),
                    v10.as_ptr(),
                    v11.as_ptr(),
                    v12.as_ptr(),
                    v13.as_ptr(),
                    v14.as_ptr(),
                    v15.as_ptr(),
                    v16.as_ptr(),
                    v17.as_ptr(),
                ],
            );
        }

        assert!(dst1 == dst2);
    }
}
