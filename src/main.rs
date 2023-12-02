use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let f = open_file("input.txt".to_string()).unwrap();
    let lines = read_lines_from_file(f).unwrap();

    let mut ic: char;
    let mut ec: char;

    let mut ii: u32;
    let mut ei: u32;

    let mut i: usize;

    let mut total = 0;
    let mut foundi: bool;
    let mut founde: bool;

    let numbers: Vec<String> = vec![
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
        "5".to_string(),
        "6".to_string(),
        "7".to_string(),
        "8".to_string(),
        "9".to_string(),
    ];

    for line in lines {
        ic = 'a';
        ec = 'a';

        foundi = false;
        founde = false;

        i = 0;

        let chars = line.chars().collect::<Vec<char>>();

        for j in 0..=chars.len() {
            let diff = j - i;

            if diff > 5 {
                i += 1;
            }

            let mut z = 0;

            while i + z < j {
                let word = &chars.as_slice()[i + z..j].iter().collect::<String>();
                if compare(word.to_string(), numbers.clone()) {
                    let c = str_to_int(word.to_string());
                    if foundi {
                        founde = true;
                        ec = c;
                    } else {
                        foundi = true;
                        ic = c;
                    }
                    break;
                }
                z += 1;
            }
        }

        let t = calc(ic, ec, foundi, founde);

        // println!("t: {}", t);
        total += t;
    }

    println!("Total: {}", total);
}

fn calc(i: char, e: char, fi: bool, fe: bool) -> u32 {
    let mut t = 0;
    if fi && fe {
        let ni = i.to_digit(10).unwrap();
        let ne = e.to_digit(10).unwrap();

        if ni > 0 {
            t += ni * 10
        }

        if ne > 0 {
            t += ne
        }
    }

    if fi && !fe {
        let ni = i.to_digit(10).unwrap();
        t += (ni * 10) + ni;
    }

    if !fi && fe {
        let nj = e.to_digit(10).unwrap();
        t += (nj * 10) + nj;
    }

    t
}

fn str_to_int(s: String) -> char {
    match s {
        s if s == "one" => '1',
        s if s == "two" => '2',
        s if s == "three" => '3',
        s if s == "four" => '4',
        s if s == "five" => '5',
        s if s == "six" => '6',
        s if s == "seven" => '7',
        s if s == "eight" => '8',
        s if s == "nine" => '9',
        s if s == "1" => '1',
        s if s == "2" => '2',
        s if s == "3" => '3',
        s if s == "4" => '4',
        s if s == "5" => '5',
        s if s == "6" => '6',
        s if s == "7" => '7',
        s if s == "8" => '8',
        s if s == "9" => '9',
        _ => '0',
    }
}

fn compare(word: String, numbers: Vec<String>) -> bool {
    let mut found = false;
    for n in numbers {
        if word == n {
            found = true;
            break;
        }
    }
    found
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
