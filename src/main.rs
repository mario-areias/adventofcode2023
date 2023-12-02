use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const MAX_NUMBER_LENGTH: usize = 5;

fn main() {
    let f = open_file("input.txt".to_string()).unwrap();
    let lines = read_lines_from_file(f).unwrap();

    let mut first: u32;
    let mut last: u32;

    let mut i: usize;
    let mut total = 0;

    for line in lines {
        // resetting values per line
        i = 0;
        first = 0;
        last = 0;

        let chars = line.chars().collect::<Vec<char>>();
        for j in 0..=chars.len() {
            let diff = j - i;

            if diff > MAX_NUMBER_LENGTH {
                i += 1;
            }

            let mut z = 0;
            while i + z < j {
                let word = &chars.as_slice()[i + z..j].iter().collect::<String>();
                let n = str_to_int(word.to_string());
                if n > 0 {
                    if first > 0 {
                        last = n;
                    } else {
                        first = n;
                    }
                    break;
                }
                z += 1;
            }
        }

        let t = calc(first, last);
        total += t;
    }

    println!("Total: {}", total);
}

fn calc(f: u32, l: u32) -> u32 {
    let mut t = 0;

    if f > 0 {
        t += f * 10
    } else {
        t += l * 10;
    }

    if l > 0 {
        t += l
    } else {
        t += f;
    }

    t
}

fn str_to_int(s: String) -> u32 {
    match s {
        s if s == "one" || s == "1" => 1,
        s if s == "two" || s == "2" => 2,
        s if s == "three" || s == "3" => 3,
        s if s == "four" || s == "4" => 4,
        s if s == "five" || s == "5" => 5,
        s if s == "six" || s == "6" => 6,
        s if s == "seven" || s == "7" => 7,
        s if s == "eight" || s == "8" => 8,
        s if s == "nine" || s == "9" => 9,
        _ => 0,
    }
}

fn open_file(input: String) -> Result<File, String> {
    let f = File::open(input);
    match f {
        Ok(file) => Ok(file),
        Err(error) => Err(error.to_string()),
    }
}

fn read_lines_from_file(file: File) -> Result<Vec<String>, String> {
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
