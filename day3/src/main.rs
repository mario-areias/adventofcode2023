use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // run from root folder
    let f = lib::open_file("day3/input.txt".to_string()).unwrap();
    let lines = lib::read_chars_from_file(f).unwrap();

    let mut hash_map_numbers: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    let mut set_symbols: HashSet<(usize, usize)> = HashSet::new();
    let mut set_stars: HashSet<(usize, usize)> = HashSet::new();

    for (i, line) in lines.iter().enumerate() {
        let mut key: Option<(usize, usize)> = None;
        let mut value: Vec<char> = vec![];

        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                if key.is_none() {
                    key = Some((i, j));
                    value = vec![*c];
                } else {
                    value.push(*c);
                }
                continue;
            }

            if key.is_some() {
                hash_map_numbers.insert(key.unwrap(), value);
                key = None;
                value = vec![];
            }

            if *c != '.' {
                set_symbols.insert((i, j));
            }

            if *c == '*' {
                set_stars.insert((i, j));
            }
        }

        match key.is_some() {
            true => {
                hash_map_numbers.insert(key.unwrap(), value);
            }
            false => (),
        }
    }

    // println!("hash_map_numbers: {:?}", hash_map_numbers);
    // println!("set_symbols: {:?}", set_symbols);

    let mut total: usize = 0;
    for (key, value) in hash_map_numbers.iter() {
        let vlen = value.len();
        let position = key;
        let i = position.0;

        let n = value.iter().collect::<String>().parse::<usize>().unwrap();
        println!("value: {}", n);

        for j in position.1..position.1 + vlen {
            let i = i as i32;
            let j = j as i32;

            let positions = [
                (i - 1, j - 1), // upper left
                (i - 1, j),     // upper
                (i - 1, j + 1), // upper right
                (i, j - 1),     // left
                (i, j + 1),     // right
                (i + 1, j - 1), // lower left
                (i + 1, j),     // lower
                (i + 1, j + 1), // lower right
            ];

            let mut found = false;
            for position in positions.iter() {
                if check(&set_symbols, position.0, position.1) {
                    println!("Found: {}", n);
                    total += n;
                    found = true;
                }
            }

            if found {
                break;
            }
        }
    }
    println!("Total: {}", total);

    // part 2
    // println!("set_stars: {:?}", set_stars);

    let mut total_2 = 0;
    for k in set_stars.iter() {
        let i = k.0 as i32;
        let j = k.1 as i32;

        let positions = [
            (i - 1, j - 1), // upper left
            (i - 1, j),     // upper
            (i - 1, j + 1), // upper right
            (i, j - 1),     // left
            (i, j + 1),     // right
            (i + 1, j - 1), // lower left
            (i + 1, j),     // lower
            (i + 1, j + 1), // lower right
        ];

        let mut found = HashSet::new();
        let mut t = 1;
        for position in positions.iter() {
            if let Some(chars) = check_number(&hash_map_numbers, position.0, position.1) {
                let n = chars.iter().collect::<String>().parse::<usize>().unwrap();
                if found.insert(n) {
                    t *= n;
                }
            }
        }

        if found.len() == 2 {
            total_2 += t;
        }
    }

    println!("Total2: {}", total_2);
}

fn check_number(
    hash_numbers: &HashMap<(usize, usize), Vec<char>>,
    i: i32,
    j: i32,
) -> Option<&Vec<char>> {
    if i < 0 || j < 0 {
        return None;
    }

    let i = i as usize;
    let j = j as usize;

    // println!("i: {}, j: {}", i, j);
    for (key, value) in hash_numbers.iter() {
        if key.0 == i && (j >= key.1 && j < key.1 + value.len()) {
            return Some(value);
        }
    }

    None
}

fn check(set: &HashSet<(usize, usize)>, i: i32, j: i32) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    set.contains(&(i as usize, j as usize))
}
