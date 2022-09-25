use std::thread;
use std::time::Duration;
fn main() {
    let handle = thread::spawn(|| {
            for i in 1..10{
            println!("hi number {} from the spawned thread!",i );
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5{
        println!("hi number {} from the main thread!",i );
        thread::sleep(Duration::from_millis(1));
    }

    /// 使用join句柄等待所有线程结束
    /// 通过将thread::spawn返回的结果保存在一个变量中，来避免新线程出现不执行或不能完整执行的情况
    /// thread::spawn的返回值类型是一个自持有所有权的JoinHandle， 调用它的join方法可以阻塞当前线程直到对应的新线程运行结束。
    handle.join().unwrap();

}
