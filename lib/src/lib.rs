use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn open_file(input: String) -> Result<File, String> {
    let f = File::open(input);
    match f {
        Ok(file) => Ok(file),
        Err(error) => Err(error.to_string()),
    }
}

pub fn read_lines_from_file(file: File) -> Result<Vec<String>, String> {
    let mut lines = Vec::new();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(lines)
}

pub fn read_chars_from_file(file: File) -> Result<Vec<Vec<char>>, String> {
    let mut lines = Vec::new();
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        match line {
            Ok(line) => lines.push(line.chars().collect::<Vec<char>>()),
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
