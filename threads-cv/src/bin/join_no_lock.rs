use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

static mut DONE: bool = false;

fn main() {
    let pair = Arc::new((Mutex::new(1), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    println!("parent: begin");
    let _handle = thread::spawn(move || {
        println!("child: begin");
        sleep(1);
        unsafe {
            DONE = true;
        }
        let (_lock, cvar) = &*pair;
        println!("child: signal");
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair2;
    while !unsafe { DONE } {
        sleep(2);
        println!("parent: wait to be signalled...");
        let _ = cvar.wait(lock.lock().unwrap()).unwrap();
    }
    println!("parent: end");
}
