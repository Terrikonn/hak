use alloc::collections::vec_deque::VecDeque;

use crate::{
    process::Process,
    syscall::syscall_sleep,
};

pub const DEFAULT_LOCK_SLEEP: usize = 10000;

pub struct Mutex<T> {
    pub inner_mutex: spin::Mutex<T>,
}

impl<T> Mutex<T> {
    pub fn sleep_lock(&mut self) {
        while let None = self.inner_mutex.try_lock() {
            syscall_sleep(DEFAULT_LOCK_SLEEP);
        }
    }

    pub const fn new(value: T) -> Self {
        Self {
            inner_mutex: spin::Mutex::new(value),
        }
    }
}
