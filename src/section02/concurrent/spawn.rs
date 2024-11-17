// Function std::thread::spawn
// F: FnOnce() -> T + Send + 'static

pub fn spawn_major() {
    let handle = std::thread::spawn(|| {
        println!("Hello from the thread!");
    });
}
