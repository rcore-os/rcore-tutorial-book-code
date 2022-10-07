use std::alloc::{alloc, Layout};
use std::env::args;
use std::io::{self, Write};
use user_lib::*;

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n").unwrap();
        std::process::exit(1);
    } else {
        let layout = Layout::new::<i32>();
        let p = unsafe { alloc(layout) };
        assert_ne!(p, 0 as *mut u8);
        println!("({}) addr pointed to by p: {:#x}", getpid(), p as usize);
        unsafe {
            *(p as *mut i32) = args().nth(1).unwrap().parse::<i32>().unwrap();
        }
        loop {
            spin(1);
            unsafe {
                *(p as *mut i32) = *(p as *mut i32) + 1;
                println!("({}) value of p: {}\n", getpid(), *(p as *mut i32));
            }
        }
    }
}
