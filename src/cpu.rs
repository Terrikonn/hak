//! Module contain RISC-V64 related abstractions above
//! some cpu instructions, registers. Also there [`cpu::TrapFrame`]
//! for process context capture.
//!
//! Check [RISC-V specifications](https://riscv.org/technical/specifications/) for further research

use core::arch::asm;

/// The frequency of QEMU timer interrupt
pub const FREQ: u64 = 10_000_000;
/// Switch process context of process 250 time per second
pub const CONTEXT_SWITCH_TIME: u64 = FREQ / 500;

/// Memory management unit virtual addressing mode
///
/// In 64-bit mode, we're given three different modes for the MMU:
///  * 0 - The MMU is off -- no protection and no translation PA = VA
///  * 8 - This is Sv39 mode -- 39-bit virtual addresses
///  * 9 - This is Sv48 mode -- 48-bit virtual addresses
#[repr(usize)]
pub enum SatpMode {
    Off = 0,
    Sv39 = 8,
    Sv48 = 9,
}

/// [Processor operating mode](https://en.wikipedia.org/wiki/CPU_modes)
#[repr(usize)]
pub enum CpuMode {
    User = 0,
    Supervisor = 1,
    Machine = 3,
}

/// General purpose registers of RISC-V architecture
#[repr(usize)]
pub enum Registers {
    Zero = 0,
    Ra,
    Sp,
    Gp,
    Tp,
    T0,
    T1,
    T2,
    S0,
    S1,
    A0, // 10
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    S2,
    S3,
    S4, // 20
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    T3,
    T4,
    T5, // 30
    T6,
}

/// Floating point registers of RISC-V architecture
#[repr(usize)]
pub enum FRegisters {
    Ft0,
    Ft1,
    Ft2,
    Ft3,
    Ft4,
    Ft5,
    Ft6,
    Ft7,
    Fs0,
    Fs1,
    Fa0, // 10
    Fa1,
    Fa2,
    Fa3,
    Fa4,
    Fa5,
    Fa6,
    Fa7,
    Fs2,
    Fs3,
    Fs4, // 20
    Fs5,
    Fs6,
    Fs7,
    Fs8,
    Fs9,
    Fs10,
    Fs11,
    Ft8,
    Ft9,
    Ft10, // 30
    Ft11,
}

/// Context of process for process context switching
///
/// The trap frame is set into a structure
/// and packed into each hart's mscratch register.
/// This allows for quick reference and full
/// context switch handling.
/// To make offsets easier, everything will be a usize (8 bytes)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    /// General purpose registers
    pub regs: [usize; 32], // 0 - 255
    /// Floating point registers
    pub fregs: [usize; 32], // 256 - 511
    /// Supervisor address tranlation and protection
    pub satp: usize, // 512 - 519
    /// Program counter
    pub pc: usize, // 520
    /// Hardware thread id
    pub hartid: usize, // 528
    /// TODO
    pub qm: usize, // 536
    /// Process id
    pub pid: usize, // 544
    /// Address translation mode scheme
    pub mode: usize, // 552
}

/// Rust requires that we initialize our structures
/// because of the move semantics. What'll happen below
/// is Rust will construct a new [`TrapFrame`] and move it
/// out of the `zero()` function below. Rust contains two
/// different "selfs" where self can refer to the object
/// in memory or Self (capital S) which refers to the
/// data type of the structure. In the case below, this
/// is `TrapFrame`.
impl TrapFrame {
    pub const fn new() -> Self {
        Self {
            regs: [0; 32],
            fregs: [0; 32],
            satp: 0,
            pc: 0,
            hartid: 0,
            qm: 1,
            pid: 0,
            mode: 0,
        }
    }
}

/// Build Supervisor Address Translation and Protection register
///
/// The SATP register contains three fields: mode, address space id, and
/// the first level table address (level 2 for Sv39). This function
/// helps make the 64-bit register contents based on those three
/// fields.
pub const fn build_satp(mode: SatpMode, asid: usize, addr: usize) -> usize {
    (mode as usize) << 60 | (asid & 0xffff) << 44 | (addr >> 12) & 0xff_ffff_ffff
}

/// Read Machine HARdware Thread id
pub fn mhartid_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mhartid", lateout(reg) rval);
        rval
    }
}

/// Read Machine Interrupt-Enable register
pub fn mie_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mie", lateout(reg) rval);
        rval
    }
}

/// Set Machine Interrupt-Enable register
pub fn mie_write(val: usize) {
    unsafe {
        asm!("csrw mie, {}", in(reg) val);
    }
}

/// Set Machine Status register
pub fn mstatus_write(val: usize) {
    unsafe {
        asm!("csrw mstatus, {}", in(reg) val);
    }
}

/// Read Machine Status register
pub fn mstatus_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mstatus", lateout(reg) rval);
        rval
    }
}

/// Set Supervisor Trap handler base address
pub fn stvec_write(val: usize) {
    unsafe {
        asm!("csrw stvec, {}", in(reg) val);
    }
}

/// Read Supervisor Trap handler base address
pub fn stvec_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, stvec", lateout(reg) rval);
        rval
    }
}

/// Set Machine Scratch register
pub fn mscratch_write(val: usize) {
    unsafe {
        asm!("csrw mscratch, {}", in(reg) val);
    }
}

/// Read Machine Scratch register
pub fn mscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mscratch", lateout(reg) rval);
        rval
    }
}

/// Swap value of Machine Scratch register
pub fn mscratch_swap(to: usize) -> usize {
    unsafe {
        let from;
        asm!("csrrw {}, mscratch, {}", lateout(reg) from, in(reg) to);
        from
    }
}

/// Set Supervisor Scratch register
pub fn sscratch_write(val: usize) {
    unsafe {
        asm!("csrw sscratch, {}", in(reg) val);
    }
}

/// Read Supervisor Scratch register
pub fn sscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, sscratch", lateout(reg) rval);
        rval
    }
}

/// Swap value of Supervisor Scratch register
pub fn sscratch_swap(to: usize) -> usize {
    unsafe {
        let from;
        asm!("csrrw {}, sscratch, {}", lateout(reg) from, in(reg) to);
        from
    }
}

/// Set Machine Exception Program Counter register
pub fn mepc_write(val: usize) {
    unsafe {
        asm!("csrw mepc, {}", in(reg) val);
    }
}

/// Read Machine Exception Program Counter register
pub fn mepc_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mepc", lateout(reg) rval);
        rval
    }
}

/// Set Supervisor Exception Program Counter register
pub fn sepc_write(val: usize) {
    unsafe {
        asm!("csrw sepc, {}", in(reg) val);
    }
}

/// Read Supervisor Exception Program Counter register
pub fn sepc_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, sepc", lateout(reg) rval);
        rval
    }
}

/// Set Supervisor Address Translation and Protection register
pub fn satp_write(val: usize) {
    unsafe {
        asm!("csrw satp, {}", in(reg) val);
    }
}

/// Read Supervisor Address Translation and Protection register
pub fn satp_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, satp", lateout(reg) rval);
        rval
    }
}

/// Take a hammer to the page tables and synchronize all of them.
///
/// This essentially flushes the entire TLB.
pub fn satp_fence(vaddr: usize, asid: usize) {
    unsafe {
        asm!("sfence.vma {}, {}", in(reg) vaddr, in(reg) asid);
    }
}

/// Synchronize based on the address space identifier
///
/// This allows us to fence a particular process rather
/// than the entire TLB.
/// The RISC-V documentation calls this a TLB flush +.
/// Since there are other memory routines involved, they
/// didn't call it a TLB flush, but it is much like
/// Intel/AMD's invtlb [] instruction.
pub fn satp_fence_asid(asid: usize) {
    unsafe {
        asm!("sfence.vma zero, {}", in(reg) asid);
    }
}

/// Memory mapped value of machine timer register
const MMIO_MTIME: *const u64 = 0x0200_BFF8 as *const u64;

/// Give Machine Timer value
pub fn get_mtime() -> usize {
    unsafe { (*MMIO_MTIME) as usize }
}

/// Copy one data from one memory location to another.
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, bytes: usize) {
    for i in 0..bytes {
        dest.add(i).write(src.add(i).read());
    }
}

/// Dumps the registers of a given [`TrapFrame`]. This is NOT the current CPU registers!
pub fn dump_registers(frame: *const TrapFrame) {
    print!("   ");
    for i in 1..32 {
        if i % 4 == 0 {
            println!();
            print!("   ");
        }
        print!("x{:2}:{:08x}   ", i, unsafe { (*frame).regs[i] });
    }
    println!();
}
