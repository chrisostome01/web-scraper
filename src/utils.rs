use std::fs::File;
use std::io::Write;
use std::error::Error;

pub fn save_to_file(filename: &str, content: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
