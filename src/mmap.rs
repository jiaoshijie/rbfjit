use libc::{c_void, size_t, c_int, off_t};
use libc::{PROT_EXEC, MAP_PRIVATE};
use std::ptr;

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

pub fn get_exec_mem(fd: i32, len: usize) -> *mut c_void {
    unsafe { mmap(ptr::null_mut(), len as size_t, PROT_EXEC, MAP_PRIVATE, fd as c_int, 0) }
}

pub fn release_exec_mem(ptr: *mut c_void, len: usize) {
    let ret = unsafe { munmap(ptr, len as size_t) };
    assert!(ret == 0);
}

pub fn to_void_fn_void(ptr: *mut c_void) -> extern "C" fn() {
    unsafe { std::mem::transmute(ptr) }
}
