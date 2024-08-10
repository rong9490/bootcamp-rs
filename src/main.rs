use tiger_cli::csv_cli::usage;

fn main() -> anyhow::Result<()> {
    let _ = usage();

    Ok(()) // 配合anyhow::Result<()>
}
