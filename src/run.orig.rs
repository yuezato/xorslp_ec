use crate::reorder::Pebble;
use crate::*;

fn calc_addr(addrs: &[*const u8], (idx, coeff): (u8, u8), iter: usize) -> *const u8 {
    unsafe { addrs[idx as usize].add(coeff as usize * iter * BLOCK_SIZE_PER_ITER) }
}

type Pos = (u8, u8);

// For avoiding TLB missess
unsafe fn prefetch_next(addr: &[*const u8], t: Pos, v: &[Pos], iter: usize) {
    use std::arch::x86_64::*;

    const FETCH_TYPE: i32 = 2;

    for ptr in v {
        let ptr = calc_addr(addr, *ptr, iter);
        _mm_prefetch(ptr as *const i8, FETCH_TYPE);
        _mm_prefetch(ptr.add(64) as *const i8, FETCH_TYPE);
        _mm_prefetch(ptr.add(128) as *const i8, FETCH_TYPE);
        _mm_prefetch(ptr.add(192) as *const i8, FETCH_TYPE);
    }

    let dst = calc_addr(addr, t, iter);
    _mm_prefetch(dst as *const i8, FETCH_TYPE);
    _mm_prefetch(dst.add(64) as *const i8, FETCH_TYPE);
    _mm_prefetch(dst.add(128) as *const i8, FETCH_TYPE);
    _mm_prefetch(dst.add(192) as *const i8, FETCH_TYPE);
}

unsafe fn execute(addr: &[*const u8], t: Pos, v: &[Pos], iter: usize) {
    #[cfg(feature = "64block")]
    use crate::xor64::*;

    #[cfg(not(feature = "64block"))]
    use crate::xor::*;

    let ptr_t = calc_addr(addr, t, iter) as *mut u8;

    match v.len() {
        2 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            avx2_page_xor2(ptr_t, ptr_v0, ptr_v1);
        }
        3 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            avx2_page_xor3(ptr_t, ptr_v0, ptr_v1, ptr_v2);
        }
        4 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            avx2_page_xor4(ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3);
        }
        5 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            avx2_page_xor5(ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4);
        }
        6 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            avx2_page_xor6(ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5);
        }
        7 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            avx2_page_xor7(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6,
            );
        }
        8 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            avx2_page_xor8(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7,
            );
        }
        9 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            avx2_page_xor9(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
            );
        }
        10 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            avx2_page_xor10(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9,
            );
        }
        11 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            let ptr_v10 = calc_addr(addr, v[10], iter);
            avx2_page_xor11(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9, ptr_v10,
            );
        }
        12 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            let ptr_v10 = calc_addr(addr, v[10], iter);
            let ptr_v11 = calc_addr(addr, v[11], iter);
            avx2_page_xor12(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9, ptr_v10, ptr_v11,
            );
        }
        13 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            let ptr_v10 = calc_addr(addr, v[10], iter);
            let ptr_v11 = calc_addr(addr, v[11], iter);
            let ptr_v12 = calc_addr(addr, v[12], iter);
            avx2_page_xor13(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9, ptr_v10, ptr_v11, ptr_v12,
            );
        }
        14 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            let ptr_v10 = calc_addr(addr, v[10], iter);
            let ptr_v11 = calc_addr(addr, v[11], iter);
            let ptr_v12 = calc_addr(addr, v[12], iter);
            let ptr_v13 = calc_addr(addr, v[13], iter);
            avx2_page_xor14(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9, ptr_v10, ptr_v11, ptr_v12, ptr_v13,
            );
        }
        15 => {
            let ptr_v0 = calc_addr(addr, v[0], iter);
            let ptr_v1 = calc_addr(addr, v[1], iter);
            let ptr_v2 = calc_addr(addr, v[2], iter);
            let ptr_v3 = calc_addr(addr, v[3], iter);
            let ptr_v4 = calc_addr(addr, v[4], iter);
            let ptr_v5 = calc_addr(addr, v[5], iter);
            let ptr_v6 = calc_addr(addr, v[6], iter);
            let ptr_v7 = calc_addr(addr, v[7], iter);
            let ptr_v8 = calc_addr(addr, v[8], iter);
            let ptr_v9 = calc_addr(addr, v[9], iter);
            let ptr_v10 = calc_addr(addr, v[10], iter);
            let ptr_v11 = calc_addr(addr, v[11], iter);
            let ptr_v12 = calc_addr(addr, v[12], iter);
            let ptr_v13 = calc_addr(addr, v[13], iter);
            let ptr_v14 = calc_addr(addr, v[14], iter);
            avx2_page_xor15(
                ptr_t, ptr_v0, ptr_v1, ptr_v2, ptr_v3, ptr_v4, ptr_v5, ptr_v6, ptr_v7, ptr_v8,
                ptr_v9, ptr_v10, ptr_v11, ptr_v12, ptr_v13, ptr_v14,
            );
        }
        arity => {
            dbg!(arity);
            
            avx2_page_generic(
                ptr_t,
                &v[0..arity]
                    .iter()
                    .map(|a| calc_addr(addr, *a, iter))
                    .collect::<Vec<_>>(),
            );
        }
    }
}

fn run(addrs: &[*const u8], seq: &[(Pos, Vec<Pos>)], iter: usize) {
    let l = seq.len();
    for i in 0..l - 1 {
        let (t, v) = &seq[i + 1];
        unsafe { prefetch_next(addrs, *t, v, iter) };

        let (t, v) = &seq[i];
        unsafe { execute(addrs, *t, v, iter) };
    }
    let (t, v) = &seq[l - 1];
    unsafe { execute(addrs, *t, v, iter) };
}

pub fn required_pebbles(seq: &[(Pebble, &[Pebble])]) -> usize {
    let mut set = BTreeSet::new();

    for (a, _) in seq {
        set.insert(a.clone());
    }

    set.iter()
        .filter_map(|t| match t {
            Pebble::Const(_) => None,
            Pebble::Var(v) => Some(*v),
        })
        .max()
        .unwrap()
        + 1
}

pub struct PageAlignedArray {
    ptr: *const u8,
    size: usize,
}

impl PageAlignedArray {
    pub fn new(size: usize) -> Option<Self> {
        use std::ptr;

        let mut out = ptr::null_mut();
        let ret = unsafe {
            libc::posix_memalign(
                &mut out, 4096, // BLOCK_SIZE_PER_ITER,
                size,
            )
        };
        if ret == 0 {
            Some(Self {
                ptr: out as *const u8,
                size,
            })
        } else {
            None
        }
    }

    pub fn head(&self) -> *const u8 {
        self.ptr
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.size) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr as *mut u8, self.size) }
    }

    pub fn split(&self, height: usize) -> Vec<&[u8]> {
        if (height == 0) {
            return Vec::new();
        }

        let mut v = Vec::new();
        assert!(
            self.size % height == 0,
            "size = {}, height = {}",
            self.size,
            height
        );
        let width = self.size / height;

        for i in 0..height {
            unsafe {
                v.push(std::slice::from_raw_parts(self.ptr.add(i * width), width));
            }
        }
        v
    }
}

impl Drop for PageAlignedArray {
    fn drop(&mut self) {
        unsafe { libc::free(self.ptr as *mut libc::c_void) }
    }
}

pub fn compile(p: Parameter, program: &[(Pebble, &[Pebble])]) -> Vec<(Pos, Vec<Pos>)> {
    use std::convert::TryInto;

    let mut new_program = Vec::new();

    let aux = |v: &Pebble| -> Pos {
        let dst: (usize, u8);
        if !v.is_var() {
            let idx = v.from_const().unwrap();
            dst = (idx, 1u8);
        } else {
            let idx = v.from_var().unwrap();
            if idx < p.nr_parity_block * 8 {
                dst = (8 * p.nr_data_block + idx, 1u8);
            } else {
                dst = (8 * p.nr_data_block + idx, 0u8);
            }
        }
        let (a, b) = dst;
        (a.try_into().unwrap(), b)
    };

    for (t, vars) in program {
        let v = aux(t);
        let vs = vars.iter().map(aux).collect();
        new_program.push((v, vs));
    }

    new_program
}

pub fn estimate_compile(program: &[(Pebble, &[Pebble])]) -> Vec<(Pos, Vec<Pos>)> {
    let mut new_program = Vec::new();

    for (_, _vars) in program {
        let v = (0, 0);
        // let vs = vars.iter().map(|_| { (1, 0)}).ceollect();
        let vs = vec![(1, 0), (1, 0)];
        new_program.push((v, vs));
    }

    new_program
}

pub fn combine_constant_target_tmp(
    constants: &[&[u8]],
    to_store: &[&[u8]],
    tmp: &[&[u8]],
) -> Vec<*const u8> {
    let mut v: Vec<*const u8> = Vec::new();

    for p in constants {
        v.push(p.as_ptr());
    }
    for p in to_store {
        v.push(p.as_ptr());
    }
    for p in tmp {
        v.push(p.as_ptr());
    }

    v
}

pub fn run_program(all_buffers: &[*const u8], iteration: usize, program: &[(Pos, Vec<Pos>)]) {
    for i in 0..iteration {
        run(all_buffers, &program, i);
    }
}
