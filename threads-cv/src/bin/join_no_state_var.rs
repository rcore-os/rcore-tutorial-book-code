use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    println!("parent: begin");
    thread::spawn(move || {
        println!("child: begin");
        let (lock, cvar) = &*pair;
        let _done = lock.lock().unwrap();
        println!("child: signal");
        cvar.notify_one();
    });
    sleep(2);
    println!("parent: wait to be signalled...");
    let (lock, cvar) = &*pair2;
    let done = lock.lock().unwrap();
    let _done = cvar.wait(done).unwrap();
    println!("parent: end");
}
