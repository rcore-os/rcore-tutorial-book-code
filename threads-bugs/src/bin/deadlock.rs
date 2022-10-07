use lazy_static::*;
use std::mem::drop;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

lazy_static! {
    static ref L1: Mutex<i32> = Mutex::new(1);
    static ref L2: Mutex<i32> = Mutex::new(2);
}

fn thread1() {
    println!("t1: begin");
    println!("t1: try to acquire L1...");
    let l1_lock = L1.lock().unwrap();
    println!("t1: L1 acquired");
    sleep(1);
    println!("t1: try to acquire L2...");
    let l2_lock = L2.lock().unwrap();
    println!("t1: L2 acquired");
    drop(l1_lock);
    drop(l2_lock);
}

fn thread2() {
    println!("                           t2: begin");
    println!("                           t2: try to acquire L2...");
    let l2_lock = L2.lock().unwrap();
    println!("                           t2: L2 acquired");
    sleep(1);
    println!("                           t2: try to acquire L1...");
    let l1_lock = L1.lock().unwrap();
    println!("                           t2: L1 acquired");
    drop(l1_lock);
    drop(l2_lock);
}

fn main() {
    println!("main: begin");
    let handle1 = thread::spawn(|| {
        thread1();
    });
    let handle2 = thread::spawn(|| {
        thread2();
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("main: end");
}
