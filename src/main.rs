use std::env;
use std::fs::read_to_string;

// use rbfjit::interpreter;
use rbfjit::jit;

pub fn read_insts() -> Vec<u8> {
    let mut args = env::args();
    let program = args.next().unwrap(); // This should not be failed!
    let Some(bf_file_path) = args.next() else {
        panic!("Usage: {program} bf_file_path");
    };

    match read_to_string(&bf_file_path) {
        Ok(insts) => insts.bytes().collect(),
        Err(err) => {
            panic!(
                "Error: Can't open file {}, because of {}",
                bf_file_path,
                err.kind()
            );
        }
    }
}

fn main() {
    let insts = read_insts();
    // interpreter::run(&insts);
    jit::run(&insts);
}
