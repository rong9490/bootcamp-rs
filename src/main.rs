use clap::Parser;
use rcli::Opts;

fn main() {
    let args: Opts = Opts::parse();
    println!("{:?}", args)
}
