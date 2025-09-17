// cargo run --package section02 --example _01_thread
// cargo run --example _01_thread

use anyhow::{Result, anyhow};
use rand::Rng;
use std::{sync::mpsc, thread, time::Duration};

const NUM_PRODUCERS: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value: usize = 1; // 临时
        // let value = rand::random::<usize>();

        let mut rng = rand::rng();

        // 生成随机 u64
        let val_u64: u64 = rng.random();
        println!("Random u64: {}", val_u64);

        tx.send(Msg::new(idx, value))?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));
        // random exit the producer
        if rand::random::<u8>() % 5 == 0 {
            println!("producer {} exit", idx);
            break;
        }
    }
    // more things to do
    Ok(())
}

impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}

// TODO 梳理spawn知识点
fn main() -> Result<()> {
    println!("_01_thread.rs !");

    // turbofish明确泛型
    // tx 发送端
    // rx 接收端
    let (tx, rx) = mpsc::channel::<i32>();

    // 创建 producers
    // for i in 0..NUM_PRODUCERS {
    //     let tx = tx.clone();
    //     thread::spawn(move || producer(i, tx));
    // }
    // drop(tx); // 释放 tx，否则 rx 无法结束

    Ok(())
}
