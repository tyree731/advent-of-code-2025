use std::io;

pub fn read_stdin_until_eof() -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    for line in io::stdin().lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(e) => return Err(e.to_string())
        }
    }
    Ok(lines)
}
