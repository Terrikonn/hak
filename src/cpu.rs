//! Module contain RISC-V64 related abstractions above
//! some cpu instructions, registers. Also there [`cpu::TrapFrame`]
//! for process context capture.
//!
//! Check [RISC-V specifications](https://riscv.org/technical/specifications/) for further research

use crate::{
    print,
    println,
};

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
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Registers {
    pub zero: usize, // 0
    pub ra: usize,   // 1
    pub sp: usize,   // 2
    pub gp: usize,   // 3
    pub tp: usize,   // 4
    pub t0: usize,   // 5
    pub t1: usize,   // 6
    pub t2: usize,   // 7
    pub s0: usize,   // 8
    pub s1: usize,   // 9
    pub a0: usize,   // 10
    pub a1: usize,   // 11
    pub a2: usize,   // 12
    pub a3: usize,   // 13
    pub a4: usize,   // 14
    pub a5: usize,   // 15
    pub a6: usize,   // 16
    pub a7: usize,   // 17
    pub s2: usize,   // 18
    pub s3: usize,   // 19
    pub s4: usize,   // 20
    pub s5: usize,   // 21
    pub s6: usize,   // 22
    pub s7: usize,   // 23
    pub s8: usize,   // 24
    pub s9: usize,   // 25
    pub s10: usize,  // 26
    pub s11: usize,  // 27
    pub t3: usize,   // 28
    pub t4: usize,   // 29
    pub t5: usize,   // 30
    pub t6: usize,   // 31
}

impl Registers {
    /// TOOD: replace with `#[derive(Default)]` when api become const
    pub const fn default() -> Self {
        Self {
            zero: 0,
            ra: 0,
            sp: 0,
            gp: 0,
            tp: 0,
            t0: 0,
            t1: 0,
            t2: 0,
            s0: 0,
            s1: 0,
            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,
            a4: 0,
            a5: 0,
            a6: 0,
            a7: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,
            s8: 0,
            s9: 0,
            s10: 0,
            s11: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
        }
    }
}

/// Floating point registers of RISC-V architecture
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct FRegisters {
    pub ft0: usize,  // 0
    pub ft1: usize,  // 1
    pub ft2: usize,  // 2
    pub ft3: usize,  // 3
    pub ft4: usize,  // 4
    pub ft5: usize,  // 5
    pub ft6: usize,  // 6
    pub ft7: usize,  // 7
    pub fs0: usize,  // 8
    pub fs1: usize,  // 9
    pub fa0: usize,  // 10
    pub fa1: usize,  // 11
    pub fa2: usize,  // 12
    pub fa3: usize,  // 13
    pub fa4: usize,  // 14
    pub fa5: usize,  // 15
    pub fa6: usize,  // 16
    pub fa7: usize,  // 17
    pub fs2: usize,  // 18
    pub fs3: usize,  // 19
    pub fs4: usize,  // 20
    pub fs5: usize,  // 21
    pub fs6: usize,  // 22
    pub fs7: usize,  // 23
    pub fs8: usize,  // 24
    pub fs9: usize,  // 25
    pub fs10: usize, // 26
    pub fs11: usize, // 27
    pub ft8: usize,  // 28
    pub ft9: usize,  // 29
    pub ft10: usize, // 30
    pub ft11: usize, // 31
}

impl FRegisters {
    pub const fn default() -> Self {
        Self {
            ft0: 0,
            ft1: 0,
            ft2: 0,
            ft3: 0,
            ft4: 0,
            ft5: 0,
            ft6: 0,
            ft7: 0,
            fs0: 0,
            fs1: 0,
            fa0: 0,
            fa1: 0,
            fa2: 0,
            fa3: 0,
            fa4: 0,
            fa5: 0,
            fa6: 0,
            fa7: 0,
            fs2: 0,
            fs3: 0,
            fs4: 0,
            fs5: 0,
            fs6: 0,
            fs7: 0,
            fs8: 0,
            fs9: 0,
            fs10: 0,
            fs11: 0,
            ft8: 0,
            ft9: 0,
            ft10: 0,
            ft11: 0,
        }
    }
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
    pub regs: Registers, // 0 - 255
    /// Floating point registers
    pub fregs: FRegisters, // 256 - 511
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
            regs: Registers::default(),
            fregs: FRegisters::default(),
            satp: 0,
            pc: 0,
            hartid: 0,
            qm: 1,
            pid: 0,
            mode: 0,
        }
    }
}

/// Machine HARdware Thread Id register
pub struct MHartId;

impl MHartId {
    /// Read Machine HARdware Thread id
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, mhartid", lateout(reg) rval);
            rval
        }
    }
}

/// Machine interrupt enable register
pub struct Mie;

impl Mie {
    /// Read Machine Interrupt-Enable register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, mie", lateout(reg) rval);
            rval
        }
    }

    /// Set Machine Interrupt-Enable register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw mie, {}", in(reg) val);
        }
    }
}

/// Machine Status register
pub struct MStatus;

impl MStatus {
    /// Set Machine Status register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw mstatus, {}", in(reg) val);
        }
    }

    /// Read Machine Status register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, mstatus", lateout(reg) rval);
            rval
        }
    }
}

/// Supervisor Trap vector register
pub struct STVec;

impl STVec {
    /// Set Supervisor Trap handler base address
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw stvec, {}", in(reg) val);
        }
    }

    /// Read Supervisor Trap handler base address
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, stvec", lateout(reg) rval);
            rval
        }
    }
}

/// Machine Scratch register
pub struct MScratch;

impl MScratch {
    /// Set Machine Scratch register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw mscratch, {}", in(reg) val);
        }
    }

    /// Read Machine Scratch register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, mscratch", lateout(reg) rval);
            rval
        }
    }

    /// Swap value of Machine Scratch register
    pub fn swap(to: usize) -> usize {
        unsafe {
            let from;
            asm!("csrrw {}, mscratch, {}", lateout(reg) from, in(reg) to);
            from
        }
    }
}

/// Supervisor Scratch register
pub struct SScratch;

impl SScratch {
    /// Set Supervisor Scratch register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw sscratch, {}", in(reg) val);
        }
    }

    /// Read Supervisor Scratch register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, sscratch", lateout(reg) rval);
            rval
        }
    }

    /// Swap value of Supervisor Scratch register
    pub fn swap(to: usize) -> usize {
        unsafe {
            let from;
            asm!("csrrw {}, sscratch, {}", lateout(reg) from, in(reg) to);
            from
        }
    }
}

/// Machine Exception Program Counter register
pub struct Mepc;

impl Mepc {
    /// Set Machine Exception Program Counter register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw mepc, {}", in(reg) val);
        }
    }

    /// Read Machine Exception Program Counter register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, mepc", lateout(reg) rval);
            rval
        }
    }
}

/// Supervisor Exception Program Counter register
pub struct Sepc;

impl Sepc {
    /// Set Supervisor Exception Program Counter register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw sepc, {}", in(reg) val);
        }
    }

    /// Read Supervisor Exception Program Counter register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, sepc", lateout(reg) rval);
            rval
        }
    }
}

/// Supervisor Address Translation and Protection register
pub struct Satp;

impl Satp {
    /// Build Supervisor Address Translation and Protection register
    ///
    /// The SATP register contains three fields: mode, address space id, and
    /// the first level table address (level 2 for Sv39). This function
    /// helps make the 64-bit register contents based on those three
    /// fields.
    pub const fn build(mode: SatpMode, asid: usize, addr: usize) -> usize {
        (mode as usize) << 60 | (asid & 0xffff) << 44 | (addr >> 12) & 0xff_ffff_ffff
    }

    /// Set Supervisor Address Translation and Protection register
    pub fn write(val: usize) {
        unsafe {
            asm!("csrw satp, {}", in(reg) val);
        }
    }

    /// Read Supervisor Address Translation and Protection register
    pub fn read() -> usize {
        unsafe {
            let rval;
            asm!("csrr {}, satp", lateout(reg) rval);
            rval
        }
    }

    /// Take a hammer to the page tables and synchronize all of them.
    ///
    /// This essentially flushes the entire TLB.
    pub fn fence(vaddr: usize, asid: usize) {
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
    pub fn fence_asid(asid: usize) {
        unsafe {
            asm!("sfence.vma zero, {}", in(reg) asid);
        }
    }
}

///  Machine Timer register
pub struct MTime;

impl MTime {
    /// Memory mapped value of machine timer register
    const MMIO_MTIME: *const u64 = 0x0200_BFF8 as *const u64;

    /// Give Machine Timer value
    pub fn get() -> usize {
        unsafe { (*Self::MMIO_MTIME) as usize }
    }
}

/// Dumps the registers of a given [`TrapFrame`]. This is NOT the current CPU registers!
pub fn dump_registers(frame: *const TrapFrame) {
    print!("   ");
    // for i in 1..32 {
    //     if i % 4 == 0 {
    //         serial_println!();
    //         serial_print!("   ");
    //     }
    //     serial_print!("x{:2}:{:08x}   ", i, unsafe { (*frame).regs[i] });
    // }
    println!();
}
