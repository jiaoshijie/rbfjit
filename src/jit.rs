use crate::prelude::*;
use crate::mmap;

struct BackPatching {
    left: usize,
    right: usize,
}

pub fn run(insts: &Vec<u8>) {
    let mut ip: usize = 0;
    let mut table: Vec<BackPatching> = Vec::new();
    let mut stack: Vec<usize> = Vec::new();

    let mut ass: Vec<u8> = Vec::new();

    while ip < insts.len() {
        match insts[ip].into() {
            OpsKind::Inc => {
                ass.extend([0x80, 0x04, 0x3e, 0x01]);  // addb
            }
            OpsKind::Dec => {
                ass.extend([0x80, 0x2c, 0x3e, 0x01]);  // subb
            }
            OpsKind::Left => {
                ass.extend([0x48, 0x83, 0xef, 0x01]);
            }
            OpsKind::Right => {
                // TODO:
                ass.extend([0x48, 0x83, 0xc7, 0x01]);
            }
            OpsKind::Output => {
                ass.extend([0x48, 0x8d, 0x04, 0x3e]);  // leaq (%rsi, %rdi, 1), %rax
                ass.extend([0x57]); // pushq %rdi
                ass.extend([0x56]); // pushq %rsi
                ass.extend([0x48, 0x89, 0xc6]); // movq %rax, %rsi
                ass.extend([0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00]); // movq $1, %rax
                ass.extend([0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]); // movq $1, %rdi
                ass.extend([0x48, 0xc7, 0xc2, 0x01, 0x00, 0x00, 0x00]); // movq $1, %rdx
                ass.extend([0x0f, 0x05]);  // syscall
                ass.extend([0x5e]); // popq %rsi
                ass.extend([0x5f]); // popq %rdi
            }
            OpsKind::Input => {
                todo!("Input");
            }
            OpsKind::JmpIfZero => {
                ass.extend([0x48, 0x31, 0xc0]);  // xor %rax, %rax
                ass.extend([0x8a, 0x04, 0x3e]);  // movb (%rsi, %rdi, 1), %al
                ass.extend([0x84, 0xc0]);  // testb %al, %al

                // 74 1byte(two's compelment signed offset) je(jz)
                // 0f 84 4bytes(two's compelment signed offset) je(jz)
                ass.extend([0x0f, 0x84]);
                ass.extend([0x00, 0x00, 0x00, 0x00]);  // placeholder

                stack.push(ass.len());
            }
            OpsKind::JmpIfNonzero => {
                ass.extend([0x48, 0x31, 0xc0]);  // xor %rax, %rax
                ass.extend([0x8a, 0x04, 0x3e]);  // movb (%rsi, %rdi, 1), %al
                ass.extend([0x84, 0xc0]);  // testb %al, %al
                // 75 1byte(two's compelment signed offset) jne(jnz)
                // 0f 85 4bytes(two's compelment signed offset) jne(jnz)

                ass.extend([0x0f, 0x85]);
                ass.extend([0x00, 0x00, 0x00, 0x00]);  // placeholder

                let matched = stack.pop().unwrap();
                table.push(BackPatching { left: matched - 4, right: ass.len() - 4 });
            }
            OpsKind::Nop => {}
        }
        ip += 1;
    }
    ass.push(0xc3);  // ret

    // Back Patching
    for it in table {
        mmap::patching(&mut ass, it.left, it.right);
    }

    let mm = mmap::get_exec_mem(ass.len());
    mmap::c_memcpy(mm, &ass, ass.len());
    let code_to_run = mmap::to_void_fn_usize_veci8(mm);
    let mut array: Vec<i8> = vec![0; 30_000];
    // dp %rdi, array %rsi
    code_to_run(0, array.as_mut_ptr());
    mmap::release_exec_mem(mm, ass.len());
}
