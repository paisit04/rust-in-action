#[macro_use] // <1>
extern crate crossbeam;

use crossbeam::channel::unbounded;
use std::thread;

fn main() {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        tx.send(42).unwrap();
    });

    select! {                                       // <1>
       recv(rx) -> msg => println!("{:?}", msg),   // <2>
    }
}
