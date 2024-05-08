use libc::{c_void, size_t, c_int, off_t};
use libc::{PROT_EXEC, MAP_PRIVATE};
use std::fs::File;
use std::ptr;
use std::os::fd::AsRawFd;

extern {
    fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t
        ) -> *mut c_void;

    fn munmap(
        addr: *mut c_void,
        len: size_t
        ) -> c_int;
}

fn main() {
    let fd = File::open("./hello.o").unwrap();
    let metadata = fd.metadata().unwrap();
    let code;
    let ret: c_int;
    unsafe {
        code = mmap(ptr::null_mut(), metadata.len() as size_t, PROT_EXEC, MAP_PRIVATE, fd.as_raw_fd(), 0);
        assert!(code != ptr::null_mut());
    }
    let c: extern "C" fn() = unsafe { std::mem::transmute(code) };
    c();
    unsafe {
        ret = munmap(code, metadata.len() as size_t);
        assert!(ret == 0);
    }
}
