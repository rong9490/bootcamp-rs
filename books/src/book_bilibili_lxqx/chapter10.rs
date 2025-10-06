// cargo run --example lesson_lxqx10
/*
 * 10.智能指针
  可调度分派函数的要求!
  多态: 动态分发, 静态分发

  impl Into<T>
  asRef

  运行时类型擦除, 函数指针
  虚表
  trait对象
  Box<dyn Person>
  ?Sized 大小不确定

  类型 p: &dyn Persion 借用不消耗所有权;
  两种类型是不兼容的
  let person_ref: &Box<dyn Persopn> = &person

  &Box<dyn Person> / 不会隐式转换

  两种方式解决
  persion_ref.deref()
  persion_ref.as_ref()


  Cell的用法(较少), RefCell
  本质是复制拷贝 cell.take() mem::take()
  *cell.get_mut() = xxx
 */

/*
  fn deref(&self) -> &T {
    &**self
  }
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
