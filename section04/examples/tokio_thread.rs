use std::{
    thread::{self, JoinHandle},
    time::Duration,
};
use tokio::{
    fs,
    runtime::{Builder, Runtime},
    time::sleep,
};

// cargo watch -x "run --package section04 --example tokio_thread"
fn main() -> () {
    println!("tokio_thread!");

    // 创建一个线程实例, 返回控制句柄
    let handle: JoinHandle<()> = thread::spawn(|| {
        let rt: Runtime = Builder::new_current_thread().enable_all().build().unwrap(); // ?? 这个Build是什么含义

        // 这个是阻塞调用函数?
        rt.block_on(run(&rt));
    });
    println!("handle size = {}", std::mem::size_of_val(&handle)); // 24 bytes
    handle.join().unwrap(); // 绑定, 阻塞, 等待
}

fn expensive_blocking_task(s: String) -> String {
    thread::sleep(Duration::from_millis(800));
    // HINT 理解blake3依赖的作用? 加密/序列化/base64?
    blake3::hash(s.as_bytes()).to_string()
}

// 将线程"runtime"运行时实例传入, 具体执行什么内容有子函数决定 --> 装饰器模式
async fn run(rt: &Runtime) {
    // spawn创建子线程, 执行 "future 1"
    rt.spawn(async {
        println!("future 1");
        let content: Vec<u8> = fs::read("Cargo.toml").await.unwrap();
        println!("content: {:?}", content.len());
    });
    println!("-------------------");

    // spawn创建子线程, 执行 "future 2"
    rt.spawn(async {
        println!("future 2");
        let result: String = expensive_blocking_task("hello".to_string());
        println!("result: {}", result);
    });
    println!("-------------------");

    sleep(Duration::from_secs(1)).await;
}
