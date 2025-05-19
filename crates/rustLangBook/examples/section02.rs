use rand::Rng;
use std::cmp::Ordering;
/// cargo build/run --example section01
use std::io;
use std::mem::{size_of, size_of_val};

fn main() -> () {
    println!("Guess the number!");
    // 种子seed --> 操作系统 || thread与当前线程相关
    let secret_num: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess: String = String::new(); // 绑定一个空字符串实例(智能指针, 8 * 3 = 24byte)
        let guess_type_size: usize = size_of::<String>();
        let guess_val_size: usize = size_of_val(&guess);
        println!(
            "guess_type_size={}, guess_val_size={}",
            guess_type_size, guess_val_size
        );

        println!("Please input your guess.");
        // 接受用户数据 Result::Ok(usize) || Result::Err 崩溃展开打印
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // shadowing 遮蔽重写 --> 错误跳过
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your gussed: {}", guess);

        // std::cmp::Ordering枚举三个变体(分支arm) --> Less/Greater/Equal
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Bingo!");
                break;
            },
        }
    }
}
