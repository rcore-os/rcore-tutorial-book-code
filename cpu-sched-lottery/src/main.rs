use std::collections::LinkedList;
use std::env::args;
use std::io::{self, Write};

fn main() {
    let argc = args().len();
    let mut list = LinkedList::<i32>::new();
    if argc != 3 {
        let mut stderr = io::stderr();
        stderr.write(b"usage: lottery <seed> <loops>\n").unwrap();
        std::process::exit(1);
    } else {
        let _seed = args().nth(1).unwrap().parse::<i32>().unwrap();
        let loops = args().nth(2).unwrap().parse::<i32>().unwrap();
        list.push_back(50);
        list.push_back(100);
        list.push_back(25);
        let gtickets = 50 + 100 + 25;
        println!("List: {:?}", list);
        for _i in 0..loops {
            let mut counter = 0;
            let winner = rand::random::<u32>() % gtickets;
            let mut winner_ticker = -1;
            for &item in list.iter() {
                counter = counter + item;
                if counter > winner as i32 {
                    winner_ticker = item;
                    break;
                }
            }
            println!("winner: {}, {}\n", winner, winner_ticker);
        }
    }
}
