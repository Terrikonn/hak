use crate::{
    fs::MinixFileSystem,
    println,
    syscall,
};
/// Test block will load raw binaries into memory to execute them. This function
/// will load ELF files and try to execute them.
pub fn test() {
    // The majority of the testing code needs to move into a system call (execv maybe?)
    MinixFileSystem::init(8);
    // let path = "/pong.elf\0".as_bytes().as_ptr();
    let path = b"/shell.elf\0".as_ptr();
    syscall::syscall_execv(path, 0);
    println!("I should never get here, execv should destroy our process.");
}
