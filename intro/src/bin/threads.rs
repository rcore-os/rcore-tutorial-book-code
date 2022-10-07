use libc::c_void;
use libc::{pthread_create, pthread_join, pthread_t};
use std::env::args;
use std::io::{self, Write};
static mut COUNTER: i32 = 0;
static mut LOOPS: i32 = 0;

extern "C" fn worker(_args: *mut c_void) -> *mut c_void {
    unsafe {
        for _i in 0..LOOPS {
            COUNTER = COUNTER + 1;
        }
    }
    0 as *mut c_void
}

fn main() {
    let mut argv = args();
    let argc = argv.len();
    if argc != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n").unwrap();
    } else {
        let mut p1: pthread_t = 0;
        let mut p2: pthread_t = 0;
        unsafe {
            LOOPS = argv.nth(1).unwrap().parse::<i32>().unwrap();
            pthread_create(&mut p1, std::ptr::null(), worker, 0 as *mut c_void);
            pthread_create(&mut p2, std::ptr::null(), worker, 0 as *mut c_void);
            pthread_join(p1, 0 as *mut *mut c_void);
            pthread_join(p2, 0 as *mut *mut c_void);
            println!("Final value   : {}\n", COUNTER);
        }
    }
}
