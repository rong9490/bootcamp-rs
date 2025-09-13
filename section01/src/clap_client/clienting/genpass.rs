use clap::Parser;
// use zxcvbn::zxcvbn;

#[derive(Debug, Parser)]
pub struct GenPassSubCommand {
    #[arg(long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

// TODO ?? 实现CmdExector
// impl CmdExector for GenPassSubCommand {
//     async fn execute(self) -> anyhow::Result<()> {
//         let ret = crate::process_genpass(
//             self.length,
//             self.uppercase,
//             self.lowercase,
//             self.number,
//             self.symbol,
//         )?;
//         println!("{}", ret);
//         // output password strength in stderr
//         let estimate = zxcvbn(&ret, &[])?;
//         eprintln!("Password strength: {}", estimate.score());
//         Ok(())
//     }
// }

// 副命令: 随机密码
pub fn deal_gen_pass(
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
    length: u8,
) -> anyhow::Result<String> {

  todo!()

    // let mut rng = rng();
    // let mut password: String = String::new();
    // let mut chars: Vec<char> = Vec::new();

    // if uppercase {
    //     chars.extend_from_slice(&('A'..='Z').collect::<Vec<char>>());
    // }
    // if lowercase {
    //     chars.extend_from_slice(&('a'..='z').collect::<Vec<char>>());
    // }
    // if number {
    //     chars.extend_from_slice(&('0'..='9').collect::<Vec<char>>());
    // }
    // if symbol {
    //     chars.extend_from_slice(&('!'..='~').collect::<Vec<char>>());
    // }
    // if chars.is_empty() {
    //     anyhow::bail!("至少选择一种字符类型");
    // }

    // for _ in 0..length {
    //     // 随机堆入, 直到长度足够
    //     // 或者 shuffle算法
    //     password.push(chars[rng.random_range(0..chars.len())]);
    // }

    // let estimate = zxcvbn(&password, &[]); // 验证密码强度 zxcvbn
    // println!("pass: {}", password);
    // eprintln!("estimate score: {}", estimate.score());

    // Ok(password)
}
