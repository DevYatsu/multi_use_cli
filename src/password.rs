use std::fs::{File, OpenOptions};

const PASSWORD_KEY: &str = "MUCLI_PASSWORD";
const CONFIG_FILE: &str = "config.txt";

use std::io::{prelude::*, Error, ErrorKind, SeekFrom, Write};

pub fn set_password(password: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CONFIG_FILE)?;

    let new_line = format!("{}={}", PASSWORD_KEY, password);

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    file.seek(SeekFrom::Start(0))?; // Move the cursor to the beginning of the file

    let filtered_lines: Vec<_> = buffer
        .lines()
        .filter(|line| !line.starts_with(&format!("{}{}", PASSWORD_KEY, "=")))
        .collect();

    file.set_len(0)?; // Truncate the file

    writeln!(file, "{}", new_line)?;

    for line in filtered_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

pub fn get_password() -> Result<String, Error> {
    let mut file: File = File::open(CONFIG_FILE)?;
    let mut buffer: String = String::new();

    file.read_to_string(&mut buffer)?;

    for line in buffer.split("\n") {
        if line.starts_with(&format!("{}{}", PASSWORD_KEY, "=")) {
            return Ok(line[PASSWORD_KEY.len() + 1..].to_string());
        }
    }
    Err(Error::new(ErrorKind::Other, "No password found in file!"))
}