// cargo run --example lesson_lxqx10
/*
 * 7.智能指针
 */

#[cfg(test)]
mod tests {
  use std::ops::Deref;
  use std::cell::Cell;
  use std::cell::RefCell;

  #[test]
  fn cell_test() {
    let name: Cell<String> = Cell::new(String::from("Venus"));
    println!("name = {}", name.take());
    // name.get_mut(); // cannot mutate immutable variable `name`
    name.replace(String::from("Hello")); // replace是可以的, mem::replace / swap交换 / 
    // 效果与set相同 --> 调用的replace方法, 无返回值
    name.set(String::from("World"));
    println!("name = {}", name.take());
  }

  #[test]
  fn ref_cell_test() {
    let name: RefCell<String> = RefCell::new(String::from("Jack!"));
    println!("name = {}", name.borrow());
  }
}
