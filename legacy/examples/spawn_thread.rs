// cargo run --example spawn_thread

use anyhow::Result;
use std::{
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Duration,
};

const NUM_PRODUCERS: i32 = 3;

#[derive(Debug)]
struct Msg {
    idx: i32,
    value: i32,
}

impl Msg {
    pub fn new(idx: i32, value: i32) -> Self {
        Self { idx, value }
    }
}

fn main() -> Result<()> {
    println!("Spawn thread!!");

    // 创建多个Producers (不需要获取JoinHandle)
    let (tx, rx) = mpsc::channel::<Msg>();
    for idx in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(idx, tx));
    }

    // 创建Consumer
    // 由于 move rx, 后续已经无法再访问rx了
    // 思考下, 如果不使用move, 会怎么样? 默认实现了move trait
    let consumer: JoinHandle<()> = thread::spawn(move || {
        for msg in rx {
            println!("consumer: {:?}", msg);
        }
        println!("永远执行不到这里");
        unreachable!();
    });

    // 等待Consumer线程结束 (统一处理线程错误)
    consumer
        .join()
        .map_err(|e| anyhow::anyhow!("Thread join error: {:?}", e))?;


    // borrow of moved value: `rx`
    // rx.recv()?;

    Ok(())
}

// (闭包函数) 数据生产者(死循环: 不断生产数据)
fn producer(idx: i32, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: i32 = rand::random::<i32>();
        tx.send(Msg::new(idx, value))?;
        let sleep_time: u64 = rand::random::<u8>() as u64 * 30;
        let dur: Duration = Duration::from_millis(sleep_time);
        thread::sleep(dur);
        if rand::random::<u8>() % 10 == 0 {
            break;
        }
    }
    Ok(())
}
