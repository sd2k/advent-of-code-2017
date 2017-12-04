use std::fs::File;
use std::io::{Read, Result};

pub fn get_file_to_string(f: &str) -> Result<String> {
    let mut o = String::new();
    File::open(f)?.read_to_string(&mut o)?;
    Ok(o.trim().to_owned())
}
