use rbfjit::mmap;
use std::fs::File;
use std::os::fd::AsRawFd;

fn main() {
    let fd = File::open("./hello.o").unwrap();
    let len = fd.metadata().unwrap().len() as usize;
    let raw_insturction = mmap::get_exec_mem(fd.as_raw_fd(), len);
    let code_to_run = mmap::to_void_fn_void(raw_insturction);
    code_to_run();
    mmap::release_exec_mem(raw_insturction, len);
}
