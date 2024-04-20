use rand::seq::SliceRandom;

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = String::new();
    let mut chars = Vec::new();

    // 使用字符集
    if upper {
        chars.extend_from_slice(b"ABCDE");
    }
    if lower {
        chars.extend_from_slice(b"abcde");
    }
    if number {
        chars.extend_from_slice(b"01234");
    }
    if symbol {
        chars.extend_from_slice(b"@#$%^");
    }

    // 开始构造
    for _ in 0..length {
      let c: &u8 = chars.choose(&mut rng).expect("chars wont empty!");
      password.push(*c as char);
    }

    Ok(())
}
