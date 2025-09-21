// cargo run --example thread1

use anyhow::Result;
use std::{thread, sync::mpsc, time::Duration};

const NUM_PRODUCERS: i32 = 10; // 线程数量

// 发送的数据结构
#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: i32,
    value: i32
}

impl Msg {
    fn new(idx: i32, value: i32) -> Self {
        Self {
            idx,
            value
        }
    }
}

fn main() -> Result<()> {
    println!("thread1.rs!");

    // 创建一个通道, 两端, 发送者 & 接收者
    // tx.clone() 可以有多个生产者
    let (tx, rx) = mpsc::channel::<Msg>();

    // 创建多个生产者
    for i in 0..NUM_PRODUCERS {
        let tx1 = tx.clone();
        thread::spawn(move || producer(i, tx1)); // 能放闭包的地方一定也能放函数
    }

    drop(tx); // 明确释放

    // 创建消费者
    let consumer = thread::spawn(move || {
        // 闭包内自动捕获(move)了rx(没有实现Copy Trait), 所以主线程已经失去了rx的所有权, 后续不可再使用!
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
    });

    // variable moved due to use in closure
    // rx.recv()?;

    // HINT consumer.join()?; anyhow也没有实现对join的异常处理 --> map_err 做一层转换
    consumer.join().map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}

fn producer(idx: i32, tx: mpsc::Sender<Msg>) -> Result<()> {
    // 每个线程具体执行内容: 循环 发送随机数 + 休眠
    loop {
        let value: i32 = rand::random::<i32>();
        tx.send(Msg::new(idx, value))?;
        // Duration数据结构
        thread::sleep(Duration::from_millis(1200));
    }

    // Ok(())
    unreachable!()
}