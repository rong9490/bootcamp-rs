use anyhow;
use bootcamp_rs::{process_csv, Opts, SubCommand};
use clap::Parser;

// cargo deny 安装最新版:
// cargo deny check
// cargo deny inti
// 数据结构不同、依赖改变
// duskdb
// cargo run -- csv --input assets/juventus.csv

// 如何处理header!
// rdr.reader();

fn main() -> anyhow::Result<()> {
    let opts: Opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            return process_csv(&opts.input, &opts.output);
            // let records = reader
            //     .deserialize()
            //     // 函数式编程 迭代器可以Map
            //     // collect 重新组装成Vec
            //     // 避免使用unwrap
            //     .map(|record| record.unwrap())
            //     .collect::<Vec<Player>>();
        }
    }

    // Ok(())
}
