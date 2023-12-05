use std::collections::HashMap;

fn main() {
    let f = lib::open_file("day5/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut seeds = vec![];
    let mut source_to_location = HashMap::new();

    for line in lines {
        if line.contains("seeds:") {
            seeds = line
                .split(':')
                .filter(|s| s != &"seeds:")
                .map(|s| s.trim())
                .flat_map(|s| s.split(' '))
                .map(|s| s.parse::<usize>())
                .filter_map(Result::ok)
                .collect();

            println!("seeds init: {:?}", seeds);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if line.contains("map:") {
            seeds = process(&seeds, &source_to_location);
            println!("seeds: {:?}", seeds);
            source_to_location = HashMap::new();
            continue;
        }

        let parts = line
            .split(' ')
            .map(|s| s.parse::<usize>())
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let destination_range = parts[0];
        let source_range = parts[1];
        let range = parts[2];

        let dest_range = (destination_range, range);

        source_to_location.insert(source_range, dest_range);
    }

    seeds = process(&seeds, &source_to_location);
    println!("seeds: {:?}", seeds);

    let min = seeds.iter().min().unwrap();
    println!("min: {}", min);
}

fn process(seeds: &Vec<usize>, source_to_location: &HashMap<usize, (usize, usize)>) -> Vec<usize> {
    let mut new_seeds = vec![];

    for seed in seeds {
        let new_seed = get(source_to_location, *seed);
        new_seeds.push(new_seed);
    }

    new_seeds
}

fn get(map: &HashMap<usize, (usize, usize)>, seed: usize) -> usize {
    for (source_range, (destination_range, range)) in map.iter() {
        if seed >= *source_range && seed < (source_range + range) {
            let diff = seed - source_range;
            let new_seed = destination_range + diff;
            return new_seed;
        }
    }

    seed
}

// seeds: 79 14 55 13
//
// seeds-to-soil
// 50 98 2
// 52 50 48
//
// ######### Seeds to soil lines #########3
//
// destination range: 50
// source range: 98
// range: 2
//
// source range: 98-99
// destination range: 50-51
//
// seeds to soil
// 98-99 -> 50-51
//
// destination range: 52
// source range: 50
// range: 48
//
// source range: 50-97
// destination range: 52-99
//
// seeds to soil
// 50-97 -> 52-99
// source + range - 1 -> destination + range - 1
//
// 79 -> 81
// 14 -> 14
// 55 -> 57
// 13 -> 13
//
//
// ######### soil to fertilizer lines #########3
// soil-to-fertilizer map:
// 0 15 37
// 37 52 2
// 39 0 15
//
// destination range: 0
// source range: 15
// range: 37
//
// source range: 15-51
// destination range: 0-36
//
// soil to fertilizer
// 15-51 -> 0-36
//
// destination range: 37
// source range: 52
// range: 2
//
// source range: 52-53
// destination range: 37-38
//
// soil to fertilizer
// 52-53 -> 37-38
//
// destination range: 39
// source range: 0
// range: 15
//
// source range: 0-14
// destination range: 39-53
//
// soil to fertilizer
// 0-14 -> 39-53
//
// 81 -> 81
// 14 -> 53
// 57 -> 57
// 13 -> 52
