use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

use rbfjit::mmap;

enum OpsKind {
    Inc,          // +
    Dec,          // -
    Left,         // <
    Right,        // >
    Output,       // .
    Input,        // ,
    JmpIfZero,    // [
    JmpIfNonzero, // ]
    Nop,
}

impl From<u8> for OpsKind {
    fn from(value: u8) -> Self {
        return match value {
            43 => Self::Inc,          // +
            45 => Self::Dec,          // -
            60 => Self::Left,         // >
            62 => Self::Right,        // <
            46 => Self::Output,       // .
            44 => Self::Input,        // ,
            91 => Self::JmpIfZero,    // [
            93 => Self::JmpIfNonzero, // ]
            _ => Self::Nop,
        };
    }
}

fn main() {
    let insts = read_insts();
    // interpreter(&insts);
    jit(&insts);
}

fn read_insts() -> Vec<u8> {
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

fn interpreter(insts: &Vec<u8>) {
    let mut ip: usize = 0;
    let mut dp: usize = 0;
    let mut array: Vec<i8> = vec![0; 30_000];
    let mut jmp_table: HashMap<usize, usize> = HashMap::new();

    while ip < insts.len() {
        match insts[ip].into() {
            OpsKind::Inc => {
                array[dp] += 1;
            }
            OpsKind::Dec => {
                array[dp] -= 1;
            }
            OpsKind::Left => {
                assert!(dp > 0);
                dp -= 1;
            }
            OpsKind::Right => {
                // TODO:
                dp += 1;
            }
            OpsKind::Output => {
                print!("{}", array[dp] as u8 as char);
                // println!("{}", array[dp]);
            }
            OpsKind::Input => {
                todo!();
            }
            OpsKind::JmpIfZero => {
                let mut matched = jmp_table.get(&ip);
                if matched.is_none() {
                    matched = gen_jmp_table(&insts, &mut jmp_table, ip);
                }

                if array[dp] == 0 {
                    ip = *matched.unwrap();
                }
            }
            OpsKind::JmpIfNonzero => {
                if array[dp] != 0 {
                    let matched = jmp_table.get(&ip);
                    assert!(matched.is_some());
                    ip = *matched.unwrap();
                }
            }
            OpsKind::Nop => {}
        }
        ip += 1;
    }
}

fn jit(insts: &Vec<u8>) {
    let mut ip: usize = 0;
    let mut jmp_table: HashMap<usize, usize> = HashMap::new();

    let mut ass: Vec<u8> = Vec::new();

    while ip < insts.len() {
        match insts[ip].into() {
            OpsKind::Inc => {
                ass.extend([0x80, 0x04, 0x3e, 0x01]);
            }
            OpsKind::Dec => {
                ass.extend([0x80, 0x2c, 0x3e, 0x01]);
            }
            OpsKind::Left => {
                ass.extend([0x48, 0x83, 0xef, 0x01]);
            }
            OpsKind::Right => {
                // TODO:
                ass.extend([0x48, 0x83, 0xc7, 0x01]);
            }
            OpsKind::Output => {
                todo!();
                // println!("{}", array[dp]);
            }
            OpsKind::Input => {
                todo!();
            }
            OpsKind::JmpIfZero => {
                let mut matched = jmp_table.get(&ip);
                if matched.is_none() {
                    matched = gen_jmp_table(&insts, &mut jmp_table, ip);
                }

            }
            OpsKind::JmpIfNonzero => {
                let matched = jmp_table.get(&ip);
                assert!(matched.is_some());
                todo!();
            }
            OpsKind::Nop => {}
        }
        ip += 1;
    }
    ass.push(0xc3);
    let mm = mmap::get_exec_mem(ass.len());
    mmap::c_memcpy(mm, &ass, ass.len());
    let code_to_run = mmap::to_void_fn_usize_veci8(mm);
    let mut array: Vec<i8> = vec![0; 30_000];
    // dp %rdi, array %rsi
    code_to_run(0, array.as_mut_ptr());
    mmap::release_exec_mem(mm, ass.len());
    println!("{}", array[0]);
    println!("{}", array[1]);
}

fn gen_jmp_table<'a>(insts: &Vec<u8>, jmp_table: &'a mut HashMap<usize, usize>, ip: usize) -> Option<&'a usize> {
    let mut stack: Vec<usize> = Vec::new();
    stack.push(ip);
    let mut inner_ip = ip + 1;

    while !stack.is_empty() && inner_ip < insts.len() {
        match insts[inner_ip].into() {
            OpsKind::JmpIfZero => {
                stack.push(inner_ip);
            }
            OpsKind::JmpIfNonzero => {
                let Some(match_ip) = stack.pop() else {
                    panic!("Error: mismatched `[` command");  // TODO:
                };

                jmp_table.insert(match_ip, inner_ip);
                jmp_table.insert(inner_ip, match_ip);
            }
            _ => {}  // do nothing
        }
        inner_ip += 1;
    }

    assert!(stack.is_empty());
    return jmp_table.get(&ip);
}
