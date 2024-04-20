// use crate::CmdExector;
// use clap::Parser;
// use rand::seq::SliceRandom;
// use zxcvbn::zxcvbn; // 可以直接访问 ../lib.rs的内容

// #[derive(Debug, Parser)]
// pub struct GenPassOpts {
//     #[arg(short, long, default_value_t = 20)]
//     pub length: u8,

//     #[arg(long, default_value_t = true)]
//     pub uppercase: bool,

//     #[arg(long, default_value_t = true)]
//     pub lowercase: bool,

//     #[arg(long, default_value_t = true)]
//     pub number: bool,

//     #[arg(long, default_value_t = true)]
//     pub symbol: bool,
// }

// // 为这个结构体实现接口Trait
// impl CmdExector for GenPassOpts {
//     async fn execute(self) -> anyhow::Result<()> {
//         let ret = crate::
//     }
// }



// // pub fn process_genpass(
// //     length: u8,
// //     upper: bool,
// //     lower: bool,
// //     number: bool,
// //     symbol: bool,
// // ) -> anyhow::Result<()> {
// //     let mut rng = rand::thread_rng();
// //     let mut password = Vec::new();
// //     let mut chars = Vec::new();

// //     // 使用字符集
// //     if upper {
// //         chars.extend_from_slice(UPPER);
// //         password.push(*UPPER.choose(&mut rng).expect("None"))
// //     }
// //     if lower {
// //         chars.extend_from_slice(LOWER);
// //         password.push(*LOWER.choose(&mut rng).expect("None"))
// //     }
// //     if number {
// //         chars.extend_from_slice(NUMBER);
// //         password.push(*NUMBER.choose(&mut rng).expect("None"))
// //     }
// //     if symbol {
// //         chars.extend_from_slice(SYMBOL);
// //         password.push(*SYMBOL.choose(&mut rng).expect("None"))
// //     }

// //     // 开始构造
// //     for _ in 0..(length - password.len() as u8) {
// //         let c: &u8 = chars.choose(&mut rng).expect("chars wont empty!");
// //         password.push(*c);
// //     }

// //     // 洗牌
// //     password.shuffle(&mut rng);

// //     let password = String::from_utf8(password)?;

// //     println!("{}", password);

// //     // 检验强度
// //     let estimate = zxcvbn(&password, &[])?;

// //     // pipe 到 std::error 里面
// //     eprintln!("Strength: {}", estimate.score());

// //     Ok(())
// // }
