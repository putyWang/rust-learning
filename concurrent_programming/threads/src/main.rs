use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];

    // 使用 thread::spawn 函数声明一个新线程，并将线程对象赋值给 new_thread 变量
    // 由于 new_thread 线程中使用了 变量 v 因此需要使用 move 关键字将 变量 v 所有权转移至 new_thread 线程中
    let new_thread = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        for i in 1..10 {
            println!("hi number {} from the new thread!", i);
            // 调用 thread::sleep 函数 使 new_thread 线程每次循环结束前 睡眠 1 毫秒
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 由于 v 的所有权已经转移至 新线程之中，因此主线程再调用 v 将不会通过编译
    // drop(v); 

    // 使用 join 方法阻塞主线程 以等待 new_thread 线程完成后在执行
    new_thread.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
