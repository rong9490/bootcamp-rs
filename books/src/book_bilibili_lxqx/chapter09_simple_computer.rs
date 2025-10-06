use std::io;

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


        todo!()

    }
}

struct UserType<T: Computer> {
    // computer: CommandLineComputer, // 不直接绑死'CommandLineComputer', 而是抽象为trait约束
    computer: T,
    expr: String,
}

impl<T: Computer> UserType<T> {
    // 实例化 (传入计算器实例)
    fn new(computer: T) -> Self {
        Self {
            computer,
            expr: String::new(),
        }
    }

    // 输入表达式
    fn type_expr(&mut self) {
        println!("请输入表达式: "); // 配合强制刷新
        // 需可变借用
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
    }
}
