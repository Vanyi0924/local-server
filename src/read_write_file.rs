use std::io::{BufRead, BufReader, Read, Write};

pub fn main() -> std::io::Result<()> {
  let file = File::open("mao.webp")?;
  let mut output = File::create("mao11.webp")?;
  let mut reader = BufReader::new(file);
  let mut buffer = Vec::new();
  reader.read_to_end(&mut buffer)?;
  output.write(&buffer)?;
  Ok(())
}