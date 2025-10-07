use std::io;
use std::io::Write;

/**
    简单计算器
*/

trait Computer {
    // 计算方法
    fn compute(&self, expr: &str) -> i32;
}

// 单元结构体(控制台计算器)
struct CommandLineComputer;

impl Computer for CommandLineComputer {
    fn compute(&self, expr: &str) -> i32 {
        // 拿到字符串如何计算; 简单表达式解析
        let mut a: String = String::new();
        let mut b: String = String::new();
        let mut op: Option<char> = None;

        for chr in expr.trim().chars() {
            // 是否数字
            if chr.is_digit(10) {
                // 判断是前面还是后面
                if op.is_none() {
                    a.push(chr);
                } else {
                    b.push(chr);
                }
                continue;
            }
            // 模式匹配(守卫)
            match chr {
                '+' | '-' | '*' | '/' if op.is_some() => {
                    op = Some(chr);
                },
                _ if chr.is_whitespace() => {
                    continue; // 忽略空白
                },
                _ => panic!("异常字符: {chr}")
            }
        }

        // 遍历完如果不完整
        if a.is_empty() || b.is_empty() || op.is_none() {
            panic!("非法表达式: {expr}")
        }

        // shadowing / parse解析的制定目标(turbofish)
        let a: i32 = a.parse::<i32>().unwrap();
        let b: i32 = b.parse::<i32>().unwrap();
        let op: char = op.unwrap();

        // 优化1: 采用HashMap运算符映射到闭包
        // 优化2: 动态执行表达式(eval), use meval::eval_str;
        match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => unreachable!()
        }
    }
}

struct UserTyper<T: Computer> {
    // computer: CommandLineComputer, // 不直接绑死'CommandLineComputer', 而是抽象为trait约束
    computer: T,
    expr: String,
}

impl<T: Computer> UserTyper<T> {
    // 实例化 (传入计算器实例)
    fn new(computer: T) -> Self {
        Self {
            computer,
            expr: String::new(),
        }
    }

    // 输入表达式
    fn type_expr(&mut self) {
        self.expr.clear(); // 允许重复录入和计算
        println!("请输入表达式: "); // 配合强制刷新
        // 需可变借用
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut self.expr).expect("failed to read line");
    }

    // 对接两端, 执行计算(独立解耦): 计算器 & 表达式
    fn compute(&self) -> i32 {
        self.computer.compute(&self.expr)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        // 需要声明可变
        let mut typer = UserTyper::new(CommandLineComputer);
        // typer.type_expr();
        UserTyper::type_expr(&mut typer); // 两种写法等价

        println!("计算结果: {:#?}", typer.compute());

        // 加个loop循环 --> xuyao
    }
}
