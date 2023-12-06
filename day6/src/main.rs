fn main() {
    let f = lib::open_file("day6/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let time: Vec<usize> = lines[0]
        .split(' ')
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    let distance: Vec<usize> = lines[1]
        .split(' ')
        .map(|s| s.trim())
        .map(|s| s.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    let mut total = 1;

    for i in 0..time.len() {
        let t = time[i];
        let d = distance[i];
        let mut single_total = 0;

        for j in 1..t {
            let r = (t - j) * j;
            if r > d {
                single_total += 1;
            }
        }

        println!("single_total: {}", single_total);
        total *= single_total;
    }

    println!("total: {}", total);

    // Part 2
    let mut parts = lines[0].split(':');
    parts.next();

    let time2: usize = parts
        .next()
        .unwrap()
        .split(' ')
        .flat_map(|s| s.trim().chars())
        .collect::<String>()
        .parse()
        .unwrap();

    let mut parts = lines[1].split(':');
    parts.next();

    let distance2: usize = parts
        .next()
        .unwrap()
        .split(' ')
        .flat_map(|s| s.trim().chars())
        .collect::<String>()
        .parse()
        .unwrap();

    let mut total_2 = 0;
    for i in 1..time2 {
        let r = (time2 - i) * i;
        if r > distance2 {
            total_2 += 1;
        }
    }

    println!("total_2: {}", total_2);
}

// x = 7ms
// y = 9mm
//
// 0ms -> 0mm
// 1ms -> 6mm
// 2ms -> 10mm
// 3ms -> 12mm
// 4ms -> 12mm
// 5ms -> 10mm
// 6ms -> 6mm
// 7ms -> 0mm
//
// r = (x - b) * (b)
// r = (7 - 3) * (3)
