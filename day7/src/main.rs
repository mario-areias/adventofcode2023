use day7::Hand;

fn main() {
    let f = lib::open_file("day7/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut hands = Vec::new();
    for line in lines {
        let mut iter = line.split_whitespace();
        let hand = Hand::from(iter.next().unwrap()).unwrap();
        let bid = iter.next().unwrap().parse::<usize>().unwrap();

        hands.push((hand, bid));
    }

    hands.sort();
    println!("{:?}", hands.len());

    let mut total = 0;
    for (i, v) in hands.iter().enumerate() {
        total += v.1 * (i + 1);
    }

    println!("{}", total);
}

// AAAAA -> five of a kind
// 33332 -> four of a kind
// 23332 -> full house
// TTT98 -> three of a kind
// 23432 -> two pairs
// A23A4 -> one pair
// 23456 -> distinct
