use std::collections::{HashMap, HashSet};

fn main() {
    let f = lib::open_file("day4/test.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut cards_score = 0;
    let mut copies_map: HashMap<usize, usize> = HashMap::new();

    let len = lines.len();

    for (i, line) in lines.iter().enumerate() {
        let card_number = i + 1;
        let copies = *copies_map.entry(card_number).or_insert(1);
        println!("Card: {} Copies: {}", card_number, copies);

        let mut parts = line.split(':');
        // discard cardnumber
        parts.next();

        // get numbers
        let numbers = parts.next().unwrap();
        let mut numbers = numbers.split('|');

        let winning = numbers.next().unwrap();
        let my_numbers = numbers.next().unwrap();

        let winning_numbers: HashSet<&str> = HashSet::from_iter(winning.trim().split(' '));

        let mut score = 0;
        let mut matches = 0;
        for number in my_numbers.trim().split(' ') {
            if winning_numbers.contains(number) {
                if number.is_empty() {
                    continue;
                }
                if score == 0 {
                    score += 1;
                } else {
                    score *= 2;
                }

                matches += 1;
            }
        }

        println!("Matches: {}", matches);

        cards_score += score;

        for j in 1..=matches {
            let n = card_number + j;
            if n == len {
                break;
            }

            copies_map
                .entry(card_number + j)
                .and_modify(|e| *e += copies)
                .or_insert(2);
        }

        println!("copies_map: {:?}", copies_map);
    }

    println!("Score Total: {}", cards_score);
    let total = copies_map.values().sum::<usize>();
    println!("Total: {}", total);
}
