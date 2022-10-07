use lazy_static::*;
use std::sync::Mutex;
use std::thread;
use user_lib::*;

lazy_static! {
    static ref SOME_INFO: Mutex<String> = Mutex::new(String::from("fail: src not init"));
}

fn main() {
    println!("ordering: begin");
    let handle = thread::spawn(move || {
        println!("{}", *SOME_INFO.lock().unwrap());
    });
    sleep(1);
    *SOME_INFO.lock().unwrap() = String::from("succeed: src has init");

    handle.join().unwrap();
    println!("ordering: end");
}
