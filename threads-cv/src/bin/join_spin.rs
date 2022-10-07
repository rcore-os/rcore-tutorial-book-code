use lazy_static::*;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

lazy_static! {
    static ref DONE: Mutex<bool> = Mutex::new(false);
}

fn main() {
    println!("parent: begin");
    thread::spawn(|| {
        println!("child");
        sleep(5);
        *DONE.lock().unwrap() = true;
    });
    while !*DONE.lock().unwrap() {} // spin
    println!("parent: end");
}
