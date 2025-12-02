use std::io;
use std::io::Read;

pub fn read_stdin_lines_until_eof() -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    for line in io::stdin().lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(lines)
}

pub fn read_stdin_until_eof() -> Result<String, String> {
    let mut buf = String::new();
    match io::stdin().read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e.to_string()),
    }
}
