use lazy_static::*;
use std::mem::drop;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

lazy_static! {
    static ref SOME_INFO: Mutex<String> = Mutex::new(String::from("fail: src not init"));
}

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);
    println!("ordering: begin");
    let handle = thread::spawn(move || {
        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        println!("{}", *SOME_INFO.lock().unwrap());
    });
    sleep(1);
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *SOME_INFO.lock().unwrap() = String::from("succeed: src has init");
    *started = true;
    cvar.notify_one();
    drop(started); // unlock
    handle.join().unwrap();
    println!("ordering: end");
}
