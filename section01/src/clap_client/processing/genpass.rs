use anyhow;
use rand::seq::SliceRandom; // 必须, 为何

const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn csv_process(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    println!("csv_process!!!!");

    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }
    if lowercase {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    Ok(String::from_utf8(password)?)
}
