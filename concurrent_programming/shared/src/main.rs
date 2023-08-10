use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // 创建封装数据 5 的互斥器
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // 直接使用 move 会将 mutex 所有权移动到子线程中
        // 直接使用move 会报错，因此需要使用 Rc 复制 mutex 指针
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            // 通过 lock() 获取 mutex 封装元素与锁
            let mut num = mutex_clone.lock().unwrap();
            *num += 1;
            // 离开作用域时，自动释放 mutex 锁
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *mutex.lock().unwrap());
}
