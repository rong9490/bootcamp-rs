use std::{thread, time::Duration};
use tokio::sync::mpsc;

// cargo watch -x "run --package section04 --example tokio_mpsc"
#[tokio::main]
async fn main() {
    println!("tokio_mpsc!");
    // tokio task send string to expensive_blocking_task for execution
    let (tx, rx) = mpsc::channel::<i32>(42);
    // let handle = worker(rx);

    tokio::spawn(async move {
        // let mut i = 0;
        // loop {
        //     i += 1;
        //     println!("sending task {}", i);
        //     tx.send(format!("task {i}")).await.unwrap();
        // }
    });

    // 阻塞, 等待
    // handle.join().unwrap();
}

// fn worker(mut rx: mpsc::Receiver<String>) -> thread::JoinHandle<()> {
//     thread::spawn(move || {
//         let (sender, receiver) = std::sync::mpsc::channel();
//         while let Some(s) = rx.blocking_recv() {
//             let sender_clone = sender.clone();
//             thread::spawn(move || {
//                 let ret = expensive_blocking_task(s);
//                 sender_clone.send(ret).unwrap();
//             });
//             let result = receiver.recv().unwrap();
//             println!("result: {}", result);
//         }
//     })
// }

// fn expensive_blocking_task(s: String) -> String {
//     thread::sleep(Duration::from_millis(800));
//     blake3::hash(s.as_bytes()).to_string()
// }
