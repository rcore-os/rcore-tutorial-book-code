use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use user_lib::*;

type synchronizer_t = Arc<(Mutex<bool>, Condvar)>;

fn sync_signal(pair: synchronizer_t) {
    let (lock, cvar) = &*pair;
    let mut done = lock.lock().unwrap();
    *done = true;
    cvar.notify_one();
}

fn child(pair: synchronizer_t) {
    println!("child");
    sleep(1);
    sync_signal(pair);
}

fn sync_wait(pair: synchronizer_t) {
    let (lock, cvar) = &*pair;
    let mut done = lock.lock().unwrap();
    while !*done {
        done = cvar.wait(done).unwrap();
    }
}

fn main() {
    println!("parent: begin");
    // sync_init
    let pair: synchronizer_t = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2: synchronizer_t = Arc::clone(&pair);

    let _handle = thread::spawn(move || {
        child(pair);
    });
    sync_wait(pair2);

    println!("parent: end");
}
