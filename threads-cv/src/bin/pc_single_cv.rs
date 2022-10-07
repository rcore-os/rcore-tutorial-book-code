use lazy_static::*;
use std::collections::VecDeque;
use std::env::args;
use std::io::{self, Write};
use std::process::exit;
use std::sync::{Condvar, Mutex};
use std::thread;

lazy_static! {
    static ref BUFFER: Mutex<VecDeque<i32>> = Mutex::new(VecDeque::new());
    static ref CV: Condvar = Condvar::new();
}

fn main() {
    let argv = args();
    let argc = argv.len();
    if argc != 4 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: cpu <string>\n").unwrap();
        exit(1);
    }
    let max = args().nth(1).unwrap().parse::<i32>().unwrap();
    let loops = args().nth(2).unwrap().parse::<i32>().unwrap();
    let consumers = args().nth(3).unwrap().parse::<i32>().unwrap();

    let producer_handle = thread::spawn(move || {
        for i in 0..loops {
            let mut buffer2 = BUFFER.lock().unwrap();
            while buffer2.len() as i32 == max {
                buffer2 = CV.wait(buffer2).unwrap();
            }
            buffer2.push_back(i); // do_fill
            CV.notify_one();
        }

        // end case: put an end-of-production marker (-1)
        // into shared BUFFER, one per consumer
        for _ in 0..consumers {
            let mut buffer2 = BUFFER.lock().unwrap();
            while buffer2.len() as i32 == max {
                buffer2 = CV.wait(buffer2).unwrap();
            }
            buffer2.push_back(-1); // do_fill
            CV.notify_one();
        }
    });
    let mut consumer_handle = VecDeque::new();
    for _ in 0..consumers {
        let handle = thread::spawn(move || {
            let mut tmp = 0;
            while tmp != -1 {
                let mut buffer2 = BUFFER.lock().unwrap();
                while buffer2.len() == 0 {
                    buffer2 = CV.wait(buffer2).unwrap();
                }
                tmp = buffer2.pop_front().unwrap(); // do get
                CV.notify_one();
            }
        });
        consumer_handle.push_back(handle);
    }
    producer_handle.join().unwrap();
    for handle in consumer_handle {
        handle.join().unwrap();
    }
}
