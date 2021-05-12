use crate::SatpMode;

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
