use tiger_cli::csv_cli::usage_csv_cli; // 使用csv_cli模块

fn main() -> anyhow::Result<()> {
    let _ = usage_csv_cli();

    Ok(()) // 配合anyhow::Result<()>
}
