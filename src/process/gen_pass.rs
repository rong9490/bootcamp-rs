use rand::seq::SliceRandom;

const UPPER: &[u8] = b"ABCDE";
const LOWER: &[u8] = b"abcde";
const NUMBER: &[u8] = b"01234";
const SYMBOL: &[u8] = b"@#$%^";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    // 使用字符集
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("None"))
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("None"))
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("None"))
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("None"))
    }

    // 开始构造
    for _ in 0..(length - password.len() as u8) {
        let c: &u8 = chars.choose(&mut rng).expect("chars wont empty!");
        password.push(*c);
    }

    // 乱序
    password.shuffle(&mut rng);

    println!("{}", String::from_utf8(password)?);

    Ok(())
}
