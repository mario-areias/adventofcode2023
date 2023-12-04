use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    // run from root folder
    let f = lib::open_file("day3/test.txt".to_string()).unwrap();
    let lines = lib::read_chars_from_file(f).unwrap();
    let mut hash_map_numbers: HashMap<(usize, usize), Vec<char>> = HashMap::new();
    let mut set_symbols: HashSet<(usize, usize)> = HashSet::new();

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
        }

        match key.is_some() {
            true => {
                hash_map_numbers.insert(key.unwrap(), value);
            }
            false => (),
        }
    }

    println!("hash_map_numbers: {:?}", hash_map_numbers);
    println!("set_symbols: {:?}", set_symbols);

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
}

fn check(set: &HashSet<(usize, usize)>, i: i32, j: i32) -> bool {
    if i < 0 || j < 0 {
        return false;
    }

    set.contains(&(i as usize, j as usize))
}

// fn main() {
//     again();
//     // run from root folder
//     let f = lib::open_file("day3/input.txt".to_string()).unwrap();
//     let lines = lib::read_chars_from_file(f).unwrap();
//
//     let mut total: usize = 0;
//     let mut total2: usize = 0;
//     let clines = lines.clone();
//
//     let mut ni: Option<usize> = None;
//     let mut nj: Option<usize> = None;
//
//     let mut number: Vec<char> = vec![];
//     for (i, line) in lines.iter().enumerate() {
//         if i > 0 && process(clines.clone(), ni, nj, i - 1) {
//             let n = number.iter().collect::<String>().parse::<usize>().unwrap();
//             total += n;
//         }
//         ni = None;
//         nj = None;
//
//         number = vec![];
//         for (j, c) in line.iter().enumerate() {
//             if c.is_ascii_digit() {
//                 number.push(*c);
//
//                 if ni.is_none() {
//                     ni = Some(j);
//                     nj = Some(j);
//                 } else {
//                     nj = Some(j);
//                 }
//             } else {
//                 if *c == '*' {
//                     let n = search_numbers(&clines, i, j);
//                     total2 += n;
//                 }
//                 if process(clines.clone(), ni, nj, i) {
//                     let n = number.iter().collect::<String>().parse::<usize>().unwrap();
//                     total += n;
//                 }
//                 number = vec![];
//                 ni = None;
//                 nj = None;
//             }
//         }
//     }
//
//     println!("Total: {}", total);
//     println!("Total2: {}", total2);
// }
//
// fn process(clines: Vec<Vec<char>>, ni: Option<usize>, nj: Option<usize>, i: usize) -> bool {
//     if ni.is_none() && nj.is_none() {
//         return false;
//     }
//
//     search_symbols(&clines, i, ni.unwrap(), nj.unwrap())
// }
//
// fn search_numbers(lines: &Vec<Vec<char>>, i: usize, si: usize) -> usize {
//     // converting to i32 to avoid overflow
//     let i = i as i32;
//     let si = si as i32;
//
//     // left of si
//     let left = (i, si - 1);
//     let upper_left = (i - 1, si - 1);
//     let bottom_left = (i + 1, si - 1);
//
//     // right of si
//     let right = (i, si + 1);
//     let upper_right = (i - 1, si + 1);
//     let bottom_right = (i + 1, si + 1);
//
//     // upper and bottom of si
//     let upper = (i - 1, si);
//     let bottom = (i + 1, si);
//
//     let positions = vec![
//         left,
//         upper_left,
//         bottom_left,
//         right,
//         upper_right,
//         bottom_right,
//         upper,
//         bottom,
//     ];
//
//     let mut number_coords = vec![];
//     for (i, j) in positions {
//         if is_digit(lines, i, j) {
//             // ignore consecutive numbers
//             if !number_coords.contains(&(i, j - 1))
//                 && !number_coords.contains(&(i, j - 2))
//                 && !number_coords.contains(&(i, j + 1))
//                 && !number_coords.contains(&(i, j + 2))
//             {
//                 number_coords.push((i, j));
//             }
//         }
//     }
//
//     if number_coords.len() != 2 {
//         println!("Error: {:?}", number_coords);
//         return 0;
//     }
//
//     let mut total = 1;
//     for (i, j) in number_coords {
//         let number = get_number(lines, i, j);
//         total *= number;
//     }
//     total
// }
//
// fn get_number(lines: &[Vec<char>], i: i32, j: i32) -> usize {
//     let i = i as usize;
//     let j = j as usize;
//
//     let line = lines.get(i).unwrap();
//     let mut chars = vec![];
//     let mut first_chars = vec![];
//
//     let mut after = j;
//     while let Some(c) = line.get(after) {
//         if c.is_ascii_digit() {
//             chars.push(*c);
//         } else {
//             break;
//         }
//
//         after += 1;
//     }
//
//     if j == 0 {
//         return chars.iter().collect::<String>().parse::<usize>().unwrap();
//     }
//
//     let mut before = j - 1;
//     while let Some(c) = line.get(before) {
//         if c.is_ascii_digit() {
//             first_chars.push(*c);
//         } else {
//             break;
//         }
//
//         if before == 0 {
//             break;
//         }
//
//         before -= 1;
//     }
//
//     first_chars.reverse();
//     let number = [first_chars, chars]
//         .iter()
//         .flatten()
//         .collect::<String>()
//         .parse::<usize>()
//         .unwrap();
//
//     number
// }
//
// fn search_symbols(lines: &Vec<Vec<char>>, i: usize, ni: usize, nj: usize) -> bool {
//     // converting to i32 to avoid overflow
//     let i = i as i32;
//     let mut ni = ni as i32;
//     let nj = nj as i32;
//
//     // left of ni
//     let left = (i, ni - 1);
//     let upper_left = (i - 1, ni - 1);
//     let bottom_left = (i + 1, ni - 1);
//
//     // right of nj
//     let right = (i, nj + 1);
//     let upper_right = (i - 1, nj + 1);
//     let bottom_right = (i + 1, nj + 1);
//
//     // upper and bottom of ni
//     let upper = (i - 1, ni);
//     let bottom = (i + 1, ni);
//
//     let mut positions = vec![
//         left,
//         upper_left,
//         bottom_left,
//         right,
//         upper_right,
//         bottom_right,
//         upper,
//         bottom,
//     ];
//
//     // only the upper and bottom from the first (ni) to the last (nj)
//     ni += 1;
//     while ni <= nj {
//         let upper = (i - 1, ni);
//         let bottom = (i + 1, ni);
//
//         positions.push(upper);
//         positions.push(bottom);
//         ni += 1;
//     }
//
//     for (i, j) in positions {
//         if is_symbol(lines, i, j) {
//             return true;
//         }
//     }
//
//     false
// }
//
// fn is_digit(lines: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
//     if i < 0 || j < 0 {
//         return false;
//     }
//
//     let i = i as usize;
//     let j = j as usize;
//
//     if i >= lines.len() || j >= lines[i].len() {
//         return false;
//     }
//
//     let c = lines[i][j];
//     if c.is_ascii_digit() {
//         return true;
//     }
//
//     false
// }
//
// fn is_symbol(lines: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
//     if i < 0 || j < 0 {
//         return false;
//     }
//
//     let i = i as usize;
//     let j = j as usize;
//
//     if i >= lines.len() || j >= lines[i].len() {
//         return false;
//     }
//
//     let c = lines[i][j];
//     if c != '.' && !c.is_ascii_digit() {
//         return true;
//     }
//
//     false
// }
