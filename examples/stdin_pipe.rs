use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut reader, format) = niffler::from_path("-")?;
    println!("Format detected: {:?}", format);

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    println!("Decompressed content: {}", buffer);

    Ok(())
}
