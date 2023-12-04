use std::collections::HashSet;

fn main() {
    let f = lib::open_file("day4/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut cards_score = 0;
    for line in lines {
        // println!("Line: {}", line);
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
            }
        }

        cards_score += score;
    }

    println!("Score Total: {}", cards_score);
}
