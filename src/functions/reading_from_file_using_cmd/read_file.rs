use super::read_from_cmd::Config;
use std::error::Error;
use std::fs;
pub fn readfile(files: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(files.file.clone())?;
    println!("file content: {:?}", content);
    Ok(())
}
