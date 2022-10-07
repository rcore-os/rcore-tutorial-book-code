use std::thread;

fn mythread(arg: &str) {
    println!("{}", arg);
}

fn main() {
    println!("main: begin");
    let handle1 = thread::spawn(|| {
        mythread("A");
    });
    let handle2 = thread::spawn(|| {
        mythread("B");
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
    println!("main: end");
}
