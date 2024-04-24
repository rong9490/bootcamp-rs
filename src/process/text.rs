// 具体执行 sign 这个命令的方法

use blake3;
use ed25519_dalek;

use crate::TextSignFormat;


pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<{}> {
  let mut reader = get_reader(input)?; // 初始化读取器
  let mut buf = Vec::new(); // 将读取到数据放入buffer
  reader.read_to_end(&mut buf)?;
  let signed = match format {
    TextSignFormat::Blake3 => blake3::hase(&buf).to_hex(),
    TextSignFormat::Ed25519 => {
      let key = ed25519_dalek::SecretKey::from_bytes(&hex::decode(key)?)?;
      
    },
  };
}
