use std::collections::HashMap;

fn main() {
    let f = lib::open_file("day5/input.txt".to_string()).unwrap();
    let lines = lib::read_lines_from_file(f).unwrap();

    let mut seeds = vec![];
    let mut seeds_part_2 = vec![];

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

            // count how many elements we need to have in seeds_part_2
            let mut count = 0;
            for i in (0..seeds.len()).step_by(2) {
                let range = seeds[i + 1];
                count += range;
            }

            // actually assing new elements to seeds_part_2
            seeds_part_2 = Vec::with_capacity(count);
            for i in (0..seeds.len()).step_by(2) {
                let seed = seeds[i];
                let range = seeds[i + 1];

                for j in seed..(seed + range) {
                    seeds_part_2.push(j);
                }
            }
            println!("len seeds_part_2: {}", seeds_part_2.len());
            // println!("seeds init: {:?}", seeds);
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if line.contains("map:") {
            seeds = process(&seeds, &source_to_location);
            seeds_part_2 = process(&seeds_part_2, &source_to_location);
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
    seeds_part_2 = process(&seeds_part_2, &source_to_location);

    // println!("seeds: {:?}", seeds);
    // println!("seeds2: {:?}", seeds_part_2);

    let min = seeds.iter().min().unwrap();
    let min_2 = seeds_part_2.iter().min().unwrap();

    println!("min: {}", min);
    println!("min2: {}", min_2);
}

fn process(seeds: &Vec<usize>, source_to_location: &HashMap<usize, (usize, usize)>) -> Vec<usize> {
    let mut new_seeds = Vec::with_capacity(seeds.len());

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
