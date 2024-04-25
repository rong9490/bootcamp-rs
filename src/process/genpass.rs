use rand::{seq::SliceRandom, thread_rng};

// HACK 大概考虑到 分离具体的调用执行, 放到 process 里面

// 具体的调用执行内容

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = thread_rng(); // 先创建操作的进程
    let mut password = Vec::new(); // 密码容器
    let mut chars = Vec::new(); // 字符集容器

    // 合并字符集, 并保证至少有一位
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("None"));
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("None"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("None"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("None"));
    }

    // 开始构造密码
    for _ in 0..(length - password.len() as u8) {
      let c: &u8 = chars.choose(&mut rng).expect("Not Empty!");
      password.push(*c);
    }

    password.shuffle(&mut rng);
    let password: String = String::from_utf8(password)?;
    Ok(password)
}
