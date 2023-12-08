use std::collections::HashMap;

pub enum Directions {
    Left,
    Right,
}

impl Directions {
    fn from_str(s: &str) -> Vec<Directions> {
        s.chars()
            .map(Directions::from_char)
            .collect::<Vec<Directions>>()
    }

    fn from_char(c: char) -> Directions {
        match c {
            'L' => Directions::Left,
            'R' => Directions::Right,
            _ => panic!("Invalid direction"),
        }
    }

    fn value(&self) -> i32 {
        match self {
            Directions::Left => 0,
            Directions::Right => 1,
        }
    }
}
fn main() {
    let f = lib::open_file("day8/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let directions = Directions::from_str(&lines[0]);

    let mut current_keys = Vec::new();
    let mut coordinates = HashMap::new();

    for line in lines.iter().skip(1) {
        if line.is_empty() {
            continue;
        }

        let mut parts = line.split('=');
        let key = parts.next().unwrap().trim();
        if key.ends_with('A') {
            current_keys.push(key);
        }

        let values = parts.next().unwrap().trim();
        // removing parenthesis
        let v = &values[1..values.len() - 1];

        let mut vparts = v.split(',');
        coordinates.insert(
            key.to_string(),
            (vparts.next().unwrap().trim(), vparts.next().unwrap().trim()),
        );
    }

    println!("{:?}", current_keys);

    let mut results = vec![];
    for key in current_keys.iter() {
        let c = count(key, &directions, &coordinates);
        results.push(c);
    }

    println!("{:?}", results);
    println!("{:?}", lcm(results));
}

fn lcm(numbers: Vec<usize>) -> usize {
    let mut result = numbers[0];
    for i in 1..numbers.len() {
        result = (result * numbers[i]) / gcd(result, numbers[i]);
    }
    result
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn count(
    key: &str,
    directions: &Vec<Directions>,
    coordinates: &HashMap<String, (&str, &str)>,
) -> usize {
    let mut i = 0;
    let mut count = 0;
    let mut current_key = key;
    while i < directions.len() {
        let index = &directions[i].value();
        i += 1;
        count += 1;

        current_key = get_next(*index, current_key, coordinates);

        if current_key.ends_with('Z') {
            break;
        }

        if i == directions.len() {
            i = 0;
        }
    }

    count
}

fn get_next<'a>(
    index: i32,
    current: &'a str,
    coordinates: &HashMap<String, (&'a str, &'a str)>,
) -> &'a str {
    let current_value = coordinates.get(current).unwrap();

    if index == 0 {
        current_value.0
    } else {
        current_value.1
    }
}
