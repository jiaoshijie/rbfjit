use rbfjit::mmap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut fd = File::open("./hello.o").unwrap();
    let mut insts = Vec::<u8>::new();
    let len = fd.read_to_end(&mut insts).unwrap();

    let mm = mmap::get_exec_mem(len);

    mmap::c_memcpy(mm, &insts, len);

    let code_to_run = mmap::to_void_fn_void(mm);
    code_to_run();
    mmap::release_exec_mem(mm, len);
}
