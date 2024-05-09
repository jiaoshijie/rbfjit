use libc::{c_int, c_void, off_t, size_t};
use libc::{MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_WRITE};
use std::ptr;

extern "C" {
    fn mmap(
        addr: *mut c_void,
        len: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: off_t,
    ) -> *mut c_void;

    fn munmap(addr: *mut c_void, len: size_t) -> c_int;

    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
}

pub fn get_exec_mem(len: usize) -> *mut c_void {
    unsafe {
        mmap(
            ptr::null_mut(),
            len as size_t,
            PROT_EXEC | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS,
            -1,
            0,
        )
    }
}

pub fn release_exec_mem(ptr: *mut c_void, len: usize) {
    let ret = unsafe { munmap(ptr, len as size_t) };
    assert!(ret == 0);
}

pub fn to_void_fn_void(ptr: *mut c_void) -> extern "C" fn() {
    unsafe { std::mem::transmute(ptr) }
}

pub fn to_void_fn_usize_veci8(ptr: *mut c_void) -> extern "C" fn(usize, *mut i8) {
    unsafe { std::mem::transmute(ptr) }
}

pub fn c_memcpy(dest: *mut c_void, src: &[u8], n: usize) {
    unsafe { memcpy(dest, src as *const [u8] as *const c_void, n as size_t); }
}
