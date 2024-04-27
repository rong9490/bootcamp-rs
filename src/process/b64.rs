use base64::{alphabet::URL_SAFE, prelude::*};

pub fn process_encode(input: &str) -> anyhow::Result<()> {
  let encoded = URL_SAFE.encode(input);
  println!("encoded:: {}", encoded);
  Ok(())
}

pub fn process_decode(input: &str) -> anyhow::Result<()> {
  // let decoded = *URL_SAFE()
  Ok(())
}