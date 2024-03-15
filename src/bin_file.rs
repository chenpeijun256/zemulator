use std::fs;

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let byte_content = fs::read(path)?;
    Ok(byte_content)
}