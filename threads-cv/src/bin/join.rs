use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    println!("parent: begin");
    let _handle = thread::spawn(move || {
        println!("child");
        sleep(1);
        let (lock, cvar) = &*pair;
        let mut done = lock.lock().unwrap();
        *done = true;
        cvar.notify_one();
    });
    let (lock, cvar) = &*pair2;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }
    println!("parent: end");
}
