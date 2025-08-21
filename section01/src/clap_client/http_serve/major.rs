use clap::Parser;
use super::serve::HttpServeOpts;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts), // 子命令Serve
}
