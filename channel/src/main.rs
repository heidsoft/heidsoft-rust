use std::sync::mpsc;
use std::thread;

fn main() {
    println!("Hello, world!");
    //创建了一个新通道
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    // 主线程接受打印
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
