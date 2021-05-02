use core::convert::TryFrom;

use crate::syscall::syscall_sleep;

pub const DEFAULT_LOCK_SLEEP: usize = 10000;

#[repr(u32)]
pub enum MutexState {
    Unlocked = 0,
    Locked = 1,
}

impl TryFrom<u32> for MutexState {
    type Error = u32;

    fn try_from(state: u32) -> Result<Self, Self::Error> {
        match state {
            0 => Ok(Self::Unlocked),
            1 => Ok(Self::Locked),
            _ => unreachable!(),
            // unexpected_state => Err(unexpected_state),
        }
    }
}

#[repr(C)]
pub struct Mutex {
    state: MutexState,
}

impl<'a> Mutex {
    pub const fn new() -> Self {
        Self {
            state: MutexState::Unlocked,
        }
    }

    pub const fn val(&'a self) -> &'a MutexState {
        &self.state
    }

    /// Try to lock the Mutex. If the mutex is already locked, this function returns false,
    /// otherwise it will return true if the mutex was acquired.
    pub fn try_lock(&mut self) -> bool {
        unsafe {
            let state: u32;
            asm!("amoswap.w.aq {}, {}, ({})", lateout(reg) state, in(reg) 1, in(reg) self);
            match MutexState::try_from(state) {
                // amoswap returns the OLD state of the lock.  If it was already locked, we didn't acquire it.
                Ok(MutexState::Locked) => false,
                Ok(MutexState::Unlocked) => true,
                _ => unreachable!(),
            }
        }
    }

    /// Do NOT sleep lock inside of an interrupt context!
    /// Never use a sleep lock for the process list. Sleeping requires
    /// the process list to function, so you'll deadlock if you do.
    pub fn sleep_lock(&mut self) {
        while !self.try_lock() {
            syscall_sleep(DEFAULT_LOCK_SLEEP);
        }
    }

    /// Can safely be used inside of an interrupt context.
    pub fn spin_lock(&mut self) {
        while !self.try_lock() {}
    }

    /// Unlock a mutex without regard for its previous state.
    pub fn unlock(&mut self) {
        unsafe {
            asm!("amoswap.w.rl zero, zero, ({})", in(reg) self);
        }
    }
}
