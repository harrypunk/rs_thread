use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx1): (Sender<u16>, Receiver<u16>) = mpsc::channel();
    let (tx2, rx2): (Sender<u16>, Receiver<u16>) = mpsc::channel();

    thread::spawn(move || loop {
        rx2.recv().iter().map(|x| x * x).for_each(|x| {
            println!("calc thread recv {}", x);
            &tx1.send(x).unwrap();
        });
    });

    loop {
        while let Ok(x) = rx1.try_recv() {
            println!("result recived {}", x);
        }
        for j in 0..3 {
            println!("x={}", j);
            tx2.send(j as u16).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    }
}
