use std::io; // 标准库(prelude)

pub fn guess() {
    println!("guessing");

    // 绑定到指定变量上面
    // ::关联函数 --> 针对类型本身的静态方法; new 创建实例
    // utf-8字符串编码
    let mut guess = String::new(); // immutable

    // stdin 返回标准输入句柄(Stdin)
    // read_line 从标准输入中读取一行, 并将其存储在给定的字符串中
    // 返回值是io::Result类型, 表示操作是否成功
    // expect 当Result返回Err时, 程序会调用panic!宏并显示错误信息, expect显示更详细的信息
    io::stdin().read_line(&mut guess).expect("Failed to read line!!");

    // &取地址符号， 访问同一块内存地址的数据
    // read_line(&mut guess) 传入一个可变的引用变量, 会直接在该变量上面直接修改数据!
    // 引用是一种较为难理解的特性。“按引用传递” --> rust自动保证指针安全。

    // 打印宏
    println!("Your guess: {}", guess);
}