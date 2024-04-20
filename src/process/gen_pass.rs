use rand::seq::SliceRandom;

// 基础字符集
const UPPER: &[u8] = b"ABCDEFG";
const LOWER: &[u8] = b"abcdefg";
const NUMBER: &[u8] = b"1234567";
const SYMBOL: &[u8] = b"@#$%^:|&";

// 不要直接传cli的参数, 尽量解耦隔离
pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng(); // 线程
    let mut password = Vec::new(); // 密码容器
    let mut chars = Vec::new(); // 字符集容器

    /* 添加字符集 */
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("Empty!"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("Empty!"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("Empty!"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("Empty!"));
    }

    for _ in 0..(length - password.len() as u8) {
        // 取出一个字符
        let c = chars.choose(&mut rng).expect("Empty!!");
        password.push(*c);
    }

    // 乱序洗牌
    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
