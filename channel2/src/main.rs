use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    //创建了一个新通道
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        //线程通过管道发送多条消息
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        
    });

    // 主线程通过管道接受消息并打印
    for received in rx {
        println!("Got: {}",received);
    }
   
}
