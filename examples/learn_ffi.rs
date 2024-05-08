use libc::{size_t, c_char};

extern {
    fn strlen(cs: *const c_char) -> size_t;
}

fn main() {
    let a: [i8; 5] = [65, 66, 67, 0, 0];
    let ap = &a as *const c_char;
    let len;
    unsafe {
        len = strlen(ap);
    }
    println!("{}", len);
}
