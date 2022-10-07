use std::env::args;
use std::io::{self, Write};
use std::thread;

static mut COUNTER: i32 = 0;

fn mythread(arg: &str, max: i32) {
    let mut i = 0;
    println!("{}: begin [addr of i: {:#x?}]", arg, &i as *const i32);
    while i < max {
        unsafe {
            COUNTER += 1;
        }
        i += 1;
    }
    println!("{}: done", arg);
}

fn main() {
    if args().len() != 2 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: main-first <loopcount>\n").unwrap();
        std::process::exit(1);
    }
    let max = args().nth(1).unwrap().parse::<i32>().unwrap();
    unsafe {
        println!(
            "main: begin [counter = {}] [{:#x?}]",
            COUNTER, &COUNTER as *const i32
        );
    }
    let handle1 = thread::spawn(move || {
        mythread("A", max);
    });
    let handle2 = thread::spawn(move || {
        mythread("B", max);
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    unsafe {
        println!(
            "main: done\n [counter: {}]\n [should: {}]",
            COUNTER,
            max * 2
        );
    }
}
