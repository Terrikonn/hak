use crate::{
    cpu::get_mtime,
    process::{
        ProcessState,
        PROCESS_LIST,
    },
};

pub fn schedule() -> usize {
    let mut frame_addr: usize = 0x1111;
    unsafe {
        // If we can't get the lock, then usually this means a kernel
        // process has the lock. So, we return 0. This has a special
        // meaning to whomever calls the scheduler to say "nobody else got scheduled"
        if let None = PROCESS_LIST.inner_mutex.try_lock() {
            return 0;
        }
        if let Some(mut pl) = PROCESS_LIST.inner_mutex.get_mut().take() {
            // Rust allows us to label loops so that break statements can be
            // targeted.
            'procfindloop: loop {
                pl.rotate_left(1);
                if let Some(prc) = pl.front_mut() {
                    match prc.get_state() {
                        ProcessState::Running => {
                            frame_addr = prc.get_frame_address();
                            break 'procfindloop;
                        },
                        ProcessState::Sleeping => {
                            // Awaken sleeping processes whose sleep until is in
                            // the past.
                            if prc.get_sleep_until() <= get_mtime() {
                                prc.set_state(ProcessState::Running);
                                frame_addr = prc.get_frame_address();
                                break 'procfindloop;
                            }
                        },
                        _ => {},
                    }
                }
            }
            PROCESS_LIST.inner_mutex.get_mut().replace(pl);
        } else {
            println!("could not take process list");
        }
        PROCESS_LIST.inner_mutex.force_unlock();
    }
    frame_addr
}
