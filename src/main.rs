// // 最终命令: rcli csv -i input.csv -o output.json --header -d ','

// // use clap::Parser;
// // use rcli::{CmdExector, Opts}; // 这里的rcli指代的是当前这整个项目

// // // 异步运行时
// // #[tokio::main]
// // async fn main() -> anyhow::Result<()> {
// //     // 这个库是什么作用的 ??
// //     tracing_subscriber::fmt::init();

// //     let opts: Opts = Opts::parse();

// //     opts.cmd.execute().await?;
// //     Ok(())
// // }

// use rcli;

// fn main() {
//     println!("Main fn!!");
//     let opts = rcli::Opts::parse();
//     println!("{:?}", opts)
// }

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}
