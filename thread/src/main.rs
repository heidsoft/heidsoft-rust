//导入线程库
use std::thread;
use std::time::Duration;

// Spawns a new thread, returning a JoinHandle for it.
// The join handle provides a [join] method that can be used to join the spawned thread. If the spawned thread panics, [join] will return an [Err] containing the argument given to panic.
// If the join handle is dropped, the spawned thread will implicitly be detached. In this case, the spawned
/// 调用thread::sleep 会强制当前线程停止执行一小段时间
/// 并允许一个不同的线程继续运行
/// 这些线程可能会交替执行，但我们无法对他们的执行顺序做出任何保障
/// 执行顺序由操作系统的线程调度策略决定
fn main() {
    // 生成新线程并保存其句柄
    let spawned_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 主线程的循环
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待生成的线程���成
    spawned_thread.join().unwrap();
}
