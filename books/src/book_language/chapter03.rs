/**
 * Static vs Dynamic Typing 动态静态类型语言
 * 分别优缺点
 * 
 * 数据类型: 标量(scalar)类型 / 复合(compound)类型
 * 
 */

fn shadowing() {
  let x = 5;
  let x = x + 1;

  {
    let x: i32 = x * 2;
    println!("The value of x in the inner scope is: {x}");
  }

  println!("The value of x is: {x}");

  {
    let mut spaces: &'static str = "       ";
    let spaces: usize = spaces.len();
  }
}