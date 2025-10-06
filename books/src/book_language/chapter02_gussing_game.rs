// 预导入模块 std::prelude
use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn guessing_game01() {
    println!("Gusee the number");
    println!("Please input your guess.");

    let mut guess: String = String::new();

    // 获取用户的输入
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guessed: {guess}");
}

fn guessing_game02() {
    println!("Gusee the number");
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 模式匹配+cmp比较: mismatched types expected reference `&String` found reference `&i32`
    // shadowing 不同类型不能比较 -> 显式转换
    let guess: u32 = guess.trim().parse::<u32>().expect("Please type a Number!");
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Equal => println!("You Win!"),
        Ordering::Greater => println!("Too big!"),
    };
}

fn guessing_game03() {
    println!("Gusee the number");
    let secret_number: u32 = rand::rng().random_range(1..=100);
    println!("The secret number is: {secret_number}");

    let mut _times: i32 = 0; // 统计执行次数

    // 循环执行
    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue, // 忽略跳过错误输入
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                return;
            }
        };
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_guessing_game01() {
        guessing_game01();
    }

    #[test]
    fn test_guessing_game02() {
        guessing_game02();
    }

    #[test]
    fn test_guessing_game03() {
        guessing_game03();
    }
}
