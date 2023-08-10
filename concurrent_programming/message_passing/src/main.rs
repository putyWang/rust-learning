use std::sync::mpsc;
use std::thread;

fn main() {
    // 使用 mpsc::channel() 创建一个信道
    // 生产者对象 赋值给 tx 消费者为 rx
    let (tx, rx) = mpsc::channel();

    // 使用 消费者.clone() 方法 创建多个消费者
    let tx_clone = tx.clone();

    thread::spawn(move || {
        let val = String::from("hi");
        // 使用 tx 的 send(发送对象) 向信道中发送 val 字符串
        println!("val is {}", val);
        tx_clone.send(val).unwrap();
        
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx_clone.send(val).unwrap();
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
        }
    });

    // 使用 消费者 rx 的 recv() 消费 生产者发送的第一条信息
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    // 使用 for 循环消费多个消息
    println!("消费所有消息");
    for received in rx {
        println!("Got: {}", received);
    }
}
