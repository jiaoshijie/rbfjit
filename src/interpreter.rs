use crate::prelude::*;
use std::collections::HashMap;

pub fn run(insts: &Vec<u8>) {
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
