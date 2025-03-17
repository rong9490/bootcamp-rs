use futures::executor::block_on;

fn main() {
  println!("start");
  let future = hello_world();
  block_on(future);
  println!("end");
}

async fn hello_world() {
  println!("hello world");
}