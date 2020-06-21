const RDTSC_CYCLES: u8 = 32;
const MS_TO_CYCLES: u32 = 4 << 20; // 4 cycles / ns * 1M ns

pub fn busy_waiting(ms: u64) {
    let nloop = ms * MS_TO_CYCLES as u64 / RDTSC_CYCLES as u64;
    unsafe {
        asm!{"
            LOOP_:
            rdtsc
            loop LOOP_;
            ",
            in("rcx") nloop,
            out("rax") _,
            out("rdx") _,
        };
    }
}
