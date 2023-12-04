use std::collections::{HashMap, HashSet};

fn main() {
    let f = lib::open_file("day4/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut cards_score = 0;
    let mut cards_map: HashMap<usize, usize> = HashMap::new();

    for (i, line) in lines.iter().enumerate() {
        let card_number = i + 1;
        let copies = *cards_map.entry(card_number).or_insert(1);

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

        println!("Card number: {}. Matches: {}", card_number, matches);

        cards_score += score;

        for j in 1..=matches {
            let n = card_number + j;
            cards_map
                .entry(n)
                .and_modify(|e| *e += copies)
                .or_insert(1 + copies);
        }

        print_map(&cards_map);
    }

    println!("Score Total: {}", cards_score);
    let total = cards_map.values().sum::<usize>();
    println!("Total: {}", total);
}

fn print_map(map: &HashMap<usize, usize>) {
    let mut sorted_keys = map.keys().collect::<Vec<&usize>>();
    sorted_keys.sort();

    print!("cards_map: {{ ");
    for key in sorted_keys {
        print!("{}:{}, ", key, map[key]);
    }
    println!(" }}");
}
